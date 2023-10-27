use std::collections::VecDeque;

use rustc_hash::FxHashSet;
use smallvec::SmallVec;

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
                let span = function.dfg.inst_span(terminator);
                let ix = function.dfg.inst_mut(terminator);
                let args: ValueList;
                match ix {
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

#[cfg(test)]
mod tests {
    use miden_hir::{
        AbiParam, Function, FunctionBuilder, Immediate, InstBuilder, Signature, SourceSpan, Type,
    };
    use miden_hir_analysis::FunctionAnalysis;
    use pretty_assertions::{assert_eq, assert_ne};

    use crate::{RewritePass, SplitCriticalEdges};

    /// Run the split critical edges pass on the following IR:
    ///
    /// The following IR is contains a critical edge to split, specifically
    /// `blk0` is critical because it has multiple predecessors, and multiple
    /// successors:
    ///
    /// ```text,ignore
    /// pub fn test(*mut u8, u32) -> *mut u8 {
    /// entry(ptr0: *mut u8, n0: u32):
    ///    ptr1 = ptrtoint ptr0 : u32;
    ///    br blk0(ptr1, n0);
    ///
    /// blk0(ptr2: u32, n1: u32):
    ///    is_null = eq ptr2, 0;
    ///    condbr is_null, blk2(ptr0), blk1(ptr2, n1);
    ///
    /// blk1(ptr3: u32, n2: u32):
    ///    ptr4 = sub ptr3, n2;
    ///    n3 = sub n2, 1;
    ///    is_zero = eq n3, 0;
    ///    condbr is_zero, blk2(ptr4), blk0(ptr4, n3);
    ///
    /// blk2(result0: *mut u8)
    ///    ret result0;
    /// }
    /// ```
    ///
    /// We expect this pass to introduce new blocks along all control flow paths
    /// where the successor has multiple predecessors. This may result in some
    /// superfluous blocks after the pass is run, but this can be addressed by
    /// running the [InlineBlocks] pass afterwards, which will flatten the CFG.
    #[test]
    fn split_critical_edges_simple_test() {
        let id = "test::sce".parse().unwrap();
        let mut function = Function::new(
            id,
            Signature::new(
                [
                    AbiParam::new(Type::Ptr(Box::new(Type::U8))),
                    AbiParam::new(Type::U32),
                ],
                [AbiParam::new(Type::Ptr(Box::new(Type::U8)))],
            ),
        );

        {
            let mut builder = FunctionBuilder::new(&mut function);
            let entry = builder.current_block();
            let (ptr0, n0) = {
                let args = builder.block_params(entry);
                (args[0], args[1])
            };

            let a = builder.create_block(); // blk0(ptr2: u32, n1: u32)
            let ptr2 = builder.append_block_param(a, Type::U32, SourceSpan::UNKNOWN);
            let n1 = builder.append_block_param(a, Type::U32, SourceSpan::UNKNOWN);
            let b = builder.create_block(); // blk1(ptr3: u32, n2: u32)
            let ptr3 = builder.append_block_param(b, Type::U32, SourceSpan::UNKNOWN);
            let n2 = builder.append_block_param(b, Type::U32, SourceSpan::UNKNOWN);
            let c = builder.create_block(); // blk2(result0: u32)
            let result0 = builder.append_block_param(c, Type::U32, SourceSpan::UNKNOWN);

            // entry
            let ptr1 = builder.ins().ptrtoint(ptr0, Type::U32, SourceSpan::UNKNOWN);
            builder.ins().br(a, &[ptr1, n0], SourceSpan::UNKNOWN);

            // blk0
            builder.switch_to_block(a);
            let is_null = builder
                .ins()
                .eq_imm(ptr2, Immediate::U32(0), SourceSpan::UNKNOWN);
            builder
                .ins()
                .cond_br(is_null, c, &[ptr0], b, &[ptr2, n1], SourceSpan::UNKNOWN);

            // blk1
            builder.switch_to_block(b);
            let ptr4 = builder.ins().sub(ptr3, n2, SourceSpan::UNKNOWN);
            let n3 = builder
                .ins()
                .sub_imm(n2, Immediate::U32(1), SourceSpan::UNKNOWN);
            let is_zero = builder
                .ins()
                .eq_imm(n3, Immediate::U32(0), SourceSpan::UNKNOWN);
            builder
                .ins()
                .cond_br(is_zero, c, &[ptr4], a, &[ptr4, n3], SourceSpan::UNKNOWN);

            // blk2
            builder.switch_to_block(c);
            let result1 =
                builder
                    .ins()
                    .inttoptr(result0, Type::Ptr(Box::new(Type::U8)), SourceSpan::UNKNOWN);
            builder.ins().ret(Some(result1), SourceSpan::UNKNOWN);
        }

        let original = function.to_string();
        let mut analysis = FunctionAnalysis::new(&function);
        let mut pass = SplitCriticalEdges;
        pass.run(&mut function, &mut analysis)
            .expect("splitting critical edges failed");

        let expected = "pub fn sce(*mut u8, u32) -> *mut u8 {
block0(v0: *mut u8, v1: u32):
    v7 = ptrtoint v0 : u32;
    br block1(v7, v1);

block1(v2: u32, v3: u32):
    v8 = eq v2, 0 : i1;
    condbr v8, block4, block2(v2, v3);

block4:
    br block3(v0);

block2(v4: u32, v5: u32):
    v9 = sub v4, v5 : u32;
    v10 = sub v5, 1 : u32;
    v11 = eq v10, 0 : i1;
    condbr v11, block6, block5;

block6:
    br block3(v9);

block5:
    br block1(v9, v10);

block3(v6: u32):
    v12 = inttoptr v6 : *mut u8;
    ret v12;
}
";

        let transformed = function.to_string();
        assert_ne!(transformed, original);
        assert_eq!(transformed.as_str(), expected);
    }
}
