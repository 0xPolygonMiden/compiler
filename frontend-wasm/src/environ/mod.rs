//! Wasm to Miden IR translation environment

// TODO: "inline" module_env::* here
mod module_env;

pub use crate::environ::module_env::{ModuleEnvironment, ModuleInfo};
