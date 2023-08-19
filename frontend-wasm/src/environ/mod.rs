//! Wasm to Miden IR translation environment

mod func_env;
mod module_env;

pub use crate::environ::func_env::FuncEnvironment;
pub use crate::environ::module_env::{ModuleEnvironment, ModuleInfo};
