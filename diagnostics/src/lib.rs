mod codemap;
mod config;
mod diagnostic;
mod emitter;
mod filename;
mod handler;
mod index;
mod source;
mod span;

pub use codespan::Location;
pub use codespan::{ByteIndex, ByteOffset};
pub use codespan::{ColumnIndex, ColumnNumber, ColumnOffset};
pub use codespan::{Index, Offset};
pub use codespan::{LineIndex, LineNumber, LineOffset};
pub use codespan::{RawIndex, RawOffset};

pub use codespan_reporting::diagnostic::{LabelStyle, Severity};
pub use codespan_reporting::files::{Error, Files};
pub use codespan_reporting::term;

pub use miden_diagnostics_macros::*;

pub use self::codemap::CodeMap;
pub use self::config::{DiagnosticsConfig, Verbosity};
pub use self::diagnostic::InFlightDiagnostic;
pub use self::emitter::{CaptureEmitter, DefaultEmitter, Emitter, NullEmitter};
pub use self::filename::FileName;
pub use self::handler::DiagnosticsHandler;
pub use self::index::SourceIndex;
pub use self::source::{SourceFile, SourceId};
pub use self::span::{SourceSpan, Span, Spanned};

pub type Diagnostic = codespan_reporting::diagnostic::Diagnostic<SourceId>;
pub type Label = codespan_reporting::diagnostic::Label<SourceId>;

pub trait ToDiagnostic {
    fn to_diagnostic(self) -> Diagnostic;
}
impl ToDiagnostic for Diagnostic {
    #[inline(always)]
    fn to_diagnostic(self) -> Diagnostic {
        self
    }
}

pub struct FatalErrorMarker;

/// Used as a return value to signify a fatal error occurred
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct FatalError;
impl FatalError {
    pub fn raise(self) -> ! {
        std::panic::resume_unwind(Box::new(FatalErrorMarker))
    }
}
impl core::fmt::Display for FatalError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "The compiler has encountered a fatal error")
    }
}
