//! Compilation and semantic tests for the whole compiler pipeline
#![feature(iter_array_chunks)]
#![feature(debug_closure_helpers)]
//#![deny(warnings)]
#![deny(missing_docs)]

mod cargo_proj;
mod compiler_test;
mod exec_emulator;

pub use compiler_test::{default_session, CargoTest, CompilerTest, CompilerTestBuilder, RustcTest};
pub use exec_emulator::execute_emulator;

#[cfg(test)]
mod rust_masm_tests;
