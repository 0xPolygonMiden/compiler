use std::{collections::BTreeMap, path::PathBuf};

use expect_test::expect_file;
use miden_core::crypto::hash::RpoDigest;
use midenc_frontend_wasm::{ImportMetadata, WasmTranslationConfig};
use midenc_hir::{InterfaceFunctionIdent, InterfaceIdent, Symbol};

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
fn rust_sdk_basic_wallet() {
    let _ = env_logger::builder().is_test(true).try_init();
    let project_name = "rust_sdk_basic_wallet";
    let config = WasmTranslationConfig::default();
    let mut test =
        CompilerTest::rust_source_cargo_miden("../rust-apps-wasm/rust-sdk/basic-wallet", config);
    let artifact_name = test.artifact_name().to_string();
    test.expect_wasm(expect_file![format!("../../expected/rust_sdk/{artifact_name}.wat")]);
    test.expect_ir(expect_file![format!("../../expected/rust_sdk/{artifact_name}.hir")]);
    test.expect_masm(expect_file![format!("../../expected/rust_sdk/{artifact_name}.masm")]);
}
