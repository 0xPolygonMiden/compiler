use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use midenc_session::{
    diagnostics::{DefaultSourceManager, IntoDiagnostic, Report, WrapErr},
    InputFile, OutputFile, OutputType, OutputTypeSpec, OutputTypes, Session, Verbosity,
};

pub fn build_masm(
    wasm_file_path: &Path,
    output_folder: &Path,
    is_bin: bool,
) -> Result<PathBuf, Report> {
    if !output_folder.exists() {
        return Err(Report::msg(format!(
            "MASM output folder '{}' does not exist.",
            output_folder.to_str().unwrap()
        )));
    }
    log::debug!(
        "Compiling '{}' Wasm to '{}' directory with midenc ...",
        wasm_file_path.to_str().unwrap(),
        &output_folder.to_str().unwrap()
    );
    let input = InputFile::from_path(wasm_file_path)
        .into_diagnostic()
        .wrap_err("Invalid input file")?;
    let output_file_folder = OutputFile::Real(output_folder.to_path_buf());
    let output_type = OutputType::Masm;
    let output_types = OutputTypes::new(vec![OutputTypeSpec {
        output_type,
        path: Some(output_file_folder.clone()),
    }]);
    let project_type = if is_bin { "--exe" } else { "--lib" };
    let source_manager = Arc::new(DefaultSourceManager::default());
    let options = midenc_compile::CompilerOptions::parse_options(&[project_type])
        .with_verbosity(Verbosity::Debug)
        .with_output_types(output_types);
    let session = Arc::new(Session::new(
        input,
        Some(output_folder.to_path_buf()),
        None,
        None,
        options,
        None,
        source_manager,
    ));
    midenc_compile::compile(session.clone())?;
    let mut output_path = output_folder.join(wasm_file_path.file_stem().unwrap());
    output_path.set_extension(output_type.extension());
    Ok(output_path)
}
