//! Compilation and semantic tests for the whole compiler pipeline

#![deny(warnings)]
#![deny(missing_docs)]

mod cargo_proj;
mod compiler_test;
mod exec_emulator;
mod exec_vm;
pub(crate) mod felt_conversion;

pub use compiler_test::{default_session, CompilerTest};
pub use exec_emulator::execute_emulator;
pub use exec_vm::execute_vm;

#[cfg(test)]
mod rust_masm_tests;

#[cfg(test)]
mod wasm_smith_tests;
