mod control_flow;
mod dominance;
mod liveness;
mod loops;

pub use self::control_flow::{BlockPredecessor, ControlFlowGraph};
pub use self::dominance::{DominanceFrontier, DominatorTree, DominatorTreePreorder};
pub use self::liveness::LivenessAnalysis;
pub use self::loops::{Loop, LoopAnalysis, LoopLevel};

pub struct ControlFlowAnalysis {
    pub cfg: ControlFlowGraph,
    pub domtree: DominatorTree,
    pub loops: LoopAnalysis,
}
impl ControlFlowAnalysis {
    pub fn compute(function: &miden_hir::Function) -> Self {
        let cfg = ControlFlowGraph::with_function(function);
        let domtree = DominatorTree::with_function(function, &cfg);
        let loops = LoopAnalysis::with_function(function, &cfg, &domtree);

        Self {
            cfg,
            domtree,
            loops,
        }
    }

    pub fn compute_liveness(&self, function: &miden_hir::Function) -> LivenessAnalysis {
        LivenessAnalysis::compute(function, self)
    }

    pub fn recompute(&mut self, function: &miden_hir::Function) {
        self.cfg.compute(&function.dfg);
        self.domtree.compute(function, &self.cfg);
        self.loops.compute(function, &self.cfg, &self.domtree);
    }
}
