use std::collections::VecDeque;

use midenc_hir::{
    self as hir,
    pass::{AnalysisManager, RewritePass, RewriteResult},
    *,
};
use midenc_hir_analysis::{ControlFlowGraph, DominatorTree};
use midenc_session::{diagnostics::IntoDiagnostic, Session};
use rustc_hash::FxHashSet;
use smallvec::SmallVec;

use crate::adt::ScopedMap;

/// This pass converts an unstructured CFG, into an equivalent CFG in which all control flow ops
/// have been converted into equivalent structured control flow ops.
///
/// For example, a CFG in which a block conditionally branches to two other blocks, both of which
/// branch to a fourth block, represents a typical if/then/else construct, and this pass will "lift"
/// such a structure into the `if.true` instruction, where the bodies of the then and else branches
/// are contained in regions, and the `br` instructions in those blocks are converted to `yield`
/// instructions to ensure control flow is contained within the containing region. A similar
/// conversion is done for loops that can be represented using the `while.true` instruction.
///
/// The end result of this transformation is a structured CFG which has a straightforward lowering
/// to Miden Assembly.
///
/// This code is inspired by/derived from _Perfect Reconstructability of Control Flow from Demand
/// Dependence Graphs_, 2015, by Helge Bahmann, Nico Reissmann, Magnus Jahre, and Jan Christian
/// Meyer. ACM Trans. Archit. Code Optim. 11, 4, Article 66 (January 2015), 25 pages.
/// https://doi.org/10.1145/2693261
///
/// The algorithm here consists of a pair of transformations applied in sequence, to any single-
/// entry, single-exit region:
///
/// 1. Lifting cycles to `while.true`
/// 2. Lifting conditional branches to `if.true`
///
/// These are applied recursively on any new single-entry, single-exit regions created by the
/// transformation, until no more unstructured control flow ops remain.
#[derive(Default, PassInfo, ModuleRewritePassAdapter)]
pub struct CfgToScf;
impl RewritePass for CfgToScf {
    type Entity = hir::Function;

    fn apply(
        &mut self,
        function: &mut Self::Entity,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> RewriteResult {
        let domtree = analyses.get_or_compute::<DominatorTree>(function, session)?;

        let mut blockq = VecDeque::from_iter(domtree.cfg_postorder().iter().copied());
        let mut instq = Vec::default();

        while let Some(block) = blockq.pop_front() {
            instq.extend(function.dfg.block_insts(block));

            while let Some(inst) = instq.pop() {}
        }
    }
}
