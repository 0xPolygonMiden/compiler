use expect_test::expect_file;
use miden_integration_tests::CompilerTest;

#[test]
fn rust_add() {
    let mut test = CompilerTest::rust_source_program(include_str!("rust_source/add.rs"));
    test.expect_wasm(expect_file!["./expected/add.wat"]);
    test.expect_ir(expect_file!["./expected/add.hir"]);
}

#[test]
fn rust_fib() {
    let mut test = CompilerTest::rust_source_program(include_str!("rust_source/fib.rs"));
    test.expect_wasm(expect_file!["./expected/fib.wat"]);
    test.expect_ir(expect_file!["./expected/fib.hir"]);
}

#[test]
fn rust_enum() {
    let mut test = CompilerTest::rust_source_program(include_str!("rust_source/enum.rs"));
    test.expect_wasm(expect_file!["./expected/enum.wat"]);
    test.expect_ir(expect_file!["./expected/enum.hir"]);
}

#[ignore = "Block inlining pass crashes"]
#[test]
fn rust_array() {
    let mut test = CompilerTest::rust_source_program(include_str!("rust_source/array.rs"));
    test.expect_wasm(expect_file!["./expected/array.wat"]);
    test.expect_ir(expect_file!["./expected/array.hir"]);
}

#[test]
fn rust_static_mut() {
    let mut test = CompilerTest::rust_source_program(include_str!("rust_source/static_mut.rs"));
    test.expect_wasm(expect_file!["./expected/static_mut.wat"]);
    test.expect_ir(expect_file!["./expected/static_mut.hir"]);
}

// #[ignore]
// #[test]
// fn dlmalloc() {
//     check_ir_files_cargo(
//         "dlmalloc_app",
//         expect_file!["./expected/dlmalloc.wat"],
//         expect_file!["./expected/dlmalloc.hir"],
//     )
// }

// #[test]
// #[ignore = "Being reworked"]
// fn signed_arith() {
//     check_ir_files(
//         include_str!("rust_source/signed_arith.rs"),
//         expect_file!["./expected/signed_arith.wat"],
//         expect_file!["./expected/signed_arith.hir"],
//     );
// }
