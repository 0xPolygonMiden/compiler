use expect_test::expect_file;
use miden_hir::Felt;
use proptest::{prelude::*, test_runner::TestRunner};

use crate::{execute_vm, CompilerTest};

#[test]
fn fib() {
    let mut test =
        CompilerTest::rust_source_cargo("fib", "miden_integration_tests_rust_fib_wasm", "fib");
    // Test expected compilation artifacts
    test.expect_wasm(expect_file!["../../expected/fib.wat"]);
    test.expect_ir(expect_file!["../../expected/fib.hir"]);
    test.expect_masm(expect_file!["../../expected/fib.masm"]);
    // let ir_masm = test.ir_masm_program();
    let vm_program = &test.vm_masm_program();

    // Run the Rust and compiled MASM code against a bunch of random inputs and compare the results
    TestRunner::default()
        .run(&(1u32..30), move |a| {
            let rust_out = miden_integration_tests_rust_fib::fib(a);
            let args = [Felt::from(a)];
            let vm_out: u32 = execute_vm(&vm_program, &args).first().unwrap().clone().into();
            prop_assert_eq!(rust_out, vm_out);
            // args.reverse();
            // let emul_out: u32 =
            //     execute_emulator(ir_masm.clone(), &args).first().unwrap().clone().into();
            // prop_assert_eq!(rust_out, emul_out);
            Ok(())
        })
        .unwrap();
}
