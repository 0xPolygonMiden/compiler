use std::{collections::BTreeMap, env, path::PathBuf};

use expect_test::expect_file;
use miden_core::crypto::hash::RpoDigest;
use midenc_codegen_masm::Package;
use midenc_frontend_wasm::WasmTranslationConfig;
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
    let config = WasmTranslationConfig::default();
    let mut test = CompilerTest::rust_source_cargo_miden(
        "../rust-apps-wasm/rust-sdk/basic-wallet",
        config,
        [],
    );
    let artifact_name = test.artifact_name().to_string();
    test.expect_wasm(expect_file![format!("../../expected/rust_sdk/{artifact_name}.wat")]);
    test.expect_ir(expect_file![format!("../../expected/rust_sdk/{artifact_name}.hir")]);
    test.expect_masm(expect_file![format!("../../expected/rust_sdk/{artifact_name}.masm")]);
}

#[test]
fn rust_sdk_p2id_note_script() {
    // Build basic-wallet package
    let args: Vec<String> = [
        "cargo",
        "miden",
        "build",
        "--manifest-path",
        "../rust-apps-wasm/rust-sdk/basic-wallet/Cargo.toml",
        "--release",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    dbg!(env::current_dir().unwrap().display());
    let outputs = cargo_miden::run(args.into_iter(), cargo_miden::OutputType::Masm)
        .expect("Failed to compile the basic-wallet package");
    let masp_path: PathBuf = outputs.first().unwrap().clone();
    dbg!(&masp_path);

    //
    // let masp = Package::read_from_file(masp_path.clone()).unwrap();
    // let basic_wallet_lib = match masp.mast {
    //     midenc_codegen_masm::MastArtifact::Executable(arc) => panic!("expected library"),
    //     midenc_codegen_masm::MastArtifact::Library(arc) => arc.clone(),
    // };
    // let mut masl_path = masp_path.clone();
    // masl_path.set_extension("masl");
    // basic_wallet_lib.write_to_file(masl_path.clone()).unwrap();

    let _ = env_logger::builder().is_test(true).try_init();

    let config = WasmTranslationConfig::default();
    let mut test = CompilerTest::rust_source_cargo_miden(
        "../rust-apps-wasm/rust-sdk/p2id-note",
        config,
        [
            // "--link-library".into(),
            // masl_path.into_os_string().into_string().unwrap().into(),
        ],
    );
    let artifact_name = test.artifact_name().to_string();
    test.expect_wasm(expect_file![format!("../../expected/rust_sdk/{artifact_name}.wat")]);
    test.expect_ir(expect_file![format!("../../expected/rust_sdk/{artifact_name}.hir")]);
    // test.expect_masm(expect_file![format!("../../expected/rust_sdk/{artifact_name}.masm")]);
}
