//! Compilation and semantic tests for the whole compiler pipeline

#![deny(warnings)]
#![deny(missing_docs)]

mod comp_test;
mod exec_emulator;
mod exec_vm;

pub use comp_test::CompTest;
pub use exec_emulator::execute_emulator;
pub use exec_vm::execute_vm;

#[cfg(test)]
mod rust_masm_tests;
