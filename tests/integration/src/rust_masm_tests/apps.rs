use expect_test::expect_file;
use miden_hir::Felt;
use proptest::prelude::*;
use proptest::test_runner::TestRunner;

use crate::execute_vm;
use crate::CompilerTest;

#[test]
fn fib() {
    let mut test =
        CompilerTest::rust_source_cargo("rust-fib", "miden_integration_tests_rust_fib", "fib");
    // Test expected compilation artifacts
    test.expect_wasm(expect_file!["../../expected/fib.wat"]);
    test.expect_ir(expect_file!["../../expected/fib.hir"]);
    test.expect_masm(expect_file!["../../expected/fib.masm"]);
    // let ir_masm = test.codegen_masm_program();
    let vm_program = &test.vm_masm_program();

    // Run the Rust and compiled MASM code against a bunch of random inputs and compare the results
    TestRunner::default()
        .run(&(1..u32::MAX / 2), move |a| {
            let rust_out = miden_integration_tests_rust::fib::fib(a) as u64;
            let args = [Felt::from(a)];
            let vm_out = execute_vm(&vm_program, &args).first().unwrap().clone();
            prop_assert_eq!(rust_out, vm_out);
            Ok(())
        })
        .unwrap();
}
