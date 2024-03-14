use miden_diagnostics::{Diagnostic, Label, SourceIndex, SourceSpan, ToDiagnostic};

use super::lexer::{LexicalError, Token};

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error(transparent)]
    Lexer(#[from] LexicalError),
    #[error("error reading {path:?}: {source}")]
    FileError {
        source: std::io::Error,
        path: std::path::PathBuf,
    },
    #[error("invalid token")]
    InvalidToken(SourceIndex),
    #[error("unexpected end of file")]
    UnexpectedEof {
        at: SourceIndex,
        expected: Vec<String>,
    },
    #[error("unrecognized token '{token}'")]
    UnrecognizedToken {
        span: SourceSpan,
        token: Token,
        expected: Vec<String>,
    },
    #[error("extraneous token '{token}'")]
    ExtraToken { span: SourceSpan, token: Token },
    #[error("expected valid u32 immediate value, got '{value}'")]
    InvalidU32 { span: SourceSpan, value: isize },
    #[error("expected valid offset value, got '{value}'")]
    InvalidOffset { span: SourceSpan, value: isize },
    #[error("expected valid alignment value, got '{value}'")]
    InvalidAlignment { span: SourceSpan, value: isize },
    #[error("expected valid address space, got '{value}'")]
    InvalidAddrSpace { span: SourceSpan, value: isize },
    #[error("parsing succeeded, but validation failed, see diagnostics for details")]
    InvalidModule,
    #[error("invalid function definition: cannot have empty body")]
    EmptyFunction { span: SourceSpan },
    #[error("invalid function import declaration: cannot have body")]
    ImportedFunctionWithBody { span: SourceSpan },
    #[error("parsing failed, see diagnostics for details")]
    Failed,
}
impl Eq for ParseError {}
impl PartialEq for ParseError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Lexer(l), Self::Lexer(r)) => l == r,
            (Self::FileError { .. }, Self::FileError { .. }) => true,
            (Self::InvalidToken(_), Self::InvalidToken(_)) => true,
            (
                Self::UnexpectedEof {
                    expected: ref l, ..
                },
                Self::UnexpectedEof {
                    expected: ref r, ..
                },
            ) => l == r,
            (
                Self::UnrecognizedToken {
                    token: lt,
                    expected: ref l,
                    ..
                },
                Self::UnrecognizedToken {
                    token: rt,
                    expected: ref r,
                    ..
                },
            ) => lt == rt && l == r,
            (Self::ExtraToken { token: l, .. }, Self::ExtraToken { token: r, .. }) => l == r,
            (Self::InvalidModule, Self::InvalidModule) => true,
            (Self::Failed, Self::Failed) => true,
            (Self::EmptyFunction { .. }, Self::EmptyFunction { .. }) => true,
            (Self::ImportedFunctionWithBody { .. }, Self::ImportedFunctionWithBody { .. }) => true,
            (Self::InvalidU32 { value: l, .. }, Self::InvalidU32 { value: r, .. }) => l == r,
            (Self::InvalidOffset { value: l, .. }, Self::InvalidOffset { value: r, .. }) => l == r,
            (Self::InvalidAlignment { value: l, .. }, Self::InvalidAlignment { value: r, .. }) => {
                l == r
            }
            (Self::InvalidAddrSpace { value: l, .. }, Self::InvalidAddrSpace { value: r, .. }) => {
                l == r
            }
            _ => false,
        }
    }
}
impl From<lalrpop_util::ParseError<SourceIndex, Token, ParseError>> for ParseError {
    fn from(err: lalrpop_util::ParseError<SourceIndex, Token, ParseError>) -> Self {
        use lalrpop_util::ParseError as LError;

        match err {
            LError::InvalidToken { location } => Self::InvalidToken(location),
            LError::UnrecognizedEof {
                location: at,
                expected,
            } => Self::UnexpectedEof { at, expected },
            LError::UnrecognizedToken {
                token: (l, token, r),
                expected,
            } => Self::UnrecognizedToken {
                span: SourceSpan::new(l, r),
                token,
                expected,
            },
            LError::ExtraToken {
                token: (l, token, r),
            } => Self::ExtraToken {
                span: SourceSpan::new(l, r),
                token,
            },
            LError::User { error } => error,
        }
    }
}
impl ToDiagnostic for ParseError {
    fn to_diagnostic(self) -> Diagnostic {
        match self {
            Self::Lexer(err) => err.to_diagnostic(),
            Self::InvalidToken(start) => Diagnostic::error()
                .with_message("invalid token")
                .with_labels(vec![Label::primary(
                    start.source_id(),
                    SourceSpan::new(start, start),
                )]),
            Self::UnexpectedEof { at, ref expected } => {
                let mut message = "expected one of: ".to_string();
                for (i, t) in expected.iter().enumerate() {
                    if i == 0 {
                        message.push_str(&format!("'{}'", t));
                    } else {
                        message.push_str(&format!(", '{}'", t));
                    }
                }

                Diagnostic::error()
                    .with_message("unexpected eof")
                    .with_labels(vec![Label::primary(at.source_id(), SourceSpan::new(at, at))
                        .with_message(message)])
            }
            Self::UnrecognizedToken {
                span, ref expected, ..
            } => {
                let mut message = "expected one of: ".to_string();
                for (i, t) in expected.iter().enumerate() {
                    if i == 0 {
                        message.push_str(&format!("'{}'", t));
                    } else {
                        message.push_str(&format!(", '{}'", t));
                    }
                }

                Diagnostic::error()
                    .with_message("unexpected token")
                    .with_labels(vec![Label::primary(span.source_id(), span).with_message(message)])
            }
            Self::ExtraToken { span, .. } => Diagnostic::error()
                .with_message("extraneous token")
                .with_labels(vec![Label::primary(span.source_id(), span)]),
            Self::InvalidU32 { span, .. } => Diagnostic::error()
                .with_message("expected valid unsigned 32-bit immediate value")
                .with_labels(vec![Label::primary(span.source_id(), span)]),
            Self::InvalidOffset { span, .. } => Diagnostic::error()
                .with_message("expected valid 32-bit offset value")
                .with_labels(vec![Label::primary(span.source_id(), span)]),
            Self::InvalidAlignment { span, .. } => Diagnostic::error()
                .with_message("expected valid alignment value")
                .with_labels(vec![Label::primary(span.source_id(), span).with_message(
                    "alignment must be a non-zero, power of two, valid for a 32-bit address space",
                )]),
            Self::InvalidAddrSpace { span, .. } => Diagnostic::error()
                .with_message("expected valid address space")
                .with_labels(vec![Label::primary(span.source_id(), span)
                    .with_message("address space must be a value in 1..=65535")]),
            Self::EmptyFunction { span } => Diagnostic::error()
                .with_message("invalid function definition")
                .with_labels(vec![Label::primary(span.source_id(), span)
                    .with_message("cannot have an empty body")]),
            Self::ImportedFunctionWithBody { span } => Diagnostic::error()
                .with_message("invalid function import declaration")
                .with_labels(vec![Label::primary(span.source_id(), span)
                    .with_message("function import declarations cannot have a body")]),
            err => Diagnostic::error().with_message(err.to_string()),
        }
    }
}
