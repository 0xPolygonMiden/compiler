use std::collections::VecDeque;
use std::rc::Rc;

use miden_hir::{self as hir, Block as BlockId, Value as ValueId, *};
use miden_hir_analysis::{BlockPredecessor, ControlFlowGraph, FunctionAnalysis, LoopAnalysis};
use rustc_hash::FxHashSet;

use crate::{adt::ScopedMap, RewritePass};

/// This pass takes as input the SSA form of a function, and ensures that the CFG of
/// that function is a tree, not a DAG, excepting loop headers.
///
/// This transformation splits vertices with multiple predecessors, by duplicating the
/// subtree of the program rooted at those vertices. As mentioned above, we do not split
/// vertices representing loop headers, in order to preserve loops in the CFG of the resulting
/// IR. However, we can consider each loop within the overall CFG of a function to be a single
/// vertex after this transformation, and with this perspective the CFG forms a tree. Loop
/// nodes are then handled specially during codegen.
///
/// The transformation is performed bottom-up, in CFG postorder.
///
/// This pass also computes the set of blocks in each loop which must be terminated with `push.0`
/// to exit the containing loop.
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
///
pub struct Treeify;
impl RewritePass for Treeify {
    type Error = anyhow::Error;

    fn run(
        &mut self,
        function: &mut hir::Function,
        analysis: &mut FunctionAnalysis,
    ) -> Result<(), Self::Error> {
        // Require the dominator tree and loop analyses
        analysis.ensure_loops(function);

        let cfg = analysis.cfg();
        let domtree = analysis.domtree();
        let loops = analysis.loops();
        let mut block_q = VecDeque::<CopyBlock>::default();
        let mut changed = false;

        for b in domtree.cfg_postorder().iter().copied() {
            if loops.is_loop_header(b).is_some() {
                // Ignore loop headers
                continue;
            }

            // Blocks with multiple predecessors cause the CFG to form a DAG,
            // we need to duplicate the CFG rooted at this block for all predecessors.
            //
            // While we could technically preserve one of the predecessors, we perform
            // some transformations during the copy that would result in copied vs original
            // trees to differ slightly, which would inhibit subsequent optimizations.
            // The original subtree blocks are detached from the function.
            if cfg.num_predecessors(b) > 1 {
                for p in cfg.pred_iter(b) {
                    assert!(block_q.is_empty());
                    block_q.push_back(CopyBlock::new(b, p));
                    while let Some(CopyBlock {
                        b,
                        ref p,
                        value_map,
                        block_map,
                    }) = block_q.pop_front()
                    {
                        // Copy this block and its children
                        if loops.is_loop_header(b).is_some() {
                            treeify_loop(
                                b,
                                p,
                                function,
                                cfg,
                                loops,
                                &mut block_q,
                                value_map,
                                block_map,
                            )?;
                        } else {
                            treeify(
                                b,
                                p,
                                function,
                                cfg,
                                loops,
                                &mut block_q,
                                value_map,
                                block_map,
                            )?;
                        }
                    }
                }

                // After treeification, the original subtree blocks cannot possibly be
                // referenced by other blocks in the function, so remove all of them
                detach_tree(b, function, cfg);

                // Mark the control flow graph as modified
                changed = true;
            }
        }

        // If we made any changes, we need to recompute all analyses
        if changed {
            analysis.recompute(function);
        }

        Ok(())
    }
}

fn treeify(
    b: BlockId,
    p: &BlockPredecessor,
    function: &mut hir::Function,
    cfg: &ControlFlowGraph,
    loops: &LoopAnalysis,
    block_q: &mut VecDeque<CopyBlock>,
    mut value_map: ScopedMap<ValueId, ValueId>,
    mut block_map: ScopedMap<BlockId, BlockId>,
) -> anyhow::Result<()> {
    // 1. Create a new block `b'`, without block arguments,
    let b_prime = function.dfg.create_block_after(p.block);
    block_map.insert(b, b_prime);
    // 2. Initialize a lookup table of old value defs to new value defs, seed it by mapping the
    //    block arguments of `b` to the values passed from the predecessor
    match function.dfg.analyze_branch(p.inst) {
        BranchInfo::NotABranch => {
            value_map.extend(
                function
                    .dfg
                    .block_args(b)
                    .iter()
                    .copied()
                    .zip(function.dfg.inst_args(p.inst).iter().copied()),
            );
        }
        BranchInfo::SingleDest(_, args) => {
            value_map.extend(
                function
                    .dfg
                    .block_args(b)
                    .iter()
                    .copied()
                    .zip(args.iter().copied()),
            );
        }
        BranchInfo::MultiDest(ref jts) => {
            for jt in jts.iter() {
                if jt.destination == b {
                    value_map.extend(
                        function
                            .dfg
                            .block_args(b)
                            .iter()
                            .copied()
                            .zip(jt.args.iter().copied()),
                    );
                    break;
                }
            }
        }
    }
    // 3. Update the predecessor instruction to reference the new block, remove block arguments.
    update_predecessor(function, p, |dest, dest_args, pool| {
        if *dest == b {
            *dest = b_prime;
            dest_args.clear(pool);
        }
    });
    // 4. Copy contents of `b` to `b'`, inserting defs in the lookup table, and mapping operands
    //    to their new "corrected" values
    copy_instructions(b, b_prime, function, &mut value_map, &block_map);
    // 5. Recursively copy all children of `b` to `b_prime`
    copy_children(
        b, b_prime, function, cfg, loops, block_q, value_map, block_map,
    )
}

fn treeify_loop(
    b: BlockId,
    p: &BlockPredecessor,
    function: &mut hir::Function,
    cfg: &ControlFlowGraph,
    loops: &LoopAnalysis,
    block_q: &mut VecDeque<CopyBlock>,
    mut value_map: ScopedMap<ValueId, ValueId>,
    mut block_map: ScopedMap<BlockId, BlockId>,
) -> anyhow::Result<()> {
    // 1. Create new block, b', with a new set of block arguments matching the original,
    // populate the value map with rewrites for the original block argument values
    let b_prime = function.dfg.create_block_after(p.block);
    block_map.insert(b, b_prime);
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
    // 2. Update the predecessor instruction to reference the new block, leave block arguments unchanged
    update_predecessor(function, p, |dest, _, _| {
        if *dest == b {
            *dest = b_prime;
        }
    });
    // 3. Copy contents of `b` to `b'`, inserting defs in the lookup table, and mapping operands
    //    to their new "corrected" values
    copy_instructions(b, b_prime, function, &mut value_map, &block_map);
    // 4. Recursively copy all children of `b` to `b_prime`
    copy_children(
        b, b_prime, function, cfg, loops, block_q, value_map, block_map,
    )
}

/// Detach `root`, and all of it's reachable children, from the layout of `function`
///
/// When called, it is assumed that `root` has been cloned to a new block,
/// along with all of it's reachable children, and its predecessor rewritten
/// to refer to the new block instead. As a result, `root` should no longer be
/// reachable in the CFG, along with its children, as they would have been cloned
/// as well.
///
/// NOTE: This does not delete the block data attached to the function, only the
/// presence of the block in the layout of the function.
fn detach_tree(root: BlockId, function: &mut hir::Function, cfg: &ControlFlowGraph) {
    let mut delete_q = VecDeque::<BlockId>::default();
    let mut visited = FxHashSet::<BlockId>::default();
    delete_q.push_back(root);
    visited.insert(root);
    while let Some(block) = delete_q.pop_front() {
        function.dfg.detach_block(block);
        for b in cfg.succ_iter(block).into_iter() {
            // Skip blocks we've already seen
            if visited.insert(b) {
                delete_q.push_back(b);
            }
        }
    }
}

fn copy_children(
    b: BlockId,
    b_prime: BlockId,
    function: &mut hir::Function,
    cfg: &ControlFlowGraph,
    loops: &LoopAnalysis,
    block_q: &mut VecDeque<CopyBlock>,
    value_map: ScopedMap<ValueId, ValueId>,
    block_map: ScopedMap<BlockId, BlockId>,
) -> anyhow::Result<()> {
    let pred = BlockPredecessor {
        inst: function
            .dfg
            .last_inst(b_prime)
            .expect("expected non-empty block"),
        block: b_prime,
    };
    let value_map = Rc::new(value_map);
    let block_map = Rc::new(block_map);
    for succ in cfg.succ_iter(b) {
        // If we've already seen this successor, and it is a loop header, then
        // we don't want to copy it, but we do want to replace the reference to
        // this block with its copy
        if let Some(succ_prime) = block_map.get(&succ) {
            if loops.is_loop_header(succ).is_some() {
                update_predecessor(function, &pred, |dest, _, _| {
                    if dest == &succ {
                        *dest = *succ_prime;
                    }
                });
                continue;
            }
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
                ref mut destination,
                ref mut args,
                ..
            }) => {
                if let Some(new_dest) = block_map.get(destination) {
                    *destination = *new_dest;
                }
                let args = args.as_mut_slice(&mut function.dfg.value_lists);
                for arg in args.iter_mut() {
                    if let Some(arg_prime) = value_map.get(arg) {
                        *arg = *arg_prime;
                    }
                }
            }
            Instruction::CondBr(hir::CondBr {
                ref mut cond,
                then_dest: (ref mut then_dest, ref mut then_args),
                else_dest: (ref mut else_dest, ref mut else_args),
                ..
            }) => {
                if let Some(cond_prime) = value_map.get(cond) {
                    *cond = *cond_prime;
                }
                if let Some(new_dest) = block_map.get(then_dest) {
                    *then_dest = *new_dest;
                }
                let then_args = then_args.as_mut_slice(&mut function.dfg.value_lists);
                for arg in then_args.iter_mut() {
                    if let Some(arg_prime) = value_map.get(arg) {
                        *arg = *arg_prime;
                    }
                }
                if let Some(new_dest) = block_map.get(else_dest) {
                    *else_dest = *new_dest;
                }
                let else_args = else_args.as_mut_slice(&mut function.dfg.value_lists);
                for arg in else_args.iter_mut() {
                    if let Some(arg_prime) = value_map.get(arg) {
                        *arg = *arg_prime;
                    }
                }
            }
            other => {
                for arg in other
                    .arguments_mut(&mut function.dfg.value_lists)
                    .iter_mut()
                {
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
    F: FnMut(&mut BlockId, &mut ValueList, &mut ValueListPool),
{
    match &mut function.dfg.insts[p.inst].data.item {
        Instruction::Br(hir::Br {
            ref mut destination,
            ref mut args,
            ..
        }) => {
            callback(destination, args, &mut function.dfg.value_lists);
        }
        Instruction::CondBr(hir::CondBr {
            then_dest: (ref mut then_dest, ref mut then_args),
            else_dest: (ref mut else_dest, ref mut else_args),
            ..
        }) => {
            assert_ne!(then_dest, else_dest, "unexpected critical edge");
            let value_lists = &mut function.dfg.value_lists;
            callback(then_dest, then_args, value_lists);
            callback(else_dest, else_args, value_lists);
        }
        Instruction::Switch(_) => {
            panic!("expected switch instructions to have been simplified prior to treeification")
        }
        _ => unreachable!(),
    }
}
