/// Read and parse the contents from `./masm/*` and compile it to MASL.
#[cfg(feature = "masl-lib")]
fn main() {
    use std::{env, path::Path, sync::Arc};

    use miden_assembly::{
        diagnostics::IntoDiagnostic, Assembler, Library as CompiledLibrary, LibraryNamespace,
    };
    // re-build the `[OUT_DIR]/assets/` file iff something in the `./masm` directory
    // or its builder changed:
    println!("cargo:rerun-if-changed=masm");

    let build_dir = env::var("OUT_DIR").unwrap();
    let build_dir = Path::new(&build_dir);
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let source_manager = Arc::new(miden_assembly::DefaultSourceManager::default());
    let namespace = "miden".parse::<LibraryNamespace>().expect("invalid base namespace");

    let asm_dir = Path::new(manifest_dir).join("masm").join("miden");
    let asm = Assembler::new(source_manager);
    let lib = CompiledLibrary::from_dir(asm_dir, namespace, asm).unwrap();
    let masl_path = build_dir
        .join("assets")
        .join("miden")
        .with_extension(CompiledLibrary::LIBRARY_EXTENSION);
    lib.write_to_file(masl_path).into_diagnostic().unwrap();
}

#[cfg(not(feature = "masl-lib"))]
fn main() {}
