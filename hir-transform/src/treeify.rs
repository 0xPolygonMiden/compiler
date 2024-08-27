use std::{
    collections::{BTreeMap, VecDeque},
    rc::Rc,
};

use midenc_hir::{
    self as hir,
    pass::{AnalysisManager, RewritePass, RewriteResult},
    Block as BlockId, Value as ValueId, *,
};
use midenc_hir_analysis::{BlockPredecessor, ControlFlowGraph, DominatorTree, Loop, LoopAnalysis};
use midenc_session::{
    diagnostics::{IntoDiagnostic, Report},
    Session,
};
use smallvec::{smallvec, SmallVec};

use crate::adt::ScopedMap;

/// This pass rewrites the CFG of a function so that it forms a tree.
///
/// While we technically call this treeification, the CFG cannot be fully converted into a
/// tree in general, as loops must be preserved (they can be copied along multiple control
/// flow paths, but we want to preserve the loop structure in the CFG).
///
/// The treeify transformation concerns itself with any block B which has multiple predecessors
/// in the control flow graph, where for at least two of those predecessors, the predecessor is
/// always visited before B, if control flows through both. This is a slightly less restrictive
/// conditon than the dominance property, but is very much related - the primary difference being
/// that unlike dominance, what we are capturing is that the predecessor block is not along a
/// loopback edge. It is quite common for a predecessor block to always be visited first in the
/// CFG, while not dominating its successor: consider an if/else expression, where control splits
/// at the `if/else`, and rejoins afterwards, the code in the final block where control is joined
/// can only be reached after either the `if` or `else` block has executed, but neither the `if`
/// nor the `else` blocks can be considered to "dominate" the final block in the graph theoretical
/// sense.
///
/// The actual treeification process works like so:
///
/// 1. For each block B, in the postorder sort of the CFG, determine if B has more than one
///    predecessor P, where P appears before B in the reverse postorder sort of the CFG. a. If
///    found, treeify the block as described in subsequent steps b. Otherwise, ignore this block and
///    proceed
/// 2. For each P, clone B to a new block B', and rewrite P such that it branches to B' rather than
///    B.
/// 3. For each successor S of B:
///   a. If S is a loop header, and S appears before B in the reverse postorder sort of the CFG,
///      then it is a loopback edge, so the corresponding edge from B' to S is left intact.
///   b. If S is a loop header, but S appears after B in the reverse postorder sort of the CFG,
///      then it is treated like other blocks (see c.)
///   c. Otherwise, clone S to S', and rewrite B' to branch to S' instead of S.
/// 4. Repeat step 2 for the successors of S, recursively, until the subgraph reachable from B
///
/// Since we are treeifying blocks from the leaves of the CFG to the root, and because we do not
/// follow loopback edges which escape/continue an outer loop - whenever we clone a subgraph of
/// the CFG, we know that it has already been treeified, as we only start to treeify a block once
/// all of the blocks reachable via that block have been treeified.
///
/// In short, we're trying to split blocks with multiple predecessors such that all blocks have
/// either zero or one predecessors, i.e. the CFG forms a tree. As mentioned previously, we must
/// make an exception for loop headers, which by definition must have at least one predecessor
/// which is a loopback edge, but this suits us just fine, as Miden Assembly provides control flow
/// instructions compatible with lowering from such a CFG.
///
/// # Examples
///
/// ## Basic DAG
///
/// This example demonstrates how the DAG of a function with multiple returns gets transformed:
///
/// ```text,ignore
/// blk0
///  |
///  v
/// blk1 -> blk3 -> ret
///  |     /
///  |    /
///  |   /
///  v  v
/// blk2
///  |
///  v
/// ret
/// ```
///
/// Becomes:
///
/// ```text,ignore
/// blk0
///  |
///  v
/// blk1 -> blk3 -> ret
///  |       |
///  |       |
///  |       |
///  v       v
/// blk2    blk2
///  |       |
///  v       v
/// ret     ret
/// ```
///
/// ## Basic Loop
///
/// This is an example of a function with multiple returns and a simple loop:
///
/// ```text,ignore
/// blk0
///  |                -------
///  v               v       |
/// blk1 -> blk3 -> blk4 -> blk5 -> ret
///  |     /
///  |    /
///  |   /
///  v  v
/// blk2
///  |
///  v
/// ret
/// ```
///
/// Becomes:
///
/// ```text,ignore
/// blk0
///  |                -------
///  v               v       |
/// blk1 -> blk3 -> blk4 -> blk5 -> ret
///  |       |
///  |       |
///  |       |
///  v       v
/// blk2    blk2
///  |       |
///  v       v
/// ret     ret
/// ```
///
/// ## Complex Loop
///
/// This is an example of a function with a complex loop (i.e. multiple exit points):
///
/// ```text,ignore
/// blk0
///  |
///  v
/// blk1
///  |  \
///  |   blk2 <-----
///  |    |         |
///  |   blk3       |
///  |   /   \      |
///  |  /     blk4--
///  | /       |
///  vv        |
/// blk5      blk6
/// ```
///
/// Becomes:
///
/// ```text,ignore
/// blk0
///  |
///  v
/// blk1
///  |  \
///  |   \
///  |    blk2 <---
///  |     |       |
///  |     v       |
///  |    blk3     |
///  |    |  \     |
///  |    |   blk4--
///  |    |    |
///  v    v    v
/// blk5 blk5  blk6
/// ```
///
/// NOTE: Here, when generating code for `blk5` and `blk6`, the loop depth is 0, so
/// we will emit a single `push.0` at the end of both blocks which will terminate the
/// containing loop, and then return from the function as we've reached the bottom
/// of the tree.
///
/// ## Nested Loops
///
/// This is an extension of the example above, but with nested loops:
///
/// ```text,ignore
/// blk0
///  |
///  v
/// blk1
///  |  \
///  |   blk2 <-------
///  |    |         | |
///  |   blk3       | |
///  |   /   \      | |
///  |  /     blk4--  |
///  | /       |      |
///  vv        v      |
/// blk5<-    blk6-->blk7-->blk8
///       |    ^             |
///       |    |_____________|
///       |                  |
///       |__________________|
/// ```
///
/// We have two loops, the outer one starting at `blk2`:
///
/// * `blk2->blk3->blk4->blk2`
/// * `blk2->blk3->blk4->blk6->blk7->blk2`
///
/// And the inner one starting at `blk6`:
///
/// * `blk6->blk7->blk8->blk6`
///
/// Additionally, there are multiple exits through the loops, depending on the path taken:
///
/// * `blk2->blk3->blk5`
/// * `blk2->blk3->blk4->blk6->blk7->blk8->blk5`
/// * `blk6->blk7->blk8->blk5`
///
/// After transformation, this becomes:
///
/// ```text,ignore
/// blk0
///  |
///  v
/// blk1
///  |  \
///  |   blk2 <-------
///  |    |         | |
///  |   blk3       | |
///  |    |  \      | |
///  |    |   blk4--  |
///  |    |    |      |
///  v    v    v      |
/// blk5 blk5 blk6-->blk7-->blk8
///            ^             | |
///            |_____________|_|
///                          |
///                          v
///                         blk5
/// ```
///
/// During codegen though, we end up with the following tree of stack machine code.
///
/// At each point where control flow either continues a loop or leaves it, we must
///
/// * Duplicate loop headers on control flow edges leading to those headers
/// * Emit N `push.0` instructions on control flow edges exiting the function from a loop depth of N
/// * Emit a combination of the above on control flow edges exiting an inner loop for an outer loop,
/// depending on what depths the predecessor and successor blocks are at
///
/// ```text,ignore
/// blk0
/// blk1
/// if.true
///   blk2
///   while.true
///     blk3
///     if.true
///       blk4
///       if.true
///         blk2         # duplicated outer loop header
///       else
///         blk6
///         while.true
///           blk7
///           if.true
///             blk2     # duplicated outer loop header
///             push.0   # break out of inner loop
///           else
///             blk8
///             if.true
///               blk6   # duplicated inner loop header
///             else
///               blk5
///               push.0 # break out of outer loop
///               push.0 # break out of inner loop
///             end
///           end
///         end
///       end
///     else
///       blk5
///       push.0         # break out of outer loop
///     end
///   end
/// else
///   blk5
/// end
/// ```
#[derive(Default, PassInfo, ModuleRewritePassAdapter)]
pub struct Treeify;
impl RewritePass for Treeify {
    type Entity = hir::Function;

    fn apply(
        &mut self,
        function: &mut Self::Entity,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> RewriteResult {
        let cfg = analyses.get_or_compute::<ControlFlowGraph>(function, session)?;
        let domtree = analyses.get_or_compute::<DominatorTree>(function, session)?;
        let loops = analyses.get_or_compute::<LoopAnalysis>(function, session)?;

        // Obtain the set of all blocks we need to check for treeification in a new vector.
        //
        // We must do this because as we treeify the CFG, we will be updating it, as well
        // as all of the analyses, such as the dominator tree, so we can't iterate it at
        // the same time as we do treeification.
        //
        // Additionally, this set never changes - we are visiting the function bottom-up,
        // so we only start to treeify a block once all of the blocks reachable via that
        // block have been treeified. As a result, the tree reachable from B is already
        // treeified.
        let to_visit = domtree.cfg_postorder().to_vec();

        // This outer loop visits all of the original blocks of the CFG postorder (bottom-up),
        // and is simply searching for blocks of the function which meet the criteria for
        // treeification.
        //
        // The inner loop is responsible for actually treeifying those blocks. This
        // necessarily has the effect of mutating the function, and therefore requires
        // us to recompute some of the analyses so that we can properly determine how
        // to handle certain blocks in the portion of the CFG being treeified, namely
        // loops (via loop headers).
        //
        // Loops require special handling, as during treeification we typically will be
        // cloning blocks that belong to the portion of the CFG rooted at the block being
        // treeified. However, if we are treeifying a block that belongs to a loop, we do
        // not want to clone along control flow edges representing continuation or breaking
        // out of an outer loop. On the other hand, if we reach a loop that is only reachable
        // via the block being treeified, we do want to copy those, as each branch of the tree
        // will need its own copy of that loop.
        //
        // To handle this, we require the ability to:
        //
        // * Identify loop headers (which requires the loops analysis)
        // * Identify the reverse postorder index of a block (which requires the dominator tree)
        //
        // The dominator tree requires the control flow graph analysis, and the loop analysis
        // requires the dominator tree - as a result, each time we modify the CFG, we must also
        // ensure that all three analyses reflect any effects of such modifications.
        //
        // However, this would be very expensive to compute as frequently as would be required
        // by this transformation. Instead, since the transformation is essentially just cloning
        // multiple copies of various subgraphs of the original CFG, we can use the analyses of
        // the original CFG as well, by mapping each copied block back to the block in the CFG
        // from which it is derived. By doing so, we can determine if that block is a loop header,
        // or how two blocks are sorted relative to each other in the reverse postorder, without
        // having to ever recompute the three analyses mentioned above.
        let mut block_infos = BlockInfos::new(cfg, domtree, loops);

        // For each block B, treeify B IFF it has multiple predecessors, where for each
        // predecessor P, P appears before B in the reverse postorder sort of the CFG.
        // Treeifying B involves creating a copy of B and the subgraph of the CFG rooted at B,
        // for each P.
        //
        // The blocks are selected this way, since by splitting these nodes in the CFG, such
        // that each predecessor gets its own copy of the subgraph reached via B, the CFG is
        // made more tree-like. Once all nodes are split, then the CFG is either a tree, or
        // a DAG that is almost a tree, with the only remaining DAG edges being loopback edges
        // for loops that appear in the CFG.
        let mut block_q = VecDeque::<CopyBlock>::default();
        let mut changed = false;
        for b in to_visit {
            // Check if this block meets the conditions for treeification
            let predecessors = block_infos
                .cfg
                .pred_iter(b)
                .filter(|bp| block_infos.rpo_cmp(bp.block, b).is_lt())
                .collect::<Vec<_>>();

            if predecessors.len() < 2 {
                continue;
            }
            log::trace!("found candidate for treeification: {b}");

            // For each predecessor, create a clone of B and all of its successors, with
            // the exception of successors which are loop headers where the loop header
            // appears before B in the reverse postorder sort of the CFG. Such edges are
            // loopback edges to an outer loop, which must be preserved, even when cloning
            // the subgraph rooted at B.
            for p in predecessors {
                assert!(block_q.is_empty());
                log::trace!("scheduling copy of {b} for predecessor {}", p.block);
                block_q.push_back(CopyBlock::new(b, p));
                let root = b;

                while let Some(CopyBlock {
                    b,
                    ref p,
                    value_map,
                    block_map,
                }) = block_q.pop_front()
                {
                    // If we enqueue a successor block to be copied, and that block is a loop header
                    // which appears before the root block in the CFG, then it is  a loopback edge
                    // that escapes the portion of the CFG being treeified, and we do not want not
                    // actually copy it.
                    if block_infos.is_loop_header(b).is_some()
                        && block_infos.rpo_cmp(b, root).is_lt()
                    {
                        log::trace!(
                            "skipping copy of {b} for {} as {b} dominates {root} (i.e. it is a \
                             loopback edge)",
                            p.block
                        );
                        continue;
                    }

                    // Copy this block and its successors
                    treeify(b, p, function, &mut block_infos, &mut block_q, value_map, block_map)?;
                }

                // Mark the control flow graph as modified
                changed = true;
            }
        }

        // If we made any changes, we need to recompute all analyses
        if !changed {
            analyses.mark_all_preserved::<Function>(&function.id);
        } else {
            // Recompute the CFG and dominator tree and remove all unreachable blocks
            let cfg = ControlFlowGraph::with_function(function);
            let domtree = DominatorTree::with_function(function, &cfg);
            let mut to_remove = vec![];
            for (b, _) in function.dfg.blocks() {
                if domtree.is_reachable(b) {
                    continue;
                }
                to_remove.push(b);
            }
            // Remove all blocks from the function that were unreachable
            for b in to_remove {
                function.dfg.detach_block(b);
            }
        }

        session.print(&function, Self::FLAG).into_diagnostic()?;
        if session.should_print_cfg(Self::FLAG) {
            use std::io::Write;
            let cfg = function.cfg_printer();
            let mut stdout = std::io::stdout().lock();
            write!(&mut stdout, "{cfg}").into_diagnostic()?;
        }
        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
fn treeify(
    b: BlockId,
    p: &BlockPredecessor,
    function: &mut hir::Function,
    block_infos: &mut BlockInfos,
    block_q: &mut VecDeque<CopyBlock>,
    mut value_map: ScopedMap<ValueId, ValueId>,
    mut block_map: ScopedMap<BlockId, BlockId>,
) -> Result<(), Report> {
    // Check if we're dealing with a loop header
    let is_loop = block_infos.is_loop_header(b).is_some();

    log::trace!(
        "starting treeification for {b} from {} (is {b} loop header? {is_loop})",
        p.block
    );

    // 1. Create a new block `b'`, without block arguments, unless it is a loop header,
    // in which case we want to preserve the block arguments, just with new value ids
    let b_prime = function.dfg.create_block_after(p.block);
    log::trace!("created block {b_prime} as clone of {b}");
    block_map.insert(b, b_prime);
    block_infos.insert_copy(b_prime, b);

    // 2. Remap values in the cloned block:
    //
    // * If this is a loop header, we need to replace references to the old block arguments with the
    //   new block arguments.
    // * If this is not a loop header, then we need to replace references to the block arguments
    //   with the values which were passed as arguments in the predecessor block
    if is_loop {
        function.dfg.clone_block_params(b, b_prime);
        for (src, dest) in function
            .dfg
            .block_params(b)
            .iter()
            .copied()
            .zip(function.dfg.block_params(b_prime).iter().copied())
        {
            value_map.insert(src, dest);
        }
    } else {
        match function.dfg.analyze_branch(p.inst) {
            BranchInfo::SingleDest(info) => {
                value_map.extend(
                    function.dfg.block_args(b).iter().copied().zip(info.args.iter().copied()),
                );
            }
            BranchInfo::MultiDest(ref infos) => {
                let info = infos.iter().find(|info| info.destination == b).unwrap();
                value_map.extend(
                    function.dfg.block_args(b).iter().copied().zip(info.args.iter().copied()),
                );
            }
            BranchInfo::NotABranch => unreachable!(),
        }
    }

    // 3. Update the predecessor instruction to reference the new block, remove block arguments if
    //    this is not a loop header.
    let mut seen = false; // Only update the first occurrance of this predecessor
    update_predecessor(function, p, |successor, pool| {
        log::trace!("maybe updating successor {} of {}", successor.destination, p.block);
        if successor.destination == b && !seen {
            seen = true;
            successor.destination = b_prime;
            if !is_loop {
                successor.args.clear(pool);
            }
        }
    });
    assert!(seen);

    // 4. Copy contents of `b` to `b'`, inserting defs in the lookup table, and mapping operands to
    //    their new "corrected" values
    copy_instructions(b, b_prime, function, &mut value_map, &block_map);

    // 5. Clone the children of `b` and append to `b_prime`, but do not clone children of `b` that
    //    are loop headers, only clone the edge.
    copy_children(b, b_prime, function, block_q, value_map, block_map)
}

#[allow(clippy::too_many_arguments)]
fn copy_children(
    b: BlockId,
    b_prime: BlockId,
    function: &mut hir::Function,
    block_q: &mut VecDeque<CopyBlock>,
    value_map: ScopedMap<ValueId, ValueId>,
    block_map: ScopedMap<BlockId, BlockId>,
) -> Result<(), Report> {
    let pred = BlockPredecessor {
        inst: function.dfg.last_inst(b_prime).expect("expected non-empty block"),
        block: b_prime,
    };
    let successors = match function.dfg.analyze_branch(function.dfg.last_inst(b).unwrap()) {
        BranchInfo::NotABranch => return Ok(()),
        BranchInfo::SingleDest(info) => smallvec![info.destination],
        BranchInfo::MultiDest(infos) => {
            SmallVec::<[_; 2]>::from_iter(infos.into_iter().map(|info| info.destination))
        }
    };
    let value_map = Rc::new(value_map);
    let block_map = Rc::new(block_map);

    for succ in successors {
        if let Some(succ_prime) = block_map.get(&succ) {
            update_predecessor(function, &pred, |successor, _| {
                if successor.destination == succ {
                    successor.destination = *succ_prime;
                }
            });
        }

        block_q.push_back(CopyBlock {
            b: succ,
            p: pred,
            value_map: ScopedMap::new(Some(value_map.clone())),
            block_map: ScopedMap::new(Some(block_map.clone())),
        });
    }

    Ok(())
}

fn copy_instructions(
    b: BlockId,
    b_prime: BlockId,
    function: &mut hir::Function,
    value_map: &mut ScopedMap<ValueId, ValueId>,
    block_map: &ScopedMap<BlockId, BlockId>,
) {
    // Initialize the cursor at the first instruction in `b`
    let mut next = {
        let cursor = function.dfg.block(b).insts.front();
        cursor.get().map(|inst_data| inst_data as *const InstNode)
    };

    while let Some(ptr) = next.take() {
        // Get the id of the instruction at the current cursor position, then advance the cursor
        let src_inst = {
            let mut cursor = unsafe { function.dfg.block(b).insts.cursor_from_ptr(ptr) };
            let id = cursor.get().unwrap().key;
            cursor.move_next();
            next = cursor.get().map(|inst_data| inst_data as *const InstNode);
            id
        };

        // Clone the source instruction data
        let inst = function.dfg.clone_inst(src_inst);

        // We need to fix up the cloned instruction data
        let data = &mut function.dfg.insts[inst];
        // First, we're going to be placing it in b', so make sure the instruction is aware of that
        data.block = b_prime;
        // Second, we need to rewrite value/block references contained in the instruction
        match data.as_mut() {
            Instruction::Br(hir::Br {
                ref mut successor, ..
            }) => {
                if let Some(new_dest) = block_map.get(&successor.destination) {
                    successor.destination = *new_dest;
                }
                let args = successor.args.as_mut_slice(&mut function.dfg.value_lists);
                for arg in args.iter_mut() {
                    if let Some(arg_prime) = value_map.get(arg) {
                        *arg = *arg_prime;
                    }
                }
            }
            Instruction::CondBr(hir::CondBr {
                ref mut cond,
                ref mut then_dest,
                ref mut else_dest,
                ..
            }) => {
                if let Some(cond_prime) = value_map.get(cond) {
                    *cond = *cond_prime;
                }
                if let Some(new_dest) = block_map.get(&then_dest.destination) {
                    then_dest.destination = *new_dest;
                }
                let then_args = then_dest.args.as_mut_slice(&mut function.dfg.value_lists);
                for arg in then_args.iter_mut() {
                    if let Some(arg_prime) = value_map.get(arg) {
                        *arg = *arg_prime;
                    }
                }
                if let Some(new_dest) = block_map.get(&else_dest.destination) {
                    else_dest.destination = *new_dest;
                }
                let else_args = else_dest.args.as_mut_slice(&mut function.dfg.value_lists);
                for arg in else_args.iter_mut() {
                    if let Some(arg_prime) = value_map.get(arg) {
                        *arg = *arg_prime;
                    }
                }
            }
            Instruction::Switch(hir::Switch {
                ref mut arg,
                ref mut arms,
                default: ref mut default_succ,
                ..
            }) => {
                if let Some(arg_prime) = value_map.get(arg) {
                    *arg = *arg_prime;
                }
                if let Some(new_default_dest) = block_map.get(&default_succ.destination) {
                    default_succ.destination = *new_default_dest;
                }
                let default_args = default_succ.args.as_mut_slice(&mut function.dfg.value_lists);
                for arg in default_args.iter_mut() {
                    if let Some(arg_prime) = value_map.get(arg) {
                        *arg = *arg_prime;
                    }
                }
                for arm in arms.iter_mut() {
                    if let Some(new_dest) = block_map.get(&arm.successor.destination) {
                        arm.successor.destination = *new_dest;
                    }
                    let args = arm.successor.args.as_mut_slice(&mut function.dfg.value_lists);
                    for arg in args.iter_mut() {
                        if let Some(arg_prime) = value_map.get(arg) {
                            *arg = *arg_prime;
                        }
                    }
                }
            }
            other => {
                for arg in other.arguments_mut(&mut function.dfg.value_lists).iter_mut() {
                    if let Some(arg_prime) = value_map.get(arg) {
                        *arg = *arg_prime;
                    }
                }
            }
        }
        // Finally, append the cloned instruction to the block layout
        let node = unsafe { UnsafeRef::from_raw(data) };
        function.dfg.block_mut(b_prime).insts.push_back(node);
        value_map.extend(
            function
                .dfg
                .inst_results(src_inst)
                .iter()
                .copied()
                .zip(function.dfg.inst_results(inst).iter().copied()),
        );
    }
}

struct CopyBlock {
    b: BlockId,
    p: BlockPredecessor,
    value_map: ScopedMap<ValueId, ValueId>,
    block_map: ScopedMap<BlockId, BlockId>,
}
impl CopyBlock {
    fn new(b: BlockId, p: BlockPredecessor) -> Self {
        Self {
            b,
            p,
            value_map: Default::default(),
            block_map: Default::default(),
        }
    }
}

#[inline]
fn update_predecessor<F>(function: &mut hir::Function, p: &BlockPredecessor, mut callback: F)
where
    F: FnMut(&mut hir::Successor, &mut ValueListPool),
{
    match &mut *function.dfg.insts[p.inst].data {
        Instruction::Br(hir::Br {
            ref mut successor, ..
        }) => {
            callback(successor, &mut function.dfg.value_lists);
        }
        Instruction::CondBr(hir::CondBr {
            ref mut then_dest,
            ref mut else_dest,
            ..
        }) => {
            assert_ne!(then_dest.destination, else_dest.destination, "unexpected critical edge");
            let value_lists = &mut function.dfg.value_lists;
            callback(then_dest, value_lists);
            callback(else_dest, value_lists);
        }
        Instruction::Switch(_) => {
            panic!("expected switch instructions to have been simplified prior to treeification")
        }
        _ => unreachable!(),
    }
}

struct BlockInfos {
    blocks: BTreeMap<BlockId, BlockId>,
    cfg: Rc<ControlFlowGraph>,
    domtree: Rc<DominatorTree>,
    loops: Rc<LoopAnalysis>,
}
impl BlockInfos {
    pub fn new(
        cfg: Rc<ControlFlowGraph>,
        domtree: Rc<DominatorTree>,
        loops: Rc<LoopAnalysis>,
    ) -> Self {
        Self {
            blocks: Default::default(),
            cfg,
            domtree,
            loops,
        }
    }

    pub fn insert_copy(&mut self, copied: BlockId, original: BlockId) {
        let resolved = self.to_original_block(original);
        self.blocks.insert(copied, resolved);
    }

    pub fn is_loop_header(&self, block_id: BlockId) -> Option<Loop> {
        let resolved = self.to_original_block(block_id);
        self.loops.is_loop_header(resolved)
    }

    pub fn rpo_cmp(&self, a: BlockId, b: BlockId) -> core::cmp::Ordering {
        let a_orig = self.to_original_block(a);
        let b_orig = self.to_original_block(b);
        self.domtree.rpo_cmp_block(a_orig, b_orig)
    }

    fn to_original_block(&self, mut block_id: BlockId) -> BlockId {
        loop {
            if let Some(copied_from) = self.blocks.get(&block_id).copied() {
                block_id = copied_from;
                continue;
            }

            break block_id;
        }
    }
}

#[cfg(test)]
mod tests {
    use midenc_hir::{
        pass::{AnalysisManager, RewritePass},
        testing::{self, TestContext},
        ModuleBuilder,
    };
    use pretty_assertions::{assert_eq, assert_ne};

    use crate::Treeify;

    /// Run the treeify pass on the IR of the [testing::sum_matrix] function.
    ///
    /// This function corresponds forms a directed, cyclic graph; containing a loop
    /// two levels deep, with control flow paths that join multiple predecessors.
    /// It has no critical edges, as if we had already run the [SplitCriticalEdges]
    /// pass, and doesn't contain any superfluous blocks:
    ///
    /// We expect this pass to identify that the exit block, `blk0` has multiple predecessors
    /// and is not a loop header, and thus a candidate for treeification. We expect `blk0`
    /// to be duplicated, so that each of it's predecessors, `entry` and `blk2` respectively,
    /// have their own copies of the block. The terminators of those blocks should be
    /// updated accordingly. Additionally, because the new versions of `blk0` have only
    /// a single predecessor, the block arguments previously needed, should be removed
    /// and the `ret` instruction should directly reference the return value originally
    /// provided via `entry`/`blk2`.
    #[test]
    fn treeify_simple_test() {
        let context = TestContext::default();

        // Define the 'test' module
        let mut builder = ModuleBuilder::new("test");
        let id = testing::sum_matrix(&mut builder, &context);
        let mut module = builder.build();
        let mut function = module.cursor_mut_at(id.function).remove().expect("undefined function");

        let original = function.to_string();
        let mut analyses = AnalysisManager::default();
        let mut rewrite = Treeify;
        rewrite
            .apply(&mut function, &mut analyses, &context.session)
            .expect("treeification failed");

        let expected = "\
(func (export #sum_matrix)
      (param (ptr u32)) (param u32) (param u32) (result u32)
    (block 0 (param v0 (ptr u32)) (param v1 u32) (param v2 u32)
        (let (v10 u32) (const.u32 0))
        (let (v11 u32) (ptrtoint v0))
        (let (v12 i1) (neq v11 0))
        (condbr v12 (block 2) (block 7)))

    (block 7
        (ret v10))

    (block 2
        (let (v13 u32) (const.u32 0))
        (let (v14 u32) (const.u32 0))
        (let (v15 u32) (mul.checked v2 4))
        (br (block 3 v10 v13 v14)))

    (block 3 (param v4 u32) (param v5 u32) (param v6 u32)
        (let (v16 i1) (lt v5 v1))
        (let (v17 u32) (mul.checked v5 v15))
        (condbr v16 (block 4 v4 v5 v6) (block 8)))

    (block 8
        (ret v4))

    (block 4 (param v7 u32) (param v8 u32) (param v9 u32)
        (let (v18 i1) (lt v9 v2))
        (condbr v18 (block 5) (block 6)))

    (block 5
        (let (v19 u32) (mul.checked v9 4))
        (let (v20 u32) (add.checked v17 v19))
        (let (v21 u32) (add.checked v11 v20))
        (let (v22 (ptr u32)) (inttoptr v21))
        (let (v23 u32) (load v22))
        (let (v24 u32) (add.checked v7 v23))
        (let (v25 u32) (incr.wrapping v9))
        (br (block 4 v24 v8 v25)))

    (block 6
        (let (v26 u32) (incr.wrapping v8))
        (let (v27 u32) (const.u32 0))
        (br (block 3 v7 v26 v27)))
)";

        let transformed = function.to_string();
        assert_ne!(transformed, original);
        assert_eq!(transformed.as_str(), expected);
    }
}
