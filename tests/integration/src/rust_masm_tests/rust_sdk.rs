use std::path::PathBuf;

use expect_test::expect_file;

use crate::{cargo_proj::project, compiler_test::sdk_crate_path, CompilerTest};

#[test]
fn account() {
    let artifact_name = "miden_sdk_account_test";
    let mut test = CompilerTest::rust_source_cargo_lib(
        "../rust-apps-wasm/rust-sdk/account-test",
        artifact_name,
        true,
        None,
        None,
    );
    test.expect_wasm(expect_file![format!(
        "../../expected/rust_sdk_account_test/{artifact_name}.wat"
    )]);
    test.expect_ir(expect_file![format!(
        "../../expected/rust_sdk_account_test/{artifact_name}.hir"
    )]);
    // test.expect_masm(expect_file![format!(
    //     "../../expected/rust_sdk_account_test/{artifact_name}.masm"
    // )]);
}

#[test]
fn basic_wallet() {
    let project_name = "rust_sdk_basic_wallet";
    let source = r#"

pub struct Account;

impl Account {
    #[no_mangle]
    pub fn receive_asset(asset: CoreAsset) {
        add_asset(asset);
    }

    #[no_mangle]
    pub fn send_asset(asset: CoreAsset, tag: Tag, note_type: NoteType, recipient: Recipient) {
        let asset = remove_asset(asset);
        create_note(asset, tag, note_type, recipient);
    }
}
"#;

    let mut test = CompilerTest::rust_source_with_sdk(project_name, source, true, None, None);
    let artifact_name = test.artifact_name();
    test.expect_wasm(expect_file![format!("../../expected/{project_name}/{artifact_name}.wat")]);
    test.expect_ir(expect_file![format!("../../expected/{project_name}/{artifact_name}.hir")]);
    // TODO: fix flaky test, "exec."_ZN19miden_sdk_tx_kernel9add_asset17h6f4cff304c095ffc" is
    // changing the suffix on every n-th run test.expect_masm(expect_file![format!("../../
    // expected/{project_name}/{artifact_name}.masm" )]);
}
