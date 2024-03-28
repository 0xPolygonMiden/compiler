//! Performs translation from Wasm to MidenIR

// Coding conventions
#![deny(warnings)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

extern crate alloc;

mod code_translator;
mod component;
mod config;
mod error;
mod miden_abi;
mod module;
mod ssa;
mod translation_utils;

#[cfg(test)]
mod test_utils;

pub use self::{
    component::build_ir::translate_component,
    config::*,
    error::WasmError,
    module::build_ir::{translate_module, translate_module_as_component},
};
