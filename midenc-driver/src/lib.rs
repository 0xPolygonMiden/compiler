mod midenc;

pub use self::midenc::Midenc;

/// A convenience alias for `Result<T, DriverError>`
pub type DriverResult<T> = Result<T, DriverError>;

/// This error type is produced by the `midenc` driver
#[derive(Debug, thiserror::Error)]
pub enum DriverError {
    /// An error was raised due to invalid command-line arguments or argument validation
    #[error(transparent)]
    Clap(#[from] clap::Error),
    /// Compilation failed
    #[error(transparent)]
    Compile(#[from] midenc_compile::CompilerError),
    /// An error occurred when reading a file
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// An unexpected error occurred
    #[error(transparent)]
    Failed(#[from] anyhow::Error),
    /// An error was emitted as a diagnostic, so we don't need to emit info to stdout
    #[error("exited due to error: see diagnostics for details")]
    Reported,
}

/// Run the driver as if it was invoked from the command-line
pub fn run<P, A>(cwd: P, args: A) -> Result<(), DriverError>
where
    P: Into<std::path::PathBuf>,
    A: IntoIterator<Item = std::ffi::OsString>,
{
    match Midenc::run(cwd, args) {
        Err(DriverError::Compile(midenc_compile::CompilerError::Stopped)) => Ok(()),
        result => result,
    }
}
