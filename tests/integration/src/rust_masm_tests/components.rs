use crate::compiler_test::default_session;
use crate::CompilerTest;
use expect_test::expect_file;
use miden_frontend_wasm::translate_component;
use miden_frontend_wasm::WasmTranslationConfig;

#[ignore = "until Wasm component translation is implemented"]
#[test]
fn wcm_add() {
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
    let _ir_module = translate_component(
        &wasm_bytes,
        WasmTranslationConfig::default(),
        &session.diagnostics,
    )
    .expect("Failed to translate Wasm to IR module");
}
