use std::sync::Arc;

use miden_codegen_masm::{Emulator, Program};
use miden_hir::{Felt, Stack};

use crate::felt_conversion::TestFelt;

/// Execute the module using the emulator with the given arguments
/// Arguments are expected to be in the order they are passed to the entrypoint function
pub fn execute_emulator(program: Arc<Program>, args: &[Felt]) -> Vec<TestFelt> {
    let mut emulator = Emulator::default();
    emulator.load_program(program).expect("failed to load program");
    {
        let stack = emulator.stack_mut();
        for arg in args.iter().copied().rev() {
            stack.push(arg);
        }
    }
    let operand_stack = emulator.start().expect("failed to invoke");
    operand_stack.stack().iter().map(|felt| TestFelt(felt.clone())).collect()
}
