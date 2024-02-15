use crate::compiler_test::default_session;
use crate::CompilerTest;
use expect_test::expect_file;
use miden_core::crypto::hash::RpoDigest;
use miden_frontend_wasm::translate_component;
use miden_frontend_wasm::ExportMetadata;
use miden_frontend_wasm::ImportMetadata;
use miden_frontend_wasm::WasmTranslationConfig;
use miden_hir::InterfaceFunctionIdent;
use miden_hir::InterfaceIdent;
use miden_hir::LiftedFunctionType;
use miden_hir::Symbol;
use miden_hir::Type;

#[test]
fn wcm_add() {
    // Has no imports
    let test = CompilerTest::rust_source_cargo_component("add-comp");
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!(
        "../../expected/components/{artifact_name}.wat"
    )]);
    test.expect_wit_bind(expect_file![format!(
        "../../expected/components/bindings/{artifact_name}_bindings.rs"
    )]);
    let wasm_bytes = test.wasm_bytes;

    let session = default_session();
    let export_metadata = [(
        Symbol::intern("add").into(),
        ExportMetadata {
            invoke_method: miden_hir::FunctionInvocationMethod::Call,
        },
    )]
    .into_iter()
    .collect();
    let config = WasmTranslationConfig {
        export_metadata,
        ..Default::default()
    };
    let component = translate_component(&wasm_bytes, &config, &session.diagnostics)
        .expect("Failed to translate Wasm to IR module");
    assert!(!component.modules().is_empty());
}

#[test]
fn wcm_inc() {
    // Imports an add component used in the above test
    let test = CompilerTest::rust_source_cargo_component("inc-comp");
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!(
        "../../expected/components/{artifact_name}.wat"
    )]);
    test.expect_wit_bind(expect_file![format!(
        "../../expected/components/bindings/{artifact_name}_bindings.rs"
    )]);
    let wasm_bytes = test.wasm_bytes;

    let session = default_session();

    let interface_function_ident = InterfaceFunctionIdent {
        interface: InterfaceIdent::from_full_ident("miden:add/add@1.0.0".to_string()),
        function: Symbol::intern("add"),
    };
    let import_metadata = [(
        interface_function_ident.clone(),
        ImportMetadata {
            digest: RpoDigest::default(),
            invoke_method: miden_hir::FunctionInvocationMethod::Call,
        },
    )]
    .into_iter()
    .collect();
    let export_metadata = [(
        Symbol::intern("inc").into(),
        ExportMetadata {
            invoke_method: miden_hir::FunctionInvocationMethod::Call,
        },
    )]
    .into_iter()
    .collect();
    let config = WasmTranslationConfig {
        import_metadata,
        export_metadata,
        ..Default::default()
    };
    let ir = translate_component(&wasm_bytes, &config, &session.diagnostics)
        .expect("Failed to translate Wasm to IR module");
    assert!(!ir.modules().is_empty());

    let export_name_sym = Symbol::intern("inc");
    let export = ir.exports().get(&export_name_sym.into()).unwrap();
    assert_eq!(export.function.function.as_symbol(), export_name_sym);
    let expected_export_func_ty = LiftedFunctionType {
        params: vec![Type::U32],
        results: vec![Type::U32],
    };
    assert_eq!(export.function_ty, expected_export_func_ty);
    let module = ir.modules().front().get().unwrap();
    dbg!(&module.imports());
    let import_info = module.imports();
    let function_id = import_info
        .imported(&module.name)
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .cloned()
        .unwrap()
        .clone();
    assert_eq!(function_id.module, module.name);
    // assert_eq!(function_id.function, interface_function_ident.function);
    let component_import = ir.imports().get(&function_id).unwrap();
    assert_eq!(
        component_import.interface_function,
        interface_function_ident
    );
    assert!(!component_import.function_ty.params.is_empty());
    let expected_import_func_ty = LiftedFunctionType {
        params: vec![Type::U32, Type::U32],
        results: vec![Type::U32],
    };
    assert_eq!(component_import.function_ty, expected_import_func_ty);
}
