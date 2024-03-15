use expect_test::expect_file;
use miden_core::crypto::hash::RpoDigest;
use miden_frontend_wasm::{ExportMetadata, ImportMetadata, WasmTranslationConfig};
use miden_hir::{InterfaceFunctionIdent, InterfaceIdent, Symbol};

use crate::CompilerTest;

#[test]
fn sdk() {
    let test = CompilerTest::rust_source_cargo_component("sdk/sdk", Default::default());
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!("../../expected/sdk_basic_wallet/{artifact_name}.wat")]);
}

#[test]
fn sdk_basic_wallet() {
    let interface = InterfaceIdent::from_full_ident("miden:base/tx-kernel@1.0.0".to_string());
    let create_note_ident = InterfaceFunctionIdent {
        interface: interface.clone(),
        function: Symbol::intern("create-note"),
    };
    let add_asset_ident = InterfaceFunctionIdent {
        interface: interface.clone(),
        function: Symbol::intern("add-asset"),
    };
    let remove_asset_ident = InterfaceFunctionIdent {
        interface: interface.clone(),
        function: Symbol::intern("remove-asset"),
    };
    let import_metadata = [
        (
            create_note_ident.clone(),
            ImportMetadata {
                digest: RpoDigest::default(),
                invoke_method: miden_hir::FunctionInvocationMethod::Call,
            },
        ),
        (
            remove_asset_ident.clone(),
            ImportMetadata {
                digest: RpoDigest::default(),
                invoke_method: miden_hir::FunctionInvocationMethod::Call,
            },
        ),
        (
            add_asset_ident.clone(),
            ImportMetadata {
                digest: RpoDigest::default(),
                invoke_method: miden_hir::FunctionInvocationMethod::Call,
            },
        ),
    ]
    .into_iter()
    .collect();
    let export_metadata = [
        (
            Symbol::intern("send-asset").into(),
            ExportMetadata {
                invoke_method: miden_hir::FunctionInvocationMethod::Call,
            },
        ),
        (
            Symbol::intern("receive-asset").into(),
            ExportMetadata {
                invoke_method: miden_hir::FunctionInvocationMethod::Call,
            },
        ),
    ]
    .into_iter()
    .collect();
    let config = WasmTranslationConfig {
        import_metadata,
        export_metadata,
        ..Default::default()
    };
    let mut test = CompilerTest::rust_source_cargo_component("sdk/basic-wallet", config);
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!("../../expected/sdk_basic_wallet/{artifact_name}.wat")]);
    test.expect_ir(expect_file![format!("../../expected/sdk_basic_wallet/{artifact_name}.hir")]);
}

#[test]
fn sdk_basic_wallet_p2id_note() {
    let test = CompilerTest::rust_source_cargo_component("sdk/p2id-note", Default::default());
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!("../../expected/sdk_basic_wallet/{artifact_name}.wat")]);
}
