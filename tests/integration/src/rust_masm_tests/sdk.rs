use std::io::Read;
use std::process::Command;
use std::process::Stdio;

use crate::compiler_test::demangle;
use crate::compiler_test::wasm_to_wat;
use cargo_metadata::Message;
use expect_test::expect_file;

fn rust_cargo(cargo_project_folder: &str, features: Option<&str>) -> Vec<std::path::PathBuf> {
    let manifest_path = format!("../rust-apps-wasm/{}/Cargo.toml", cargo_project_folder);
    // dbg!(&pwd);
    let target_triple = "wasm32-wasi";
    // let target_triple = "wasm32-unknown-unknown";
    let temp_dir = std::env::temp_dir();
    let target_dir = temp_dir.join(cargo_project_folder);
    dbg!(&target_dir);
    let mut cargo_build_cmd = Command::new("cargo");
    cargo_build_cmd
        .arg("build")
        .arg("--manifest-path")
        .arg(manifest_path)
        .arg("--release")
        // compile std as part of crate graph compilation
        // https://doc.rust-lang.org/cargo/reference/unstable.html#build-std
        .arg("-Z")
        .arg("build-std=core,alloc")
        .arg("-Z")
        // abort on panic without message formatting (core::fmt uses call_indirect)
        .arg("build-std-features=panic_immediate_abort")
        .arg(format!("--target={target_triple}"));
    if features.is_some() {
        cargo_build_cmd.arg("--features").arg(features.unwrap());
        cargo_build_cmd.arg("--bins");
    } else {
        cargo_build_cmd.arg("--lib");
    }
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

pub fn expect_wasm(wasm_bytes: &[u8], expected_wat_file: expect_test::ExpectFile) {
    let wat = demangle(&wasm_to_wat(wasm_bytes));
    expected_wat_file.assert_eq(&wat);
}

#[test]
fn sdk_account_basic_wallet() {
    let mut wasm_artifacts = rust_cargo("sdk-basic-wallet", None);
    assert_eq!(wasm_artifacts.len(), 1);
    let lib_wasm_file = wasm_artifacts.pop().unwrap();
    let lib_file_name = lib_wasm_file
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let lib_wasm_bytes = std::fs::read(lib_wasm_file.clone()).unwrap();
    expect_wasm(
        &lib_wasm_bytes,
        expect_file![format!("../../expected/sdk_account_{lib_file_name}.wat")],
    );
    // TODO: lib artifact is overriden in this cargo call
    let wasm_artifacts_bin = rust_cargo("sdk-basic-wallet", Some("build_notes"));
    assert_eq!(wasm_artifacts_bin.len(), 3);
    for wasm_file in &wasm_artifacts_bin {
        let file_name = wasm_file.file_stem().unwrap().to_str().unwrap();
        if file_name == lib_file_name {
            continue;
        }
        let wasm_bytes = std::fs::read(wasm_file).unwrap();
        expect_wasm(
            &wasm_bytes,
            expect_file![format!("../../expected/sdk_account_{file_name}.wat")],
        );
    }
}
