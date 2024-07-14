#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{collections::VecDeque, sync::Arc};

use miden_core::Felt;
use proptest::{prop_assert_eq, test_runner::TestCaseError};

use crate::{execute_emulator, execute_vm, felt_conversion::PopFromStack};

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
    ir_program: Arc<midenc_codegen_masm::Program>,
    args: &[Felt],
) -> Result<(), TestCaseError>
where
    T: Clone + PopFromStack + std::cmp::PartialEq + std::fmt::Debug,
{
    let mut out = VecDeque::from(execute_vm(vm_program, args));
    let vm_out = T::try_pop(&mut out).expect("invalid result");
    dbg!(&vm_out);
    prop_assert_eq!(rust_out.clone(), vm_out, "VM output mismatch");
    // TODO: Uncomment after https://github.com/0xPolygonMiden/compiler/issues/228 is fixed
    // let emul_out: T = (*execute_emulator(ir_program.clone(), args).first().unwrap()).into();
    // prop_assert_eq!(rust_out, emul_out, "Emulator output mismatch");
    Ok(())
}
