mod control_flow;
mod dominance;
mod liveness;
mod loops;
mod validation;

pub use self::control_flow::{BlockPredecessor, ControlFlowGraph};
pub use self::dominance::{DominanceFrontier, DominatorTree, DominatorTreePreorder};
pub use self::liveness::LivenessAnalysis;
pub use self::loops::{Loop, LoopAnalysis, LoopLevel};
pub use self::validation::{ModuleValidator, Rule};

use anyhow::anyhow;

/// This structure provides access to various analyses for a single [miden_hir::Function].
///
/// The default analysis available upon construction is the [ControlFlowGraph], which is
/// required by all other analyses, and can be used to query basic information about the
/// structure of the program in terms of basic blocks (i.e. block predecessors and successors).
///
/// All other analyses must be explicitly required via the various `ensure_*` functions,
/// which will only compute each analysis once, unless `recompute` is called.
///
/// It is up to the owner of this structure to ensure that analyses are recomputed when
/// the original function is modified. This is not done automatically, so forgetting to
/// can result in unexpected compilation failures, or worse, miscompilation.
pub struct FunctionAnalysis {
    cfg: ControlFlowGraph,
    domtree: Option<DominatorTree>,
    loops: Option<LoopAnalysis>,
    liveness: Option<LivenessAnalysis>,
}
impl FunctionAnalysis {
    /// Construct the default analysis results for `function`
    ///
    /// The only analysis which is computed immediately is the [ControlFlowGraph], all
    /// others are lazily loaded as required by various compiler passes.
    pub fn new(function: &miden_hir::Function) -> Self {
        let cfg = ControlFlowGraph::with_function(function);

        Self {
            cfg,
            domtree: None,
            loops: None,
            liveness: None,
        }
    }

    /// Get a reference to the current [ControlFlowGraph] analysis
    #[inline(always)]
    pub fn cfg(&self) -> &ControlFlowGraph {
        &self.cfg
    }

    /// Get a mutable reference to the current [ControlFlowGraph] analysis
    #[inline(always)]
    pub fn cfg_mut(&mut self) -> &mut ControlFlowGraph {
        &mut self.cfg
    }

    /// Get a reference to the current [DominatorTree] analysis
    ///
    /// NOTE: This function will panic if the analysis has not yet been computed.
    pub fn domtree(&self) -> &DominatorTree {
        self.domtree
            .as_ref()
            .expect("dominator tree analysis is unavailable")
    }

    /// Get a reference to the current [LoopAnalysis] analysis
    ///
    /// NOTE: This function will panic if the analysis has not yet been computed.
    pub fn loops(&self) -> &LoopAnalysis {
        self.loops.as_ref().expect("loop analysis is unavailable")
    }

    /// Get a reference to the current [LoopAnalysis] analysis
    ///
    /// NOTE: This function will panic if the analysis has not yet been computed.
    pub fn liveness(&self) -> &LivenessAnalysis {
        self.liveness
            .as_ref()
            .expect("liveness analysis is unavailable")
    }

    /// Returns an error if the [DominatorTree] analysis is not available
    pub fn require_domtree(&self) -> anyhow::Result<()> {
        if self.domtree.is_some() {
            Ok(())
        } else {
            Err(anyhow!(
                "missing analysis requirement: dominator tree analysis is unavailable"
            ))
        }
    }

    /// Returns an error if the [LoopAnalysis] analysis is not available
    pub fn require_loops(&self) -> anyhow::Result<()> {
        if self.loops.is_some() {
            Ok(())
        } else {
            Err(anyhow!(
                "missing analysis requirement: loop analysis is unavailable"
            ))
        }
    }

    /// Returns an error if the [LivenessAnalysis] analysis is not available
    pub fn require_liveness(&self) -> anyhow::Result<()> {
        if self.liveness.is_some() {
            Ok(())
        } else {
            Err(anyhow!(
                "missing analysis requirement: liveness analysis is unavailable"
            ))
        }
    }

    /// Returns an error if any of the analysis data is not available
    pub fn require_all(&self) -> anyhow::Result<()> {
        // Liveness requires everything else
        self.require_liveness()
    }

    /// Ensures that the [DominatorTree] analysis is computed for `function`
    pub fn ensure_domtree(&mut self, function: &miden_hir::Function) {
        self.domtree
            .get_or_insert_with(|| DominatorTree::with_function(function, &self.cfg));
    }

    /// Ensures that the [LoopAnalysis] analysis is computed for `function`
    pub fn ensure_loops(&mut self, function: &miden_hir::Function) {
        self.ensure_domtree(function);
        self.loops.get_or_insert_with(|| {
            LoopAnalysis::with_function(function, &self.cfg, self.domtree.as_ref().unwrap())
        });
    }

    /// Ensures that the [LoopAnalysis] analysis is computed for `function`
    pub fn ensure_liveness(&mut self, function: &miden_hir::Function) {
        self.ensure_loops(function);
        self.liveness.get_or_insert_with(|| {
            LivenessAnalysis::compute(
                function,
                &self.cfg,
                self.domtree.as_ref().unwrap(),
                self.loops.as_ref().unwrap(),
            )
        });
    }

    /// Ensures that all control flow analyses have been computed for `function`
    pub fn ensure_all(&mut self, function: &miden_hir::Function) {
        // Liveness requires all other analyses, so we need only ensure that liveness
        // has been computed to ensure the rest.
        self.ensure_liveness(function);
    }

    /// Recomputes all previously computed analyses for `function`
    pub fn recompute(&mut self, function: &miden_hir::Function) {
        self.cfg.compute(&function.dfg);
        self.cfg_changed(function);
    }

    /// Recomputes all computed analyses downstream of the control flow graph for `function`
    pub fn cfg_changed(&mut self, function: &miden_hir::Function) {
        // If the dominator tree hasn't been computed, no other
        // analyses could possibly have been computed yet.
        let Some(domtree) = self.domtree.as_mut() else {
            return;
        };
        domtree.compute(function, &self.cfg);

        // Likewise for loop analysis - we can't compute liveness without it
        let Some(loops) = self.loops.as_mut() else {
            return;
        };
        loops.compute(function, &self.cfg, domtree);

        if let Some(liveness) = self.liveness.as_mut() {
            liveness.recompute(function, &self.cfg, domtree, loops);
        }
    }
}
