//! Compilation and semantic tests for the whole compiler pipeline
#![feature(iter_array_chunks)]
//#![deny(warnings)]
#![deny(missing_docs)]

mod cargo_proj;
mod compiler_test;
mod exec_emulator;
mod exec_vm;
pub(crate) mod felt_conversion;

pub use compiler_test::{default_session, CompilerTest};
pub use exec_emulator::execute_emulator;
pub use exec_vm::MidenExecutor;

#[cfg(test)]
mod rust_masm_tests;
