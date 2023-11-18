use miden_core::Program;
use miden_core::StackInputs;
use miden_hir::Felt;
use miden_processor::DefaultHost;
use miden_processor::ExecutionOptions;

use crate::felt_conversion::TestFelt;

/// Execute the module using the VM with the given arguments
pub fn execute_vm(program: &Program, args: &[Felt]) -> Vec<TestFelt> {
    let stack_inputs = StackInputs::new(args.to_vec());
    let trace = miden_processor::execute(
        program,
        stack_inputs,
        DefaultHost::default(),
        ExecutionOptions::default(),
    )
    .expect("failed to execute program on VM");
    trace
        .stack_outputs()
        .stack()
        .into_iter()
        .map(|i| TestFelt(i.clone().into()))
        .collect()
}
