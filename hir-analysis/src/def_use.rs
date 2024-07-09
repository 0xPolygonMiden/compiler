use cranelift_entity::SecondaryMap;
use intrusive_collections::{intrusive_adapter, LinkedListLink};
use midenc_hir::{Block, BranchInfo, DataFlowGraph, Inst, Instruction, Value, ValueData};

use crate::DominatorTree;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ValueDef {
    /// This corresponds to the original definition of a value as an instruction result
    Inst {
        /// The result index
        num: u16,
        /// The defining instruction
        inst: Inst,
    },
    /// This corresponds to the original definition of a value as a block parameter
    Param {
        /// The parameter index
        num: u16,
        /// The defining block
        block: Block,
    },
    /// This represents a reload of a spilled value
    Reload {
        /// The reload instruction
        inst: Inst,
    },
    /// This definition corresponds to an implicit phi node in the dataflow graph.
    ///
    /// Specifically, this is like `Param`, but for a block parameter that doesn't yet exist,
    /// but _must_ be created in order for SSA form to be preserved.
    Phi {
        /// The block in which this phi must be materialized as a block parameter
        block: Block,
    },
}

/// The def-use graph provides us with three useful pieces of information:
///
/// * The set of users for any value in a function
/// * Whether a value is ever used
/// * The nearest dominating definition for a specific use of a value, taking into account spills
///
/// It is computed by visiting the reachable blocks of a function, and building a graph of all the
/// value definitions and their uses.
#[derive(Default)]
pub struct DefUseGraph {
    values: SecondaryMap<Value, Users>,
}
impl DefUseGraph {
    /// Get the set of users for `value`
    pub fn users(&self, value: Value) -> &Users {
        &self.values[value]
    }

    /// Get a mutable reference to the set of users for `value`
    pub fn users_mut(&mut self, value: Value) -> &mut Users {
        &mut self.values[value]
    }

    /// Returns true if `value` has any reachable uses
    pub fn is_used(&self, value: Value) -> bool {
        !self.values[value].is_empty()
    }

    /// This function will return the nearest definition of `value` which dominates `user`
    /// in the CFG, taking into account the reload pseudo-instruction, such that reloads of
    /// `value` are considered definitions.
    ///
    /// TODO(pauls): Look into treating aliases created via block arguments as valid definitions.
    pub fn nearest_dominating_definition(
        &self,
        user: Inst,
        value: Value,
        dfg: &DataFlowGraph,
        domtree: &DominatorTree,
    ) -> Value {
        // Check if `value` is defined by any instructions in the block containing `user`,
        // between `user` and the block header. If no definition is found, check if the
        // block itself defines the value via its parameter list.
        if let Some(found) = dfg.nearest_definition_in_block(user, value) {
            return found;
        }

        // If we didn't find a definition in the current block, and the current block is
        // in the iterated dominance frontier of the reloads of `value`, then we would
        // expect there to be a block argument introduced in the current block which joins
        // together multiple control-dependent definitions of `value` which dominate the
        // uses in this block.
        //
        // In such cases, we return an error indicating that we have identified a missing phi.
        //
        // The caller must choose how to proceed, but the assumption is that the caller will
        // insert a new block argument, wire up definitions of `value` to that phi, and use
        // the new phi value as the nearest dominating definition.

        // If we reach here, then the containing block has no definition for `value`, so the next
        // place where a valid definition can occur, is in the immediate dominator of `user_block`.
        //
        // The following process is repeated recursively until we reach a definition of `value`:
        //
        // 1. Find the immediate dominator of the current block, and get a cursor to the end of the
        //   instruction list of that block
        // 2. Walk up the block from the end, looking for a valid definition of `value`
        // 3. If we reach the block header, check if `value` is defined as a block paramter in that
        //   block, and if not, go back to step 1.
        //
        // TODO(pauls): If we observe that the value we're interested in, is passed as a block
        // argument to a block that dominates (or contains) `user`, we can treat the block
        // parameter as a valid definition of `value`, and rewrite uses of `value` to use
        // the block parameter instead. This would have the effect of reducing the live
        // range of `value`, reducing operand stack pressure, and potentially removing the
        // need for some spills/reloads.
        let mut current_block = dfg.inst_block(user).unwrap();
        while let Some(idom) = domtree.idom(current_block) {
            current_block = dfg.inst_block(idom).unwrap();

            if let Some(found) = dfg.nearest_definition_in_block(idom, value) {
                return found;
            }
        }

        // If we reach here, then the current block must be the entry block or unreachable, and we
        // did not find the definition in that block. In this case we must raise an
        // assertion, because it should not be possible to reach this point with a valid
        // dataflow graph.
        unreachable!("expected to find a definition for {value}, but no valid definition exists");
    }

    pub fn replace_uses_in(
        &mut self,
        value: Value,
        replacement: Value,
        user: Inst,
        dfg: &mut DataFlowGraph,
    ) {
        // Remove all users of `value` from `user` in the def/use graph
        let mut replacing = UserList::default();
        {
            let mut cursor = self.values[value].cursor_mut();
            while let Some(current_use) = cursor.get() {
                if current_use.inst == user && current_use.value == value {
                    replacing.push_back(cursor.remove().unwrap());
                }
            }
        }

        // Rewrite the dataflow graph to effect the replacements
        for current_use in replacing.iter() {
            match current_use.ty {
                Use::Operand { index } => {
                    let args = dfg.insts[current_use.inst].arguments_mut(&mut dfg.value_lists);
                    args[index as usize] = replacement;
                }
                Use::BlockArgument { succ, index } => match dfg.insts[current_use.inst].as_mut() {
                    Instruction::Br(ref mut b) => {
                        assert_eq!(succ, 0);
                        let args = b.args.as_mut_slice(&mut dfg.value_lists);
                        args[index as usize] = replacement;
                    }
                    Instruction::CondBr(midenc_hir::CondBr {
                        ref mut then_dest,
                        ref mut else_dest,
                        ..
                    }) => {
                        let args = match succ {
                            0 => then_dest.1.as_mut_slice(&mut dfg.value_lists),
                            1 => else_dest.1.as_mut_slice(&mut dfg.value_lists),
                            n => unreachable!(
                                "unexpected successor index {n} for conditional branch"
                            ),
                        };
                        args[index as usize] = replacement;
                    }
                    Instruction::Switch(_) => {
                        unimplemented!("support for switch arms with arguments is not implemented")
                    }
                    _ => unreachable!(),
                },
            }
        }

        // Add new uses of `replacement` to `user` in the def/use graph
        let replacement_users = &mut self.values[replacement];
        for mut current_use in replacing.into_iter() {
            current_use.value = replacement;
            replacement_users.push_back(current_use);
        }
    }

    pub fn compute(dfg: &DataFlowGraph, domtree: &DominatorTree) -> Self {
        // For now, we're interested in computing a def/use graph that only contains definitions
        // and uses actually in the function layout,  and which are reachable from the entry. This
        // means that some valid values which are in the function layout, but in an unreachable
        // block, will be treated as undefined and/or as having no users. A more complete def/use
        // graph can be computed directly from the `DataFlowGraph`, but would then require us to
        // always validate that a given definition (or use) is one we are actually interested in,
        // so in essence we are just pre-filtering the graph.
        let mut graph = Self {
            values: SecondaryMap::with_capacity(dfg.values.len()),
        };
        for block in domtree.cfg_postorder().iter().rev().copied() {
            for arg in dfg.block_args(block) {
                graph.define(*arg);
            }
            for inst in dfg.block_insts(block) {
                match dfg.analyze_branch(inst) {
                    BranchInfo::NotABranch => {
                        for result in dfg.inst_results(inst) {
                            graph.define(*result);
                        }
                        graph.insert_operand_uses(inst, dfg, domtree);
                    }
                    BranchInfo::SingleDest(_, args) => {
                        debug_assert_eq!(
                            dfg.inst_results(inst),
                            &[],
                            "branch instructions cannot have results"
                        );
                        graph.insert_operand_uses(inst, dfg, domtree);
                        for (index, value) in args.iter().copied().enumerate() {
                            debug_assert!(def_dominates_use(value, inst, dfg, domtree));
                            let user = Box::new(User {
                                link: Default::default(),
                                inst,
                                value,
                                ty: Use::BlockArgument {
                                    succ: 0,
                                    index: u16::try_from(index).expect("too many arguments"),
                                },
                            });
                            graph.insert_use(user);
                        }
                    }
                    BranchInfo::MultiDest(ref jts) => {
                        debug_assert_eq!(
                            dfg.inst_results(inst),
                            &[],
                            "branch instructions cannot have results"
                        );
                        graph.insert_operand_uses(inst, dfg, domtree);
                        for (succ, jt) in jts.iter().enumerate() {
                            let succ = u16::try_from(succ).expect("too many successors");
                            for (index, value) in jt.args.iter().copied().enumerate() {
                                debug_assert!(def_dominates_use(value, inst, dfg, domtree));
                                let user = Box::new(User {
                                    link: Default::default(),
                                    inst,
                                    value,
                                    ty: Use::BlockArgument {
                                        succ,
                                        index: u16::try_from(index).expect("too many arguments"),
                                    },
                                });
                                graph.insert_use(user);
                            }
                        }
                    }
                }
            }
        }

        graph
    }

    #[inline]
    fn define(&mut self, value: Value) {
        self.values[value] = Users::default();
    }

    #[inline]
    fn insert_use(&mut self, user: Box<User>) {
        self.values[user.value].push_back(user);
    }

    fn insert_operand_uses(&mut self, inst: Inst, dfg: &DataFlowGraph, domtree: &DominatorTree) {
        for (index, value) in dfg.inst_args(inst).iter().copied().enumerate() {
            debug_assert!(def_dominates_use(value, inst, dfg, domtree));
            let user = Box::new(User {
                link: Default::default(),
                inst,
                value,
                ty: Use::Operand {
                    index: u16::try_from(index).expect("too many arguments"),
                },
            });
            self.insert_use(user);
        }
    }
}

fn def_dominates_use(
    value: Value,
    user: Inst,
    dfg: &DataFlowGraph,
    domtree: &DominatorTree,
) -> bool {
    match dfg.value_data(value) {
        ValueData::Inst { inst, .. } => domtree.dominates(*inst, user, dfg),
        ValueData::Param { block, .. } => {
            if dfg.inst_block(user).unwrap() == *block {
                true
            } else {
                domtree.dominates(*block, user, dfg)
            }
        }
    }
}

intrusive_adapter!(pub UserAdapter = Box<User>: User { link: LinkedListLink });

pub type UserList = intrusive_collections::LinkedList<UserAdapter>;
pub type UserCursorMut<'a> = intrusive_collections::linked_list::CursorMut<'a, UserAdapter>;

#[derive(Default)]
pub struct Users {
    list: intrusive_collections::LinkedList<UserAdapter>,
}
impl Clone for Users {
    fn clone(&self) -> Self {
        Self {
            list: Default::default(),
        }
    }
}
impl Users {
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn push_back(&mut self, user: Box<User>) {
        self.list.push_back(user);
    }

    pub fn pop_front(&mut self) -> Option<Box<User>> {
        self.list.pop_front()
    }

    pub fn iter(&self) -> impl Iterator<Item = &User> + '_ {
        self.list.iter()
    }

    pub fn cursor_mut(&mut self) -> UserCursorMut<'_> {
        self.list.front_mut()
    }

    pub fn take(&mut self) -> Self {
        Self {
            list: self.list.take(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    link: LinkedListLink,
    /// The user is always an instruction
    pub inst: Inst,
    /// The value being used
    pub value: Value,
    /// The type of use
    pub ty: Use,
}
impl User {
    #[inline]
    pub fn new(inst: Inst, value: Value, ty: Use) -> Self {
        Self {
            link: Default::default(),
            inst,
            value,
            ty,
        }
    }
}
impl Eq for User {}
impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.inst == other.inst && self.value == other.value && self.ty == other.ty
    }
}
impl core::hash::Hash for User {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.inst.hash(state);
        self.value.hash(state);
        self.ty.hash(state);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Use {
    /// The value is used as an operand at `index`
    Operand { index: u16 },
    /// The value is used as the block argument at `index`, for successor at index `succ`
    BlockArgument {
        /// The index of the successor, where successor indices are computed based on the
        /// order in which they are referenced by the instruction, i.e. a `cond_br` instruction
        /// always has two successors, the first would have index 0, the second index 1.
        ///
        /// We reference blocks this way, rather than using the block identifier, to handle
        /// cases like the `switch` instruction, where a block can be referenced multiple times,
        /// unlike `cond_br`, which requires that its two successors are unique. In order to
        /// disambiguate block references for `switch`, we use the successor index, as it makes
        /// for convenient access in conjunction with `analyze_branch`.
        succ: u16,
        /// The index of the block argument, when considering the argument list for the given
        /// successor block identified by `succ`
        index: u16,
    },
}
impl Use {
    pub fn is_operand(&self) -> bool {
        matches!(self, Use::Operand { .. })
    }

    pub fn is_block_argument(&self) -> bool {
        matches!(self, Use::BlockArgument { .. })
    }

    pub fn index(&self) -> usize {
        match self {
            Self::Operand { index } | Self::BlockArgument { index, .. } => *index as usize,
        }
    }
}
