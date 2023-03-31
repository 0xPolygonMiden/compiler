mod block;
mod builder;
mod dataflow;
mod function;
mod immediates;
mod instruction;
mod layout;
mod value;

pub use self::block::{Block, BlockData};
pub use self::builder::{InstBuilder, InstBuilderBase};
pub use self::dataflow::DataFlowGraph;
pub use self::function::{FuncRef, Function, Signature, Visibility};
pub use self::immediates::Immediate;
pub use self::instruction::{BranchInfo, CallInfo, Inst, InstAdapter, InstNode, Instruction};
pub use self::layout::{ArenaMap, LayoutAdapter, LayoutNode, OrderedArenaMap};
pub use self::value::{Value, ValueData, ValueList, ValueListPool};
