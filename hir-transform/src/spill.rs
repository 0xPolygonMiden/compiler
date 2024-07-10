#![allow(clippy::mutable_key_type)]
use std::collections::{BTreeMap, VecDeque};

use midenc_hir::{
    self as hir,
    adt::{SmallMap, SmallSet},
    pass::{AnalysisManager, RewritePass, RewriteResult},
    *,
};
use midenc_hir_analysis::{
    spill::Placement, ControlFlowGraph, DominanceFrontier, DominatorTree, SpillAnalysis, Use, User,
};
use midenc_session::Session;
use rustc_hash::FxHashSet;

/// This pass inserts spills and reloads as computed by running a [SpillAnalysis] on the given
/// function, recording materialized splits, spills, and reloads in the analysis results.
///
/// **IMPORTANT:** This pass is intended to be combined with the [RewriteSpills] pass when used
/// as part of a compilation pipeline - it performs the first phase of a two-phase transformation,
/// and compilation _will_ fail if you forget to apply [RewriteSpills] after this pass when the
/// [SpillAnalysis] directed spills to be injected.
///
/// ## Design
///
/// The full spills transformation is split into an analysis and two rewrite passes, corresponding
/// to the three phases of the transformation:
///
/// 1. Analyze the function to determine if and when to spill values, which values to spill, and
///    where to place reloads.
/// 2. Insert the computed spills and reloads, temporarily breaking the SSA form of the program
/// 3. Reconstruct SSA form, by rewriting uses of spilled values to use the nearest dominating
///    definition, inserting block parameters as needed to ensure that all uses are strictly
///    dominated by the corresponding definitions.
///
/// Additionally, splitting it up this way makes each phase independently testable and verifiable,
/// essential due to the complexity of the overall transformation.
///
/// This pass corresponds to Phase 2 above, application of computed spills and reloads. It is very
/// simple, and can be seen as essentially materializing the analysis results in the IR. In addition
/// to setting the stage for Phase 3, this pass can also be used to validate that the scheduling of
/// spills and reloads is correct, matching the order in which we expect those operations to occur.
///
/// ## Notes About The Validity of Emitted IR
///
/// It is implied in my earlier notice, but I want to make it explicit here - this pass may produce
/// IR that is semantically invalid. Such IR is technically valid, and self-consistent, but cannot
/// be compiled to Miden Assembly. First, the spill and reload pseudo-instructions are expected to
/// only ever exist in the IR during application of the [InsertSpills] and  [RewriteSpills] passes;
/// later passes do not know how to handle them, and may panic if encountered, particularly the code
/// generation pass, which will raise an error on any unhandled instructions. Second, the semantics
/// of spills and reloads dictates that when a spill occurs, the live range of the spilled value is
/// terminated; and may only be resurrected by an explicit reload of that value. However, because
/// the new definition produced by a reload instruction is not actually used in the IR until after
/// the [RewriteSpills] pass is applied, the IR immediately after the [InsertSpills] pass is
/// semantically invalid - values will be dropped from the operand stack by a spill, yet there will
/// be code later in the same function which expects them to still be live (and thus on the operand
/// stack), which will fail to compile.
///
/// **TL;DR:** Unless testing or debugging, always apply [InsertSpills] and [RewriteSpills]
/// consecutively!
#[derive(Default, PassInfo, ModuleRewritePassAdapter)]
pub struct InsertSpills;
impl RewritePass for InsertSpills {
    type Entity = hir::Function;

    fn apply(
        &mut self,
        function: &mut Self::Entity,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> RewriteResult {
        let mut spills = {
            // Compute the spills
            let spills = analyses.get_or_compute::<SpillAnalysis>(function, session)?;
            // If there are no spills to process, we're done
            if !spills.has_spills() {
                analyses.mark_all_preserved::<Function>(&function.id);
                return Ok(());
            }
            // Drop the reference to the analysis so that we can take ownership of it
            drop(spills);
            analyses.take::<SpillAnalysis>(&function.id).unwrap()
        };

        // Apply all splits
        for split_info in spills.splits_mut() {
            let mut builder = FunctionBuilder::at(
                function,
                InsertionPoint::after(split_info.predecessor.block.into()),
            );

            // Create the split
            let split = builder.create_block();
            builder.switch_to_block(split);

            // Record the block we created for this split
            split_info.split = Some(split);

            // Rewrite the terminator in the predecessor so that it transfers control to the
            // original successor via `split`, moving any block arguments into the
            // unconditional branch that terminates `split`.
            let span = builder.func.dfg.inst_span(split_info.predecessor.inst);
            let ix = builder.func.dfg.inst_mut(split_info.predecessor.inst);
            let args = match ix {
                Instruction::Br(Br {
                    ref mut destination,
                    args,
                    ..
                }) => {
                    assert_eq!(*destination, split_info.block);
                    *destination = split;
                    args.take()
                }
                Instruction::CondBr(CondBr {
                    then_dest,
                    else_dest,
                    ..
                }) => {
                    if then_dest.0 == split_info.block {
                        then_dest.0 = split;
                        then_dest.1.take()
                    } else {
                        assert_eq!(else_dest.0, split_info.block);
                        else_dest.0 = split;
                        else_dest.1.take()
                    }
                }
                Instruction::Switch(Switch {
                    arms, r#default, ..
                }) => {
                    if r#default == &split_info.block {
                        *r#default = split;
                    }
                    for (_, arm) in arms.iter_mut() {
                        if arm == &split_info.block {
                            *arm = split;
                        }
                    }
                    ValueList::default()
                }
                ix => unimplemented!("unhandled branch instruction: {}", ix.opcode()),
            };
            builder.ins().Br(Opcode::Br, Type::Unit, split_info.block, args, span);
        }

        // Insert all spills
        for spill_info in spills.spills.iter_mut() {
            let ip = match spill_info.place {
                Placement::Split(split) => {
                    let split_block = spills.splits[split.as_u32() as usize]
                        .split
                        .expect("expected split to have been materialized");
                    let terminator = function.dfg.last_inst(split_block).unwrap();
                    InsertionPoint::before(terminator.into())
                }
                Placement::At(ip) => ip,
            };
            let mut builder = FunctionBuilder::at(function, ip);
            let mut args = ValueList::default();
            args.push(spill_info.value, &mut builder.func.dfg.value_lists);
            let inst = builder.ins().PrimOp(Opcode::Spill, Type::Unit, args, spill_info.span).0;
            spill_info.inst = Some(inst);
        }

        // Insert all reloads
        for reload in spills.reloads.iter_mut() {
            let ip = match reload.place {
                Placement::Split(split) => {
                    let split_block = spills.splits[split.as_u32() as usize]
                        .split
                        .expect("expected split to have been materialized");
                    let terminator = function.dfg.last_inst(split_block).unwrap();
                    InsertionPoint::before(terminator.into())
                }
                Placement::At(ip) => ip,
            };

            let ty = function.dfg.value_type(reload.value).clone();
            let mut builder = FunctionBuilder::at(function, ip);
            let mut args = ValueList::default();
            args.push(reload.value, &mut builder.func.dfg.value_lists);
            let inst = builder.ins().PrimOp(Opcode::Reload, ty, args, reload.span).0;
            reload.inst = Some(inst);
        }

        // Save the updated analysis results, and mark it preserved for later passes
        analyses.insert(function.id, spills);
        analyses.mark_preserved::<SpillAnalysis>(&function.id);

        Ok(())
    }
}

/// This pass rewrites a function annotated by the [InsertSpills] pass, by means of the spill
/// and reload pseudo-instructions, such that the resulting function is semantically equivalent
/// to the original function, but with the additional property that the function will keep the
/// operand stack depth <= 16 at all times.
///
/// This rewrite consists of the following main objectives:
///
/// * Match all uses of spilled values with the nearest dominating definition, modifying the IR as
///   required to ensure that all uses are strictly dominated by their definitions.
/// * Allocate sufficient procedure locals to store concurrently-active spills
/// * Rewrite all `spill` instructions to primitive `local.store` instructions
/// * Rewrite used `reload` instructions to primitive `local.load` instructions
/// * Remove unused `reload` instructions as dead code
///
/// **NOTE:** This pass is intended to be combined with the [InsertSpills] pass. If run on its own,
/// it is effectively a no-op, so it is safe to do, but nonsensical. In a normal compilation
/// pipeline, this pass is run immediately after [InsertSpills]. It is _not_ safe to run other
/// passes betweeen [InsertSpills] and [RewriteSpills], unless that pass specifically is designed to
/// preserve the results of the [SpillAnalysis] computed and used by [InsertSpills] to place spills
/// and reloads. Conversely, you can't just run [InsertSpills] without this pass, or the resulting
/// IR will fail to codegen.
///
/// ## Design
///
/// See [SpillAnalysis] and [InsertSpills] for more context and details.
///
/// The primary purpose of this pass is twofold: reconstruct SSA form after insertion of spills and
/// reloads by [InsertSpills], and lowering of the spill and reload pseudo-instructions to primitive
/// stores and loads from procedure-local variables. It is the final, and most important phase of
/// the spills transformation.
///
/// Unlike [InsertSpills], which mainly just materializes the results of the [SpillAnalysis], this
/// pass must do a tricky combo of dataflow analysis and rewrite in a single postorder traversal of
/// the CFG (i.e. bottom-up):
///
/// * We need to find uses of spilled values as we encounter them, and keep track of them until
/// we find an appropriate definition for each use.
/// * We need to propagate uses up the dominance tree until all uses are matched with definitions
/// * We need to rewrite uses when we find a definition
/// * We need to identify whether a block we are about to leave (on our way up the CFG), is in
/// the iterated dominance frontier for the set of spilled values we've found uses for. If it is,
/// we must append a new block parameter, rewrite the terminator of any predecessor blocks, and
/// rewrite all uses found so far by using the new block parameter as the dominating definition.
///
/// Technically, this pass could be generalized a step further, such that it fixes up invalid
/// def-use relationships in general, rather than just the narrow case of spills/reloads - but it is
/// more efficient to keep it specialized for now, we can always generalize later.
///
/// This pass guarantees that:
///
/// 1. No `spill` or `reload` instructions remain in the IR
/// 2. The semantics of the original IR on which [InsertSpills] was run, will be preserved, if:
///   * The original IR was valid
///   * No modification to the IR was made between [InsertSpills] and [RewriteSpills]
/// 3. The resulting function, once compiled to Miden Assembly, will keep the operand stack depth <=
///    16 elements, so long as the schedule produced by the backend preserves the scheduling
///    semantics. For example, spills/reloads are computed based on an implied scheduling of
///    operations, given by following the control flow graph, and visiting instructions in a block
///    top-down. If the backend reschedules operations for more optimal placement of operands on the
///    operand stack, it is possible that this rescheduling could result in the operand stack depth
///    exceeding 16 elements. However, at this point, it is not expected that this will be a
///    practical issue, even if it does occur, since the introduction of spills and reloads, not
///    only place greater constraints on backend scheduling, but also ensure that more live ranges
///    are split, and thus operands will spend less time on the operand stack overall. Time will
///    tell whether this holds true or not.
#[derive(Default, PassInfo, ModuleRewritePassAdapter)]
pub struct RewriteSpills;
impl RewritePass for RewriteSpills {
    type Entity = hir::Function;

    fn apply(
        &mut self,
        function: &mut Self::Entity,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> RewriteResult {
        // At this point, we've potentially emitted spills/reloads, but these are not yet being
        // used to split the live ranges of the SSA values to which they apply. Our job now, is
        // to walk the CFG bottom-up, finding uses of values for which we have issued reloads,
        // and then looking for the dominating definition (either original, or reload) that controls
        // that use, rewriting the use with the SSA value corresponding to the reloaded value.
        //
        // This has the effect of "reconstructing" the SSA form - although in our case it is more
        // precise to say that we are fixing up the original program to reflect the live-range
        // splits that we have computed (and inserted pseudo-instructions for). In the original
        // paper, they actually had multiple definitions of reloaded SSA values, which is why
        // this phase is referred to as "reconstructing", as it is intended to recover the SSA
        // property that was lost once multiple definitions are introduced.
        //
        //   * For each original definition of a spilled value `v`, get the new definitions of `v`
        //     (reloads) and the uses of `v`.
        //   * For each use of `v`, walk the dominance tree upwards until a definition of `v` is
        //     found that is responsible for that use. If an iterated dominance frontier is passed,
        //     a block argument is inserted such that appropriate definitions from each predecessor
        //     are wired up to that block argument, which is then the definition of `v` responsible
        //     for subsequent uses. The predececessor instructions which branch to it are new uses
        //     which we visit in the same manner as described above. After visiting all uses, any
        //     definitions (reloads) which are dead will have no uses of the reloaded value, and can
        //     thus be eliminated.

        // We consume the spill analysis in this pass, as it will no longer be valid after this
        let spills = match analyses.get::<SpillAnalysis>(&function.id) {
            Some(spills) if spills.has_spills() => spills,
            _ => {
                analyses.mark_all_preserved::<Function>(&function.id);
                return Ok(());
            }
        };
        let cfg = analyses.get_or_compute::<ControlFlowGraph>(function, session).unwrap();
        let domtree = analyses.get_or_compute::<DominatorTree>(function, session).unwrap();
        let domf = DominanceFrontier::compute(&domtree, &cfg, function);

        // Insert additional phi nodes as follows:
        //
        // 1. For each spilled value V
        // 2. Obtain the set of blocks, R, containing a reload of V
        // 3. For each block B in the dominance frontier of R, insert a phi in B for V
        // 4. In each block of R, add a new block argument to B, passing V initially
        // 5. Traverse the CFG bottom-up, finding uses of V, until we reach an inserted phi, a
        //    reload, or the original definition. Rewrite all found uses of V up to that point, to
        //    use this definition.
        let mut required_phis = BTreeMap::<Value, SmallSet<Block, 2>>::default();
        for info in spills.reloads() {
            let r_block = function.dfg.inst_block(info.inst.unwrap()).unwrap();
            let domf_r = required_phis.entry(info.value).or_default();
            domf_r.extend(domf.iter(&r_block));
        }

        let mut inserted_phis = BTreeMap::<Block, SmallMap<Value, Value, 2>>::default();
        for (value, domf_r) in required_phis {
            // Add phi to each B in DF(R)
            let data = function.dfg.value_data(value);
            let ty = data.ty().clone();
            let span = data.span();
            for b in domf_r {
                // Allocate new block parameter/phi, if one is not already present
                let phis = inserted_phis.entry(b).or_default();
                if let adt::smallmap::Entry::Vacant(entry) = phis.entry(value) {
                    let phi = function.dfg.append_block_param(b, ty.clone(), span);
                    entry.insert(phi);

                    // Append `value` as new argument to every predecessor to satisfy new parameter
                    for pred in cfg.pred_iter(b) {
                        function.dfg.append_branch_destination_argument(pred.inst, b, value);
                    }
                }
            }
        }

        // Traverse the CFG bottom-up, doing the following along the way:
        //
        // 0. Merge the "used" sets of each successor of the current block (see remaining steps for
        //    how the "used" set is computed for a block). NOTE: We elaborate in step 4 on how to
        //    handle computing the "used" set for a successor, from the "used" set at the start of
        //    the successor block.
        // 1. If we encounter a use of a spilled value, record the location of that use in the set
        // of uses we're seeking a dominating definition for, i.e. the "used" set
        // 2. If we reach a definition for a value with uses in the "used" set:
        //   * If the definition is the original definition of the value, no action is needed, so we
        //     remove all uses of that value from the "used" set.
        //   * If the definition is a reload, rewrite all of the uses in the "used" set to use the
        //     reload instead, removing them from the "used" set. Mark the reload used.
        // 3. When we reach the start of the block, the state of the "used" set is associated with
        //    the current block. This will be used as the starting state of the "used" set in each
        //    predecessor of the block
        // 4. When computing the "used" set in the predecessor (i.e. step 0), we also check whether
        //    a given successor is in the iterated dominance frontier for any values in the "used"
        //    set of that successor. If so, we need to insert a block parameter for each such value,
        //    rewrite all uses of that value to use the new block parameter, and add the "used"
        //    value as an additional argument to that successor. The resulting "used" set will thus
        //    retain a single entry for each of the values for which uses were rewritten
        //    (corresponding to the block arguments for the successor), but all of the uses
        //    dominated by the introduced block parameter are no longer in the set, as their
        //    dominating definition has been found. Any values in the "used" set for which the
        //    successor is not in the iterated dominance frontier for that value, are retained in
        //    the "used" set without any changes.
        let mut used_sets = BTreeMap::<Block, BTreeMap<Value, FxHashSet<User>>>::default();
        let mut block_q = VecDeque::from_iter(domtree.cfg_postorder().iter().copied());
        while let Some(block_id) = block_q.pop_front() {
            // Compute the "used" set for this block
            let mut used = BTreeMap::<Value, FxHashSet<User>>::default();
            for succ in cfg.succ_iter(block_id) {
                if let Some(succ_used) = used_sets.get_mut(&succ) {
                    // Union the used set from this successor with the others
                    for (value, users) in succ_used.iter() {
                        used.entry(*value).or_default().extend(users.iter().cloned());
                    }
                }
            }

            // Traverse the block bottom-up, recording uses of spilled values while looking for
            // definitions
            let mut insts = function.dfg.block(block_id).insts().collect::<Vec<_>>();
            while let Some(current_inst) = insts.pop() {
                // If `current_inst` is a branch or terminator, it cannot define a value, so
                // we simply record any uses, and move on.
                match function.dfg.analyze_branch(current_inst) {
                    BranchInfo::SingleDest(_, args) => {
                        for (index, arg) in args.iter().enumerate() {
                            if spills.is_spilled(arg) {
                                used.entry(*arg).or_default().insert(User::new(
                                    current_inst,
                                    *arg,
                                    Use::BlockArgument {
                                        succ: 0,
                                        index: index as u16,
                                    },
                                ));
                            }
                        }
                    }
                    BranchInfo::MultiDest(ref jts) => {
                        for (succ_index, jt) in jts.iter().enumerate() {
                            for (index, arg) in jt.args.iter().enumerate() {
                                if spills.is_spilled(arg) {
                                    used.entry(*arg).or_default().insert(User::new(
                                        current_inst,
                                        *arg,
                                        Use::BlockArgument {
                                            succ: succ_index as u16,
                                            index: index as u16,
                                        },
                                    ));
                                }
                            }
                        }
                    }
                    BranchInfo::NotABranch => {
                        // Does this instruction provide a definition for any spilled values?
                        let ix = function.dfg.inst(current_inst);

                        let is_reload = matches!(ix.opcode(), Opcode::Reload);
                        if is_reload {
                            // We have found a new definition for a spilled value, we must rewrite
                            // all uses of the spilled value found so
                            // far, with the reloaded value
                            let spilled = ix.arguments(&function.dfg.value_lists)[0];
                            let reloaded = function.dfg.first_result(current_inst);

                            if let Some(to_rewrite) = used.remove(&spilled) {
                                debug_assert!(
                                    !to_rewrite.is_empty(),
                                    "expected empty use sets to be removed"
                                );

                                for user in to_rewrite.iter() {
                                    match user.ty {
                                        Use::BlockArgument {
                                            succ: succ_succ,
                                            index,
                                        } => {
                                            function.dfg.replace_successor_argument(
                                                user.inst,
                                                succ_succ as usize,
                                                index as usize,
                                                reloaded,
                                            );
                                        }
                                        Use::Operand { index } => {
                                            function.dfg.replace_argument(
                                                user.inst,
                                                index as usize,
                                                reloaded,
                                            );
                                        }
                                    }
                                }
                            } else {
                                // This reload is unused, so remove it entirely, and move to the
                                // next instruction
                                //let mut cursor = function
                                //.dfg
                                //.block_mut(block_id)
                                //.cursor_mut_at_inst(current_inst);
                                //cursor.remove();
                                continue;
                            }
                        }

                        for spilled in function
                            .dfg
                            .inst_results(current_inst)
                            .iter()
                            .filter(|result| spills.is_spilled(result))
                        {
                            // This op is the original definition for `spilled`, so remove any uses
                            // of it we've accumulated so far, as they
                            // do not need to be rewritten
                            used.remove(spilled);
                        }
                    }
                }

                // Record any uses of spilled values in the argument list for `current_inst` (except
                // reloads and spills)
                let ignored = matches!(
                    function.dfg.inst(current_inst).opcode(),
                    Opcode::Reload | Opcode::Spill
                );
                if !ignored {
                    for (index, arg) in function.dfg.inst_args(current_inst).iter().enumerate() {
                        if spills.is_spilled(arg) {
                            used.entry(*arg).or_default().insert(User::new(
                                current_inst,
                                *arg,
                                Use::Operand {
                                    index: index as u16,
                                },
                            ));
                        }
                    }
                }
            }

            // At the top of the block, if any of the block parameters are in the "used" set, remove
            // those uses, as the block parameters are the original definitions for
            // those values, and thus no rewrite is needed.
            for arg in function.dfg.block_args(block_id) {
                used.remove(arg);
            }

            // If we have inserted any phis in this block, rewrite uses of the spilled values they
            // represent.
            if let Some(phis) = inserted_phis.get(&block_id) {
                for (spilled, phi) in phis.iter() {
                    if let Some(to_rewrite) = used.remove(spilled) {
                        debug_assert!(
                            !to_rewrite.is_empty(),
                            "expected empty use sets to be removed"
                        );

                        for user in to_rewrite.iter() {
                            match user.ty {
                                Use::BlockArgument {
                                    succ: succ_succ,
                                    index,
                                } => {
                                    function.dfg.replace_successor_argument(
                                        user.inst,
                                        succ_succ as usize,
                                        index as usize,
                                        *phi,
                                    );
                                }
                                Use::Operand { index } => {
                                    function.dfg.replace_argument(user.inst, index as usize, *phi);
                                }
                            }
                        }
                    } else {
                        // TODO(pauls): This phi is unused, we should be able to remove it
                        continue;
                    }
                }
            }

            // What remains are the unsatisfied uses of spilled values for this block and its
            // successors
            used_sets.insert(block_id, used);
        }

        // For each spilled value, allocate a procedure local, rewrite the spill instruction as a
        // `local.store`
        let mut locals = BTreeMap::<Value, LocalId>::default();
        for spill_info in spills.spills() {
            let value = spill_info.value;
            let local = function.dfg.alloc_local(spill_info.ty.clone());
            assert_eq!(locals.insert(value, local), None, "unexpected duplicate spill for {value}");
            let builder = ReplaceBuilder::new(
                &mut function.dfg,
                spill_info.inst.expect("expected spill to have been materialized"),
            );
            builder.store_local(local, value, spill_info.span);
        }

        // Rewrite all used reload instructions as `local.load` instructions from the corresponding
        // procedure local
        for reload_info in spills.reloads() {
            let inst = reload_info.inst.expect("expected reload to have been materialized");
            let spilled = function.dfg.inst_args(inst)[0];
            let builder = ReplaceBuilder::new(&mut function.dfg, inst);
            builder.load_local(locals[&spilled], reload_info.span);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use midenc_hir::testing::TestContext;
    use pretty_assertions::{assert_ne, assert_str_eq};

    use super::*;

    #[test]
    fn spills_loop_nest() {
        let context = TestContext::default();
        let id = "test::spill".parse().unwrap();
        let mut function = Function::new(
            id,
            Signature::new(
                [
                    AbiParam::new(Type::Ptr(Box::new(Type::U64))),
                    AbiParam::new(Type::U64),
                    AbiParam::new(Type::U64),
                ],
                [AbiParam::new(Type::U64)],
            ),
        );

        {
            let mut builder = FunctionBuilder::new(&mut function);
            let entry = builder.current_block();
            let (v0, v1, v2) = {
                let args = builder.block_params(entry);
                (args[0], args[1], args[2])
            };

            let block1 = builder.create_block();
            let block2 = builder.create_block();
            let block3 = builder.create_block();
            let block4 = builder.create_block();
            let block5 = builder.create_block();
            let block6 = builder.create_block();

            // entry
            let v3 = builder.ins().u64(0, SourceSpan::UNKNOWN);
            let v4 = builder.ins().u64(0, SourceSpan::UNKNOWN);
            let v5 = builder.ins().u64(0, SourceSpan::UNKNOWN);
            builder.ins().br(block1, &[v3, v4, v5], SourceSpan::UNKNOWN);

            // block1 - outer loop header
            let v6 = builder.append_block_param(block1, Type::U64, SourceSpan::UNKNOWN);
            let v7 = builder.append_block_param(block1, Type::U64, SourceSpan::UNKNOWN);
            let v8 = builder.append_block_param(block1, Type::U64, SourceSpan::UNKNOWN);
            builder.switch_to_block(block1);
            let v9 = builder.ins().eq(v6, v1, SourceSpan::UNKNOWN);
            builder.ins().cond_br(v9, block2, &[], block3, &[], SourceSpan::UNKNOWN);

            // block2 - outer exit
            builder.switch_to_block(block2);
            builder.ins().ret(Some(v8), SourceSpan::UNKNOWN);

            // block3 - split edge
            builder.switch_to_block(block3);
            builder.ins().br(block4, &[v7, v8], SourceSpan::UNKNOWN);

            // block4 - inner loop
            let v10 = builder.append_block_param(block4, Type::U64, SourceSpan::UNKNOWN);
            let v11 = builder.append_block_param(block4, Type::U64, SourceSpan::UNKNOWN);
            builder.switch_to_block(block4);
            let v12 = builder.ins().eq(v10, v2, SourceSpan::UNKNOWN);
            builder.ins().cond_br(v12, block5, &[], block6, &[], SourceSpan::UNKNOWN);

            // block5 - inner latch
            builder.switch_to_block(block5);
            let v13 = builder.ins().add_imm_unchecked(v6, Immediate::U64(1), SourceSpan::UNKNOWN);
            builder.ins().br(block1, &[v13, v10, v11], SourceSpan::UNKNOWN);

            // block6 - inner body
            builder.switch_to_block(block6);
            let v14 = builder.ins().add_imm_unchecked(v6, Immediate::U64(1), SourceSpan::UNKNOWN);
            let v15 = builder.ins().mul_unchecked(v14, v2, SourceSpan::UNKNOWN);
            let v16 = builder.ins().add_unchecked(v10, v15, SourceSpan::UNKNOWN);
            let v17 = builder.ins().ptrtoint(v0, Type::U64, SourceSpan::UNKNOWN);
            let v18 = builder.ins().add_unchecked(v17, v16, SourceSpan::UNKNOWN);
            let v19 =
                builder.ins().inttoptr(v18, Type::Ptr(Box::new(Type::U64)), SourceSpan::UNKNOWN);
            let v20 = builder.ins().load(v19, SourceSpan::UNKNOWN);
            let v21 = builder.ins().add_unchecked(v11, v20, SourceSpan::UNKNOWN);
            let v22 = builder.ins().add_imm_unchecked(v10, Immediate::U64(1), SourceSpan::UNKNOWN);
            builder.ins().br(block4, &[v22, v21], SourceSpan::UNKNOWN);
        }

        let original = function.to_string();
        let mut analyses = AnalysisManager::default();
        let mut rewrite = InsertSpills;
        rewrite
            .apply(&mut function, &mut analyses, &context.session)
            .expect("spill insertion failed");

        //analyses.invalidate::<Function>(&function.id);

        //let mut rewrite = RewriteSpills;
        //rewrite
        //.apply(&mut function, &mut analyses, &context.session)
        //.expect("spill cleanup failed");

        // TODO: Test is failing because the spill of v1 is improperly placed. The spill occurs
        // prior to entry to the inner loop, as it is not used in the inner loop, and is next
        // used in the header of the outer loop, which the inner loop conditionally branches
        // to. However, this places the spill inside a loop, when its definition exists outside
        // the loop. Instead, we would expect the spill to be placed on entry to the outer loop,
        // as even though it is easily held on the operand stack at that point, spilling it
        // there ensures that we only need a single reload when continuing the outer loop from
        // the inner loop, and no spills inside the loop.
        let expected = "\
        )";

        let transformed = function.to_string();
        assert_ne!(transformed, original);
        assert_str_eq!(transformed.as_str(), expected);
    }
}
