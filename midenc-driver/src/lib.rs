mod midenc;

pub use clap::Error as ClapError;
pub use midenc_session::diagnostics;
use midenc_session::diagnostics::Report;

pub use self::midenc::Midenc;

/// A convenience alias for `Result<T, Report>`
pub type DriverResult<T> = Result<T, Report>;

/// Run the driver as if it was invoked from the command-line
pub fn run<P, A>(cwd: P, args: A) -> Result<(), Report>
where
    P: Into<std::path::PathBuf>,
    A: IntoIterator<Item = std::ffi::OsString>,
{
    setup_diagnostics();

    match Midenc::run(cwd, args) {
        Err(report) => match report.downcast::<midenc_compile::CompilerStopped>() {
            Ok(_) => Ok(()),
            Err(report) => Err(report),
        },
        result => result,
    }
}

fn setup_diagnostics() {
    use diagnostics::ReportHandlerOpts;

    let result =
        diagnostics::reporting::set_hook(Box::new(|_| Box::new(ReportHandlerOpts::new().build())));
    if result.is_ok() {
        diagnostics::reporting::set_panic_hook();
    }
}
