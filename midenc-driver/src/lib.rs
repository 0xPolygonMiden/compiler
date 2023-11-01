mod cli;
pub mod commands;
mod options;

pub use self::cli::Midenc;
pub use self::options::*;

/// This error type is produced by the `midenc` driver
#[derive(Debug, thiserror::Error)]
pub enum DriverError {
    /// An error was raised due to invalid command-line arguments or argument validation
    #[error(transparent)]
    Clap(#[from] clap::Error),
    /// An invalid compiler option was given
    #[error(transparent)]
    InvalidOption(midenc_session::InvalidOptionError),
    /// Occurs if no inputs are given to the compiler
    #[error("expected at least one input to compile")]
    NoInputs,
    /// An invalid input was given to the compiler
    #[error(transparent)]
    InvalidInput(#[from] midenc_session::InvalidInputError),
    /// An error occurred while parsing/translating a Wasm module
    #[error(transparent)]
    WasmError(#[from] miden_frontend_wasm::WasmError),
    /// An error occurred while parsing an HIR module
    #[error(transparent)]
    ParseHirError(#[from] miden_hir::parser::ParseError),
    /// An error ocurred when reading a file
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// An error occured while compiling a program
    #[error(transparent)]
    Failed(#[from] anyhow::Error),
}
impl From<midenc_session::InvalidOptionError> for DriverError {
    fn from(err: midenc_session::InvalidOptionError) -> Self {
        use midenc_session::InvalidOptionError;
        match err {
            InvalidOptionError::InvalidInput(err) => Self::InvalidInput(err),
        }
    }
}

/// Run the driver as if it was invoked from the command-line
pub fn run<P, A>(cwd: P, args: A) -> Result<(), DriverError>
where
    P: Into<std::path::PathBuf>,
    A: IntoIterator<Item = std::ffi::OsString>,
{
    Midenc::run(cwd, args)
}
