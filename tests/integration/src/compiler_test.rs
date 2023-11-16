#![allow(dead_code)]

use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;

use miden_codegen_masm::MasmCompiler;
use miden_codegen_masm::Program;
use miden_diagnostics::term::termcolor::ColorChoice;
use miden_diagnostics::CodeMap;
use miden_diagnostics::DefaultEmitter;
use miden_diagnostics::DiagnosticsConfig;
use miden_diagnostics::DiagnosticsHandler;
use miden_diagnostics::Emitter;
use miden_diagnostics::NullEmitter;
use miden_diagnostics::SourceSpan;
use miden_diagnostics::Verbosity;
use miden_frontend_wasm::translate_module;
use miden_frontend_wasm::WasmTranslationConfig;

use miden_hir::FunctionIdent;
use miden_hir::Ident;
use miden_hir::Symbol;

enum CompilerTestSource {
    Rust(String),
    RustCargo {
        cargo_project_folder_name: String,
        artifact_name: String,
    },
    // Wasm(String),
    // Ir(String),
}

/// Compile to different stages (e.g. Wasm, IR, MASM) and compare the results against expected output
pub struct CompilerTest {
    diagnostics: DiagnosticsHandler,
    source: CompilerTestSource,
    entrypoint: Option<FunctionIdent>,
    wasm: Option<Vec<u8>>,
    hir: Option<miden_hir::Module>,
    ir_masm: Option<miden_codegen_masm::Program>,
}

impl CompilerTest {
    /// Set the Rust source code to compile using a Cargo project and binary bundle name
    pub fn rust_source_cargo(
        cargo_project_folder: &str,
        artifact_name: &str,
        entrypoint: &str,
    ) -> Self {
        let manifest_path = format!("../rust-wasm/{}/Cargo.toml", cargo_project_folder);
        // dbg!(&pwd);
        let temp_dir = std::env::temp_dir();
        let target_dir = temp_dir.join(cargo_project_folder);
        let output = Command::new("cargo")
            .arg(format!(
                "+{}",
                std::env::var("CARGO_MAKE_TOOLCHAIN").unwrap()
            ))
            .arg("build")
            .arg("--manifest-path")
            .arg(manifest_path)
            .arg("--release")
            // .arg("--bins")
            .arg("--target=wasm32-unknown-unknown")
            // .arg("--features=wasm-target")
            .arg("--target-dir")
            .arg(target_dir.clone())
            // compile std as part of crate graph compilation
            // https://doc.rust-lang.org/cargo/reference/unstable.html#build-std
            .arg("-Z")
            .arg("build-std=core,alloc")
            .arg("-Z")
            // abort on panic without message formatting (core::fmt uses call_indirect)
            .arg("build-std-features=panic_immediate_abort")
            .output()
            .expect("Failed to execute cargo build.");
        if !output.status.success() {
            eprintln!("pwd: {:?}", std::env::current_dir().unwrap());
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            panic!("Rust to Wasm compilation failed!");
        }
        let target_bin_file_path = Path::new(&target_dir)
            .join("wasm32-unknown-unknown")
            .join("release")
            .join(artifact_name)
            .with_extension("wasm");
        // dbg!(&target_bin_file_path);
        let mut target_bin_file = fs::File::open(target_bin_file_path).unwrap();
        let mut wasm_bytes = vec![];
        Read::read_to_end(&mut target_bin_file, &mut wasm_bytes).unwrap();
        fs::remove_dir_all(target_dir).unwrap();

        let diagnostics = make_diagnostics();

        CompilerTest {
            diagnostics,
            source: CompilerTestSource::RustCargo {
                cargo_project_folder_name: cargo_project_folder.to_string(),
                artifact_name: artifact_name.to_string(),
            },
            wasm: Some(wasm_bytes),
            hir: None,
            entrypoint: Some(FunctionIdent {
                module: Ident::new(Symbol::intern("noname"), SourceSpan::default()),
                function: Ident::new(
                    Symbol::intern(entrypoint.to_string()),
                    SourceSpan::default(),
                ),
            }),
            ir_masm: None,
        }
    }

    /// Set the Rust source code to compile
    pub fn rust_source(rust_source: &str) -> Self {
        CompilerTest {
            diagnostics: make_diagnostics(),
            source: CompilerTestSource::Rust(rust_source.to_string()),
            wasm: Some(compile_wasm(rust_source)),
            hir: None,
            entrypoint: None,
            ir_masm: None,
        }
    }

    /// Compare the compiled Wasm against the expected output
    pub fn expect_wasm(&mut self, expected_wat_file: expect_test::ExpectFile) {
        let wasm_bytes = self.wasm.as_ref().expect("Wasm is not compiled");
        let wat = demangle(&wasm_to_wat(wasm_bytes));
        expected_wat_file.assert_eq(&wat);
    }

    /// Compare the compiled IR against the expected output
    pub fn expect_ir(&mut self, expected_hir_file: expect_test::ExpectFile) {
        let wasm_bytes = self.wasm.as_ref().expect("Wasm is not compiled");
        let module = wasm_to_ir_module(wasm_bytes, &self.diagnostics);
        let ir = demangle(&module.to_string());
        self.hir = Some(module);
        expected_hir_file.assert_eq(&ir);
    }

    /// Compare the compiled MASM against the expected output
    pub fn expect_masm(&mut self, _expected_masm_file: expect_test::ExpectFile) {
        let hir = self.hir.take().expect("IR is not compiled");
        let program_with_entrypoint =
            ir_module_to_masm_program(hir, self.entrypoint.unwrap(), &self.diagnostics);
        // TODO: check midenc PR if it fixes the issue with to_program_ast (invalid entrypoint name)
        // expected_masm_file.assert_eq(&program_with_entrypoint.to_program_ast().to_string());
        self.ir_masm = Some(program_with_entrypoint);
    }

    /// Get the compiled MASM as [`miden_codegen_masm::Program`]
    pub fn codegen_masm_program(mut self) -> miden_codegen_masm::Program {
        self.ir_masm.take().expect("MASM is not compiled")
    }

    /// Get the compiled MASM as [`miden_assembly::Module`]
    pub fn asm_masm_module(&self) -> miden_assembly::Module {
        todo!()
    }
}

fn compile_wasm(rust_source: &str) -> Vec<u8> {
    let rustc_opts = [
        "-C",
        "opt-level=z", // optimize for size
        "--target",
        "wasm32-unknown-unknown",
    ];
    let file_name = hash_string(&[rust_source]);
    let temp_dir = std::env::temp_dir();
    let input_file = temp_dir.join(format!("{file_name}.rs"));
    let output_file = temp_dir.join(format!("{file_name}.wasm"));
    fs::write(&input_file, rust_source).unwrap();
    let output = Command::new("rustc")
        .args(&rustc_opts)
        .arg(&input_file)
        .arg("-o")
        .arg(&output_file)
        .output()
        .expect("Failed to execute rustc.");
    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("Rust to Wasm compilation failed!");
    }
    let wasm = fs::read(&output_file).unwrap();
    fs::remove_file(&input_file).unwrap();
    fs::remove_file(&output_file).unwrap();
    return wasm;
}

fn wasm_to_wat(wasm_bytes: &Vec<u8>) -> String {
    let mut wasm_printer = wasmprinter::Printer::new();
    // disable printing of the "producers" section because it contains a rustc version
    // to not brake tests when rustc is updated
    wasm_printer.add_custom_section_printer("producers", |_, _, _| Ok(()));
    let wat = wasm_printer.print(wasm_bytes.as_ref()).unwrap();
    wat
}

fn wasm_to_ir_module(wasm_bytes: &[u8], diagnostics: &DiagnosticsHandler) -> miden_hir::Module {
    let module =
        translate_module(wasm_bytes, &WasmTranslationConfig::default(), diagnostics).unwrap();
    module
}

fn ir_module_to_masm_program(
    hir: miden_hir::Module,
    entrypoint: FunctionIdent,
    diagnostics: &DiagnosticsHandler,
) -> Program {
    let mut compiler = MasmCompiler::new(diagnostics);
    let program = compiler.compile_module(hir.into()).unwrap();
    let program_with_entrypoint = Program {
        modules: program.modules,
        entrypoint: Some(entrypoint),
        segments: program.segments,
    };
    program_with_entrypoint
}

fn default_emitter(verbosity: Verbosity, color: ColorChoice) -> Arc<dyn Emitter> {
    match verbosity {
        Verbosity::Silent => Arc::new(NullEmitter::new(color)),
        _ => Arc::new(DefaultEmitter::new(color)),
    }
}

fn demangle(name: &str) -> String {
    let mut input = name.as_bytes();
    let mut demangled = Vec::new();
    let include_hash = false;
    rustc_demangle::demangle_stream(&mut input, &mut demangled, include_hash).unwrap();
    String::from_utf8(demangled).unwrap()
}

fn make_diagnostics() -> DiagnosticsHandler {
    let codemap = Arc::new(CodeMap::new());
    let diagnostics = DiagnosticsHandler::new(
        DiagnosticsConfig {
            verbosity: Verbosity::Debug,
            warnings_as_errors: false,
            no_warn: false,
            display: Default::default(),
        },
        codemap,
        default_emitter(Verbosity::Debug, ColorChoice::Auto),
    );
    diagnostics
}

fn hash_string(inputs: &[&str]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    for input in inputs {
        hasher.update(input);
    }
    format!("{:x}", hasher.finalize())
}
