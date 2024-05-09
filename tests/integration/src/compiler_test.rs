use core::panic;
use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::Arc,
};

use miden_assembly::{
    ast::{ModuleKind, ProcedureName},
    Assembler, LibraryPath,
};
use miden_codegen_masm::{Function, Module};
use miden_diagnostics::{CodeMap, SourceSpan};
use miden_frontend_wasm::{translate, WasmTranslationConfig};
use miden_hir::{FunctionIdent, Ident, Signature, Symbol};
use miden_stdlib::StdLibrary;
use midenc_session::{
    InputFile, InputType, Options, OutputType, OutputTypeSpec, OutputTypes, ProjectType, Session,
};

use crate::cargo_proj::project;

pub enum CompilerTestSource {
    Rust(String),
    RustCargo {
        cargo_project_folder_name: String,
        artifact_name: String,
    },
    RustCargoLib {
        artifact_name: String,
    },
    RustCargoComponent {
        artifact_name: String,
    },
}

impl CompilerTestSource {
    pub fn artifact_name(&self) -> String {
        match self {
            CompilerTestSource::RustCargo {
                cargo_project_folder_name: _,
                artifact_name,
            } => artifact_name.clone(),
            CompilerTestSource::RustCargoLib { artifact_name } => artifact_name.clone(),
            CompilerTestSource::RustCargoComponent { artifact_name } => artifact_name.clone(),
            _ => panic!("Not a Rust Cargo project"),
        }
    }
}

#[derive(derive_more::From)]
pub enum HirArtifact {
    Program(Box<miden_hir::Program>),
    Module(Box<miden_hir::Module>),
    Component(Box<miden_hir::Component>),
}

impl HirArtifact {
    pub fn unwrap_module(&self) -> &miden_hir::Module {
        match self {
            HirArtifact::Module(module) => module,
            _ => panic!("Expected a Module"),
        }
    }

    pub fn unwrap_program(&self) -> &miden_hir::Program {
        match self {
            Self::Program(program) => program,
            _ => panic!("attempted to unwrap a program, but had a component"),
        }
    }

    pub fn unwrap_component(&self) -> &miden_hir::Component {
        match self {
            Self::Component(program) => program,
            _ => panic!("attempted to unwrap a component, but had a program"),
        }
    }
}

/// Compile to different stages (e.g. Wasm, IR, MASM) and compare the results against expected
/// output
pub struct CompilerTest {
    /// The Wasm translation configuration
    pub config: WasmTranslationConfig,
    /// The compiler session
    pub session: Arc<Session>,
    /// The source code used to compile the test
    pub source: CompilerTestSource,
    /// The entrypoint function to use when building the IR
    entrypoint: Option<FunctionIdent>,
    /// The compiled IR
    pub hir: Option<HirArtifact>,
    /// The compiled MASM
    pub ir_masm: Option<Arc<miden_codegen_masm::Program>>,
}

impl CompilerTest {
    /// Compile the Wasm component from a Rust Cargo project using cargo-component
    pub fn rust_source_cargo_component(
        cargo_project_folder: PathBuf,
        config: WasmTranslationConfig,
    ) -> Self {
        let manifest_path = cargo_project_folder.join("Cargo.toml");
        let mut cargo_build_cmd = Command::new("cargo");
        let compiler_workspace_dir = get_workspace_dir();
        // Enable Wasm bulk-memory proposal (uses Wasm `memory.copy` op instead of `memcpy` import)
        // Remap the compiler workspace directory to `~` to have a reproducible build that does not
        // have the absolute local path baked into the Wasm binary
        cargo_build_cmd.env(
            "RUSTFLAGS",
            format!(
                "-C target-feature=+bulk-memory --remap-path-prefix {compiler_workspace_dir}=~"
            ),
        );
        cargo_build_cmd
            .arg("component")
            .arg("build")
            .arg("--manifest-path")
            .arg(manifest_path)
            .arg("--release")
            // compile std as part of crate graph compilation
            // https://doc.rust-lang.org/cargo/reference/unstable.html#build-std
            .arg("-Z")
            .arg("build-std=std,core,alloc,panic_abort")
            .arg("-Z")
            // abort on panic without message formatting (core::fmt uses call_indirect)
            .arg("build-std-features=panic_immediate_abort");
        let mut child = cargo_build_cmd
            .arg("--message-format=json-render-diagnostics")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to execute cargo build {}.",
                    cargo_build_cmd
                        .get_args()
                        .map(|arg| format!("'{}'", arg.to_str().unwrap()))
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            });
        let wasm_artifacts = find_wasm_artifacts(&mut child);
        let output = child.wait().expect("Couldn't get cargo's exit status");
        if !output.success() {
            report_cargo_error(child);
        }
        assert!(output.success());
        assert_eq!(wasm_artifacts.len(), 1, "Expected one Wasm artifact");
        let wasm_comp_path = &wasm_artifacts.first().unwrap();
        let artifact_name = wasm_comp_path.file_stem().unwrap().to_str().unwrap().to_string();
        let input_file = InputFile::from_path(wasm_comp_path).unwrap();
        Self {
            config,
            session: default_session(input_file),
            source: CompilerTestSource::RustCargoComponent { artifact_name },
            entrypoint: None,
            hir: None,
            ir_masm: None,
        }
    }

    /// Set the Rust source code to compile a library Cargo project to Wasm module
    pub fn rust_source_cargo_lib(
        cargo_project_folder: PathBuf,
        is_build_std: bool,
        entry_func_name: Option<String>,
    ) -> Self {
        let manifest_path = cargo_project_folder.join("Cargo.toml");
        let mut cargo_build_cmd = Command::new("cargo");
        let compiler_workspace_dir = get_workspace_dir();
        // Enable Wasm bulk-memory proposal (uses Wasm `memory.copy` op instead of `memcpy` import)
        // Remap the compiler workspace directory to `~` to have a reproducible build that does not
        // have the absolute local path baked into the Wasm binary
        cargo_build_cmd.env(
            "RUSTFLAGS",
            format!(
                "-C target-feature=+bulk-memory --remap-path-prefix {compiler_workspace_dir}=~"
            ),
        );
        cargo_build_cmd
            .arg("build")
            .arg("--manifest-path")
            .arg(manifest_path)
            .arg("--release")
            .arg("--target=wasm32-wasi");
        if is_build_std {
            // compile std as part of crate graph compilation
            // https://doc.rust-lang.org/cargo/reference/unstable.html#build-std
            cargo_build_cmd.arg("-Z")
            .arg("build-std=std,core,alloc,panic_abort")
            .arg("-Z")
            // abort on panic without message formatting (core::fmt uses call_indirect)
            .arg("build-std-features=panic_immediate_abort");
        }
        let mut child = cargo_build_cmd
            .arg("--message-format=json-render-diagnostics")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to execute cargo build {}.",
                    cargo_build_cmd
                        .get_args()
                        .map(|arg| format!("'{}'", arg.to_str().unwrap()))
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            });
        let mut wasm_artifacts = find_wasm_artifacts(&mut child);
        let output = child.wait().expect("Couldn't get cargo's exit status");
        if !output.success() {
            report_cargo_error(child);
        }
        assert!(output.success());
        // filter out dependencies
        wasm_artifacts.retain(|path| {
            let path_str = path.to_str().unwrap();
            !path_str.contains("release/deps")
        });
        // dbg!(&wasm_artifacts);
        assert_eq!(wasm_artifacts.len(), 1, "Expected one Wasm artifact");
        let wasm_comp_path = &wasm_artifacts.first().unwrap();
        let artifact_name = wasm_comp_path.file_stem().unwrap().to_str().unwrap().to_string();
        dbg!(&artifact_name);
        let entrypoint = entry_func_name.map(|func_name| FunctionIdent {
            module: Ident::new(Symbol::intern(artifact_name.clone()), SourceSpan::default()),
            function: Ident::new(Symbol::intern(func_name.to_string()), SourceSpan::default()),
        });
        let input_file = InputFile::from_path(wasm_artifacts.first().unwrap()).unwrap();
        Self {
            config: WasmTranslationConfig::default(),
            session: default_session(input_file),
            source: CompilerTestSource::RustCargoLib { artifact_name },
            entrypoint,
            hir: None,
            ir_masm: None,
        }
    }

    /// Set the Rust source code to compile using a Cargo project and binary bundle name
    pub fn rust_source_cargo(
        cargo_project_folder: &str,
        artifact_name: &str,
        entrypoint: &str,
    ) -> Self {
        let manifest_path = format!("../rust-apps-wasm/{}/Cargo.toml", cargo_project_folder);
        // dbg!(&pwd);
        let temp_dir = std::env::temp_dir();
        let target_dir = temp_dir.join(cargo_project_folder);
        let output = Command::new("cargo")
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

        let input_file = InputFile::from_path(target_bin_file_path).unwrap();
        let session = default_session(input_file);
        let entrypoint = FunctionIdent {
            module: Ident::new(Symbol::intern(artifact_name), SourceSpan::default()),
            function: Ident::new(Symbol::intern(entrypoint.to_string()), SourceSpan::default()),
        };
        CompilerTest {
            config: WasmTranslationConfig::default(),
            session,
            source: CompilerTestSource::RustCargo {
                cargo_project_folder_name: cargo_project_folder.to_string(),
                artifact_name: artifact_name.to_string(),
            },
            entrypoint: Some(entrypoint),
            hir: None,
            ir_masm: None,
        }
    }

    /// Set the Rust source code to compile
    pub fn rust_source_program(rust_source: &str) -> Self {
        let wasm_file = compile_rust_file(rust_source);
        let session = default_session(wasm_file);
        CompilerTest {
            config: WasmTranslationConfig::default(),
            session,
            source: CompilerTestSource::Rust(rust_source.to_string()),
            entrypoint: None,
            hir: None,
            ir_masm: None,
        }
    }

    /// Set the Rust source code to compile and add a binary operation test
    pub fn rust_fn_body(rust_source: &str) -> Self {
        let rust_source = format!(
            r#"
            #![no_std]
            #![no_main]

            #[panic_handler]
            fn my_panic(_info: &core::panic::PanicInfo) -> ! {{
                loop {{}}
            }}

            #[no_mangle]
            pub extern "C" fn entrypoint{}
            "#,
            rust_source
        );
        let wasm_file = compile_rust_file(&rust_source);
        let wasm_filestem = wasm_file.filestem().to_string();
        let session = default_session(wasm_file);
        let entrypoint = FunctionIdent {
            module: Ident {
                name: Symbol::intern(wasm_filestem),
                span: SourceSpan::default(),
            },
            function: Ident {
                name: Symbol::intern("entrypoint"),
                span: SourceSpan::default(),
            },
        };

        CompilerTest {
            config: WasmTranslationConfig::default(),
            session,
            source: CompilerTestSource::Rust(rust_source.to_string()),
            entrypoint: Some(entrypoint),
            hir: None,
            ir_masm: None,
        }
    }

    /// Set the Rust source code to compile with `miden-prelude` (stdlib + intrinsics)
    pub fn rust_fn_body_with_prelude(name: &str, rust_source: &str) -> Self {
        let cwd = std::env::current_dir().unwrap();
        let miden_prelude_path =
            cwd.parent().unwrap().parent().unwrap().join("sdk").join("prelude");
        let miden_prelude_path_str = miden_prelude_path.to_str().unwrap();
        // dbg!(&miden_prelude_path);
        let proj = project(name)
            .file(
                "Cargo.toml",
                format!(
                    r#"
                [package]
                name = "{name}"
                version = "0.0.1"
                edition = "2015"
                authors = []

                [dependencies]
                wee_alloc = {{ version = "0.4.5", default-features = false}}
                miden-prelude = {{ path = "{miden_prelude_path_str}" }}

                [lib]
                crate-type = ["cdylib"]

                [profile.release]
                panic = "abort"
                # optimize for size
                opt-level = "z"
            "#
                )
                .as_str(),
            )
            .file(
                "src/lib.rs",
                format!(
                    r#"
                #![no_std]
                #![no_main]

                #[panic_handler]
                fn my_panic(_info: &core::panic::PanicInfo) -> ! {{
                    loop {{}}
                }}


                #[global_allocator]
                static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

                extern crate miden_prelude;
                use miden_prelude::*;

                #[no_mangle]
                pub extern "C" fn entrypoint{}
            "#,
                    rust_source
                )
                .as_str(),
            )
            .build();

        Self::rust_source_cargo_lib(proj.root(), false, Some("entrypoint".to_string()))
    }

    /// Compare the compiled Wasm against the expected output
    pub fn expect_wasm(&self, expected_wat_file: expect_test::ExpectFile) {
        let wasm_bytes = self.wasm_bytes();
        let wat = demangle(&wasm_to_wat(&wasm_bytes));
        expected_wat_file.assert_eq(&wat);
    }

    fn wasm_to_ir(&self) -> HirArtifact {
        let ir_component = translate(&self.wasm_bytes(), &self.config, &self.session.diagnostics)
            .expect("Failed to translate Wasm binary to IR component");
        Box::new(ir_component).into()
    }

    /// Get the compiled IR, compiling the Wasm if it has not been compiled yet
    pub fn hir(&mut self) -> &HirArtifact {
        if self.hir.is_none() {
            self.hir = Some(self.wasm_to_ir());
        }
        self.hir.as_ref().unwrap()
    }

    /// Compare the compiled IR against the expected output
    pub fn expect_ir(&mut self, expected_hir_file: expect_test::ExpectFile) {
        match self.hir() {
            HirArtifact::Program(hir_program) => {
                // Program does not implement pretty printer yet, use the first module
                let ir_module = demangle(
                    hir_program
                        .modules()
                        .iter()
                        .take(1)
                        .collect::<Vec<&miden_hir::Module>>()
                        .first()
                        .expect("no module in IR program")
                        .to_string()
                        .as_str(),
                );
                expected_hir_file.assert_eq(&ir_module);
            }
            HirArtifact::Component(hir_component) => {
                let ir_component = demangle(&hir_component.to_string());
                expected_hir_file.assert_eq(&ir_component);
            }
            HirArtifact::Module(hir_module) => {
                let ir_module = demangle(&hir_module.to_string());
                expected_hir_file.assert_eq(&ir_module);
            }
        }
    }

    /// Compare the compiled MASM against the expected output
    pub fn expect_masm(&mut self, expected_masm_file: expect_test::ExpectFile) {
        let program = self.ir_masm_program();
        expected_masm_file.assert_eq(&program.to_string());
    }

    /// Get the compiled MASM as [`miden_assembly::Program`]
    pub fn vm_masm_program(&mut self) -> miden_core::Program {
        let mut assembler = Assembler::default()
            .with_library(&StdLibrary::default())
            .expect("Failed to load stdlib");
        let program = self.ir_masm_program();
        // TODO: get code map from the self.diagnostics
        let codemap = CodeMap::new();
        for module in program.modules() {
            if !module.kind.is_library() {
                continue;
            }
            let core_module = module.to_ast(&codemap).expect("failed to translate MASM IR module");
            assembler
                .add_module(core_module)
                .unwrap_or_else(|err| panic!("VM assembler failed to validate module:\n{}", err));
        }
        let exe = program
            .modules()
            .find(|m| m.name.is_exec_path())
            .expect("expected executable module")
            .to_ast(&codemap)
            .expect("failed to translate MASM IR module");
        assembler
            .assemble(exe)
            .unwrap_or_else(|err| panic!("VM assembler failed to compile program:\n{}", err))
    }

    /// Get the compiled MASM as [`miden_codegen_masm::Program`]
    pub fn ir_masm_program(&mut self) -> Arc<miden_codegen_masm::Program> {
        if self.ir_masm.is_none() {
            let ir_masm = self.compile_wasm_to_masm_program();
            let frozen = ir_masm.freeze();
            self.ir_masm = Some(frozen);
        }
        self.ir_masm.clone().unwrap()
    }

    /// The compiled Wasm component/module
    fn wasm_bytes(&self) -> Vec<u8> {
        match &self.session.input.file {
            InputType::Real(file_path) => fs::read(file_path).unwrap(),
            InputType::Stdin { name: _, input } => input.clone(),
        }
    }

    fn compile_wasm_to_masm_program(&self) -> Box<miden_codegen_masm::Program> {
        match midenc_compile::compile_to_memory(self.session.clone()).unwrap() {
            midenc_compile::Compiled::Program(p) => p,
            midenc_compile::Compiled::Modules(modules) => {
                assert_eq!(modules.len(), 1, "Expected one module");
                let mut program = miden_codegen_masm::Program::empty();
                for module in modules.into_iter() {
                    program.insert(module);
                }
                if let Some(entrypoint) = self.entrypoint {
                    // Start module name with "zzz_" to ensure it is the last module in the program
                    // due to the RBTree ordering (used to store modules of a program)
                    let module_name = "zzz_entrypoint_module";
                    let func_name = format!("{module_name}::{}", ProcedureName::MAIN_PROC_NAME);
                    let mut entrypoint_function =
                        Function::new(func_name.parse().unwrap(), Signature::new(vec![], vec![]));
                    entrypoint_function.attrs.set(miden_hir::attributes::ENTRYPOINT);
                    let body = entrypoint_function.block_mut(entrypoint_function.body.id());
                    body.push(miden_hir::MasmOp::Exec(entrypoint));
                    let mut name = LibraryPath::new(module_name).unwrap();
                    name.set_namespace(miden_assembly::LibraryNamespace::Exec);
                    let mut module = Module::new(name, ModuleKind::Executable);
                    module.push_back(Box::new(entrypoint_function));
                    module.imports.add(entrypoint);
                    program.insert(Box::new(module));
                }
                program.into()
            }
        }
    }
}

/// Get the directory for the top-level workspace
fn get_workspace_dir() -> String {
    // Get the directory for the integration test suite project
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_manifest_dir_path = Path::new(&cargo_manifest_dir);
    // "Exit" the integration test suite project directory to the compiler workspace directory
    // i.e. out of the `tests/integration` directory
    let compiler_workspace_dir =
        cargo_manifest_dir_path.parent().unwrap().parent().unwrap().to_str().unwrap();
    compiler_workspace_dir.to_string()
}

fn report_cargo_error(child: std::process::Child) {
    eprintln!("pwd: {:?}", std::env::current_dir().unwrap());
    let mut stderr = Vec::new();
    child.stderr.unwrap().read_exact(&mut stderr).expect("Failed to read stderr");
    let stderr = String::from_utf8(stderr).expect("Failed to parse stderr");
    eprintln!("stderr: {}", stderr);
    panic!("Rust to Wasm compilation failed!");
}

fn find_wasm_artifacts(child: &mut std::process::Child) -> Vec<std::path::PathBuf> {
    let mut wasm_artifacts = Vec::new();
    let reader = std::io::BufReader::new(child.stdout.take().unwrap());
    for message in cargo_metadata::Message::parse_stream(reader) {
        if let cargo_metadata::Message::CompilerArtifact(artifact) =
            message.expect("Failed to parse cargo metadata")
        {
            // find the Wasm artifact in artifact.filenames
            for filename in artifact.filenames {
                if filename.as_str().ends_with(".wasm") {
                    wasm_artifacts.push(filename.into_std_path_buf());
                }
            }
        }
    }
    wasm_artifacts
}

pub(crate) fn demangle(name: &str) -> String {
    let mut input = name.as_bytes();
    let mut demangled = Vec::new();
    let include_hash = false;
    rustc_demangle::demangle_stream(&mut input, &mut demangled, include_hash).unwrap();
    String::from_utf8(demangled).unwrap()
}

fn wasm_to_wat(wasm_bytes: &[u8]) -> String {
    let mut wasm_printer = wasmprinter::Printer::new();
    // disable printing of the "producers" section because it contains a rustc version
    // to not brake tests when rustc is updated
    wasm_printer.add_custom_section_printer("producers", |_, _, _| Ok(()));
    let wat = wasm_printer.print(wasm_bytes.as_ref()).unwrap();
    wat
}
fn compile_rust_file(rust_source: &str) -> InputFile {
    let rustc_opts = [
        "-C",
        "opt-level=z", // optimize for size
        "--target",
        "wasm32-unknown-unknown",
    ];
    let file_name = format!("test_rust_{}", hash_string(rust_source));
    let proj_dir = std::env::temp_dir().join(&file_name);
    if proj_dir.exists() {
        fs::remove_dir_all(&proj_dir).unwrap();
        fs::create_dir_all(&proj_dir).unwrap();
    } else {
        fs::create_dir_all(&proj_dir).unwrap();
    }
    let input_file = proj_dir.join(format!("{file_name}.rs"));
    let output_file = proj_dir.join(format!("{file_name}.wasm"));
    fs::write(&input_file, rust_source).unwrap();
    let output = Command::new("rustc")
        .args(rustc_opts)
        .arg(&input_file)
        .arg("-o")
        .arg(&output_file)
        .output()
        .expect("Failed to execute rustc.");
    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("Rust to Wasm compilation failed!");
    }
    InputFile::from_path(output_file).unwrap()
}

/// Create a default session for testing
pub fn default_session(input_file: InputFile) -> Arc<Session> {
    let output_type = OutputType::Masm;
    let output_types = OutputTypes::new(vec![OutputTypeSpec {
        output_type,
        path: None,
    }]);
    let options = Options::default().with_output_types(output_types);
    let session = Session::new(Default::default(), input_file, None, None, None, options, None)
        .with_project_type(ProjectType::Library);
    Arc::new(session)
}

fn hash_string(inputs: &str) -> String {
    let hash = <sha2::Sha256 as sha2::Digest>::digest(inputs.as_bytes());
    format!("{:x}", hash)
}
