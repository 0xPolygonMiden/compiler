use expect_test::expect_file;

use crate::CompilerTest;

#[test]
fn sdk() {
    let test = CompilerTest::rust_source_cargo_component("sdk/sdk", Default::default());
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!("../../expected/sdk_basic_wallet/{artifact_name}.wat")]);
}

#[test]
fn sdk_basic_wallet() {
    let test = CompilerTest::rust_source_cargo_component("sdk/basic-wallet", Default::default());
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!("../../expected/sdk_basic_wallet/{artifact_name}.wat")]);
}

#[test]
fn sdk_basic_wallet_p2id_note() {
    let test = CompilerTest::rust_source_cargo_component("sdk/p2id-note", Default::default());
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!("../../expected/sdk_basic_wallet/{artifact_name}.wat")]);
}
