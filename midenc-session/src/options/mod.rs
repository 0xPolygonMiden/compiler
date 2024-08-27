use std::{
    fmt,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

use clap::ValueEnum;

use crate::{
    diagnostics::{ColorChoice, DiagnosticsConfig, Emitter},
    LinkLibrary, OutputTypes, ProjectType, TargetEnv,
};

/// This struct contains all of the configuration options for the compiler
pub struct Options {
    /// The name of the program being compiled
    pub name: Option<String>,
    /// The type of project we're compiling this session
    pub project_type: ProjectType,
    /// The name of the function to call as the entrypoint
    pub entrypoint: Option<String>,
    /// The current target environment for this session
    pub target: TargetEnv,
    /// The optimization level for the current program
    pub optimize: OptLevel,
    /// The level of debugging info for the current program
    pub debug: DebugInfo,
    /// The type of outputs to emit
    pub output_types: OutputTypes,
    /// The paths in which to search for Miden Assembly libraries to link against
    pub search_paths: Vec<PathBuf>,
    /// The set of Miden libraries to link against
    pub link_libraries: Vec<LinkLibrary>,
    /// The location of the libraries which are shipped with the compiler
    pub sysroot: Option<PathBuf>,
    /// Whether, and how, to color terminal output
    pub color: ColorChoice,
    /// The current diagnostics configuration
    pub diagnostics: DiagnosticsConfig,
    /// The current working directory of the compiler
    pub current_dir: PathBuf,
    /// Only parse inputs
    pub parse_only: bool,
    /// Only perform semantic analysis on the input
    pub analyze_only: bool,
    /// Run the linker on the inputs, but do not generate Miden Assembly
    pub link_only: bool,
    /// Generate Miden Assembly from the inputs without the linker
    pub no_link: bool,
    /// Print IR to stdout after each pass
    pub print_ir_after_all: bool,
    /// Print IR to stdout each time the named passes are applied
    pub print_ir_after_pass: Vec<String>,
    /// Save intermediate artifacts in memory during compilation
    pub save_temps: bool,
    /// We store any leftover argument matches in the session options for use
    /// by any downstream crates that register custom flags
    arg_matches: clap::ArgMatches,
}

impl fmt::Debug for Options {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Options")
            .field("name", &self.name)
            .field("project_type", &self.project_type)
            .field("entrypoint", &self.entrypoint)
            .field("target", &self.target)
            .field("optimize", &self.optimize)
            .field("debug", &self.debug)
            .field("output_types", &self.output_types)
            .field("search_paths", &self.search_paths)
            .field("link_libraries", &self.link_libraries)
            .field("sysroot", &self.sysroot)
            .field("color", &self.color)
            .field("diagnostics", &self.diagnostics)
            .field("current_dir", &self.current_dir)
            .field("parse_only", &self.parse_only)
            .field("analyze_only", &self.analyze_only)
            .field("link_only", &self.link_only)
            .field("no_link", &self.no_link)
            .field("save_temps", &self.save_temps)
            .field("print_ir_after_all", &self.print_ir_after_all)
            .field("print_ir_after_pass", &self.print_ir_after_pass)
            .field_with("extra_arguments", |f| {
                let mut map = f.debug_map();
                for id in self.arg_matches.ids() {
                    use clap::parser::ValueSource;
                    // Don't print CompilerOptions arg group
                    if id.as_str() == "CompilerOptions" {
                        continue;
                    }
                    // Don't print default values
                    if matches!(
                        self.arg_matches.value_source(id.as_str()),
                        Some(ValueSource::DefaultValue)
                    ) {
                        continue;
                    }
                    map.key(&id.as_str()).value_with(|f| {
                        let mut list = f.debug_list();
                        if let Some(occurs) = self
                            .arg_matches
                            .try_get_raw_occurrences(id.as_str())
                            .expect("expected flag")
                        {
                            list.entries(occurs.flatten());
                        }
                        list.finish()
                    });
                }
                map.finish()
            })
            .finish()
    }
}

impl Default for Options {
    fn default() -> Self {
        let current_dir = std::env::current_dir().expect("could not get working directory");
        let target = TargetEnv::default();
        let project_type = ProjectType::default_for_target(target);
        Self::new(None, target, project_type, current_dir, None)
    }
}

impl Options {
    pub fn new(
        name: Option<String>,
        target: TargetEnv,
        project_type: ProjectType,
        current_dir: PathBuf,
        sysroot: Option<PathBuf>,
    ) -> Self {
        let sysroot = sysroot.or_else(|| {
            std::env::var("HOME").ok().map(|home| {
                Path::new(&home)
                    .join(".miden")
                    .join("toolchains")
                    .join(crate::MIDENC_BUILD_VERSION)
            })
        });

        let arg_matches = crate::flags::default_arg_matches(None::<&str>).unwrap();

        Self {
            name,
            target,
            project_type,
            entrypoint: None,
            optimize: OptLevel::None,
            debug: DebugInfo::None,
            output_types: Default::default(),
            search_paths: vec![],
            link_libraries: vec![],
            sysroot,
            color: Default::default(),
            diagnostics: Default::default(),
            current_dir,
            parse_only: false,
            analyze_only: false,
            link_only: false,
            no_link: false,
            save_temps: false,
            print_ir_after_all: false,
            print_ir_after_pass: vec![],
            arg_matches,
        }
    }

    #[inline(always)]
    pub fn with_color(mut self, color: ColorChoice) -> Self {
        self.color = color;
        self
    }

    #[inline(always)]
    pub fn with_verbosity(mut self, verbosity: Verbosity) -> Self {
        self.diagnostics.verbosity = verbosity;
        self
    }

    #[inline(always)]
    pub fn with_debug_info(mut self, debug: DebugInfo) -> Self {
        self.debug = debug;
        self
    }

    #[inline(always)]
    pub fn with_optimization(mut self, level: OptLevel) -> Self {
        self.optimize = level;
        self
    }

    pub fn with_warnings(mut self, warnings: Warnings) -> Self {
        self.diagnostics.warnings = warnings;
        self
    }

    #[inline(always)]
    pub fn with_output_types(mut self, output_types: OutputTypes) -> Self {
        self.output_types = output_types;
        self
    }

    #[doc(hidden)]
    pub fn with_arg_matches(mut self, matches: clap::ArgMatches) -> Self {
        self.arg_matches = matches;
        self
    }

    #[doc(hidden)]
    pub fn set_arg_matches(&mut self, matches: clap::ArgMatches) {
        self.arg_matches = matches;
    }

    /// Get a new [Emitter] based on the current options.
    pub fn default_emitter(&self) -> Arc<dyn Emitter> {
        use crate::diagnostics::{DefaultEmitter, NullEmitter};

        match self.diagnostics.verbosity {
            Verbosity::Silent => Arc::new(NullEmitter::new(self.color)),
            _ => Arc::new(DefaultEmitter::new(self.color)),
        }
    }

    /// Returns true if source location information should be emitted by the compiler
    #[inline(always)]
    pub fn emit_source_locations(&self) -> bool {
        matches!(self.debug, DebugInfo::Line | DebugInfo::Full)
    }

    /// Returns true if rich debugging information should be emitted by the compiler
    #[inline(always)]
    pub fn emit_debug_decorators(&self) -> bool {
        matches!(self.debug, DebugInfo::Full)
    }

    /// Returns true if debug assertions are enabled
    #[inline(always)]
    pub fn emit_debug_assertions(&self) -> bool {
        self.debug != DebugInfo::None && matches!(self.optimize, OptLevel::None | OptLevel::Basic)
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

/// This enum describes what type of debugging information to emit in compiled programs
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, ValueEnum)]
pub enum DebugInfo {
    /// Do not emit debug info in the final output
    None,
    /// Emit source location information in the final output
    #[default]
    Line,
    /// Emit all available debug information in the final output
    Full,
}

/// This enum represents the behavior of the compiler with regard to warnings
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, ValueEnum)]
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

/// This enum represents the type of messages produced by the compiler during execution
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Verbosity {
    /// Emit additional debug/trace information during compilation
    Debug,
    /// Emit the standard informational, warning, and error messages
    #[default]
    Info,
    /// Only emit warnings and errors
    Warning,
    /// Only emit errors
    Error,
    /// Do not emit anything to stdout/stderr
    Silent,
}
