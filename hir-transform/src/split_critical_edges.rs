use std::collections::VecDeque;

use rustc_hash::FxHashSet;
use smallvec::SmallVec;

use miden_diagnostics::Spanned;
use miden_hir::{self as hir, Block as BlockId, *};
use miden_hir_analysis::FunctionAnalysis;

use super::RewritePass;

/// This pass operates on the SSA IR, and ensures that there are no critical
/// edges in the control flow graph.
///
/// A critical edge occurs when control flow may exit a block, which we'll call `P`, to
/// more than one successor block, which we'll call `S`, where any `S` has more than one
/// predecessor from which it may receive control. Put another way, in the control flow graph,
/// a critical edge is one which connects two nodes where the source node has multiple outgoing
/// edges, and the destination node has multiple incoming edges.
///
/// These types of edges cause unnecessary complications with certain types of dataflow analyses
/// and transformations, and so we fix this by splitting these edges. This is done by introducing
/// a new block, `B`, in which we insert a branch to `S` with whatever arguments were originally
/// provided in `P`, and then rewriting the branch in `P` that went to `S`, to go to `B` instead.
///
/// After this pass completes, no node in the control flow graph will have both multiple predecessors
/// and multiple successors.
///
pub struct SplitCriticalEdges;
impl RewritePass for SplitCriticalEdges {
    type Error = anyhow::Error;

    fn run(
        &mut self,
        function: &mut hir::Function,
        analysis: &mut FunctionAnalysis,
    ) -> Result<(), Self::Error> {
        // Search for blocks with multiple successors with edges to blocks with
        // multiple predecessors; these blocks form critical edges in the control
        // flow graph which must be split.
        //
        // We split the critical edge by inserting a new block after the predecessor
        // and updating the predecessor instruction to transfer to the new block
        // instead. We then insert an unconditional branch in the new block that
        // passes the block arguments that were meant for the "real" successor.
        let mut visited = FxHashSet::<BlockId>::default();
        let mut worklist = VecDeque::<BlockId>::default();
        worklist.push_back(function.dfg.entry_block());

        let cfg = analysis.cfg_mut();

        while let Some(p) = worklist.pop_front() {
            // If we've already visited a block, skip it
            if !visited.insert(p) {
                continue;
            }

            // Make sure we visit all of the successors of this block next
            for b in cfg.succ_iter(p) {
                worklist.push_back(b);
            }

            // Unless this block has multiple successors, skip it
            if cfg.num_successors(p) < 2 {
                continue;
            }

            let succs = SmallVec::<[BlockId; 2]>::from_iter(cfg.succ_iter(p));
            for b in succs.into_iter() {
                // Unless this successor has multiple predecessors, skip it
                if cfg.num_predecessors(b) < 2 {
                    continue;
                }

                // We found a critical edge, so perform the following steps:
                //
                // * Create a new block, placed after the predecessor in the layout
                // * Rewrite the terminator of the predecessor to refer to the new
                // block, but without passing any block arguments
                // * Insert an unconditional branch to the successor with the block
                // arguments of the original terminator
                // * Recompute the control flow graph for affected blocks
                let split = function.dfg.create_block_after(p);
                let terminator = function.dfg.last_inst(p).unwrap();
                let ix = function.dfg.inst_mut(terminator);
                let span = ix.span();
                let args: ValueList;
                match &mut ix.data.item {
                    Instruction::Br(hir::Br {
                        ref mut destination,
                        args: ref mut orig_args,
                        ..
                    }) => {
                        args = orig_args.take();
                        *destination = split;
                    }
                    Instruction::CondBr(hir::CondBr {
                        then_dest: (ref mut then_dest, ref mut then_args),
                        else_dest: (ref mut else_dest, ref mut else_args),
                        ..
                    }) => {
                        if *then_dest == b {
                            *then_dest = split;
                            args = then_args.take();
                        } else {
                            *else_dest = split;
                            args = else_args.take();
                        }
                    }
                    Instruction::Switch(_) => unimplemented!(),
                    _ => unreachable!(),
                }
                function.dfg.insert_inst(
                    InsertionPoint {
                        at: ProgramPoint::Block(split),
                        action: Insert::After,
                    },
                    Instruction::Br(hir::Br {
                        op: hir::Opcode::Br,
                        destination: b,
                        args,
                    }),
                    Type::Unknown,
                    span,
                );

                cfg.recompute_block(&function.dfg, split);
            }

            cfg.recompute_block(&function.dfg, p);
        }

        Ok(())
    }
}
