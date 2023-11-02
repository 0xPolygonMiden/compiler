use expect_test::expect_file;
use miden_hir::Felt;
use miden_hir::StarkField;
// use proptest::prelude::*;
// use proptest::test_runner::TestRunner;

use crate::execute_emulator;
use crate::CompilerTest;

#[test]
fn fib() {
    let mut test =
        CompilerTest::rust_source_cargo("rust-fib", "miden_integration_tests_rust_fib", "fib");
    // Test expected compilation artifacts
    test.expect_wasm(expect_file!["../../expected/fib.wat"]);
    test.expect_ir(expect_file!["../../expected/fib.hir"]);
    test.expect_masm(expect_file!["../../expected/fib.masm"]);
    let ir_masm = test.codegen_masm_program();
    // let asm_masm = &test.asm_masm_module();

    // TODO: make emulator to not consume the program so we can run it multiple times
    // Run the Rust and compiled MASM code against a bunch of random inputs and compare the results
    // let mut runner = TestRunner::default();
    // runner
    //     .run(&(any::<u32>(), 1..u32::MAX), move |(a, b)| {
    let a = 9;
    let rust_out = miden_integration_tests_rust::fib::fib(a) as u64;
    let args = [Felt::from(a)];
    let emulator_out = execute_emulator(ir_masm, &args).first().unwrap().clone();
    // TODO: run on VM
    // let vm_out = execute_vm(&asm_masm, &args).first().unwrap().clone();
    assert_eq!(rust_out, emulator_out.as_int());

    // prop_assert_eq!(rust_out, vm_out.as_int());
    // Ok(())
    // })
    // .unwrap();
}
