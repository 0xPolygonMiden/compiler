use expect_test::expect_file;
use miden_hir::Felt;
use miden_hir::StarkField;

use crate::execute_emulator;
use crate::CompilerTest;

#[test]
fn i32_gt_u() {
    let mut test = CompilerTest::rust_source_main_fn("(a: u32, b: u32) -> bool { a > b }");
    // Test expected compilation artifacts
    test.expect_wasm(expect_file!["../../expected/gt_u.wat"]);
    test.expect_ir(expect_file!["../../expected/gt_u.hir"]);
    test.expect_masm(expect_file!["../../expected/gt_u.masm"]);
    let ir_masm = test.codegen_masm_program();

    // TODO: run via proptest when emulator doesn't consume the program
    let a: u32 = 8;
    let b: u32 = 3;
    let rust_out = (a > b) as u64;
    let args = [Felt::from(a), Felt::from(b)];
    let emulator_out = execute_emulator(ir_masm, &args).first().unwrap().clone();
    assert_eq!(rust_out, emulator_out.as_int());
}
