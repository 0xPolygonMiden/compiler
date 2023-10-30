mod input_types;
mod output_types;

pub use self::input_types::{FileType, Input, InvalidFileTypeError};
pub use self::output_types::{OutputType, OutputTypes};

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use miden_diagnostics::term::termcolor::ColorChoice;
use miden_diagnostics::{DiagnosticsConfig, Emitter, FileName, Verbosity};
use rustc_hash::FxHashSet;

use crate::Emit;

#[derive(Debug, thiserror::Error)]
pub enum InvalidOptionError {
    /// An input specified to the compiler is not valid
    #[error(transparent)]
    InvalidInput(#[from] std::io::Error),
}

pub struct CompileFlag {
    pub name: &'static str,
    pub short: Option<char>,
    pub long: Option<&'static str>,
    pub help: Option<&'static str>,
    pub env: Option<&'static str>,
    pub action: FlagAction,
    pub default_missing_value: Option<&'static str>,
    pub default_value: Option<&'static str>,
}
impl CompileFlag {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            short: None,
            long: None,
            help: None,
            env: None,
            action: FlagAction::Set,
            default_missing_value: None,
            default_value: None,
        }
    }

    pub const fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }

    pub const fn long(mut self, long: &'static str) -> Self {
        self.long = Some(long);
        self
    }

    pub const fn action(mut self, action: FlagAction) -> Self {
        self.action = action;
        self
    }

    pub const fn help(mut self, help: &'static str) -> Self {
        self.help = Some(help);
        self
    }

    pub const fn env(mut self, env: &'static str) -> Self {
        self.env = Some(env);
        self
    }

    pub const fn default_value(mut self, value: &'static str) -> Self {
        self.default_value = Some(value);
        self
    }

    pub const fn default_missing_value(mut self, value: &'static str) -> Self {
        self.default_missing_value = Some(value);
        self
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FlagAction {
    Set,
    Append,
    SetTrue,
    SetFalse,
    Count,
}
impl From<FlagAction> for clap::ArgAction {
    fn from(action: FlagAction) -> Self {
        match action {
            FlagAction::Set => Self::Set,
            FlagAction::Append => Self::Append,
            FlagAction::SetTrue => Self::SetTrue,
            FlagAction::SetFalse => Self::SetFalse,
            FlagAction::Count => Self::Count,
        }
    }
}

inventory::collect!(CompileFlag);

/// This struct contains all of the configuration options for the compiler
#[derive(Clone, Debug)]
pub struct Options {
    /// The name of the program being compiled
    pub name: String,
    /// The type of outputs to emit
    pub output_types: OutputTypes,
    /// Whether, and how, to color terminal output
    pub color: ColorChoice,
    /// The current diagnostics configuration
    pub diagnostics: DiagnosticsConfig,
    /// True if we should emit artifacts to stdout
    pub stdout: bool,
    /// True if we're compiling to evaluate the artifacts using the emulator or VM
    pub eval: bool,
    /// The current working directory of the compiler
    pub current_dir: PathBuf,
    /// The directory in which intermediate compiler artifacts will be emitted
    pub target_dir: PathBuf,
    /// The directory in which to place compiler outputs
    pub output_dir: PathBuf,
    /// The file where compiler output will be written
    pub output_file: Option<PathBuf>,
    /// This is the set of inputs
    pub inputs: Vec<Input>,
    /// The passes to run against IR modules
    pub passes: Option<Vec<String>>,
    /// Print IR to stdout after each pass
    pub print_ir_after_all: bool,
    /// Print IR to stdout each time the named pass is applied
    pub print_ir_after_pass: Option<String>,
}
impl Options {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cwd: PathBuf,
        target_dir: Option<PathBuf>,
        output_dir: Option<PathBuf>,
        output_file: Option<PathBuf>,
        output_types: OutputTypes,
        input_paths: Vec<PathBuf>,
        color: ColorChoice,
        verbosity: Verbosity,
    ) -> Result<Box<Self>, InvalidOptionError> {
        use std::io::Read;

        const STDIN_DASH: &str = "-";
        const STDIN_FLAG: &str = "--stdin";

        fn is_stdin(path: &Path) -> bool {
            let dash: &OsStr = STDIN_DASH.as_ref();
            let flag: &OsStr = STDIN_FLAG.as_ref();
            path == dash || path == flag
        }

        // If the output directory is not set, use the current working directory
        let output_dir_is_set = output_dir.is_some();
        let output_dir = output_dir.unwrap_or(cwd.clone());

        // If the output file was set and the output directory was set, we must
        // make the output file relative to the output directory. However, if
        // the output_file is an absolute path, then we treat that as overriding
        // the output directory.
        //
        // If the output directory was not specified, we use the current working
        // directory as the default, using the same rules as above.
        let output_file = if output_dir_is_set {
            output_file.map(|of| {
                if of.is_absolute() {
                    of
                } else {
                    output_dir.join(of)
                }
            })
        } else {
            output_file.map(|of| if of.is_absolute() { of } else { cwd.join(of) })
        };

        // Detect the name of the program being compiled, using the following rules:
        //
        // 1. If an output file was specified, use the file stem of that path
        // 2. If there is a single file/directory path input, use the base name of that path
        // 3. For all other cases, fall back to the base name of the current working directory
        // 3. If the current working directory is unavailable, use "noname"
        let num_inputs = input_paths.len();
        let name = match output_file.as_ref() {
            Some(of) => of
                .file_stem()
                .expect("invalid output file path")
                .to_string_lossy()
                .into_owned(),
            None => match input_paths.first() {
                Some(path) if num_inputs == 1 && !is_stdin(path) => path
                    .file_stem()
                    .expect("invalid input file path")
                    .to_string_lossy()
                    .into_owned(),
                _ => cwd
                    .file_stem()
                    .map(|os| os.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "noname".to_string()),
            },
        };

        let mut seen = FxHashSet::<PathBuf>::default();
        let mut inputs = Vec::with_capacity(input_paths.len());
        for path in input_paths.into_iter() {
            if !seen.insert(path.clone()) {
                continue;
            }

            if is_stdin(&path) {
                let mut content = Vec::with_capacity(1024);
                std::io::stdin().read_to_end(&mut content)?;
                let name = path.to_string_lossy().into_owned();
                let name = FileName::Virtual(name.into());
                inputs.push(Input::Stdin(name, content));
                continue;
            }

            if !path.exists() {
                return Err(InvalidOptionError::InvalidInput(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("{} does not exist", path.display()),
                )));
            }

            if path.is_file() {
                inputs.push(Input::File(path.into()));
            } else {
                return Err(InvalidOptionError::InvalidInput(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("{} is not a file", path.display()),
                )));
            }
        }

        let target_dir = target_dir.unwrap_or_else(|| cwd.join("target/miden"));
        Ok(Box::new(Self {
            name,
            output_types,
            color,
            diagnostics: DiagnosticsConfig {
                verbosity,
                warnings_as_errors: false,
                no_warn: false,
                display: Default::default(),
            },
            stdout: false,
            eval: false,
            current_dir: cwd,
            target_dir,
            output_dir,
            output_file,
            inputs,
            passes: None,
            print_ir_after_all: false,
            print_ir_after_pass: None,
        }))
    }

    pub fn enable_warnings(&mut self) {
        self.diagnostics.warnings_as_errors = false;
        self.diagnostics.no_warn = false;
    }

    pub fn disable_warnings(&mut self) {
        self.diagnostics.warnings_as_errors = false;
        self.diagnostics.no_warn = true;
    }

    pub fn warnings_as_errors(&mut self, value: bool) {
        self.diagnostics.warnings_as_errors = value;
    }

    /// If true, we should emit to stdout in addition to files on disk
    #[inline]
    pub fn should_emit_to_stdout(&self) -> bool {
        self.stdout
    }

    /// Get a new [miden_diagnostics::Emitter] based on the current options.
    pub fn default_emitter(&self) -> Arc<dyn Emitter> {
        use miden_diagnostics::{DefaultEmitter, NullEmitter};

        match self.diagnostics.verbosity {
            Verbosity::Silent => Arc::new(NullEmitter::new(self.color)),
            _ => Arc::new(DefaultEmitter::new(self.color)),
        }
    }

    /// Emit an item to stdout/file system depending on the current configuration
    pub fn emit<E: Emit>(&self, item: &E) -> std::io::Result<()> {
        // Do not emit artifacts when evaluating in-memory
        if self.eval {
            return Ok(());
        }

        let output_type = item.output_type();
        if self.output_types.contains(output_type.into()) {
            if self.should_emit_to_stdout() {
                item.write_to_stdout()?;
            }
            if let Some(of) = self.output_file.as_ref() {
                let of = of.with_extension(output_type.unwrap_extension());
                item.write_to_file(&of)?;
            } else {
                let of = self
                    .output_dir
                    .join(&self.name)
                    .with_extension(output_type.unwrap_extension());
                item.write_to_file(&of)?;
            }
        }

        Ok(())
    }
}
