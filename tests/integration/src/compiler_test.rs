#![allow(dead_code)]

use core::panic;
use std::{
    fmt::Write,
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::Arc,
};

use miden_assembly::{ast::ModuleKind, diagnostics::Report, Assembler, LibraryPath};
use miden_core::Program;
use miden_diagnostics::SourceSpan;
use miden_stdlib::StdLibrary;
use midenc_frontend_wasm::{translate, WasmTranslationConfig};
use midenc_hir::{FunctionIdent, Ident, Symbol};
use midenc_session::{
    InputFile, InputType, Options, OutputType, OutputTypeSpec, OutputTypes, ProjectType, Session,
};

use crate::cargo_proj::project;

type LinkMasmModules = Vec<(LibraryPath, String)>;

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
    Program(Box<midenc_hir::Program>),
    Module(Box<midenc_hir::Module>),
    Component(Box<midenc_hir::Component>),
}

impl HirArtifact {
    pub fn unwrap_module(&self) -> &midenc_hir::Module {
        match self {
            HirArtifact::Module(module) => module,
            _ => panic!("Expected a Module"),
        }
    }

    pub fn unwrap_program(&self) -> &midenc_hir::Program {
        match self {
            Self::Program(program) => program,
            _ => panic!("attempted to unwrap a program, but had a component"),
        }
    }

    pub fn unwrap_component(&self) -> &midenc_hir::Component {
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
    pub masm_program: Option<Box<miden_core::Program>>,
    /// The MASM source code
    pub masm_src: Option<String>,
    /// The extra MASM modules to link to the compiled MASM program
    pub link_masm_modules: LinkMasmModules,
}

impl Default for CompilerTest {
    fn default() -> Self {
        Self {
            config: WasmTranslationConfig::default(),
            session: Arc::new(dummy_session()),
            source: CompilerTestSource::Rust(String::new()),
            entrypoint: None,
            hir: None,
            masm_program: None,
            masm_src: None,
            link_masm_modules: Vec::new(),
        }
    }
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
            ..Default::default()
        }
    }

    /// Set the Rust source code to compile a library Cargo project to Wasm module
    pub fn rust_source_cargo_lib(
        cargo_project_folder: PathBuf,
        artifact_name: &str,
        is_build_std: bool,
        entry_func_name: Option<String>,
    ) -> Self {
        let expected_wasm_artifact_path = wasm_artifact_path(&cargo_project_folder, artifact_name);
        // dbg!(&wasm_artifact_path);
        let wasm_artifact_path = if !skip_rust_compilation(&cargo_project_folder, artifact_name)
            || !expected_wasm_artifact_path.exists()
        {
            let manifest_path = cargo_project_folder.join("Cargo.toml");
            let mut cargo_build_cmd = Command::new("cargo");
            let compiler_workspace_dir = get_workspace_dir();
            // Enable Wasm bulk-memory proposal (uses Wasm `memory.copy` op instead of `memcpy`
            // import) Remap the compiler workspace directory to `~` to have a
            // reproducible build that does not have the absolute local path baked into
            // the Wasm binary
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

            // Find the Wasm artifacts from the cargo build output for debugging purposes
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
            dbg!(&wasm_artifacts);
            assert_eq!(wasm_artifacts.len(), 1, "Expected one Wasm artifact");
            wasm_artifacts.first().unwrap().to_path_buf()
        } else {
            expected_wasm_artifact_path
        };

        let entrypoint = entry_func_name.map(|func_name| FunctionIdent {
            module: Ident::new(Symbol::intern(artifact_name), SourceSpan::default()),
            function: Ident::new(Symbol::intern(func_name.to_string()), SourceSpan::default()),
        });
        let input_file = InputFile::from_path(wasm_artifact_path).unwrap();
        Self {
            config: WasmTranslationConfig::default(),
            session: default_session(input_file),
            source: CompilerTestSource::RustCargoLib {
                artifact_name: artifact_name.to_string(),
            },
            entrypoint,
            ..Default::default()
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
            session,
            source: CompilerTestSource::RustCargo {
                cargo_project_folder_name: cargo_project_folder.to_string(),
                artifact_name: artifact_name.to_string(),
            },
            entrypoint: Some(entrypoint),
            ..Default::default()
        }
    }

    /// Set the Rust source code to compile
    pub fn rust_source_program(rust_source: &str) -> Self {
        let wasm_file = compile_rust_file(rust_source);
        let session = default_session(wasm_file);
        CompilerTest {
            session,
            source: CompilerTestSource::Rust(rust_source.to_string()),
            ..Default::default()
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
            session,
            source: CompilerTestSource::Rust(rust_source.to_string()),
            entrypoint: Some(entrypoint),
            ..Default::default()
        }
    }

    /// Set the Rust source code to compile with `miden-stdlib-sys` (stdlib + intrinsics)
    pub fn rust_fn_body_with_stdlib_sys(name: &str, rust_source: &str, is_build_std: bool) -> Self {
        let miden_stdlib_sys_path_str = stdlib_sys_crate_path();
        let proj = project(name)
            .file(
                "Cargo.toml",
                format!(
                    r#"
                [package]
                name = "{name}"
                version = "0.0.1"
                edition = "2021"
                authors = []

                [dependencies]
                wee_alloc = {{ version = "0.4.5", default-features = false}}
                miden-stdlib-sys = {{ path = "{miden_stdlib_sys_path_str}" }}

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

                extern crate miden_stdlib_sys;
                use miden_stdlib_sys::*;

                #[no_mangle]
                pub extern "C" fn entrypoint{}
            "#,
                    rust_source
                )
                .as_str(),
            )
            .build();
        Self::rust_source_cargo_lib(proj.root(), name, is_build_std, Some("entrypoint".to_string()))
    }

    /// Set the Rust source code to compile with `miden-stdlib-sys` (stdlib + intrinsics)
    pub fn rust_fn_body_with_sdk(name: &str, rust_source: &str, is_build_std: bool) -> Self {
        let cwd = std::env::current_dir().unwrap();
        let miden_sdk_path = cwd.parent().unwrap().parent().unwrap().join("sdk").join("sdk");
        let miden_sdk_path_str = miden_sdk_path.to_str().unwrap();
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
                miden-sdk = {{ path = "{miden_sdk_path_str}" }}

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

                extern crate miden_sdk;
                use miden_sdk::*;

                extern crate alloc;
                use alloc::vec::Vec;

                #[no_mangle]
                pub extern "C" fn entrypoint{}
            "#,
                    rust_source
                )
                .as_str(),
            )
            .build();

        Self::rust_source_cargo_lib(proj.root(), name, is_build_std, Some("entrypoint".to_string()))
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
                        .collect::<Vec<&midenc_hir::Module>>()
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
        let program = demangle(self.masm_src().as_str());
        expected_masm_file.assert_eq(&program);
    }

    /// Get the compiled MASM as [`miden_core::Program`]
    pub fn masm_program(&mut self) -> Box<miden_core::Program> {
        if self.masm_program.is_none() {
            let (masm, src) = self.compile_wasm_to_masm_program();
            self.masm_src = Some(src);
            let unwrapped = masm.unwrap_or_else(|e| panic!("Failed to assemble MASM: {:?}", e));
            self.masm_program = Some(unwrapped.into());
        }
        self.masm_program.clone().unwrap()
    }

    /// Get the MASM source code
    pub fn masm_src(&mut self) -> String {
        if self.masm_src.is_none() {
            let (masm, src) = self.compile_wasm_to_masm_program();
            self.masm_src = Some(src);
            self.masm_program = masm.ok().map(Box::from);
        }
        self.masm_src.clone().unwrap()
    }

    /// The compiled Wasm component/module
    fn wasm_bytes(&self) -> Vec<u8> {
        match &self.session.input.file {
            InputType::Real(file_path) => fs::read(file_path)
                .unwrap_or_else(|_| panic!("Failed to read Wasm file: {}", file_path.display())),
            InputType::Stdin { name: _, input } => input.clone(),
        }
    }

    pub(crate) fn compile_wasm_to_masm_program(
        &self,
    ) -> (Result<miden_core::Program, Report>, String) {
        match midenc_compile::compile_to_memory(self.session.clone()).unwrap() {
            midenc_compile::Compiled::Program(_p) => todo!("Program compilation not yet supported"),
            midenc_compile::Compiled::Modules(modules) => {
                let src = expected_masm_prog_source_from_modules(
                    &modules,
                    self.entrypoint,
                    &self.link_masm_modules,
                );
                let prog =
                    masm_prog_from_modules(&modules, self.entrypoint, &self.link_masm_modules);
                (prog, src)
            }
        }
    }
}

fn wasm_artifact_path(cargo_project_folder: &Path, artifact_name: &str) -> PathBuf {
    cargo_project_folder
        .to_path_buf()
        .join("target")
        .join("wasm32-wasi")
        .join("release")
        .join(artifact_name)
        .with_extension("wasm")
}

/// Directs if we should do the Rust compilation step or not
pub fn skip_rust_compilation(cargo_project_folder: &Path, artifact_name: &str) -> bool {
    let expected_wasm_artifact_path = wasm_artifact_path(cargo_project_folder, artifact_name);
    let skip_rust = std::env::var("SKIP_RUST").is_ok() && expected_wasm_artifact_path.exists();
    if skip_rust {
        eprintln!("Skipping Rust compilation");
    };
    skip_rust
}

// Assemble the VM MASM program from the compiled IR MASM modules
fn masm_prog_from_modules(
    modules: &[Box<midenc_codegen_masm::Module>],
    entrypoint: Option<FunctionIdent>,
    link_masm_modules: &LinkMasmModules,
) -> Result<Program, Report> {
    let mut assembler = Assembler::default().with_library(&StdLibrary::default())?;
    for (path, src) in link_masm_modules {
        let options = miden_assembly::CompileOptions {
            kind: ModuleKind::Library,
            warnings_as_errors: false,
            path: Some(path.clone()),
        };
        assembler.add_module_with_options(src, options)?;
    }
    for module in modules {
        let module_src = format!("{}", module);
        // eprintln!("{}", &module_src);
        let path = module.id.as_str().to_string();
        let library_path = LibraryPath::new(path).unwrap();
        // dbg!(&library_path);
        let options = miden_assembly::CompileOptions {
            kind: ModuleKind::Library,
            warnings_as_errors: false,
            path: Some(library_path),
        };
        assembler.add_module_with_options(module_src, options)?;
    }
    if let Some(entrypoint) = entrypoint {
        let prog_source = masm_prog_source(entrypoint);
        assembler.assemble_program(prog_source)
    } else {
        todo!()
    }
}

// Generate the MASM program source code from the compiled IR MASM modules
fn expected_masm_prog_source_from_modules(
    modules: &[Box<midenc_codegen_masm::Module>],
    entrypoint: Option<FunctionIdent>,
    link_masm_modules: &LinkMasmModules,
) -> String {
    let mut src = String::new();
    for (path, module_src) in link_masm_modules {
        writeln!(src, "# mod {path}\n").unwrap();
        writeln!(src, "{module_src}").unwrap();
    }
    for module in modules {
        let module_src = format!("{}", module);
        let path = module.id.as_str().to_string();
        if !path.contains("intrinsic") {
            // print only user modules and not intrinsic modules
            writeln!(src, "# mod {path}\n").unwrap();
            write!(src, "{module_src}").unwrap();
        }
    }
    if let Some(entrypoint) = entrypoint {
        let prog_source = masm_prog_source(entrypoint);
        src.push_str(&prog_source);
    } else {
        todo!()
    }
    src
}

// Generate the MASM program source code (call the entrypoint function)
fn masm_prog_source(entrypoint: FunctionIdent) -> String {
    let module_name = entrypoint.module.as_str();
    let function_name = entrypoint.function.as_str();
    format!(
        r#"
begin
    exec.::{module_name}::{function_name}  
end"#,
    )
}

fn stdlib_sys_crate_path() -> String {
    let cwd = std::env::current_dir().unwrap();
    cwd.parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("sdk")
        .join("stdlib-sys")
        .to_str()
        .unwrap()
        .to_string()
}

pub fn sdk_crate_path() -> String {
    let cwd = std::env::current_dir().unwrap();
    cwd.parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("sdk")
        .join("sdk")
        .to_str()
        .unwrap()
        .to_string()
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

fn dummy_session() -> Session {
    let output_type = OutputType::Masm;
    let output_types = OutputTypes::new(vec![OutputTypeSpec {
        output_type,
        path: None,
    }]);
    let options = Options::default().with_output_types(output_types);
    Session::new(
        Default::default(),
        InputFile::from_path(PathBuf::from("dummy.wasm")).unwrap(),
        None,
        None,
        None,
        options,
        None,
    )
    .with_project_type(ProjectType::Library)
}

/// Create a default session for testing
pub fn default_session(input_file: InputFile) -> Arc<Session> {
    let default_session = dummy_session();
    let session = Session::new(
        Default::default(),
        input_file,
        None,
        None,
        None,
        default_session.options,
        None,
    )
    .with_project_type(ProjectType::Library);
    Arc::new(session)
}

fn hash_string(inputs: &str) -> String {
    let hash = <sha2::Sha256 as sha2::Digest>::digest(inputs.as_bytes());
    format!("{:x}", hash)
}
