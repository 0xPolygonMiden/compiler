mod cli;
pub mod commands;
mod options;

pub use self::cli::Midenc;
pub use self::options::*;

/// A convenience alias for `Result<T, DriverError>`
pub type DriverResult<T> = Result<T, DriverError>;

/// This error type is produced by the `midenc` driver
#[derive(Debug, thiserror::Error)]
pub enum DriverError {
    /// An error was raised due to invalid command-line arguments or argument validation
    #[error(transparent)]
    Clap(#[from] clap::Error),
    /// The compilation pipeline was stopped early
    #[error("compilation was canceled by user")]
    Stopped,
    /// An invalid input was given to the compiler
    #[error(transparent)]
    InvalidInput(#[from] midenc_session::InvalidInputError),
    /// An error occurred while parsing/translating a Wasm module
    #[error(transparent)]
    WasmError(#[from] miden_frontend_wasm::WasmError),
    /// An error occurred while parsing an HIR module
    #[error(transparent)]
    Parsing(#[from] miden_hir::parser::ParseError),
    /// An error occurred while running an analysis
    #[error(transparent)]
    Analysis(#[from] miden_hir::pass::AnalysisError),
    /// An error occurred while rewriting an IR entity
    #[error(transparent)]
    Rewriting(#[from] miden_hir::pass::RewriteError),
    /// An error occurred while converting from one dialect to another
    #[error(transparent)]
    Conversion(#[from] miden_hir::pass::ConversionError),
    /// An error occurred while linking a program
    #[error(transparent)]
    Linker(#[from] miden_hir::LinkerError),
    /// An error ocurred when reading a file
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// An error occured while compiling a program
    #[error(transparent)]
    Failed(#[from] anyhow::Error),
}
impl From<miden_hir::ModuleConflictError> for DriverError {
    fn from(err: miden_hir::ModuleConflictError) -> DriverError {
        Self::Linker(miden_hir::LinkerError::ModuleConflict(err.0))
    }
}

/// Run the driver as if it was invoked from the command-line
pub fn run<P, A>(cwd: P, args: A) -> Result<(), DriverError>
where
    P: Into<std::path::PathBuf>,
    A: IntoIterator<Item = std::ffi::OsString>,
{
    match Midenc::run(cwd, args) {
        Err(DriverError::Stopped) => Ok(()),
        result => result,
    }
}
