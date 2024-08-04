#![allow(dead_code)]

use core::panic;
use std::{
    borrow::Cow,
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::Arc,
};

use miden_assembly::LibraryPath;
use midenc_frontend_wasm::{translate, WasmTranslationConfig};
use midenc_hir::{FunctionIdent, Ident, SourceSpan, Symbol};
use midenc_session::{InputFile, InputType, OutputType, Session};

use crate::cargo_proj::project;

type LinkMasmModules = Vec<(LibraryPath, String)>;

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

/// Configuration for tests which use as input, the artifact produced by a Cargo build
pub struct CargoTest {
    project_dir: PathBuf,
    manifest_path: Option<Cow<'static, str>>,
    target_dir: Option<PathBuf>,
    name: Cow<'static, str>,
    target: Cow<'static, str>,
    entrypoint: Option<Cow<'static, str>>,
    build_std: bool,
    build_alloc: bool,
}
impl CargoTest {
    /// Create a new `cargo` test with the given name, and project directory
    pub fn new(name: impl Into<Cow<'static, str>>, project_dir: PathBuf) -> Self {
        Self {
            project_dir,
            manifest_path: None,
            target_dir: None,
            name: name.into(),
            target: "wasm32-wasi".into(),
            entrypoint: None,
            build_std: false,
            build_alloc: false,
        }
    }

    /// Specify whether to build the entire standard library as part of the crate graph
    #[inline]
    pub fn with_build_std(mut self, build_std: bool) -> Self {
        self.build_std = build_std;
        self
    }

    /// Specify whether to build libcore and liballoc as part of the crate graph (implied by
    /// `with_build_std`)
    #[inline]
    pub fn with_build_alloc(mut self, build_alloc: bool) -> Self {
        self.build_alloc = build_alloc;
        self
    }

    /// Specify the target triple to pass to Cargo
    #[inline]
    pub fn with_target(mut self, target: impl Into<Cow<'static, str>>) -> Self {
        self.target = target.into();
        self
    }

    /// Specify the target directory for Cargo
    #[inline]
    pub fn with_target_dir(mut self, target_dir: impl Into<PathBuf>) -> Self {
        self.target_dir = Some(target_dir.into());
        self
    }

    /// Specify the name of the entrypoint function (just the function name, no namespace)
    #[inline]
    pub fn with_entrypoint(mut self, entrypoint: impl Into<Cow<'static, str>>) -> Self {
        self.entrypoint = Some(entrypoint.into());
        self
    }

    /// Override the Cargo manifest path
    #[inline]
    pub fn with_manifest_path(mut self, manifest_path: impl Into<Cow<'static, str>>) -> Self {
        self.manifest_path = Some(manifest_path.into());
        self
    }

    /// Get a [PathBuf] representing the path to the expected Cargo artifact
    pub fn wasm_artifact_path(&self) -> PathBuf {
        self.project_dir
            .join("target")
            .join(self.target.as_ref())
            .join("release")
            .join(self.name.as_ref())
            .with_extension("wasm")
    }
}

/// Configuration for tests which use as input, the artifact produced by an invocation of `rustc`
pub struct RustcTest {
    target_dir: Option<PathBuf>,
    name: Cow<'static, str>,
    target: Cow<'static, str>,
    output_name: Option<Cow<'static, str>>,
    source_code: Cow<'static, str>,
    rustflags: Vec<Cow<'static, str>>,
}
impl RustcTest {
    /// Construct a new `rustc` input with the given name and source code content
    pub fn new(
        name: impl Into<Cow<'static, str>>,
        source_code: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            target_dir: None,
            name: name.into(),
            target: "wasm32-unknown-unknown".into(),
            output_name: None,
            source_code: source_code.into(),
            // Always use spec-compliant C ABI behavior
            rustflags: vec!["-Z".into(), "wasm_c_abi=spec".into()],
        }
    }
}

/// The various types of input artifacts that can be used to drive compiler tests
pub enum CompilerTestInputType {
    /// A project that uses `cargo component build` to produce a Wasm module to use as input
    CargoComponent(CargoTest),
    /// A project that uses `cargo build` to produce a core Wasm module to use as input
    Cargo(CargoTest),
    /// A project that uses `rustc` to produce a core Wasm module to use as input
    Rustc(RustcTest),
}
impl From<CargoTest> for CompilerTestInputType {
    fn from(config: CargoTest) -> Self {
        Self::Cargo(config)
    }
}
impl From<RustcTest> for CompilerTestInputType {
    fn from(config: RustcTest) -> Self {
        Self::Rustc(config)
    }
}

/// [CompilerTestBuilder] is used to obtain a [CompilerTest], and subsequently run that test.
///
/// Testing the compiler involves orchestrating a number of complex components. First, we must
/// obtain the input we wish to feed into `midenc` for the test. Typically, we have some Rust
/// source code, or a Cargo project, and we must compile that first, in order to get the Wasm
/// module/component which will be passed to `midenc`. This first phase requires some configuration,
/// and that configuration affects later phases (such as the name of the artifact produced).
///
/// Secondly, we need to prepare the [midenc_session::Session] object for the compiler. This is
/// where we specify inputs, and various bits of configuration that are important to the test, or
/// which are needed in order to obtain useful diagnostic output. This phase requires us to
/// construct the base configuration here, but make it possible to extend/alter in each specific
/// test.
///
/// Lastly, we must run the test, and in order to do this, we must know where our inputs and outputs
/// are, so that we can fetch files/data/etc. as needed; know the names of things to be called, and
/// more.
pub struct CompilerTestBuilder {
    /// The Wasm translation configuration
    config: WasmTranslationConfig,
    /// The source code used to compile the test
    source: CompilerTestInputType,
    /// The entrypoint function to use when building the IR
    entrypoint: Option<FunctionIdent>,
    /// The extra MASM modules to link to the compiled MASM program
    link_masm_modules: LinkMasmModules,
    /// Extra flags to pass to the midenc driver
    midenc_flags: Vec<Cow<'static, str>>,
    /// Extra RUSTFLAGS to set when compiling Rust code
    rustflags: Vec<Cow<'static, str>>,
    /// The cargo workspace directory of the compiler
    workspace_dir: String,
}
impl CompilerTestBuilder {
    /// Construct a new [CompilerTestBuilder] for the given source type configuration
    pub fn new(source: impl Into<CompilerTestInputType>) -> Self {
        let workspace_dir = get_workspace_dir();
        let mut source = source.into();
        let mut rustflags = match source {
            CompilerTestInputType::Rustc(ref mut config) => core::mem::take(&mut config.rustflags),
            _ => vec![],
        };
        rustflags.extend([
            // Enable bulk-memory features (e.g. native memcpy/memset instructions)
            "-C".into(),
            "target-feature=+bulk-memory".into(),
            // Remap the compiler workspace to `.` so that build outputs do not embed user-
            // specific paths, which would cause expect tests to break
            "--remap-path-prefix".into(),
            format!("{workspace_dir}=../../").into(),
        ]);
        Self {
            config: Default::default(),
            source,
            entrypoint: None,
            link_masm_modules: vec![],
            midenc_flags: vec![],
            rustflags,
            workspace_dir,
        }
    }

    /// Override the default [WasmTranslationConfig] for the test
    pub fn with_wasm_translation_config(&mut self, config: WasmTranslationConfig) -> &mut Self {
        self.config = config;
        self
    }

    /// Specify the entrypoint function to call during the test
    pub fn with_entrypoint(&mut self, entrypoint: FunctionIdent) -> &mut Self {
        self.entrypoint = Some(entrypoint);
        self.midenc_flags.extend([
            "--entrypoint".into(),
            format!("{}::{}", entrypoint.module.as_str(), entrypoint.function.as_str()).into(),
        ]);
        self
    }

    /// Append additional `midenc` compiler flags
    pub fn with_midenc_flags<I, S>(&mut self, flags: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'static, str>>,
    {
        self.midenc_flags.extend(flags.into_iter().map(|flag| flag.into()));
        self
    }

    /// Append additional flags to the value of `RUSTFLAGS` used when invoking `cargo` or `rustc`
    pub fn with_rustflags<I, S>(&mut self, flags: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'static, str>>,
    {
        self.rustflags.extend(flags.into_iter().map(|flag| flag.into()));
        self
    }

    /// Add additional Miden Assembly module sources, to be linked with the program under test.
    pub fn link_with_masm_module(
        &mut self,
        fully_qualified_name: impl AsRef<str>,
        source: impl Into<String>,
    ) -> &mut Self {
        let name = fully_qualified_name.as_ref();
        let path = LibraryPath::new(name)
            .unwrap_or_else(|err| panic!("invalid miden assembly module name '{name}': {err}"));
        self.link_masm_modules.push((path, source.into()));
        self
    }

    /// Consume the builder, invoke any tools required to obtain the inputs for the test, and if
    /// successful, return a [CompilerTest], ready for evaluation.
    pub fn build(self) -> CompilerTest {
        // Set up the command used to compile the test inputs (typically Rust -> Wasm)
        let mut command = match self.source {
            CompilerTestInputType::CargoComponent(_) => {
                let mut cmd = Command::new("cargo");
                cmd.arg("component").arg("build");
                cmd
            }
            CompilerTestInputType::Cargo(_) => {
                let mut cmd = Command::new("cargo");
                cmd.arg("build");
                cmd
            }
            CompilerTestInputType::Rustc(_) => Command::new("rustc"),
        };

        // Extract the directory in which source code is presumed to exist (or will be placed)
        let project_dir = match self.source {
            CompilerTestInputType::CargoComponent(CargoTest {
                ref project_dir, ..
            })
            | CompilerTestInputType::Cargo(CargoTest {
                ref project_dir, ..
            }) => Cow::Borrowed(project_dir.as_path()),
            CompilerTestInputType::Rustc(RustcTest { ref target_dir, .. }) => target_dir
                .as_deref()
                .map(Cow::Borrowed)
                .unwrap_or_else(|| Cow::Owned(std::env::temp_dir())),
        };

        // Cargo-based source types share a lot of configuration in common
        match self.source {
            CompilerTestInputType::CargoComponent(ref config)
            | CompilerTestInputType::Cargo(ref config) => {
                let manifest_path = project_dir.join("Cargo.toml");
                command
                    .arg("--manifest-path")
                    .arg(manifest_path)
                    .arg("--release")
                    .arg("--target")
                    .arg(config.target.as_ref());

                if config.build_std {
                    // compile std as part of crate graph compilation
                    // https://doc.rust-lang.org/cargo/reference/unstable.html#build-std
                    command.arg("-Z").arg("build-std=core,alloc,std,panic_abort");

                    // abort on panic without message formatting (core::fmt uses call_indirect)
                    command.arg("-Z").arg("build-std-features=panic_immediate_abort");
                } else if config.build_alloc {
                    // compile libcore and liballoc as part of crate graph compilation
                    // https://doc.rust-lang.org/cargo/reference/unstable.html#build-std
                    command.arg("-Z").arg("build-std=core,alloc");

                    // abort on panic without message formatting (core::fmt uses call_indirect)
                    command.arg("-Z").arg("build-std-features=panic_immediate_abort");
                }

                // Render Cargo output as JSON
                command.arg("--message-format=json-render-diagnostics");
            }
            _ => (),
        }

        // All test source types support custom RUSTFLAGS
        if !self.rustflags.is_empty() {
            let mut flags = String::with_capacity(
                self.rustflags.iter().map(|flag| flag.len()).sum::<usize>() + self.rustflags.len(),
            );
            for (i, flag) in self.rustflags.iter().enumerate() {
                if i > 0 {
                    flags.push(' ');
                }
                flags.push_str(flag.as_ref());
            }
            command.env("RUSTFLAGS", flags);
        }

        // Pipe output of command to terminal
        command.stdout(Stdio::piped());

        // Build test
        match self.source {
            CompilerTestInputType::CargoComponent(_) => {
                let mut child = command.spawn().unwrap_or_else(|_| {
                    panic!(
                        "Failed to execute command: {}",
                        command
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
                let artifact_name =
                    wasm_comp_path.file_stem().unwrap().to_str().unwrap().to_string();
                let input_file = InputFile::from_path(wasm_comp_path).unwrap();
                let mut inputs = vec![input_file];
                inputs.extend(self.link_masm_modules.into_iter().map(|(path, content)| {
                    let path = path.to_string();
                    InputFile::new(
                        midenc_session::FileType::Masm,
                        InputType::Stdin {
                            name: path.into(),
                            input: content.into_bytes(),
                        },
                    )
                }));

                CompilerTest {
                    config: self.config,
                    session: default_session(inputs, &self.midenc_flags),
                    artifact_name: artifact_name.into(),
                    entrypoint: self.entrypoint,
                    ..Default::default()
                }
            }
            CompilerTestInputType::Cargo(config) => {
                let expected_wasm_artifact_path = config.wasm_artifact_path();
                let skip_rust_compilation =
                    std::env::var("SKIP_RUST").is_ok() && expected_wasm_artifact_path.exists();
                let wasm_artifact_path = if !skip_rust_compilation {
                    let mut child = command.spawn().unwrap_or_else(|_| {
                        panic!(
                            "Failed to execute command: {}",
                            command
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
                    wasm_artifacts.swap_remove(0)
                } else {
                    drop(command);
                    expected_wasm_artifact_path
                };

                let entrypoint = config
                    .entrypoint
                    .as_deref()
                    .map(|entry| FunctionIdent {
                        module: Ident::new(
                            Symbol::intern(config.name.as_ref()),
                            SourceSpan::default(),
                        ),
                        function: Ident::new(Symbol::intern(entry), SourceSpan::default()),
                    })
                    .or(self.entrypoint);
                let input_file = InputFile::from_path(wasm_artifact_path).unwrap();
                let mut inputs = vec![input_file];
                inputs.extend(self.link_masm_modules.into_iter().map(|(path, content)| {
                    let path = path.to_string();
                    InputFile::new(
                        midenc_session::FileType::Masm,
                        InputType::Stdin {
                            name: path.into(),
                            input: content.into_bytes(),
                        },
                    )
                }));
                CompilerTest {
                    config: self.config,
                    session: default_session(inputs, &self.midenc_flags),
                    artifact_name: config.name,
                    entrypoint,
                    ..Default::default()
                }
            }
            CompilerTestInputType::Rustc(config) => {
                // Ensure we have a fresh working directory prepared
                let working_dir = config
                    .target_dir
                    .clone()
                    .unwrap_or_else(|| std::env::temp_dir().join(config.name.as_ref()));
                if working_dir.exists() {
                    fs::remove_dir_all(&working_dir).unwrap();
                }
                fs::create_dir_all(&working_dir).unwrap();

                // Prepare inputs
                let basename = working_dir.join(config.name.as_ref());
                let input_file = basename.with_extension("rs");
                fs::write(&input_file, config.source_code.as_ref()).unwrap();

                // Output is the same name as the input, just with a different extension
                let output_file = basename.with_extension("wasm");

                let output = command
                    .args(["-C", "opt-level=z"]) // optimize for size
                    .arg("--target")
                    .arg(config.target.as_ref())
                    .arg("-o")
                    .arg(&output_file)
                    .arg(&input_file)
                    .output()
                    .expect("rustc invocation failed");
                if !output.status.success() {
                    eprintln!("pwd: {:?}", std::env::current_dir().unwrap());
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                    panic!("Rust to Wasm compilation failed!");
                }
                let input_file = InputFile::from_path(output_file).unwrap();
                let mut inputs = vec![input_file];
                inputs.extend(self.link_masm_modules.into_iter().map(|(path, content)| {
                    let path = path.to_string();
                    InputFile::new(
                        midenc_session::FileType::Masm,
                        InputType::Stdin {
                            name: path.into(),
                            input: content.into_bytes(),
                        },
                    )
                }));
                CompilerTest {
                    config: self.config,
                    session: default_session(inputs, &self.midenc_flags),
                    artifact_name: config.name,
                    entrypoint: self.entrypoint,
                    ..Default::default()
                }
            }
        }
    }
}

/// Convenience builders
impl CompilerTestBuilder {
    /// Compile the Wasm component from a Rust Cargo project using cargo-component
    pub fn rust_source_cargo_component(
        cargo_project_folder: PathBuf,
        config: WasmTranslationConfig,
    ) -> Self {
        let name = cargo_project_folder
            .file_stem()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or("".to_string());
        let mut builder = CompilerTestBuilder::new(CompilerTestInputType::CargoComponent(
            CargoTest::new(name, cargo_project_folder),
        ));
        builder.with_wasm_translation_config(config);
        builder
    }

    /// Set the Rust source code to compile a library Cargo project to Wasm module
    pub fn rust_source_cargo_lib(
        cargo_project_folder: PathBuf,
        artifact_name: &str,
        is_build_std: bool,
        entry_func_name: Option<String>,
        midenc_flags: &[&str],
    ) -> Self {
        let config = CargoTest::new(artifact_name.to_string(), cargo_project_folder)
            .with_build_std(is_build_std);
        let mut builder = CompilerTestBuilder::new(match entry_func_name {
            Some(entry) => config.with_entrypoint(entry),
            None => config,
        });
        builder.with_midenc_flags(midenc_flags.iter().map(|flag| flag.to_string()));
        builder
    }

    /// Set the Rust source code to compile using a Cargo project and binary bundle name
    pub fn rust_source_cargo(
        cargo_project_folder: &str,
        artifact_name: &str,
        entrypoint: &str,
    ) -> Self {
        let temp_dir = std::env::temp_dir();
        let target_dir = temp_dir.join(cargo_project_folder);
        let project_dir = Path::new(&format!("../rust-apps-wasm/{cargo_project_folder}"))
            .canonicalize()
            .unwrap_or_else(|_| {
                panic!("unknown project folder: ../rust-apps-wasm/{cargo_project_folder}")
            });
        let config = CargoTest::new(artifact_name.to_string(), project_dir)
            .with_build_alloc(true)
            .with_target_dir(target_dir)
            .with_target("wasm32-unknown-unknown")
            .with_entrypoint(entrypoint.to_string());
        CompilerTestBuilder::new(config)
    }

    /// Set the Rust source code to compile
    pub fn rust_source_program(rust_source: impl Into<Cow<'static, str>>) -> Self {
        let rust_source = rust_source.into();
        let name = format!("test_rust_{}", hash_string(&rust_source));
        CompilerTestBuilder::new(RustcTest::new(name, rust_source))
    }

    /// Set the Rust source code to compile and add a binary operation test
    pub fn rust_fn_body(rust_source: &str, midenc_flags: &[&str]) -> Self {
        let rust_source = format!(
            r#"
            #![no_std]
            #![no_main]

            #[panic_handler]
            fn my_panic(_info: &core::panic::PanicInfo) -> ! {{
                core::arch::wasm32::unreachable()
            }}

            #[no_mangle]
            pub extern "C" fn entrypoint{}
            "#,
            rust_source
        );
        let name = format!("test_rust_{}", hash_string(&rust_source));
        let module_name = Ident::with_empty_span(Symbol::intern(&name));
        let mut builder = CompilerTestBuilder::new(RustcTest::new(name, rust_source));
        builder
            .with_midenc_flags(midenc_flags.iter().map(|flag| flag.to_string()))
            .with_entrypoint(FunctionIdent {
                module: module_name,
                function: Ident::with_empty_span(Symbol::intern("entrypoint")),
            });
        builder
    }

    /// Set the Rust source code to compile with `miden-stdlib-sys` (stdlib + intrinsics)
    pub fn rust_fn_body_with_stdlib_sys(
        name: &str,
        rust_source: &str,
        is_build_std: bool,
        midenc_flags: &[&str],
    ) -> Self {
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
                debug = true
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
                    core::arch::wasm32::unreachable()
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
        Self::rust_source_cargo_lib(
            proj.root(),
            name,
            is_build_std,
            Some("entrypoint".to_string()),
            midenc_flags,
        )
    }

    /// Set the Rust source code to compile with `miden-stdlib-sys` (stdlib + intrinsics)
    pub fn rust_fn_body_with_sdk(
        name: &str,
        rust_source: &str,
        is_build_std: bool,
        midenc_flags: &[&str],
    ) -> Self {
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
                edition = "2021"
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
                debug = true
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
                    core::arch::wasm32::unreachable()
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

        Self::rust_source_cargo_lib(
            proj.root(),
            name,
            is_build_std,
            Some("entrypoint".to_string()),
            midenc_flags,
        )
    }
}

/// Compile to different stages (e.g. Wasm, IR, MASM) and compare the results against expected
/// output
pub struct CompilerTest {
    /// The Wasm translation configuration
    pub config: WasmTranslationConfig,
    /// The compiler session
    pub session: Arc<Session>,
    /// The artifact name from which this test is derived
    artifact_name: Cow<'static, str>,
    /// The entrypoint function to use when building the IR
    entrypoint: Option<FunctionIdent>,
    /// The compiled IR
    hir: Option<HirArtifact>,
    /// The MASM source code
    masm_src: Option<String>,
    /// The compiled IR MASM program
    ir_masm_program: Option<Result<Arc<midenc_codegen_masm::Program>, String>>,
    /// The compiled VM program
    vm_masm_program: Option<Result<Arc<miden_core::Program>, String>>,
}

impl Default for CompilerTest {
    fn default() -> Self {
        Self {
            config: WasmTranslationConfig::default(),
            session: dummy_session(&[]),
            artifact_name: Cow::Borrowed("unknown"),
            entrypoint: None,
            hir: None,
            masm_src: None,
            ir_masm_program: None,
            vm_masm_program: None,
        }
    }
}

impl CompilerTest {
    /// Return the name of the artifact this test is derived from
    pub fn artifact_name(&self) -> &str {
        self.artifact_name.as_ref()
    }

    /// Compile the Wasm component from a Rust Cargo project using cargo-component
    pub fn rust_source_cargo_component(
        cargo_project_folder: PathBuf,
        config: WasmTranslationConfig,
    ) -> Self {
        CompilerTestBuilder::rust_source_cargo_component(cargo_project_folder, config).build()
    }

    /// Set the Rust source code to compile a library Cargo project to Wasm module
    pub fn rust_source_cargo_lib(
        cargo_project_folder: PathBuf,
        artifact_name: &str,
        is_build_std: bool,
        entry_func_name: Option<String>,
        midenc_flags: &[&str],
    ) -> Self {
        CompilerTestBuilder::rust_source_cargo_lib(
            cargo_project_folder,
            artifact_name,
            is_build_std,
            entry_func_name,
            midenc_flags,
        )
        .build()
    }

    /// Set the Rust source code to compile using a Cargo project and binary bundle name
    pub fn rust_source_cargo(
        cargo_project_folder: &str,
        artifact_name: &str,
        entrypoint: &str,
    ) -> Self {
        CompilerTestBuilder::rust_source_cargo(cargo_project_folder, artifact_name, entrypoint)
            .build()
    }

    /// Set the Rust source code to compile
    pub fn rust_source_program(rust_source: impl Into<Cow<'static, str>>) -> Self {
        CompilerTestBuilder::rust_source_program(rust_source).build()
    }

    /// Set the Rust source code to compile and add a binary operation test
    pub fn rust_fn_body(rust_source: &str, midenc_flags: &[&str]) -> Self {
        CompilerTestBuilder::rust_fn_body(rust_source, midenc_flags).build()
    }

    /// Set the Rust source code to compile with `miden-stdlib-sys` (stdlib + intrinsics)
    pub fn rust_fn_body_with_stdlib_sys(
        name: &str,
        rust_source: &str,
        is_build_std: bool,
        midenc_flags: &[&str],
    ) -> Self {
        CompilerTestBuilder::rust_fn_body_with_stdlib_sys(
            name,
            rust_source,
            is_build_std,
            midenc_flags,
        )
        .build()
    }

    /// Set the Rust source code to compile with `miden-stdlib-sys` (stdlib + intrinsics)
    pub fn rust_fn_body_with_sdk(
        name: &str,
        rust_source: &str,
        is_build_std: bool,
        midenc_flags: &[&str],
    ) -> Self {
        CompilerTestBuilder::rust_fn_body_with_sdk(name, rust_source, is_build_std, midenc_flags)
            .build()
    }

    /// Compare the compiled Wasm against the expected output
    pub fn expect_wasm(&self, expected_wat_file: expect_test::ExpectFile) {
        let wasm_bytes = self.wasm_bytes();
        let wat = demangle(&wasm_to_wat(&wasm_bytes));
        expected_wat_file.assert_eq(&wat);
    }

    fn wasm_to_ir(&self) -> HirArtifact {
        let ir_component = translate(&self.wasm_bytes(), &self.config, &self.session)
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
                // Program does not implement pretty printer yet, use the module containing the
                // entrypoint function, or the first module found if no entrypoint is set
                let ir_module = hir_program
                    .entrypoint()
                    .map(|entry| {
                        hir_program
                            .modules()
                            .find(&entry.module)
                            .get()
                            .expect("missing entrypoint module")
                    })
                    .unwrap_or_else(|| {
                        hir_program.modules().iter().next().expect("expected at least one module")
                    })
                    .to_string();
                let ir_module = demangle(&ir_module);
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

    /// Get the compiled IR MASM program
    pub fn ir_masm_program(&mut self) -> Arc<midenc_codegen_masm::Program> {
        if self.ir_masm_program.is_none() {
            self.compile_wasm_to_masm_program();
        }
        match self.ir_masm_program.as_ref().unwrap().as_ref() {
            Ok(prog) => prog.clone(),
            Err(msg) => panic!("{msg}"),
        }
    }

    /// Get the compiled MASM as [`miden_core::Program`]
    pub fn vm_masm_program(&mut self) -> Arc<miden_core::Program> {
        if self.vm_masm_program.is_none() {
            self.compile_wasm_to_masm_program();
        }
        match self.vm_masm_program.as_ref().unwrap().as_ref() {
            Ok(prog) => prog.clone(),
            Err(msg) => panic!("{msg}"),
        }
    }

    /// Get the MASM source code
    pub fn masm_src(&mut self) -> String {
        if self.masm_src.is_none() {
            self.compile_wasm_to_masm_program();
        }
        self.masm_src.clone().unwrap()
    }

    /// The compiled Wasm component/module
    fn wasm_bytes(&self) -> Vec<u8> {
        match &self.session.inputs[0].file {
            InputType::Real(file_path) => fs::read(file_path)
                .unwrap_or_else(|_| panic!("Failed to read Wasm file: {}", file_path.display())),
            InputType::Stdin { name: _, input } => input.clone(),
        }
    }

    pub(crate) fn compile_wasm_to_masm_program(&mut self) {
        let output = midenc_compile::compile_to_memory(self.session.clone())
            .unwrap()
            .expect("no codegen outputs produced, was linking disabled?");
        let midenc_codegen_masm::MasmArtifact::Executable(program) = output else {
            panic!("expected executable output, got library");
        };
        let src = program.to_string();
        let vm_prog = program.assemble(&self.session).map_err(format_report);
        self.masm_src = Some(src);
        self.ir_masm_program = Some(Ok(program.into()));
        self.vm_masm_program = Some(vm_prog);
    }
}

fn format_report(report: miden_assembly::diagnostics::Report) -> String {
    use miden_assembly::diagnostics::reporting::PrintDiagnostic;

    PrintDiagnostic::new(report).to_string()
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

fn dummy_session(flags: &[&str]) -> Arc<Session> {
    let dummy = InputFile::from_path(PathBuf::from("dummy.wasm")).unwrap();
    default_session([dummy], flags)
}

/// Create a default session for testing
pub fn default_session<S, I>(inputs: I, extra_flags: &[S]) -> Arc<Session>
where
    I: IntoIterator<Item = InputFile>,
    S: AsRef<str>,
{
    use midenc_hir::diagnostics::{
        reporting::{self, ReportHandlerOpts},
        SingleThreadedSourceManager,
    };

    let result = reporting::set_hook(Box::new(|_| Box::new(ReportHandlerOpts::new().build())));
    if result.is_ok() {
        reporting::set_panic_hook();
    }

    let mut inputs = inputs.into_iter();
    let input_file = inputs.next().expect("must provide at least one input file");
    let mut flags = vec!["--lib", "--debug=full"];
    flags.extend(extra_flags.iter().map(|flag| flag.as_ref()));
    let source_manager = Arc::new(SingleThreadedSourceManager::default());
    let mut options = midenc_compile::CompilerOptions::parse_options(&flags);
    options.output_types.insert(OutputType::Masm, None);
    let mut session = Session::new(input_file, None, None, None, options, None, source_manager);
    for extra_input in inputs {
        session.inputs.push(extra_input);
    }
    Arc::new(session)
}

fn hash_string(inputs: &str) -> String {
    let hash = <sha2::Sha256 as sha2::Digest>::digest(inputs.as_bytes());
    format!("{:x}", hash)
}
