use miden_core::{Program, StackInputs};
use miden_processor::{DefaultHost, ExecutionOptions};
use midenc_hir::Felt;

use crate::felt_conversion::TestFelt;

/// Execute the module using the VM with the given arguments
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
