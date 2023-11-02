mod duration;
mod emit;
mod flags;
mod inputs;
mod options;
mod outputs;
mod statistics;

pub use self::duration::HumanDuration;
pub use self::emit::Emit;
pub use self::flags::{CompileFlag, FlagAction};
pub use self::inputs::{FileType, InputFile, InputType, InvalidInputError};
pub use self::options::*;
pub use self::outputs::{OutputFile, OutputFiles, OutputType, OutputTypeSpec, OutputTypes};
pub use self::statistics::Statistics;

use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use clap::ValueEnum;
use miden_diagnostics::{CodeMap, DiagnosticsHandler, Emitter};
use miden_hir_symbol::Symbol;

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
    /// The type of project we're compiling this session
    pub project_type: ProjectType,
    /// The current target environment for this session
    pub target: TargetEnv,
    /// Configuration for the current compiler session
    pub options: Options,
    /// The current source map
    pub codemap: Arc<CodeMap>,
    /// The current diagnostics handler
    pub diagnostics: Arc<DiagnosticsHandler>,
    /// The location of all libraries shipped with the compiler
    pub sysroot: PathBuf,
    /// The input being compiled
    pub input: InputFile,
    /// The outputs to be produced by the compiler during compilation
    pub output_files: OutputFiles,
    /// Statistics gathered from the current compiler session
    pub statistics: Statistics,
    /// We store any leftover argument matches in the session for use
    /// by any downstream crates that register custom flags
    arg_matches: clap::ArgMatches,
}
impl Session {
    pub fn new(
        target: TargetEnv,
        input: InputFile,
        output_dir: Option<PathBuf>,
        output_file: Option<OutputFile>,
        tmp_dir: Option<PathBuf>,
        options: Options,
        emitter: Option<Arc<dyn Emitter>>,
    ) -> Self {
        // TODO: Make sure we pin this down when we need to ship stuff with compiler
        let sysroot = match &options.sysroot {
            Some(sysroot) => sysroot.clone(),
            None => std::env::current_dir().unwrap(),
        };
        let codemap = Arc::new(CodeMap::new());

        let diagnostics = Arc::new(DiagnosticsHandler::new(
            options.diagnostics.clone(),
            codemap.clone(),
            emitter.unwrap_or_else(|| options.default_emitter()),
        ));

        let output_files = match output_file {
            None => {
                let output_dir = output_dir.unwrap_or_default();
                let stem = options
                    .name
                    .clone()
                    .unwrap_or_else(|| input.filestem().to_owned());

                OutputFiles::new(
                    stem,
                    output_dir,
                    None,
                    tmp_dir,
                    options.output_types.clone(),
                )
            }
            Some(out_file) => OutputFiles::new(
                out_file
                    .filestem()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap()
                    .to_string(),
                output_dir.unwrap_or_default(),
                Some(out_file),
                tmp_dir,
                options.output_types.clone(),
            ),
        };

        let project_type = ProjectType::default_for_target(target);
        Self {
            project_type,
            target,
            options,
            codemap,
            diagnostics,
            sysroot,
            input,
            output_files,
            statistics: Default::default(),
            arg_matches: Default::default(),
        }
    }

    pub fn with_project_type(mut self, ty: ProjectType) -> Self {
        self.project_type = ty;
        self
    }

    #[doc(hidden)]
    pub fn with_arg_matches(mut self, matches: clap::ArgMatches) -> Self {
        self.arg_matches = matches;
        self
    }

    /// Get the value of a custom flag with action `FlagAction::SetTrue` or `FlagAction::SetFalse`
    pub fn get_flag(&self, name: &str) -> bool {
        self.arg_matches.get_flag(name)
    }

    /// Get the count of a specific custom flag with action `FlagAction::Count`
    pub fn get_flag_count(&self, name: &str) -> usize {
        self.arg_matches.get_count(name) as usize
    }

    /// Get the value of a specific custom flag
    pub fn get_flag_value<T>(&self, name: &str) -> Option<&T>
    where
        T: core::any::Any + Clone + Send + Sync + 'static,
    {
        self.arg_matches.get_one(name)
    }

    /// Iterate over values of a specific custom flag
    pub fn get_flag_values<T>(&self, name: &str) -> Option<clap::parser::ValuesRef<'_, T>>
    where
        T: core::any::Any + Clone + Send + Sync + 'static,
    {
        self.arg_matches.get_many(name)
    }

    /// Get the remaining [clap::ArgMatches] left after parsing the base session configuration
    pub fn matches(&self) -> &clap::ArgMatches {
        &self.arg_matches
    }

    pub fn out_filename(&self, outputs: &OutputFiles, progname: Symbol) -> OutputFile {
        let default_filename = self.filename_for_input(outputs, progname);
        let out_filename = outputs
            .outputs
            .get(&OutputType::Masl)
            .and_then(|s| s.to_owned())
            .or_else(|| outputs.out_file.clone())
            .unwrap_or(default_filename);

        if let OutputFile::Real(ref path) = out_filename {
            self.check_file_is_writeable(path);
        }

        out_filename
    }

    pub fn filename_for_input(&self, outputs: &OutputFiles, progname: Symbol) -> OutputFile {
        match self.project_type {
            ProjectType::Program => {
                let out_filename = outputs.path(OutputType::Masl);
                if let OutputFile::Real(ref path) = out_filename {
                    OutputFile::Real(path.with_extension(OutputType::Masl.extension()))
                } else {
                    out_filename
                }
            }
            ProjectType::Library => OutputFile::Real(
                outputs
                    .out_dir
                    .join(format!("{progname}.{}", OutputType::Masl.extension())),
            ),
        }
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

    /// Emit an item to stdout/file system depending on the current configuration
    pub fn emit<E: Emit>(&self, item: &E) -> std::io::Result<()> {
        let output_type = item.output_type();
        if self.should_emit(output_type) {
            match self.output_files.path(output_type) {
                OutputFile::Real(path) => {
                    let path = if let Some(name) = item.name() {
                        path.with_file_name(name.as_str())
                            .with_extension(output_type.extension())
                    } else {
                        path
                    };
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
