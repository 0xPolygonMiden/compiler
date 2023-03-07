use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::bail;
use clap::ColorChoice;
use miden_diagnostics::{FileName, Verbosity};

use crate::driver::Warnings;

#[derive(Clone, Debug)]
pub struct Options {
    /// The name of the program being compiled
    pub name: String,
    /// The type of outputs to emit
    pub output_types: OutputTypes,
    /// Whether, and how, to color terminal output
    pub color: ColorChoice,
    /// True if warnings should be promoted to errors
    pub warnings_as_errors: bool,
    /// True if warnings should be silenced
    pub no_warn: bool,
    /// The verbosity of terminal output produced by the compiler
    pub verbosity: Verbosity,
    /// The current working directory of the compiler
    pub current_dir: PathBuf,
    /// This is the set of inputs
    pub input_files: Vec<FileName>,
    /// The directory in which compiler artifacts will be emitted
    pub output_dir: Option<PathBuf>,
}
impl Options {
    pub fn new(
        cwd: PathBuf,
        inputs: Vec<PathBuf>,
        output_dir: Option<PathBuf>,
        warn: Warnings,
        verbosity: Verbosity,
    ) -> anyhow::Result<Arc<Self>> {
        let (warnings_as_errors, no_warn) = match warn {
            Warnings::Auto => (false, false),
            Warnings::None => (false, true),
            Warnings::Error => (true, false),
        };

        // Use the current working directory name as the program name, falling back to the
        // file stem of the first input file.
        let name = match cwd.file_stem().map(|os| os.to_string_lossy().into_owned()) {
            None if inputs.is_empty() => "app".to_string(),
            None => inputs
                .first()
                .unwrap()
                .file_stem()
                .map(|os| os.to_string_lossy().into_owned())
                .unwrap(),
            Some(name) => name,
        };

        let input_files = if inputs.is_empty() {
            vec![]
        } else {
            let num_inputs = inputs.len();
            let first_os = inputs.first().unwrap();
            let first = first_os.to_str();

            match first {
                Some("-") => {
                    if num_inputs > 1 {
                        bail!("stdin as an input cannot be combined with other inputs");
                    }
                    vec!["stdin".into()]
                }
                other => {
                    let first_filename: FileName = other
                        .map(PathBuf::from)
                        .unwrap_or_else(|| PathBuf::from(first_os))
                        .into();
                    let mut files = vec![];
                    if first_filename.is_dir() {
                        bail!(
                            "must specify a file path, '{}' is a directory",
                            &first_filename
                        );
                    }
                    files.push(first_filename);

                    for input in inputs {
                        let path = PathBuf::from(input);
                        if !path.exists() {
                            bail!(
                                "invalid input path, no such file or directory: {}",
                                path.display()
                            );
                        }
                        if path.is_dir() {
                            bail!(
                                "must specify a file path, '{}' is a directory",
                                path.display()
                            );
                        }
                        files.push(path.into());
                    }

                    files
                }
            }
        };

        let output_types = OutputTypes::default();

        Ok(Arc::new(Self {
            name,
            output_types,
            color: ColorChoice::Auto,
            warnings_as_errors,
            no_warn,
            verbosity,
            current_dir: cwd,
            input_files,
            output_dir,
        }))
    }
}

bitflags::bitflags! {
    pub struct OutputTypes: u32 {
        /// The abstract syntax tree produced by parsing Miden IR
        const AST = 1;
        /// The IR produced by lowering from another frontend (e.g. Sway)
        const IR = 1 << 1;
        /// The assembly produced by lowering Miden IR through the compiler
        const ASM = 1 << 2;

        /// An alias which represents output of all artifact types
        const ALL = Self::AST.bits | Self::IR.bits | Self::ASM.bits;
    }
}
impl Default for OutputTypes {
    fn default() -> Self {
        Self::ASM
    }
}
impl FromStr for OutputTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ast" => Ok(Self::AST),
            "ir" => Ok(Self::IR),
            "asm" => Ok(Self::ASM),
            "all" => Ok(Self::ALL),
            _ => Err(()),
        }
    }
}
