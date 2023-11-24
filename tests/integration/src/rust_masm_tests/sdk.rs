use crate::CompilerTest;
use expect_test::expect_file;

#[test]
fn sdk_account() {
    let mut test = CompilerTest::rust_source_cargo("sdk-basic-wallet", "basic_wallet", None);
    // Test expected compilation artifacts
    test.expect_wasm(expect_file!["../../expected/sdk_basic_wallet.wat"]);
    // test.expect_ir(expect_file!["../../expected/sdk_account.hir"]);
    // test.expect_masm(expect_file!["../../expected/sdk_account.masm"]);
}
