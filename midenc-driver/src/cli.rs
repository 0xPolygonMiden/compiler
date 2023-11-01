use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::Arc;

use clap::{ColorChoice, Parser, Subcommand};
use miden_diagnostics::{CodeMap, Emitter};
use miden_hir::FunctionIdent;
use midenc_session::{Options, OutputType, OutputTypes};

use crate::commands::{self, Breakpoint};
use crate::{DriverError, Operand, VerbosityFlag, Warnings};

/// This struct provides the command-line interface used by `midenc`
#[derive(Debug, Parser)]
#[command(name = "midenc")]
#[command(author, version, about = "A compiler for Miden Assembly", long_about = None)]
pub struct Midenc {
    /// Specify what type and level of informational output to emit
    #[arg(global(true), value_enum, value_name = "LEVEL", short = 'v', next_line_help(true), default_value_t = VerbosityFlag::Info, default_missing_value = "debug")]
    verbosity: VerbosityFlag,
    /// Specify how warnings should be treated by the compiler.
    #[arg(
        global(true),
        value_enum,
        value_name = "LEVEL",
        short = 'W',
        next_line_help(true),
        default_value_t = Warnings::All,
    )]
    warn: Warnings,
    /// Whether, and how, to color terminal output
    #[arg(global(true), value_enum, long, default_value_t = ColorChoice::Auto, default_missing_value = "auto")]
    color: ColorChoice,
    /// Write all intermediate compiler artifacts to `<dir>`
    ///
    /// Defaults to a directory named `target` in the current working directory
    #[arg(
        global(true),
        value_name = "DIR",
        long = "target-dir",
        env = "MIDENC_TARGET_DIR"
    )]
    target_dir: Option<PathBuf>,
    /// The command to execute
    #[command(subcommand)]
    command: Commands,
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
        use midenc_session::CompileFlag;

        let command = <Self as clap::CommandFactory>::command();
        let command = command.mut_subcommand("compile", |cmd| {
            inventory::iter::<CompileFlag>
                .into_iter()
                .fold(cmd, |cmd, flag| {
                    let arg = clap::Arg::new(flag.name)
                        .long(flag.long.unwrap_or(flag.name))
                        .action(clap::ArgAction::from(flag.action));
                    let arg = if let Some(help) = flag.help {
                        arg.help(help)
                    } else {
                        arg
                    };
                    let arg = if let Some(short) = flag.short {
                        arg.short(short)
                    } else {
                        arg
                    };
                    let arg = if let Some(env) = flag.env {
                        arg.env(env)
                    } else {
                        arg
                    };
                    let arg = if let Some(value) = flag.default_missing_value {
                        arg.default_missing_value(value)
                    } else {
                        arg
                    };
                    let arg = if let Some(value) = flag.default_value {
                        arg.default_value(value)
                    } else {
                        arg
                    };
                    cmd.arg(arg)
                })
        });

        let mut matches = command.try_get_matches_from(args)?;
        let cli = <Self as clap::FromArgMatches>::from_arg_matches_mut(&mut matches)
            .map_err(format_error::<Self>)?;

        cli.invoke(cwd.into(), emitter, matches)
    }

    fn invoke(
        self,
        cwd: PathBuf,
        emitter: Option<Arc<dyn Emitter>>,
        matches: clap::ArgMatches,
    ) -> Result<(), DriverError> {
        use miden_diagnostics::term::termcolor::ColorChoice as MDColorChoice;

        let color = match self.color {
            ColorChoice::Auto => MDColorChoice::Auto,
            ColorChoice::Always => MDColorChoice::Always,
            ColorChoice::Never => MDColorChoice::Never,
        };

        match self.command {
            Commands::Compile {
                input,
                output_file,
                output_dir,
                output_types,
                passes,
                print_ir_after_all,
                print_ir_after_pass,
            } => {
                let mut output_types = OutputTypes::new(output_types);
                if output_types.is_empty() {
                    output_types.insert(OutputType::Masl, None);
                }
                let mut options = Options::new(cwd)
                    .with_color(color)
                    .with_verbosity(self.verbosity.into())
                    .with_warnings(self.warn)
                    .with_output_types(output_types);
                options.passes = passes;
                options.print_ir_after_all = print_ir_after_all;
                options.print_ir_after_pass = print_ir_after_pass;

                let session = Session::new(
                    project_type,
                    target,
                    input,
                    output_dir,
                    output_file,
                    tmp_dir,
                    options,
                    emitter,
                )
                .with_arg_matches(matches);

                commands::compile(Arc::new(session))
            }
            _ => unimplemented!(),
        }
    }
}

fn format_error<I: clap::CommandFactory>(err: clap::Error) -> clap::Error {
    let mut cmd = I::command();
    err.format(&mut cmd)
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Invoke the compiler frontend using the provided set of inputs
    Compile {
        /// The input file to compile
        ///
        /// You may specify `-` to read from stdin, otherwise you must provide a path
        #[arg(required(true), value_name = "FILE")]
        input: InputFile,
        /// Write output to `<filename>`
        #[arg(short = 'o', value_name = "FILENAME")]
        output_file: Option<OutputFile>,
        /// Write output to compiler-chosen filename in `<dir>`
        #[arg(long = "output-dir", value_name = "DIR", env = "MIDENC_OUT_DIR")]
        output_dir: Option<PathBuf>,
        /// Specify one or more output types for the compiler to emit
        #[arg(long = "emit", value_name = "SPEC", value_delimiter = ',')]
        output_types: Vec<OutputTypeSpec>,
        /// Specify which IR passes to run
        ///
        /// Example: `--passes split-critical-edges,treeify`
        ///
        /// The above will apply those passes, in that order, and then exit.
        #[arg(long = "passes", value_name = "PASSES", value_delimiter = ',')]
        passes: Option<Vec<String>>,
        /// Print the IR after each pass is applied
        #[arg(long = "print-ir-after-all", default_value_t = false)]
        print_ir_after_all: bool,
        /// Print the IR after running a specific pass
        #[arg(long = "print-ir-after-pass", value_name = "PASS")]
        print_ir_after_pass: Option<String>,
    },
    /// Start an interactive debugging session by compiling the given program to
    /// Miden Assembly, and running it with the Miden VM emulator.
    ///
    /// NOTE: The emulator does not support all Miden VM functionality.
    ///
    /// This command drops you into a shell which provides a primitive debugger
    /// with breakpoints and the ability to step through code and inspect the state
    /// of the operand stack and linear memory, as well as dump values on the stack
    /// to a desired representation. Think of this like `lldb` for Miden Assembly.
    Debug {
        /// Specify the fully-qualified name of the function to invoke as the program entrypoint
        ///
        /// For example, `foo::bar`.
        ///
        /// NOTE: If unset, the program will not be run until specified in the interactive shell.
        #[arg(value_name = "NAME", short = 'm', long = "main")]
        entrypoint: Option<FunctionIdent>,
        /// Optional breakpoints to set before running the program
        #[arg(value_name = "EXPR", short = 'b', long = "breakpoint")]
        breakpoints: Vec<Breakpoint>,
        /// The input file to compile
        #[arg(required(true), value_name = "FILE")]
        input: InputFile,
        /// Optional arguments to place on the operand stack before calling the program entrypoint.
        ///
        /// Arguments will be pushed on the operand stack in the order of appearance,
        ///
        /// Example: `-- a b` will push `a` on the stack, then `b`.
        ///
        /// These arguments must be valid field element values expressed in decimal format.
        #[arg(last(true), value_name = "ARGV")]
        args: Vec<Operand>,
    },
    /// Execute a compiled function using the Miden VM emulator.
    ///
    /// The emulator is more restrictive, but is faster than the Miden VM, and
    /// provides a wider array of debugging and introspection features when troubleshooting
    /// programs compiled by `midenc`.
    Exec {
        /// Specify the fully-qualified name of the function to invoke as the program entrypoint
        ///
        /// For example, `foo::bar`
        #[arg(value_name = "NAME", short = 'm', long = "main")]
        entrypoint: Option<FunctionIdent>,
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
        args: Vec<Operand>,
    },
    /// Compile and run a program with the Miden VM
    ///
    /// The program will be compiled to Miden Assembly and then run with the Miden VM.
    ///
    /// The inputs given must constitute a valid executable program.
    Run {
        /// Specify the fully-qualified name of the function to invoke as the program entrypoint
        ///
        /// For example, `foo::bar`
        #[arg(value_name = "NAME", short = 'm', long = "main")]
        entrypoint: Option<FunctionIdent>,
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
        args: Vec<Operand>,
    },
}
