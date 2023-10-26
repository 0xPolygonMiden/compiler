use expect_test::expect_file;
use miden_compiler_tests_rust_source::div_u::div_u;
use miden_hir::Felt;
use miden_hir::StarkField;
use proptest::prelude::*;
use proptest::test_runner::TestRunner;

use crate::execute_emulator;
use crate::execute_vm;
use crate::CompTest;

#[test]
fn u32_div() {
    // Test expected compilation artifacts
    let mut test = CompTest::new();
    test.rust_source("compiler-tests-rust-source", "div_u_app")
        .expect_wasm(expect_file!["./expected/div_u.wat"])
        .expect_ir(expect_file!["./expected/div_u.mir"])
        .expect_masm(expect_file!["./expected/div_u.masm"]);
    let ir_masm = &test.codegen_masm_module();
    let asm_masm = &test.asm_masm_module();

    // Run the Rust and compiled MASM code against a bunch of random inputs and compare the results
    let mut runner = TestRunner::default();
    runner
        .run(&(any::<u32>(), 1..u32::MAX), move |(a, b)| {
            let rust_out = div_u(a, b) as u64;
            let args = [Felt::from(a), Felt::from(b)];
            let emulator_out = execute_emulator(&ir_masm, &args).first().unwrap().clone();
            let vm_out = execute_vm(&asm_masm, &args).first().unwrap().clone();
            prop_assert_eq!(rust_out, emulator_out.as_int());
            prop_assert_eq!(rust_out, vm_out.as_int());
            Ok(())
        })
        .unwrap();
}
