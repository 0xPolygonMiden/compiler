use core::{fmt, num::IntErrorKind};

use miden_diagnostics::{Diagnostic, SourceIndex, SourceSpan, ToDiagnostic};

/// Errors that may occur during lexing of the source
#[derive(Clone, Debug, thiserror::Error)]
pub enum LexicalError {
    #[error("invalid integer value: {}", DisplayIntErrorKind(reason))]
    InvalidInt {
        span: SourceSpan,
        reason: IntErrorKind,
    },
    #[error("encountered unexpected character '{found}'")]
    UnexpectedCharacter { start: SourceIndex, found: char },
    #[error("unclosed string")]
    UnclosedString { span: SourceSpan },
    #[error("invalid module identifier")]
    InvalidModuleIdentifier { span: SourceSpan },
    #[error("invalid function identifier")]
    InvalidFunctionIdentifier { span: SourceSpan },
}
impl PartialEq for LexicalError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::InvalidInt { reason: lhs, .. }, Self::InvalidInt { reason: rhs, .. }) => {
                lhs == rhs
            }
            (
                Self::UnexpectedCharacter { found: lhs, .. },
                Self::UnexpectedCharacter { found: rhs, .. },
            ) => lhs == rhs,
            (Self::UnclosedString { .. }, Self::UnclosedString { .. }) => true,
            (Self::InvalidModuleIdentifier { .. }, Self::InvalidModuleIdentifier { .. }) => true,
            (Self::InvalidFunctionIdentifier { .. }, Self::InvalidFunctionIdentifier { .. }) => {
                true
            }
            _ => false,
        }
    }
}
impl ToDiagnostic for LexicalError {
    fn to_diagnostic(self) -> Diagnostic {
        use miden_diagnostics::Label;

        match self {
            Self::InvalidInt { span, ref reason } => Diagnostic::error()
                .with_message("invalid integer literal")
                .with_labels(vec![Label::primary(span.source_id(), span)
                    .with_message(format!("{}", DisplayIntErrorKind(reason)))]),
            Self::UnexpectedCharacter { start, .. } => {
                Diagnostic::error().with_message("unexpected character").with_labels(vec![
                    Label::primary(start.source_id(), SourceSpan::new(start, start)),
                ])
            }
            Self::UnclosedString { span, .. } => Diagnostic::error()
                .with_message("unclosed string")
                .with_labels(vec![Label::primary(span.source_id(), span)]),
            Self::InvalidModuleIdentifier { span, .. } => Diagnostic::error()
                .with_message("invalid module identifier")
                .with_labels(vec![Label::primary(span.source_id(), span).with_message(
                    "module names must be non-empty, start with 'a-z', and only contain ascii \
                     alpha-numeric characters, '_', or '::' as a namespacing operator",
                )]),
            Self::InvalidFunctionIdentifier { span, .. } => Diagnostic::error()
                .with_message("invalid function identifier")
                .with_labels(vec![Label::primary(span.source_id(), span).with_message(
                    "function names must be non-empty, and start with '_' or 'a-z'",
                )]),
        }
    }
}

struct DisplayIntErrorKind<'a>(&'a IntErrorKind);
impl<'a> fmt::Display for DisplayIntErrorKind<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            IntErrorKind::Empty => write!(f, "unable to parse empty string as integer"),
            IntErrorKind::InvalidDigit => write!(f, "invalid digit"),
            IntErrorKind::PosOverflow => write!(f, "value is too big"),
            IntErrorKind::NegOverflow => write!(f, "value is too big"),
            IntErrorKind::Zero => write!(f, "zero is not a valid value here"),
            other => write!(f, "unable to parse integer value: {:?}", other),
        }
    }
}
