use std::io::Read;
use std::process::Command;
use std::process::Stdio;

use crate::compiler_test::demangle;
use crate::compiler_test::wasm_to_wat;
use cargo_metadata::Message;
use expect_test::expect_file;

fn rust_cargo_component(cargo_project_folder: &str) -> Vec<std::path::PathBuf> {
    let manifest_path = format!("../rust-apps-wasm/{}/Cargo.toml", cargo_project_folder);
    // dbg!(&pwd);
    let mut cargo_build_cmd = Command::new("cargo");
    cargo_build_cmd
        .arg("component")
        .arg("build")
        .arg("--manifest-path")
        .arg(manifest_path)
        .arg("--release");
    // compile std as part of crate graph compilation
    // https://doc.rust-lang.org/cargo/reference/unstable.html#build-std
    // .arg("-Z")
    // .arg("build-std=core,alloc")
    // .arg("-Z")
    // // abort on panic without message formatting (core::fmt uses call_indirect)
    // .arg("build-std-features=panic_immediate_abort");
    let mut child = cargo_build_cmd
        .arg("--message-format=json-render-diagnostics")
        .stdout(Stdio::piped())
        .spawn()
        .expect(
            format!(
                "Failed to execute cargo build {}.",
                cargo_build_cmd
                    .get_args()
                    .map(|arg| format!("'{}'", arg.to_str().unwrap()))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
            .as_str(),
        );
    let reader = std::io::BufReader::new(child.stdout.take().unwrap());
    let mut wasm_artifacts = Vec::new();
    for message in cargo_metadata::Message::parse_stream(reader) {
        match message.expect("Failed to parse cargo metadata") {
            Message::CompilerArtifact(artifact) => {
                // find the Wasm artifact in artifact.filenames
                for filename in artifact.filenames {
                    if filename.as_str().ends_with(".wasm") {
                        wasm_artifacts.push(filename.into_std_path_buf());
                    }
                }
            }
            _ => (),
        }
    }
    let output = child.wait().expect("Couldn't get cargo's exit status");
    if !output.success() {
        eprintln!("pwd: {:?}", std::env::current_dir().unwrap());
        let mut stderr = Vec::new();
        child
            .stderr
            .unwrap()
            .read(&mut stderr)
            .expect("Failed to read stderr");
        let stderr = String::from_utf8(stderr).expect("Failed to parse stderr");
        eprintln!("stderr: {}", stderr);
        panic!("Rust to Wasm compilation failed!");
    }
    assert!(output.success());
    wasm_artifacts
}
fn expect_wasm(wasm_bytes: &[u8], expected_wat_file: expect_test::ExpectFile) {
    let wat = demangle(&wasm_to_wat(wasm_bytes));
    expected_wat_file.assert_eq(&wat);
}

#[test]
fn sdk_basic_wallet() {
    let wasm_comp = rust_cargo_component("basic-wallet/basic-wallet")
        .first()
        .cloned()
        .unwrap();
    let wasm_comp_filename = wasm_comp.file_stem().unwrap().to_str().unwrap().to_string();
    let wasm_bytes = std::fs::read(wasm_comp.clone()).unwrap();
    expect_wasm(
        &wasm_bytes,
        expect_file![format!(
            "../../expected/sdk_basic_wallet/{wasm_comp_filename}.wat"
        )],
    );
}

#[test]
fn sdk_basic_wallet_p2id_note() {
    let wasm_comp = rust_cargo_component("basic-wallet/p2id-note")
        .first()
        .cloned()
        .unwrap();
    let wasm_comp_filename = wasm_comp.file_stem().unwrap().to_str().unwrap().to_string();
    let wasm_bytes = std::fs::read(wasm_comp.clone()).unwrap();
    expect_wasm(
        &wasm_bytes,
        expect_file![format!(
            "../../expected/sdk_basic_wallet/{wasm_comp_filename}.wat"
        )],
    );
}
