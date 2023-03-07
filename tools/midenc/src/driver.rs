use std::ffi::OsString;
use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;

use clap::{Parser, Subcommand, ValueEnum};

use miden_diagnostics::{CodeMap, Emitter, Verbosity};

use crate::compiler::{self, Options};

#[derive(Debug, Copy, Clone, Default, ValueEnum)]
pub enum Warnings {
    /// Disable all warnings
    None,
    /// Enable all warnings
    #[default]
    Auto,
    /// Promotes warnings to errors
    Error,
}
impl fmt::Display for Warnings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => f.write_str("none"),
            Self::Auto => f.write_str("auto"),
            Self::Error => f.write_str("error"),
        }
    }
}

#[derive(Debug, Parser)]
#[command(name = "midenc")]
#[command(author, version, about = "A compiler for Miden Assembly", long_about = None)]
pub struct Driver {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Compile {
        /// Write all compiler artifacts to DIR
        #[arg(value_name = "DIR", long = "output-dir")]
        output_dir: Option<PathBuf>,
        /// Modify how warnings are treated by the compiler.
        #[arg(
            value_enum,
            value_name = "LEVEL",
            short = 'W',
            next_line_help(true),
            default_value_t = Warnings::Auto,
            default_missing_value = "auto",
        )]
        warn: Warnings,
        /// When set, produces more verbose output during compilation
        #[arg(short = 'v', long, default_value_t = false)]
        verbose: bool,
        /// Path(s) to the source file(s) to compile.
        ///
        /// You may also use `-` as a file name to read a file from stdin.
        #[arg(last(true), value_name = "INPUTS")]
        inputs: Vec<PathBuf>,
    },
}

pub fn run_compiler(cwd: PathBuf, args: impl Iterator<Item = OsString>) -> anyhow::Result<i32> {
    run_compiler_with_emitter(cwd, args, None)
}

pub fn run_compiler_with_emitter(
    cwd: PathBuf,
    args: impl Iterator<Item = OsString>,
    emitter: Option<Arc<dyn Emitter>>,
) -> anyhow::Result<i32> {
    let driver = Driver::try_parse_from(args)?;
    match driver.command {
        Commands::Compile {
            inputs,
            output_dir,
            warn,
            verbose,
        } => {
            let codemap = Arc::new(CodeMap::new());
            let verbosity = if verbose {
                Verbosity::Debug
            } else {
                Verbosity::Info
            };
            let options = Options::new(cwd, inputs, output_dir, warn, verbosity)?;
            compiler::compile(options, codemap, emitter).map(|_| 0)
        }
    }
}
