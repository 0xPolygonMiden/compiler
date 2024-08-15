use std::collections::VecDeque;

use expect_test::expect_file;
use midenc_debug::{Executor, PopFromStack, PushToStack};
use midenc_hir::Felt;
use proptest::{prelude::*, test_runner::TestRunner};

use crate::CompilerTest;

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
            let mut args = Vec::<Felt>::default();
            PushToStack::try_push(&a, &mut args);

            let exec = Executor::new(args);
            let output: u32 = exec.execute_into(vm_program, &test.session);
            dbg!(output);
            prop_assert_eq!(rust_out, output);
            // args.reverse();
            // let emul_out: u32 =
            //     execute_emulator(ir_masm.clone(), &args).first().unwrap().clone().into();
            // prop_assert_eq!(rust_out, emul_out);
            Ok(())
        })
        .unwrap();
}
