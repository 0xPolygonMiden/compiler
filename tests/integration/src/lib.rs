//! Compilation and semantic tests for the whole compiler pipeline

#![deny(warnings)]
#![deny(missing_docs)]

mod compiler_test;
mod exec_emulator;
mod exec_vm;
pub(crate) mod felt_conversion;

pub use compiler_test::CompilerTest;
pub use exec_emulator::execute_emulator;
pub use exec_vm::execute_vm;

#[cfg(test)]
mod rust_masm_tests;
