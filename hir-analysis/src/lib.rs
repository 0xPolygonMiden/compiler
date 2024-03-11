mod control_flow;
mod data;
pub mod dependency_graph;
mod dominance;
mod liveness;
mod loops;
mod treegraph;
mod validation;

pub use self::{
    control_flow::{BlockPredecessor, ControlFlowGraph},
    data::{GlobalVariableAnalysis, GlobalVariableLayout},
    dependency_graph::DependencyGraph,
    dominance::{DominanceFrontier, DominatorTree, DominatorTreePreorder},
    liveness::LivenessAnalysis,
    loops::{Loop, LoopAnalysis, LoopLevel},
    treegraph::{OrderedTreeGraph, TreeGraph},
    validation::{ModuleValidationAnalysis, Rule},
};
