use miden_core::{Program, StackInputs};
use miden_processor::{DefaultHost, ExecutionError, ExecutionOptions};
use midenc_hir::Felt;

use crate::felt_conversion::TestFelt;

/// Execute the program using the VM with the given arguments
/// Arguments are expected to be in the order they are passed to the entrypoint function
pub fn execute_vm(program: &Program, args: &[Felt]) -> Vec<TestFelt> {
    // Reverse the arguments to counteract the StackInputs::new() reversing them into a stack
    let stack_inputs = StackInputs::new(args.to_vec()).expect("invalid stack inputs");
    dbg!(&stack_inputs);
    let trace = miden_processor::execute(
        program,
        stack_inputs,
        DefaultHost::default(),
        ExecutionOptions::default(),
    )
    .expect("failed to execute program on VM");
    trace.stack_outputs().stack().iter().map(|i| TestFelt(*i)).collect()
}

/// Execute the program using the VM with the given arguments
/// Prints the trace (VM state) after each step to stdout
/// Arguments are expected to be in the order they are passed to the entrypoint function
#[allow(unused)]
pub fn execute_vm_tracing(
    program: &Program,
    args: &[Felt],
) -> Result<Vec<TestFelt>, ExecutionError> {
    // Reverse the arguments to counteract the StackInputs::new() reversing them into a stack
    let args_reversed = args.iter().copied().rev().collect();
    let stack_inputs = StackInputs::new(args_reversed).expect("invalid stack inputs");
    let vm_state_iterator =
        miden_processor::execute_iter(program, stack_inputs, DefaultHost::default());
    let mut last_stack = Vec::new();
    for vm_state in vm_state_iterator {
        let vm_state = vm_state?;
        eprintln!("{}", vm_state);
        last_stack.clone_from(&vm_state.stack);
    }
    Ok(last_stack.into_iter().map(TestFelt).collect())
}
