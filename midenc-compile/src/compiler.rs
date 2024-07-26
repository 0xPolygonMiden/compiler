use std::{path::PathBuf, sync::Arc};

use clap::{Args, ColorChoice, Parser};
use miden_diagnostics::{term::termcolor::ColorChoice as MDColorChoice, Emitter};
use midenc_session::{
    DebugInfo, InputFile, OptLevel, Options, OutputFile, OutputType, OutputTypeSpec, OutputTypes,
    ProjectType, Session, TargetEnv, VerbosityFlag, Warnings,
};

/// Compile a program from WebAssembly or Miden IR, to Miden Assembly.
#[derive(Debug, Args)]
pub struct Compiler {
    /// The input file to compile
    ///
    /// You may specify `-` to read from stdin, otherwise you must provide a path
    #[arg(required(true), value_name = "FILE")]
    pub input: InputFile,
    /// Write all intermediate compiler artifacts to `<dir>`
    ///
    /// Defaults to a directory named `target` in the current working directory
    #[arg(
        hide(true),
        long,
        value_name = "DIR",
        env = "MIDENC_TARGET_DIR",
        help_heading = "Output"
    )]
    pub target_dir: Option<PathBuf>,
    /// The working directory for the compiler
    ///
    /// By default this will be the working directory the compiler is executed from
    #[arg(long, value_name = "DIR", help_heading = "Output")]
    pub working_dir: Option<PathBuf>,
    /// Write output to compiler-chosen filename in `<dir>`
    #[arg(
        long,
        value_name = "DIR",
        env = "MIDENC_OUT_DIR",
        help_heading = "Output"
    )]
    pub output_dir: Option<PathBuf>,
    /// Write output to `<filename>`
    #[arg(long, short = 'o', value_name = "FILENAME", help_heading = "Output")]
    pub output_file: Option<PathBuf>,
    /// Write output to stdout
    #[arg(long, conflicts_with("output_file"), help_heading = "Output")]
    pub stdout: bool,
    #[command(flatten)]
    pub options: CompilerOptions,
}

/// Used to parse `CompilerOptions` for tests
#[derive(Debug, Parser)]
#[command(name = "midenc")]
pub struct TestCompiler {
    #[command(flatten)]
    pub options: CompilerOptions,
}

#[derive(Debug, Args)]
pub struct CompilerOptions {
    /// Specify what type and level of informational output to emit
    #[arg(
        long = "verbose",
        short = 'v',
        value_enum,
        value_name = "LEVEL",
        next_line_help(true),
        default_value_t = VerbosityFlag::Info,
        default_missing_value = "debug",
        help_heading = "Diagnostics"
    )]
    pub verbosity: VerbosityFlag,
    /// Specify how warnings should be treated by the compiler.
    #[arg(
        long,
        short = 'W',
        value_enum,
        value_name = "LEVEL",
        next_line_help(true),
        default_value_t = Warnings::All,
        default_missing_value = "all",
        help_heading = "Diagnostics"
    )]
    pub warn: Warnings,
    /// Whether, and how, to color terminal output
    #[arg(long, value_enum, default_value_t = ColorChoice::Auto, default_missing_value = "auto", help_heading = "Diagnostics")]
    pub color: ColorChoice,
    /// The target environment to compile for
    #[arg(long, value_name = "TARGET", default_value_t = TargetEnv::Base, help_heading = "Compiler")]
    pub target: TargetEnv,
    /// Tells the compiler to produce an executable Miden program
    ///
    /// When the target is `base` or `rollup`, this defaults to true
    #[arg(
        long = "exe",
        default_value_t = true,
        default_value_if("target", "emu", Some("false")),
        help_heading = "Compiler"
    )]
    pub is_program: bool,
    /// Tells the compiler to produce a Miden library
    ///
    /// When the target is `emu`, this defaults to true
    #[arg(
        long = "lib",
        conflicts_with("is_program"),
        default_value_t = false,
        default_value_if("target", "emu", Some("true")),
        help_heading = "Compiler"
    )]
    pub is_library: bool,
    /// Specify one or more output types for the compiler to emit
    #[arg(
        long = "emit",
        value_name = "SPEC",
        value_delimiter = ',',
        help_heading = "Output"
    )]
    pub output_types: Vec<OutputTypeSpec>,
    #[arg(
        long,
        value_enum,
        value_name = "LEVEL",
        next_line_help(true),
        default_value_t = DebugInfo::Line,
        default_missing_value = "line",
        help_heading = "Output"
    )]
    pub debug: DebugInfo,
    #[arg(
        long = "optimize",
        value_enum,
        value_name = "LEVEL",
        next_line_help(true),
        default_value_t = OptLevel::None,
        default_missing_value = "none",
        help_heading = "Output"
    )]
    pub opt_level: OptLevel,
    /// Print the IR after each pass is applied
    #[arg(long, default_value_t = false, help_heading = "Passes")]
    pub print_ir_after_all: bool,
    /// Print the IR after running a specific pass
    #[arg(long, value_name = "PASS", help_heading = "Passes")]
    pub print_ir_after_pass: Option<String>,
}
impl Compiler {
    /// Use this configuration to obtain a [Session] used for compilation
    pub fn into_session(self, emitter: Option<Arc<dyn Emitter>>) -> Session {
        let tmp_dir = self.target_dir.unwrap_or_else(std::env::temp_dir);
        let output_file = match self.output_file {
            Some(path) => Some(OutputFile::Real(path)),
            None if self.stdout => Some(OutputFile::Stdout),
            None => None,
        };
        let cwd = self.working_dir;
        let options = self.options.into_options(cwd);

        Session::new(self.input, self.output_dir, output_file, Some(tmp_dir), options, emitter)
    }
}

impl CompilerOptions {
    pub fn parse_options(extra_args: &[&str]) -> midenc_session::Options {
        let command = <TestCompiler as clap::CommandFactory>::command();
        let command = crate::register_flags(command);
        let mut matches = command
            .try_get_matches_from(extra_args)
            .expect("expected default arguments to parse successfully");
        let compile_matches = matches.clone();

        let copts = <Self as clap::FromArgMatches>::from_arg_matches_mut(&mut matches)
            .map_err(format_error::<TestCompiler>)
            .unwrap_or_else(|err| panic!("{err}"));

        copts.into_options(None).with_arg_matches(compile_matches)
    }

    pub fn into_options(self, working_dir: Option<PathBuf>) -> Options {
        let cwd = working_dir
            .unwrap_or_else(|| std::env::current_dir().expect("no working directory available"));

        let color = match self.color {
            ColorChoice::Auto => MDColorChoice::Auto,
            ColorChoice::Always => MDColorChoice::Always,
            ColorChoice::Never => MDColorChoice::Never,
        };

        let mut output_types = OutputTypes::new(self.output_types);
        if output_types.is_empty() {
            output_types.insert(OutputType::Mast, None);
        }

        let project_type = if self.is_program {
            ProjectType::Program
        } else {
            ProjectType::Library
        };
        let mut options = Options::new(self.target, project_type, cwd)
            .with_color(color)
            .with_verbosity(self.verbosity.into())
            .with_warnings(self.warn)
            .with_debug_info(self.debug)
            .with_optimization(self.opt_level)
            .with_output_types(output_types);
        options.print_ir_after_all = self.print_ir_after_all;
        options.print_ir_after_pass = self.print_ir_after_pass;
        options
    }
}

fn format_error<I: clap::CommandFactory>(err: clap::Error) -> clap::Error {
    let mut cmd = I::command();
    err.format(&mut cmd)
}
