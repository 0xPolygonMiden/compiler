use std::{ffi::OsString, path::PathBuf, rc::Rc, sync::Arc};

use clap::{ColorChoice, Parser, Subcommand};
use log::Log;
use midenc_compile as compile;
use midenc_debug as debugger;
use midenc_hir::FunctionIdent;
use midenc_session::{
    diagnostics::{Emitter, Report},
    InputFile, Verbosity, Warnings,
};

use crate::ClapDiagnostic;

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
    Compile {
        /// The input file to compile
        ///
        /// You may specify `-` to read from stdin, otherwise you must provide a path
        #[arg(required(true), value_name = "FILE")]
        input: InputFile,
        #[command(flatten)]
        options: compile::Compiler,
    },
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
            default_value_t = Verbosity::Info,
            default_missing_value = "debug",
            help_heading = "Diagnostics",
        )]
        verbosity: Verbosity,
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
    /// Run a program under the interactive Miden VM debugger
    ///
    /// This command starts a TUI-based interactive debugger with the given program loaded.
    Debug {
        /// Specify the path to a Miden program file to execute.
        ///
        /// Miden Assembly programs are emitted by the compiler with a `.masl` extension.
        ///
        /// You may use `-` as a file name to read a file from stdin.
        #[arg(required(true), value_name = "FILE")]
        input: InputFile,
        /// Specify the path to a file containing program inputs.
        ///
        /// Program inputs are stack and advice provider values which the program can
        /// access during execution. The inputs file is a JSON file which describes
        /// what the inputs are, or where to source them from.
        #[arg(long, value_name = "FILE")]
        inputs: Option<debugger::DebuggerConfig>,
        /// Arguments to place on the operand stack before calling the program entrypoint.
        ///
        /// Arguments will be pushed on the operand stack in the order of appearance,
        ///
        /// Example: `-- a b` will push `a` on the stack, then `b`.
        ///
        /// These arguments must be valid field element values expressed in decimal format.
        ///
        /// NOTE: These arguments will override any stack values provided via --inputs
        #[arg(last(true), value_name = "ARGV")]
        args: Vec<debugger::Felt>,
        #[command(flatten)]
        options: debugger::Debugger,
    },
}

impl Midenc {
    pub fn run<P, A>(
        cwd: P,
        args: A,
        logger: Box<dyn Log>,
        filter: log::LevelFilter,
    ) -> Result<(), Report>
    where
        P: Into<PathBuf>,
        A: IntoIterator<Item = OsString>,
    {
        Self::run_with_emitter(cwd, args, None, logger, filter)
    }

    pub fn run_with_emitter<P, A>(
        cwd: P,
        args: A,
        emitter: Option<Arc<dyn Emitter>>,
        logger: Box<dyn Log>,
        filter: log::LevelFilter,
    ) -> Result<(), Report>
    where
        P: Into<PathBuf>,
        A: IntoIterator<Item = OsString>,
    {
        let command = <Self as clap::CommandFactory>::command();
        let command = command.mut_subcommand("compile", midenc_session::flags::register_flags);

        let mut matches = command.try_get_matches_from(args).map_err(ClapDiagnostic::from)?;
        let compile_matches = matches.subcommand_matches("compile").cloned().unwrap_or_default();
        let cli = <Self as clap::FromArgMatches>::from_arg_matches_mut(&mut matches)
            .map_err(format_error::<Self>)
            .map_err(ClapDiagnostic::from)?;

        cli.invoke(cwd.into(), emitter, logger, filter, compile_matches)
    }

    fn invoke(
        self,
        cwd: PathBuf,
        emitter: Option<Arc<dyn Emitter>>,
        logger: Box<dyn Log>,
        filter: log::LevelFilter,
        matches: clap::ArgMatches,
    ) -> Result<(), Report> {
        match self.command {
            Commands::Compile { input, mut options } => {
                log::set_boxed_logger(logger)
                    .unwrap_or_else(|err| panic!("failed to install logger: {err}"));
                log::set_max_level(filter);
                if options.working_dir.is_none() {
                    options.working_dir = Some(cwd);
                }
                let session = options.into_session(vec![input], emitter).with_arg_matches(matches);
                compile::compile(Rc::new(session))
            }
            Commands::Debug {
                input,
                inputs,
                args,
                mut options,
            } => {
                if options.working_dir.is_none() {
                    options.working_dir = Some(cwd);
                }
                let session = options.into_session(vec![input], emitter);
                let args = args.into_iter().map(|felt| felt.0).collect();
                debugger::run(inputs, args, Rc::new(session), logger)
            }
            _ => unimplemented!(),
        }
    }
}

fn format_error<I: clap::CommandFactory>(err: clap::Error) -> clap::Error {
    let mut cmd = I::command();
    err.format(&mut cmd)
}
