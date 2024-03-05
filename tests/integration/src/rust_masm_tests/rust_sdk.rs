use expect_test::expect_file;

use crate::CompilerTest;

#[test]
fn account() {
    let test = CompilerTest::rust_source_cargo_lib("rust-sdk/account-test");
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!(
        "../../expected/rust_sdk_account_test/{artifact_name}.wat"
    )]);
    // test.expect_ir(expect_file![format!(
    //     "../../expected/rust_sdk_account_test/{artifact_name}.hir"
    // )]);
}
