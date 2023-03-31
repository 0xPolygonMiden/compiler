use miden_diagnostics::SourceSpan;

use crate::types::Type;

use super::*;

pub trait InstBuilderBase<'f> {
    fn data_flow_graph(&self) -> &DataFlowGraph;
    fn data_flow_graph_mut(&mut self) -> &mut DataFlowGraph;
    fn build(
        self,
        data: Instruction,
        ctrl_ty: Type,
        span: SourceSpan,
    ) -> (Inst, &'f mut DataFlowGraph);
}

pub trait InstBuilder<'f>: InstBuilderBase<'f> {}

impl<'f, T: InstBuilderBase<'f>> InstBuilder<'f> for T {}
