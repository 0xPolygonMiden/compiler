use core::panic;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;

use miden_diagnostics::Verbosity;
use midenc_session::InputFile;
use midenc_session::OutputFile;
use midenc_session::OutputType;
use midenc_session::OutputTypeSpec;
use midenc_session::OutputTypes;
use midenc_session::ProjectType;
use midenc_session::Session;
use midenc_session::TargetEnv;

pub fn compile(target: TargetEnv, bin_name: Option<String>, output_file: &PathBuf) {
    // for cargo env var see https://doc.rust-lang.org/cargo/reference/environment-variables.html
    let mut cargo_build_cmd = Command::new("cargo");
    cargo_build_cmd
        .arg("build")
        .arg("--release")
        .arg("--target=wasm32-unknown-unknown");

    let (project_type, artifact_name) = if let Some(bin_name) = bin_name {
        cargo_build_cmd.arg("--bin").arg(bin_name.clone());
        (ProjectType::Program, bin_name)
    } else {
        // TODO: parse artifact name for lib from Cargo.toml (package.name?)
        (ProjectType::Library, "miden_lib".to_string())
    };
    let cwd = std::env::current_dir().unwrap();
    let target_dir = std::env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| cwd.join("target"));
    let release_folder = target_dir.join("wasm32-unknown-unknown").join("release");
    if !release_folder.exists() {
        panic!(
            "Cargo build failed, expected release folder at path: {}",
            release_folder.to_str().unwrap()
        );
    }
    let target_bin_file_path = release_folder.join(artifact_name).with_extension("wasm");
    if target_bin_file_path.exists() {
        // remove existing Wasm file since cargo build might not generate a new one silently
        //  e.g. if crate-type = ["cdylib"] is not set in Cargo.toml
        std::fs::remove_file(&target_bin_file_path).unwrap();
    }

    let output = cargo_build_cmd.output().expect(
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
    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("Rust to Wasm compilation failed!");
    }
    if !target_bin_file_path.exists() {
        panic!(
            "Cargo build failed, expected Wasm artifact at path: {}",
            target_bin_file_path.to_str().unwrap()
        );
    }

    let input = InputFile::from_path(target_bin_file_path).expect("Invalid Wasm artifact path");
    let output_file = OutputFile::Real(output_file.clone());
    let output_types = OutputTypes::new(vec![OutputTypeSpec {
        output_type: OutputType::Masm,
        path: Some(output_file.clone()),
    }]);
    let options = midenc_session::Options::new(cwd)
        // .with_color(color)
        .with_verbosity(Verbosity::Debug)
        // .with_warnings(self.warn)
        .with_output_types(output_types);
    let session = Arc::new(
        Session::new(target, input, None, Some(output_file), None, options, None)
            // .with_arg_matches(matches)
            .with_project_type(project_type),
    );
    match midenc_driver::commands::compile(session.clone()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            panic!("Compilation failed!");
        }
    }
}
