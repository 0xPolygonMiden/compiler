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

    let interface_tx = InterfaceIdent::from_full_ident("miden:core-import/tx@1.0.0".to_string());
    let create_note_ident = InterfaceFunctionIdent {
        interface: interface_tx,
        function: Symbol::intern("create-note"),
    };
    let interface_intrinsics_mem =
        InterfaceIdent::from_full_ident("miden:core-import/intrinsics-mem@1.0.0".to_string());
    let heap_base_ident = InterfaceFunctionIdent {
        interface: interface_intrinsics_mem,
        function: Symbol::intern("heap-base"),
    };
    let interface_intrinsics_felt =
        InterfaceIdent::from_full_ident("miden:core-import/intrinsics-felt@1.0.0".to_string());
    // TODO: remove this after it's translated in the frontend
    let felt_add_ident = InterfaceFunctionIdent {
        interface: interface_intrinsics_felt,
        function: Symbol::intern("add"),
    };

    let interface_stdlib_crypto_hashes =
        InterfaceIdent::from_full_ident("miden:core-import/stdlib-crypto-hashes@1.0.0".to_string());

    let blake3_hash_one_to_one_ident = InterfaceFunctionIdent {
        interface: interface_stdlib_crypto_hashes,
        function: Symbol::intern("blake3-hash-one-to-one"),
    };

    let interface_account =
        InterfaceIdent::from_full_ident("miden:core-import/account@1.0.0".to_string());
    let add_asset_ident = InterfaceFunctionIdent {
        interface: interface_account,
        function: Symbol::intern("add-asset"),
    };
    let remove_asset_ident = InterfaceFunctionIdent {
        interface: interface_account,
        function: Symbol::intern("remove-asset"),
    };
    let import_metadata: BTreeMap<InterfaceFunctionIdent, ImportMetadata> = [
        (
            create_note_ident,
            ImportMetadata {
                digest: RpoDigest::default(),
            },
        ),
        (
            remove_asset_ident,
            ImportMetadata {
                digest: RpoDigest::default(),
            },
        ),
        (
            add_asset_ident,
            ImportMetadata {
                digest: RpoDigest::default(),
            },
        ),
        (
            heap_base_ident,
            ImportMetadata {
                digest: RpoDigest::default(),
            },
        ),
        (
            felt_add_ident,
            ImportMetadata {
                digest: RpoDigest::default(),
            },
        ),
        (
            blake3_hash_one_to_one_ident,
            ImportMetadata {
                digest: RpoDigest::default(),
            },
        ),
    ]
    .into_iter()
    .collect();
    let config = WasmTranslationConfig {
        import_metadata: import_metadata.clone(),
        ..Default::default()
    };

    let mut test =
        CompilerTest::rust_source_cargo_miden("../rust-apps-wasm/rust-sdk/basic-wallet", config);
    let artifact_name = test.artifact_name().to_string();
    test.expect_wasm(expect_file![format!("../../expected/rust_sdk/{artifact_name}.wat")]);
    test.expect_ir(expect_file![format!("../../expected/rust_sdk/{artifact_name}.hir")]);
    // test.expect_masm(expect_file![format!("../../expected/rust_sdk/{artifact_name}.masm")]);
    // TODO: fix flaky test, "exec."_ZN19miden_sdk_tx_kernel9add_asset17h6f4cff304c095ffc" is
    // changing the suffix on every n-th run test.expect_masm(expect_file![format!("../../
    // expected/{project_name}/{artifact_name}.masm" )]);
}
