use anyhow::bail;
use cargo_metadata::Message;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::sync::Arc;

use anyhow::Context;
use miden_diagnostics::Verbosity;
use midenc_session::InputFile;
use midenc_session::OutputFile;
use midenc_session::OutputType;
use midenc_session::OutputTypeSpec;
use midenc_session::OutputTypes;
use midenc_session::ProjectType;
use midenc_session::Session;
use midenc_session::TargetEnv;

pub fn compile(
    target: TargetEnv,
    bin_name: Option<String>,
    output_file: &PathBuf,
) -> anyhow::Result<()> {
    // for cargo env var see https://doc.rust-lang.org/cargo/reference/environment-variables.html
    let mut cargo_build_cmd = Command::new("cargo");
    cargo_build_cmd
        .arg("build")
        .arg("--release")
        .arg("--target=wasm32-unknown-unknown");
    let project_type = if let Some(ref bin_name) = bin_name {
        cargo_build_cmd.arg("--bin").arg(bin_name.clone());
        ProjectType::Program
    } else {
        ProjectType::Library
    };
    println!("Compiling Wasm with cargo build ...");
    let mut child = cargo_build_cmd
        .arg("--message-format=json-render-diagnostics")
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| {
            format!(
                "Failed to execute cargo build {}.",
                cargo_build_cmd
                    .get_args()
                    .map(|arg| format!("'{}'", arg.to_str().unwrap()))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        })?;
    let reader = std::io::BufReader::new(child.stdout.take().unwrap());
    let mut wasm_artifacts = Vec::new();
    for message in cargo_metadata::Message::parse_stream(reader) {
        match message.context("Failed to parse cargo metadata")? {
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
        bail!("Rust to Wasm compilation failed!");
    }

    if wasm_artifacts.is_empty() {
        match project_type {
            ProjectType::Library => bail!("Cargo build failed, no Wasm artifact found. Check if crate-type = [\"cdylib\"] is set in Cargo.toml"),
            ProjectType::Program => bail!("Cargo build failed, no Wasm artifact found."),
        }
    }
    if wasm_artifacts.len() > 1 {
        bail!(
            "Cargo build failed, multiple Wasm artifacts found: {:?}. Only one Wasm artifact is expected.",
            wasm_artifacts
        );
    }
    let wasm_file_path = wasm_artifacts[0].clone();
    match project_type {
        ProjectType::Program => {
            let bin_name = bin_name.unwrap();
            if !wasm_file_path.ends_with(format!("{}.wasm", bin_name)) {
                bail!(
            "Cargo build failed, Wasm artifact name {} does not match the expected name '{}'.",
            wasm_file_path.to_str().unwrap(),
            bin_name
        );
            }
        }
        ProjectType::Library => (),
    }

    println!(
        "Compiling '{}' Wasm to {} MASM with midenc ...",
        wasm_file_path.to_str().unwrap(),
        &output_file.as_path().to_str().unwrap()
    );
    let input = InputFile::from_path(wasm_file_path).context("Invalid input file")?;
    let output_file = OutputFile::Real(output_file.clone());
    let output_types = OutputTypes::new(vec![OutputTypeSpec {
        output_type: OutputType::Masm,
        path: Some(output_file.clone()),
    }]);
    let cwd = std::env::current_dir().context("Failed to get current working directory")?;
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
    midenc_compile::compile(session.clone()).context("Wasm to MASM compilation failed!")
}
