use super::lexer::{LexicalError, Token};
use crate::{
    diagnostics::{miette, ByteIndex, Diagnostic, SourceSpan},
    DisplayValues,
};

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum ParseError {
    #[diagnostic(transparent)]
    #[error(transparent)]
    Lexer(#[from] LexicalError),
    #[error("error reading {path:?}: {source}")]
    #[diagnostic()]
    FileError {
        #[source]
        source: std::io::Error,
        path: std::path::PathBuf,
    },
    #[error("invalid token")]
    #[diagnostic()]
    InvalidToken(#[label] SourceSpan),
    #[error("unexpected end of file")]
    #[diagnostic()]
    UnexpectedEof {
        #[label("expected one of: {}", DisplayValues::new(expected.iter()))]
        at: SourceSpan,
        expected: Vec<String>,
    },
    #[error("unrecognized token '{token}'")]
    UnrecognizedToken {
        #[label("expected one of: {}", DisplayValues::new(expected.iter()))]
        span: SourceSpan,
        token: Token,
        expected: Vec<String>,
    },
    #[error("extraneous token '{token}'")]
    ExtraToken {
        #[label]
        span: SourceSpan,
        token: Token,
    },
    #[error("expected valid u32 immediate value, got '{value}'")]
    InvalidU32 {
        #[label]
        span: SourceSpan,
        value: isize,
    },
    #[error("expected valid u16 value, got '{value}'")]
    InvalidU16 {
        #[label]
        span: SourceSpan,
        value: isize,
    },
    #[error("expected valid offset value, got '{value}'")]
    InvalidOffset {
        #[label]
        span: SourceSpan,
        value: isize,
    },
    #[error("expected valid alignment value, got '{value}'")]
    InvalidAlignment {
        #[label]
        span: SourceSpan,
        value: isize,
    },
    #[error("expected valid address space, got '{value}'")]
    InvalidAddrSpace {
        #[label]
        span: SourceSpan,
        value: isize,
    },
    #[error("invalid function definition: cannot have empty body")]
    EmptyFunction {
        #[label]
        span: SourceSpan,
    },
    #[error("invalid function import declaration: cannot have body")]
    ImportedFunctionWithBody {
        #[label]
        span: SourceSpan,
    },
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
            (Self::EmptyFunction { .. }, Self::EmptyFunction { .. }) => true,
            (Self::ImportedFunctionWithBody { .. }, Self::ImportedFunctionWithBody { .. }) => true,
            (Self::InvalidU32 { value: l, .. }, Self::InvalidU32 { value: r, .. }) => l == r,
            (Self::InvalidU16 { value: l, .. }, Self::InvalidU16 { value: r, .. }) => l == r,
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
impl From<lalrpop_util::ParseError<ByteIndex, Token, ParseError>> for ParseError {
    fn from(err: lalrpop_util::ParseError<ByteIndex, Token, ParseError>) -> Self {
        use lalrpop_util::ParseError as LError;

        match err {
            LError::InvalidToken { location } => {
                Self::InvalidToken(SourceSpan::at(Default::default(), location))
            }
            LError::UnrecognizedEof {
                location: at,
                expected,
            } => Self::UnexpectedEof {
                at: SourceSpan::at(Default::default(), at),
                expected,
            },
            LError::UnrecognizedToken {
                token: (l, token, r),
                expected,
            } => Self::UnrecognizedToken {
                span: SourceSpan::new(Default::default(), l..r),
                token,
                expected,
            },
            LError::ExtraToken {
                token: (l, token, r),
            } => Self::ExtraToken {
                span: SourceSpan::new(Default::default(), l..r),
                token,
            },
            LError::User { error } => error,
        }
    }
}
