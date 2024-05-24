use std::path::PathBuf;

use expect_test::expect_file;

use crate::{cargo_proj::project, compiler_test::sdk_crate_path, CompilerTest};

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
    // test.expect_masm(expect_file![format!(
    //     "../../expected/rust_sdk_account_test/{artifact_name}.masm"
    // )]);
}

#[test]
fn basic_wallet() {
    let sdk_crate_path = sdk_crate_path();
    let project_name = "rust_sdk_basic_wallet";
    let proj = project(project_name)
        .file(
            "Cargo.toml",
                format!(
                r#"

                [package]
                name = "{project_name}"
                version = "0.0.1"
                edition = "2021"
                authors = []

                [dependencies]
                wee_alloc = {{ version = "0.4.5", default-features = false}}
                miden-sdk = {{ path = "{sdk_crate_path}" }}

                [lib]
                crate-type = ["cdylib"]

                [profile.release]
                panic = "abort"
                # optimize for size
                opt-level = "z"
            "#).as_str()
        )
        .file(
            "src/lib.rs",
            r#"
                #![no_std]

                #[panic_handler]
                fn my_panic(_info: &core::panic::PanicInfo) -> ! {
                    loop {}
                }

                #[global_allocator]
                static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

                use miden_sdk::*;

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
            "#,
        )
        .build();

    let mut test = CompilerTest::rust_source_cargo_lib(proj.root(), true, None);
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!("../../expected/{project_name}/{artifact_name}.wat")]);
    test.expect_ir(expect_file![format!("../../expected/{project_name}/{artifact_name}.hir")]);
    // TODO: fix flaky test, "exec."_ZN19miden_sdk_tx_kernel9add_asset17h6f4cff304c095ffc" is
    // changing the suffix on every n-th run test.expect_masm(expect_file![format!("../../
    // expected/{project_name}/{artifact_name}.masm" )]);
}
