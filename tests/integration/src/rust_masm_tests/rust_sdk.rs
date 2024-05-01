use std::path::PathBuf;

use expect_test::expect_file;

use crate::CompilerTest;

#[test]
fn account() {
    let mut test = CompilerTest::rust_source_cargo_lib(
        PathBuf::from("../rust-apps-wasm/rust-sdk/account-test"),
        true,
        None,
    );
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!(
        "../../expected/rust_sdk_account_test/{artifact_name}.wat"
    )]);
    test.expect_ir(expect_file![format!(
        "../../expected/rust_sdk_account_test/{artifact_name}.hir"
    )]);
}
