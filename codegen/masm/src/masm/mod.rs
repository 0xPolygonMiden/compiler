mod function;
mod module;

pub use self::function::{Function, FunctionListAdapter};
pub use self::module::Module;
pub use miden_hir::{Local, LocalId, MasmBlock as Block, MasmBlockId as BlockId, MasmOp as Op};
