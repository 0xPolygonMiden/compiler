use std::collections::VecDeque;

use rustc_hash::FxHashSet;

use miden_hir::{self as hir, Block as BlockId, *};
use miden_hir_analysis::{ControlFlowGraph, FunctionAnalysis};

use super::RewritePass;

/// This pass operates on the SSA IR, and inlines superfluous blocks which serve no
/// purpose. Such blocks have no block arguments, and have a single predecessor.
///
/// Blocks like this may have been introduced for the following reasons:
///
/// * Due to less than optimal lowering to SSA form
/// * To split critical edges in preparation for dataflow analysis and related transformations,
/// but ultimately no code introduced along those edges, and critical edges no longer present
/// an obstacle to further optimization or codegen.
/// * During treeification of the CFG, where blocks with multiple predecessors were duplicated
/// to produce a CFG in tree form, where no blocks (other than loop headers) have multiple
/// predecessors. This process removed block arguments from these blocks, and rewrote instructions
/// dominated by those block arguments to reference the values passed from the original predecessor
/// to whom the subtree is attached. This transformation can expose a chain of blocks which all have
/// a single predecessor and successor, introducing branches where none are needed, and by removing
/// those redundant branches, all of the code from blocks in the chain can be inlined in the first
/// block of the chain.
pub struct InlineBlocks;
impl RewritePass for InlineBlocks {
    type Error = anyhow::Error;

    fn run(
        &mut self,
        function: &mut hir::Function,
        analysis: &mut FunctionAnalysis,
    ) -> Result<(), Self::Error> {
        let cfg = analysis.cfg_mut();

        let mut changed = false;
        let mut visited = FxHashSet::<BlockId>::default();
        let mut worklist = VecDeque::<BlockId>::default();
        worklist.push_back(function.dfg.entry_block());

        // First, search down the CFG for non-loop header blocks with only a single successor.
        // These blocks form possible roots of a chain of blocks that can be inlined.
        //
        // For each such root, we then check if the successor block has a single predecessor,
        // if so, then we can remove the terminator instruction from the root block, and then
        // move all of the code from the successor block into the root block. We can then repeat
        // this process until we inline a terminator instruction that is not an unconditional branch
        // to a single successor.
        while let Some(p) = worklist.pop_front() {
            // If we've already visited a block, skip it
            if !visited.insert(p) {
                continue;
            }

            // If this block has multiple successors, or multiple predecessors, add all of it's
            // successors to the work queue and move on.
            if cfg.num_successors(p) > 1 || cfg.num_predecessors(p) > 1 {
                for b in cfg.succ_iter(p) {
                    worklist.push_back(b);
                }
                continue;
            }

            // This block is a candidate for inlining
            //
            // If inlining can proceed, do so until we reach a point where the inlined terminator
            // returns from the function, has multiple successors, or branches to a block with
            // multiple predecessors.
            while let BranchInfo::SingleDest(b, args) = function
                .dfg
                .analyze_branch(function.dfg.last_inst(p).unwrap())
            {
                // If this successor has other predecessors, it can't be inlined, so
                // add it to the work list and move on
                if cfg.num_predecessors(b) > 1 {
                    worklist.push_back(b);
                    break;
                }

                // Only inline if the successor has no block arguments
                //
                // TODO: We can inline blocks with arguments as well, but with higher cost,
                // as we must visit all uses of the block arguments and update them. This
                // is left as a future extension of this pass should we find that it is
                // valuable as an optimization.
                if !args.is_empty() {
                    break;
                }

                inline(b, p, function, cfg);

                // Mark that the control flow graph as modified
                changed = true;
            }
        }

        if changed {
            analysis.cfg_changed(function);
        }

        Ok(())
    }
}

fn inline(from: BlockId, to: BlockId, function: &mut hir::Function, cfg: &mut ControlFlowGraph) {
    assert_ne!(from, to);
    {
        let mut from_insts = function.dfg.block_mut(from).insts.take();
        let to_insts = &mut function.dfg.block_mut(to).insts;
        // Remove the original terminator
        to_insts.pop_back();
        // Move all instructions from their original block to the parent,
        // updating the instruction data along the way to reflect the change
        // in location
        while let Some(unsafe_ix_ref) = from_insts.pop_front() {
            let ix_ptr = UnsafeRef::into_raw(unsafe_ix_ref);
            unsafe {
                let ix = &mut *ix_ptr;
                ix.block = to;
            }
            to_insts.push_back(unsafe { UnsafeRef::from_raw(ix_ptr) });
        }
    }
    // Detach the original block from the function
    function.dfg.detach_block(from);
    // Update the control flow graph to reflect the changes
    cfg.detach_block(from);
    cfg.recompute_block(&function.dfg, to);
}
