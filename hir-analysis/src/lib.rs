mod control_flow;
mod data;
mod def_use;
pub mod dependency_graph;
mod dominance;
mod liveness;
mod loops;
pub mod spill;
mod treegraph;
mod validation;

pub use self::{
    control_flow::{BlockPredecessor, ControlFlowGraph},
    data::{GlobalVariableAnalysis, GlobalVariableLayout},
    def_use::{DefUseGraph, Use, User, UserList, Users, ValueDef},
    dependency_graph::DependencyGraph,
    dominance::{DominanceFrontier, DominatorTree, DominatorTreePreorder},
    liveness::LivenessAnalysis,
    loops::{Loop, LoopAnalysis, LoopLevel},
    spill::{Reload, ReloadInfo, Spill, SpillAnalysis, SpillInfo},
    treegraph::{OrderedTreeGraph, TreeGraph},
    validation::{ModuleValidationAnalysis, Rule},
};
