use std::sync::Arc;

use miden_core::Felt;
use proptest::{prop_assert_eq, test_runner::TestCaseError};

use crate::{execute_vm, felt_conversion::TestFelt};

mod abi_transform;
mod apps;
mod components;
mod instructions;
mod intrinsics;
mod rust_sdk;
mod wit_sdk;

pub fn run_masm_vs_rust<T>(
    rust_out: T,
    vm_program: &miden_core::Program,
    _ir_masm: Arc<miden_codegen_masm::Program>,
    args: &[Felt],
) -> Result<(), TestCaseError>
where
    T: Clone + From<TestFelt> + std::cmp::PartialEq + std::fmt::Debug,
{
    let vm_out: T = (*execute_vm(vm_program, args).first().unwrap()).into();
    dbg!(&vm_out);
    prop_assert_eq!(rust_out.clone(), vm_out, "VM output mismatch");
    // TODO: eq for i64 and u64 fails with invalid operand stack size error
    // let emul_out: T = execute_emulator(ir_masm.clone(), &args).first().unwrap().clone().into();
    // prop_assert_eq!(rust_out, emul_out, "Emulator output mismatch");
    Ok(())
}
