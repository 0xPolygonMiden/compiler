#![feature(debug_closure_helpers)]
extern crate alloc;

pub mod diagnostics;
mod duration;
mod emit;
mod flags;
mod inputs;
mod libs;
mod options;
mod outputs;
mod statistics;

use std::{
    fmt,
    path::{Path, PathBuf},
    sync::Arc,
};

/// The version associated with the current compiler toolchain
pub const MIDENC_BUILD_VERSION: &str = env!("MIDENC_BUILD_VERSION");

/// The git revision associated with the current compiler toolchain
pub const MIDENC_BUILD_REV: &str = env!("MIDENC_BUILD_REV");

use clap::ValueEnum;
use midenc_hir_symbol::Symbol;

pub use self::{
    diagnostics::{DiagnosticsHandler, Emitter, SourceManager},
    duration::HumanDuration,
    emit::Emit,
    flags::{CompileFlag, FlagAction},
    inputs::{FileType, InputFile, InputType, InvalidInputError},
    libs::{LibraryKind, LinkLibrary},
    options::*,
    outputs::{OutputFile, OutputFiles, OutputType, OutputTypeSpec, OutputTypes},
    statistics::Statistics,
};

/// The type of project being compiled
#[derive(Debug, Copy, Clone, Default)]
pub enum ProjectType {
    /// Compile a Miden program that can be run on the Miden VM
    #[default]
    Program,
    /// Compile a Miden library which can be linked into a program
    Library,
}
impl ProjectType {
    pub fn default_for_target(target: TargetEnv) -> Self {
        match target {
            // We default to compiling a program unless we find later
            // that we do not have an entrypoint.
            TargetEnv::Base | TargetEnv::Rollup => Self::Program,
            // The emulator can run either programs or individual library functions,
            // so we compile as a library and delegate the choice of how to run it
            // to the emulator
            TargetEnv::Emu => Self::Library,
        }
    }
}

/// This struct provides access to all of the metadata and configuration
/// needed during a single compilation session.
pub struct Session {
    /// The name of this session
    pub name: String,
    /// Configuration for the current compiler session
    pub options: Options,
    /// The current source manager
    pub source_manager: Arc<dyn SourceManager>,
    /// The current diagnostics handler
    pub diagnostics: Arc<DiagnosticsHandler>,
    /// The inputs being compiled
    pub inputs: Vec<InputFile>,
    /// The outputs to be produced by the compiler during compilation
    pub output_files: OutputFiles,
    /// Statistics gathered from the current compiler session
    pub statistics: Statistics,
}

impl fmt::Debug for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inputs = self.inputs.iter().map(|input| input.file_name()).collect::<Vec<_>>();
        f.debug_struct("Session")
            .field("name", &self.name)
            .field("options", &self.options)
            .field("inputs", &inputs)
            .field("output_files", &self.output_files)
            .field("statistics", &self.statistics)
            .finish_non_exhaustive()
    }
}

impl Session {
    pub fn new<I>(
        inputs: I,
        output_dir: Option<PathBuf>,
        output_file: Option<OutputFile>,
        target_dir: PathBuf,
        options: Options,
        emitter: Option<Arc<dyn Emitter>>,
        source_manager: Arc<dyn SourceManager>,
    ) -> Self
    where
        I: IntoIterator<Item = InputFile>,
    {
        let inputs = inputs.into_iter().collect::<Vec<_>>();

        Self::make(inputs, output_dir, output_file, target_dir, options, emitter, source_manager)
    }

    fn make(
        inputs: Vec<InputFile>,
        output_dir: Option<PathBuf>,
        output_file: Option<OutputFile>,
        target_dir: PathBuf,
        options: Options,
        emitter: Option<Arc<dyn Emitter>>,
        source_manager: Arc<dyn SourceManager>,
    ) -> Self {
        let diagnostics = Arc::new(DiagnosticsHandler::new(
            options.diagnostics,
            source_manager.clone(),
            emitter.unwrap_or_else(|| options.default_emitter()),
        ));

        let output_dir = output_dir
            .as_deref()
            .or_else(|| output_file.as_ref().and_then(|of| of.parent()))
            .map(|path| path.to_path_buf());

        let name = options
            .name
            .clone()
            .or_else(|| {
                output_file
                    .as_ref()
                    .and_then(|of| of.filestem().map(|stem| stem.to_string_lossy().into_owned()))
            })
            .unwrap_or_else(|| match inputs.first() {
                Some(InputFile {
                    file: InputType::Real(ref path),
                    ..
                }) => path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .or_else(|| path.extension().and_then(|stem| stem.to_str()))
                    .unwrap_or_else(|| {
                        panic!(
                            "invalid input path: '{}' has no file stem or extension",
                            path.display()
                        )
                    })
                    .to_string(),
                Some(
                    input @ InputFile {
                        file: InputType::Stdin { ref name, .. },
                        ..
                    },
                ) => {
                    let name = name.as_str();
                    if matches!(name, Some("empty") | Some("stdin")) {
                        options
                            .current_dir
                            .file_stem()
                            .and_then(|stem| stem.to_str())
                            .unwrap_or(name.unwrap())
                            .to_string()
                    } else {
                        input.filestem().to_owned()
                    }
                }
                None => "out".to_owned(),
            });

        let output_files = OutputFiles::new(
            name.clone(),
            options.current_dir.clone(),
            output_dir.unwrap_or_else(|| options.current_dir.clone()),
            output_file,
            target_dir,
            options.output_types.clone(),
        );

        Self {
            name,
            options,
            source_manager,
            diagnostics,
            inputs,
            output_files,
            statistics: Default::default(),
        }
    }

    pub fn with_project_type(mut self, ty: ProjectType) -> Self {
        self.options.project_type = ty;
        self
    }

    #[doc(hidden)]
    pub fn with_arg_matches(mut self, matches: clap::ArgMatches) -> Self {
        self.options.set_arg_matches(matches);
        self
    }

    #[doc(hidden)]
    pub fn with_output_type(mut self, ty: OutputType, path: Option<OutputFile>) -> Self {
        self.output_files.outputs.insert(ty, path.clone());
        self.options.output_types.insert(ty, path.clone());
        self
    }

    /// Get the value of a custom flag with action `FlagAction::SetTrue` or `FlagAction::SetFalse`
    #[inline]
    pub fn get_flag(&self, name: &str) -> bool {
        self.options.get_flag(name)
    }

    /// Get the count of a specific custom flag with action `FlagAction::Count`
    #[inline]
    pub fn get_flag_count(&self, name: &str) -> usize {
        self.options.get_flag_count(name)
    }

    /// Get the value of a specific custom flag
    #[inline]
    pub fn get_flag_value<T>(&self, name: &str) -> Option<&T>
    where
        T: core::any::Any + Clone + Send + Sync + 'static,
    {
        self.options.get_flag_value(name)
    }

    /// Iterate over values of a specific custom flag
    #[inline]
    pub fn get_flag_values<T>(&self, name: &str) -> Option<clap::parser::ValuesRef<'_, T>>
    where
        T: core::any::Any + Clone + Send + Sync + 'static,
    {
        self.options.get_flag_values(name)
    }

    /// Get the remaining [clap::ArgMatches] left after parsing the base session configuration
    #[inline]
    pub fn matches(&self) -> &clap::ArgMatches {
        self.options.matches()
    }

    /// The name of this session (used as the name of the project, output file, etc.)
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn out_file(&self) -> OutputFile {
        let out_file = self.output_files.output_file(OutputType::Mast, None);

        if let OutputFile::Real(ref path) = out_file {
            self.check_file_is_writeable(path);
        }

        out_file
    }

    fn check_file_is_writeable(&self, file: &Path) {
        if let Ok(m) = file.metadata() {
            if m.permissions().readonly() {
                self.diagnostics
                    .fatal(format!("file is not writeable: {}", file.display()))
                    .raise();
            }
        }
    }

    pub fn parse_only(&self) -> bool {
        self.options.output_types.parse_only()
    }

    pub fn should_codegen(&self) -> bool {
        self.options.output_types.should_codegen()
    }

    pub fn should_link(&self) -> bool {
        self.options.output_types.should_link()
    }

    pub fn should_emit(&self, ty: OutputType) -> bool {
        self.options.output_types.contains_key(&ty)
    }

    /// Get the path to emit the given [OutputType] to
    pub fn emit_to(&self, ty: OutputType, name: Option<Symbol>) -> Option<PathBuf> {
        if self.should_emit(ty) {
            match self.output_files.output_file(ty, name.map(|n| n.as_str())) {
                OutputFile::Real(path) => Some(path),
                OutputFile::Stdout => None,
            }
        } else {
            None
        }
    }

    /// Emit an item to stdout/file system depending on the current configuration
    pub fn emit<E: Emit>(&self, item: &E) -> std::io::Result<()> {
        let output_type = item.output_type();
        if self.should_emit(output_type) {
            let name = item.name().map(|n| n.as_str());
            match self.output_files.output_file(output_type, name) {
                OutputFile::Real(path) => {
                    item.write_to_file(&path)?;
                }
                OutputFile::Stdout => {
                    item.write_to_stdout()?;
                }
            }
        }

        Ok(())
    }
}

/// This enum describes the different target environments targetable by the compiler
#[derive(Debug, Copy, Clone, Default, ValueEnum)]
pub enum TargetEnv {
    /// The emulator environment, which has a more restrictive instruction set
    Emu,
    /// The default Miden VM environment
    #[default]
    Base,
    /// The Miden Rollup environment, using the Rollup kernel
    Rollup,
}
impl fmt::Display for TargetEnv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Emu => f.write_str("emu"),
            Self::Base => f.write_str("base"),
            Self::Rollup => f.write_str("rollup"),
        }
    }
}
