use expect_test::expect_file;
use miden_hir::Felt;
use proptest::prop_assert_eq;
use proptest::test_runner::TestRunner;

use crate::execute_vm;
use crate::CompilerTest;

#[test]
fn i32_gt_u() {
    let mut test = CompilerTest::rust_source_main_fn("(a: u32, b: u32) -> bool { a > b }");
    // Test expected compilation artifacts
    test.expect_wasm(expect_file!["../../expected/gt_u.wat"]);
    test.expect_ir(expect_file!["../../expected/gt_u.hir"]);
    test.expect_masm(expect_file!["../../expected/gt_u.masm"]);
    // let ir_masm = test.codegen_masm_program();
    let vm_program = test.vm_masm_program();

    // Run the Rust and compiled MASM code against a bunch of random inputs and compare the results
    TestRunner::default()
        .run(&(0..(u32::MAX / 2), 0..(u32::MAX / 2)), move |(a, b)| {
            let rust_out = (a > b) as u64;
            let mut args = [Felt::from(a), Felt::from(b)];
            args.reverse();
            let vm_out = execute_vm(&vm_program, &args).first().unwrap().clone();
            prop_assert_eq!(rust_out, vm_out);
            Ok(())
        })
        .unwrap();
}
