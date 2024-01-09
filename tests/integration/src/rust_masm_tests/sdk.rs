use crate::CompilerTest;
use expect_test::expect_file;

#[test]
fn sdk() {
    let test = CompilerTest::rust_source_cargo_component("sdk/sdk");
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!(
        "../../expected/sdk_basic_wallet/{artifact_name}.wat"
    )]);
}

#[test]
fn sdk_basic_wallet() {
    let test = CompilerTest::rust_source_cargo_component("sdk/basic-wallet");
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!(
        "../../expected/sdk_basic_wallet/{artifact_name}.wat"
    )]);
    test.expect_wit_bind(expect_file![format!(
        "../../expected/sdk_basic_wallet/bindings/{artifact_name}_bindings.rs"
    )]);
}

#[test]
fn sdk_basic_wallet_p2id_note() {
    let test = CompilerTest::rust_source_cargo_component("sdk/p2id-note");
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!(
        "../../expected/sdk_basic_wallet/{artifact_name}.wat"
    )]);
    test.expect_wit_bind(expect_file![format!(
        "../../expected/sdk_basic_wallet/bindings/{artifact_name}_bindings.rs"
    )]);
}
