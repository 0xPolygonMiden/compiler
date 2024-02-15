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

#[test]
fn rust_array() {
    let mut test = CompilerTest::rust_source_program(include_str!("rust_source/array.rs"));
    test.expect_wasm(expect_file!["./expected/array.wat"]);
    test.expect_ir(expect_file!["./expected/array.hir"]);
    assert!(
        test.hir
            .unwrap()
            .unwrap_program()
            .segments()
            .last()
            .unwrap()
            .is_readonly(),
        "data segment should be readonly"
    );
}

#[test]
fn rust_static_mut() {
    let mut test = CompilerTest::rust_source_program(include_str!("rust_source/static_mut.rs"));
    test.expect_wasm(expect_file!["./expected/static_mut.wat"]);
    test.expect_ir(expect_file!["./expected/static_mut.hir"]);
    assert!(
        !test
            .hir
            .unwrap()
            .unwrap_program()
            .segments()
            .last()
            .unwrap()
            .is_readonly(),
        "data segment should be mutable"
    );
}
