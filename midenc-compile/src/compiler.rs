use std::{path::PathBuf, sync::Arc};

use clap::{Args, ColorChoice, Parser};
use midenc_session::{
    diagnostics::{ColorChoice as MDColorChoice, DefaultSourceManager, Emitter},
    DebugInfo, InputFile, LinkLibrary, OptLevel, Options, OutputFile, OutputType, OutputTypeSpec,
    OutputTypes, ProjectType, Session, TargetEnv, Verbosity, Warnings,
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
    /// The path to the root directory of the Miden toolchain libraries
    ///
    /// By default this is assumed to be ~/.miden/toolchains/<version>
    #[arg(
        long,
        value_name = "DIR",
        env = "MIDENC_SYSROOT",
        help_heading = "Compiler"
    )]
    pub sysroot: Option<PathBuf>,
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
        default_value_t = Verbosity::Info,
        default_missing_value = "debug",
        help_heading = "Diagnostics"
    )]
    pub verbosity: Verbosity,
    /// Specify how warnings should be treated by the compiler.
    #[arg(
        long,
        short = 'W',
        value_enum,
        value_name = "LEVEL",
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
    /// Specify the function to call as the entrypoint for the program
    #[arg(long = "entrypoint", help_heading = "Compiler", hide(true))]
    pub entrypoint: Option<String>,
    /// Tells the compiler to produce an executable Miden program
    ///
    /// When the target is `base` or `rollup`, this defaults to true
    #[arg(
        long = "exe",
        default_value_t = true,
        default_value_if("target", "emu", Some("false")),
        help_heading = "Linker"
    )]
    pub is_program: bool,
    /// Tells the compiler to produce a Miden library
    ///
    /// When the target is `emu`, this defaults to true
    #[arg(
        long = "lib",
        conflicts_with("is_program"),
        conflicts_with("entrypoint"),
        default_value_t = false,
        default_value_if("target", "emu", Some("true")),
        help_heading = "Linker"
    )]
    pub is_library: bool,
    /// Specify one or more search paths for link libraries requested via `-l`
    #[arg(
        long = "search-path",
        short = 'L',
        value_name = "PATH",
        help_heading = "Linker"
    )]
    pub search_path: Vec<PathBuf>,
    /// Link compiled projects to the specified library NAME.
    ///
    /// The optional KIND can be provided to indicate what type of library it is.
    ///
    /// NAME must either be an absolute path (with extension when applicable), or
    /// a library namespace (no extension). The former will be used as the path
    /// to load the library, without looking for it in the library search paths,
    /// while the latter will be located in the search path based on its KIND.
    ///
    /// See below for valid KINDs:
    #[arg(
        long = "link-library",
        short = 'l',
        value_name = "[KIND=]NAME",
        next_line_help(true),
        help_heading = "Linker"
    )]
    pub link_libraries: Vec<LinkLibrary>,
    /// Specify one or more output types for the compiler to emit
    ///
    /// The format for SPEC is `KIND[=PATH]`. You can specify multiple items at
    /// once by separating each SPEC with a comma, you can also pass this flag
    /// multiple times.
    ///
    /// PATH must be a directory in which to place the outputs, or `-` for stdout.
    #[arg(
        long = "emit",
        value_name = "SPEC",
        value_delimiter = ',',
        next_line_help(true),
        help_heading = "Output"
    )]
    pub output_types: Vec<OutputTypeSpec>,
    /// Specify what level of debug information to emit in compilation artifacts
    #[arg(
        long,
        value_enum,
        value_name = "LEVEL",
        next_line_help(true),
        default_value_t = DebugInfo::Full,
        default_missing_value = "full",
        num_args(0..=1),
        help_heading = "Output"
    )]
    pub debug: DebugInfo,
    /// Specify what type, and to what degree, of optimizations to apply to code during
    /// compilation.
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
        let source_manager = Arc::new(DefaultSourceManager::default());
        let tmp_dir = self.target_dir.unwrap_or_else(std::env::temp_dir);
        let output_file = match self.output_file {
            Some(path) => Some(OutputFile::Real(path)),
            None if self.stdout => Some(OutputFile::Stdout),
            None => None,
        };
        let cwd = self.working_dir;
        let sysroot = self.sysroot;
        let options = self.options.into_options(cwd, sysroot);

        Session::new(
            self.input,
            self.output_dir,
            output_file,
            Some(tmp_dir),
            options,
            emitter,
            source_manager,
        )
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

        copts.into_options(None, None).with_arg_matches(compile_matches)
    }

    pub fn into_options(self, working_dir: Option<PathBuf>, sysroot: Option<PathBuf>) -> Options {
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
        let mut options = Options::new(self.target, project_type, cwd, sysroot)
            .with_color(color)
            .with_verbosity(self.verbosity.into())
            .with_warnings(self.warn)
            .with_debug_info(self.debug)
            .with_optimization(self.opt_level)
            .with_output_types(output_types);
        options.search_paths = self.search_path;
        options.link_libraries = self.link_libraries;
        options.entrypoint = self.entrypoint;
        options.print_ir_after_all = self.print_ir_after_all;
        options.print_ir_after_pass = self.print_ir_after_pass;
        options
    }
}

fn format_error<I: clap::CommandFactory>(err: clap::Error) -> clap::Error {
    let mut cmd = I::command();
    err.format(&mut cmd)
}
