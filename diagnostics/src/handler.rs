use std::io::Write;
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::term::termcolor::{Color, ColorSpec, WriteColor};
use crate::*;

pub struct DiagnosticsHandler {
    emitter: Arc<dyn Emitter>,
    pub(crate) codemap: Arc<CodeMap>,
    err_count: AtomicUsize,
    verbosity: Verbosity,
    warnings_as_errors: bool,
    no_warn: bool,
    silent: bool,
    pub(crate) display: crate::term::Config,
}
// We can safely implement these traits for DiagnosticsHandler,
// as the only two non-atomic fields are read-only after creation
unsafe impl Send for DiagnosticsHandler {}
unsafe impl Sync for DiagnosticsHandler {}
impl DiagnosticsHandler {
    pub fn new(
        config: DiagnosticsConfig,
        codemap: Arc<CodeMap>,
        emitter: Arc<dyn Emitter>,
    ) -> Self {
        let no_warn = config.no_warn || config.verbosity > Verbosity::Warning;
        Self {
            emitter,
            codemap,
            err_count: AtomicUsize::new(0),
            verbosity: config.verbosity,
            warnings_as_errors: config.warnings_as_errors,
            no_warn,
            silent: config.verbosity == Verbosity::Silent,
            display: config.display,
        }
    }

    pub fn lookup_file_id(&self, filename: impl Into<FileName>) -> Option<SourceId> {
        let filename = filename.into();
        self.codemap.get_file_id(&filename)
    }

    pub fn has_errors(&self) -> bool {
        self.err_count.load(Ordering::Relaxed) > 0
    }

    pub fn abort_if_errors(&self) {
        if self.has_errors() {
            FatalError.raise();
        }
    }

    /// Emits an error message and produces a FatalError object
    /// which can be used to terminate execution immediately
    pub fn fatal(&self, err: impl ToString) -> FatalError {
        self.error(err);
        FatalError
    }

    /// Report a diagnostic, forcing its severity to Error
    pub fn error(&self, error: impl ToString) {
        self.err_count.fetch_add(1, Ordering::Relaxed);
        let diagnostic = Diagnostic::error().with_message(error.to_string());
        self.emit(diagnostic);
    }

    /// Report a diagnostic, forcing its severity to Warning
    pub fn warn(&self, warning: impl ToString) {
        if self.warnings_as_errors {
            return self.error(warning);
        }
        let diagnostic = Diagnostic::warning().with_message(warning.to_string());
        self.emit(diagnostic);
    }

    /// Emits an informational message
    pub fn info(&self, message: impl ToString) {
        if self.verbosity > Verbosity::Info {
            return;
        }
        let info_color = self.display.styles.header(Severity::Help);
        let mut buffer = self.emitter.buffer();
        buffer.set_color(&info_color).ok();
        write!(&mut buffer, "info").unwrap();
        buffer.set_color(&self.display.styles.header_message).ok();
        write!(&mut buffer, ": {}", message.to_string()).unwrap();
        buffer.reset().ok();
        write!(&mut buffer, "\n").unwrap();
        self.emitter.print(buffer).unwrap();
    }

    /// Emits a debug message
    pub fn debug(&self, message: impl ToString) {
        if self.verbosity > Verbosity::Debug {
            return;
        }
        let mut debug_color = self.display.styles.header_message.clone();
        debug_color.set_fg(Some(Color::Blue));
        let mut buffer = self.emitter.buffer();
        buffer.set_color(&debug_color).ok();
        write!(&mut buffer, "debug").unwrap();
        buffer.set_color(&self.display.styles.header_message).ok();
        write!(&mut buffer, ": {}", message.to_string()).unwrap();
        buffer.reset().ok();
        write!(&mut buffer, "\n").unwrap();
        self.emitter.print(buffer).unwrap();
    }

    /// Emits a note
    pub fn note(&self, message: impl ToString) {
        if self.verbosity > Verbosity::Info {
            return;
        }
        self.emit(Diagnostic::note().with_message(message.to_string()));
    }

    /// Prints a warning-like message with the given prefix
    ///
    /// NOTE: This does not get promoted to an error if warnings-as-errors is set,
    /// as it is intended for informational purposes, not issues with the code being compiled
    pub fn notice(&self, prefix: &str, message: impl ToString) {
        if self.verbosity > Verbosity::Info {
            return;
        }
        self.write_prefixed(
            self.display.styles.header(Severity::Warning),
            prefix,
            message,
        );
    }

    /// Prints a success message with the given prefix
    pub fn success(&self, prefix: &str, message: impl ToString) {
        if self.silent {
            return;
        }
        self.write_prefixed(self.display.styles.header(Severity::Note), prefix, message);
    }

    /// Prints an error message with the given prefix
    pub fn failed(&self, prefix: &str, message: impl ToString) {
        self.err_count.fetch_add(1, Ordering::Relaxed);
        self.write_prefixed(self.display.styles.header(Severity::Error), prefix, message);
    }

    fn write_prefixed(&self, color: &ColorSpec, prefix: &str, message: impl ToString) {
        let mut buffer = self.emitter.buffer();
        buffer.set_color(&color).ok();
        write!(&mut buffer, "{:>12} ", prefix).unwrap();
        buffer.reset().ok();
        writeln!(&mut buffer, "{}", message.to_string()).unwrap();
        self.emitter.print(buffer).unwrap();
    }

    /// Generates an in-flight diagnostic for more complex diagnostics use cases
    ///
    /// The caller is responsible for dropping/emitting the diagnostic using the in-flight APIs
    pub fn diagnostic(&self, severity: Severity) -> InFlightDiagnostic<'_> {
        InFlightDiagnostic::new(self, severity)
    }

    /// Emits the given diagnostic
    #[inline(always)]
    pub fn emit(&self, diagnostic: impl ToDiagnostic) {
        if self.silent {
            return;
        }

        let mut diagnostic = diagnostic.to_diagnostic();
        match diagnostic.severity {
            Severity::Note if self.verbosity > Verbosity::Info => return,
            Severity::Warning if self.no_warn => return,
            Severity::Warning if self.warnings_as_errors => {
                diagnostic.severity = Severity::Error;
            }
            _ => (),
        }

        let mut buffer = self.emitter.buffer();
        crate::term::emit(
            &mut buffer,
            &self.display,
            self.codemap.deref(),
            &diagnostic,
        )
        .unwrap();
        self.emitter.print(buffer).unwrap();
    }
}
