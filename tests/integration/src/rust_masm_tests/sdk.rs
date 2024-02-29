use crate::CompilerTest;
use expect_test::expect_file;
use miden_core::crypto::hash::RpoDigest;
use miden_frontend_wasm::ImportMetadata;
use miden_frontend_wasm::WasmTranslationConfig;
use miden_hir::FunctionExportName;
use miden_hir::{InterfaceFunctionIdent, InterfaceIdent, Symbol};
use rustc_hash::FxHashMap;

#[test]
fn sdk() {
    let test = CompilerTest::rust_source_cargo_component("sdk/sdk", Default::default());
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!(
        "../../expected/sdk_basic_wallet/{artifact_name}.wat"
    )]);
}

#[test]
fn sdk_basic_wallet() {
    let interface_tx = InterfaceIdent::from_full_ident("miden:base/tx@1.0.0".to_string());
    let create_note_ident = InterfaceFunctionIdent {
        interface: interface_tx.clone(),
        function: Symbol::intern("create-note"),
    };
    let interface_account = InterfaceIdent::from_full_ident("miden:base/account@1.0.0".to_string());
    let add_asset_ident = InterfaceFunctionIdent {
        interface: interface_account.clone(),
        function: Symbol::intern("add-asset"),
    };
    let remove_asset_ident = InterfaceFunctionIdent {
        interface: interface_account.clone(),
        function: Symbol::intern("remove-asset"),
    };
    let import_metadata: FxHashMap<InterfaceFunctionIdent, ImportMetadata> = [
        (
            create_note_ident.clone(),
            ImportMetadata {
                digest: RpoDigest::default(),
            },
        ),
        (
            remove_asset_ident.clone(),
            ImportMetadata {
                digest: RpoDigest::default(),
            },
        ),
        (
            add_asset_ident.clone(),
            ImportMetadata {
                digest: RpoDigest::default(),
            },
        ),
    ]
    .into_iter()
    .collect();
    let expected_exports: Vec<FunctionExportName> = vec![
        Symbol::intern("send-asset").into(),
        Symbol::intern("receive-asset").into(),
    ];
    let config = WasmTranslationConfig {
        import_metadata: import_metadata.clone(),
        ..Default::default()
    };
    let mut test = CompilerTest::rust_source_cargo_component("sdk/basic-wallet", config);
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!(
        "../../expected/sdk_basic_wallet/{artifact_name}.wat"
    )]);
    test.expect_ir(expect_file![format!(
        "../../expected/sdk_basic_wallet/{artifact_name}.hir"
    )]);
    let ir = test.hir().unwrap_component();
    for (_, import) in ir.imports() {
        assert!(import_metadata.contains_key(&import.interface_function));
    }
    for name in expected_exports {
        assert!(ir.exports().contains_key(&name));
    }
}

#[test]
fn sdk_basic_wallet_p2id_note() {
    let test = CompilerTest::rust_source_cargo_component("sdk/p2id-note", Default::default());
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!(
        "../../expected/sdk_basic_wallet/{artifact_name}.wat"
    )]);
}
