use std::{env, path::Path, sync::Arc};

use miden_assembly::{
    diagnostics::{IntoDiagnostic, Result},
    Assembler, Library as CompiledLibrary, LibraryNamespace,
};

/// Read and parse the contents from `./masm/*` and compile it to MASL.
fn main() -> Result<()> {
    // re-build the `[OUT_DIR]/assets/` file iff something in the `./masm` directory
    // or its builder changed:
    println!("cargo:rerun-if-changed=masm");

    let build_dir = env::var("OUT_DIR").unwrap();
    let build_dir = Path::new(&build_dir);
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let source_manager = Arc::new(miden_assembly::DefaultSourceManager::default());
    let namespace = "miden".parse::<LibraryNamespace>().expect("invalid base namespace");

    let tx_asm_dir = Path::new(manifest_dir).join("masm").join("tx");
    let asm = Assembler::new(source_manager);
    let txlib = CompiledLibrary::from_dir(tx_asm_dir, namespace, asm)?;
    let tx_masl_path = build_dir
        .join("assets")
        .join("tx")
        .with_extension(CompiledLibrary::LIBRARY_EXTENSION);
    txlib.write_to_file(tx_masl_path).into_diagnostic()?;

    Ok(())
}
