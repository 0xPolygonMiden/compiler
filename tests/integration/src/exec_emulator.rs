use std::sync::Arc;

use miden_codegen_masm::Emulator;
use miden_codegen_masm::Program;
use miden_hir::Felt;
use miden_hir::Stack;

use crate::felt_conversion::TestFelt;

/// Execute the module using the emulator with the given arguments
pub fn execute_emulator(program: Arc<Program>, args: &[Felt]) -> Vec<TestFelt> {
    let entrypoint = program.entrypoint.expect("cannot execute a library");
    let mut emulator = Emulator::default();
    emulator
        .load_program(program)
        .expect("failed to load program");
    emulator
        .invoke(entrypoint, args)
        .expect("failed to invoke")
        .stack()
        .iter()
        .map(|felt| TestFelt(felt.clone()))
        .collect()
}
