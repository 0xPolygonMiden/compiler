use std::{ffi::OsString, path::PathBuf, sync::Arc};

use clap::{ColorChoice, Parser, Subcommand};
use miden_diagnostics::Emitter;
use midenc_compile as compile;
use midenc_hir::FunctionIdent;
use midenc_session::{InputFile, TargetEnv, VerbosityFlag, Warnings};

use super::DriverError;

/// This struct provides the command-line interface used by `midenc`
#[derive(Debug, Parser)]
#[command(name = "midenc")]
#[command(author, version, about = "A compiler for Miden Assembly", long_about = None)]
pub struct Midenc {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Compile(compile::Compiler),
    /// Execute a compiled function using the Miden VM emulator.
    ///
    /// The emulator is more restrictive, but is faster than the Miden VM, and
    /// provides a wider array of debugging and introspection features when troubleshooting
    /// programs compiled by `midenc`.
    Exec {
        /// Specify one or more input files to compile as part of the program to execute
        ///
        /// You may use `-` as a file name to read a file from stdin.
        #[arg(required(true), value_name = "FILE")]
        input: InputFile,
        /// Arguments to place on the operand stack before calling the program entrypoint.
        ///
        /// Arguments will be pushed on the operand stack in the order of appearance,
        ///
        /// Example: `-- a b` will push `a` on the stack, then `b`.
        ///
        /// These arguments must be valid field element values expressed in decimal format.
        #[arg(last(true), value_name = "ARGV")]
        args: Vec<String>,
        /// Specify what type and level of informational output to emit
        #[arg(
            long = "verbose",
            short = 'v',
            value_name = "LEVEL",
            value_enum,
            default_value_t = VerbosityFlag::Info,
            default_missing_value = "debug",
            help_heading = "Diagnostics",
        )]
        verbosity: VerbosityFlag,
        /// Specify how warnings should be treated by the compiler.
        #[arg(
            long,
            short = 'W',
            value_name = "LEVEL",
            value_enum,
            default_value_t = Warnings::All,
            help_heading = "Diagnostics",
        )]
        warn: Warnings,
        /// Whether, and how, to color terminal output
        #[arg(long, value_enum, default_value_t = ColorChoice::Auto, default_missing_value = "auto", help_heading = "Diagnostics")]
        color: ColorChoice,
        /// Write all intermediate compiler artifacts to `<dir>`
        ///
        /// Defaults to a directory named `target` in the current working directory
        #[arg(
            long,
            value_name = "DIR",
            hide(true),
            env = "MIDENC_TARGET_DIR",
            help_heading = "Output"
        )]
        target_dir: Option<PathBuf>,
        /// Specify the fully-qualified name of the function to invoke as the program entrypoint
        ///
        /// For example, `foo::bar`
        #[arg(long, short = 'e', value_name = "NAME")]
        entrypoint: Option<FunctionIdent>,
    },
    /// Compile and run a program with the Miden VM
    ///
    /// The program will be compiled to Miden Assembly and then run with the Miden VM.
    ///
    /// The inputs given must constitute a valid executable program.
    Run {
        /// Specify one or more input files to compile as part of the program to execute
        ///
        /// You may use `-` as a file name to read a file from stdin.
        #[arg(required(true), value_name = "FILE")]
        input: InputFile,
        /// Arguments to place on the operand stack before calling the program entrypoint.
        ///
        /// Arguments will be pushed on the operand stack in the order of appearance,
        ///
        /// Example: `-- a b` will push `a` on the stack, then `b`.
        ///
        /// These arguments must be valid field element values expressed in decimal format.
        #[arg(last(true), value_name = "ARGV")]
        args: Vec<String>,
        /// Specify what type and level of informational output to emit
        #[arg(
            long = "verbose",
            short = 'v',
            value_name = "LEVEL",
            value_enum,
            default_value_t = VerbosityFlag::Info,
            default_missing_value = "debug",
            help_heading = "Diagnostics",
        )]
        verbosity: VerbosityFlag,
        /// Specify how warnings should be treated by the compiler.
        #[arg(
            long,
            short = 'W',
            value_name = "LEVEL",
            value_enum,
            default_value_t = Warnings::All,
            help_heading = "Diagnostics",
        )]
        warn: Warnings,
        /// Whether, and how, to color terminal output
        #[arg(long, value_enum, default_value_t = ColorChoice::Auto, default_missing_value = "auto", help_heading = "Diagnostics")]
        color: ColorChoice,
        /// Write all intermediate compiler artifacts to `<dir>`
        ///
        /// Defaults to a directory named `target` in the current working directory
        #[arg(
            long,
            value_name = "DIR",
            hide(true),
            env = "MIDENC_TARGET_DIR",
            help_heading = "Output"
        )]
        target_dir: Option<PathBuf>,
        /// The target environment to compile for
        #[arg(long, value_name = "TARGET", hide(true), default_value_t = TargetEnv::Base)]
        target: TargetEnv,
        /// Specify the fully-qualified name of the function to invoke as the program entrypoint
        ///
        /// For example, `foo::bar`
        #[arg(long, short = 'e', value_name = "NAME")]
        entrypoint: Option<FunctionIdent>,
    },
}

impl Midenc {
    pub fn run<P, A>(cwd: P, args: A) -> Result<(), DriverError>
    where
        P: Into<PathBuf>,
        A: IntoIterator<Item = OsString>,
    {
        Self::run_with_emitter(cwd, args, None)
    }

    pub fn run_with_emitter<P, A>(
        cwd: P,
        args: A,
        emitter: Option<Arc<dyn Emitter>>,
    ) -> Result<(), DriverError>
    where
        P: Into<PathBuf>,
        A: IntoIterator<Item = OsString>,
    {
        let command = <Self as clap::CommandFactory>::command();
        let command = command.mut_subcommand("compile", compile::register_flags);

        let mut matches = command.try_get_matches_from(args)?;
        let compile_matches = matches.subcommand_matches("compile").cloned().unwrap_or_default();
        let cli = <Self as clap::FromArgMatches>::from_arg_matches_mut(&mut matches)
            .map_err(format_error::<Self>)?;

        cli.invoke(cwd.into(), emitter, compile_matches)
    }

    fn invoke(
        self,
        cwd: PathBuf,
        emitter: Option<Arc<dyn Emitter>>,
        matches: clap::ArgMatches,
    ) -> Result<(), DriverError> {
        match self.command {
            Commands::Compile(mut config) => {
                if config.working_dir.is_none() {
                    config.working_dir = Some(cwd);
                }
                let session = config.into_session(emitter).with_arg_matches(matches);
                match compile::compile(Arc::new(session)) {
                    Ok(_) => Ok(()),
                    Err(compile::CompilerError::Reported) => Err(DriverError::Reported),
                    Err(err) => Err(DriverError::Compile(err)),
                }
            }
            _ => unimplemented!(),
        }
    }
}

fn format_error<I: clap::CommandFactory>(err: clap::Error) -> clap::Error {
    let mut cmd = I::command();
    err.format(&mut cmd)
}
