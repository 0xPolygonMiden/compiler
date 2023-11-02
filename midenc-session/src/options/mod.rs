use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use clap::ValueEnum;
use miden_diagnostics::term::termcolor::ColorChoice;
use miden_diagnostics::{DiagnosticsConfig, Emitter, Verbosity};

use crate::OutputTypes;

/// This struct contains all of the configuration options for the compiler
#[derive(Debug)]
pub struct Options {
    /// The name of the program being compiled
    pub name: Option<String>,
    /// The optimization level for the current program
    pub optimize: OptLevel,
    /// The type of outputs to emit
    pub output_types: OutputTypes,
    /// The paths in which to search for Miden Assembly libraries to link against
    pub search_paths: Vec<PathBuf>,
    /// The location of the libraries which are shipped with the compiler
    pub sysroot: Option<PathBuf>,
    /// Whether, and how, to color terminal output
    pub color: ColorChoice,
    /// The current diagnostics configuration
    pub diagnostics: DiagnosticsConfig,
    /// The current working directory of the compiler
    pub current_dir: PathBuf,
    /// Print IR to stdout after each pass
    pub print_ir_after_all: bool,
    /// Print IR to stdout each time the named pass is applied
    pub print_ir_after_pass: Option<String>,
}
impl Default for Options {
    fn default() -> Self {
        let current_dir = std::env::current_dir().expect("could not get working directory");
        Self::new(current_dir)
    }
}
impl Options {
    pub fn new(current_dir: PathBuf) -> Self {
        Self {
            name: None,
            optimize: OptLevel::None,
            output_types: Default::default(),
            search_paths: vec![],
            sysroot: None,
            color: Default::default(),
            diagnostics: Default::default(),
            current_dir,
            print_ir_after_all: false,
            print_ir_after_pass: None,
        }
    }

    pub fn with_color(mut self, color: ColorChoice) -> Self {
        self.color = color;
        self
    }

    pub fn with_verbosity(mut self, verbosity: Verbosity) -> Self {
        self.diagnostics.verbosity = verbosity;
        self
    }

    pub fn with_warnings(mut self, warnings: Warnings) -> Self {
        match warnings {
            Warnings::None => {
                self.diagnostics.warnings_as_errors = false;
                self.diagnostics.no_warn = true;
            }
            Warnings::All => {
                self.diagnostics.warnings_as_errors = false;
                self.diagnostics.no_warn = false;
            }
            Warnings::Error => {
                self.diagnostics.warnings_as_errors = true;
                self.diagnostics.no_warn = false;
            }
        }
        self
    }

    pub fn with_output_types(mut self, output_types: OutputTypes) -> Self {
        self.output_types = output_types;
        self
    }

    /// Get a new [miden_diagnostics::Emitter] based on the current options.
    pub fn default_emitter(&self) -> Arc<dyn Emitter> {
        use miden_diagnostics::{DefaultEmitter, NullEmitter};

        match self.diagnostics.verbosity {
            Verbosity::Silent => Arc::new(NullEmitter::new(self.color)),
            _ => Arc::new(DefaultEmitter::new(self.color)),
        }
    }
}

/// This enum describes the degree to which compiled programs will be optimized
#[derive(Debug, Copy, Clone, Default, ValueEnum)]
pub enum OptLevel {
    /// No optimizations at all
    None,
    /// Only basic optimizations are applied, e.g. constant propagation
    Basic,
    /// Most optimizations are applied, except when the cost is particularly high.
    #[default]
    Balanced,
    /// All optimizations are applied, with all tradeoffs in favor of runtime performance
    Max,
    /// Most optimizations are applied, but tuned to trade runtime performance for code size
    Size,
    /// Only optimizations which reduce code size are applied
    SizeMin,
}

/// This enum represents the behavior of the compiler with regard to warnings
#[derive(Debug, Copy, Clone, Default, ValueEnum)]
pub enum Warnings {
    /// Disable all warnings
    None,
    /// Enable all warnings
    #[default]
    All,
    /// Promotes warnings to errors
    Error,
}
impl fmt::Display for Warnings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => f.write_str("none"),
            Self::All => f.write_str("auto"),
            Self::Error => f.write_str("error"),
        }
    }
}
impl FromStr for Warnings {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "all" => Ok(Self::All),
            "error" => Ok(Self::Error),
            _ => Err(()),
        }
    }
}
