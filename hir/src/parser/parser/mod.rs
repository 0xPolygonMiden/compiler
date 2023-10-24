// Simple macro used in the grammar definition for constructing spans
macro_rules! span {
    ($l:expr, $r:expr) => {
        miden_diagnostics::SourceSpan::new($l, $r)
    };
    ($i:expr) => {
        miden_diagnostics::SourceSpan::new($i, $i)
    };
}

lalrpop_mod!(
    #[allow(clippy::all)]
    grammar,
    "/parser/parser/grammar.rs"
);

use std::sync::Arc;

use miden_diagnostics::{
    CodeMap, Diagnostic, DiagnosticsHandler, Label, SourceIndex, SourceSpan, ToDiagnostic,
};
use miden_parsing::{Scanner, Source};

use crate::parser::{
    ast,
    lexer::{Lexed, Lexer, LexicalError, Token},
};

pub type Parser = miden_parsing::Parser<()>;

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
            (Self::Failed, Self::Failed) => true,
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
                    .with_labels(vec![Label::primary(
                        at.source_id(),
                        SourceSpan::new(at, at),
                    )
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
                    .with_labels(vec![
                        Label::primary(span.source_id(), span).with_message(message)
                    ])
            }
            Self::ExtraToken { span, .. } => Diagnostic::error()
                .with_message("extraneous token")
                .with_labels(vec![Label::primary(span.source_id(), span)]),
            err => Diagnostic::error().with_message(err.to_string()),
        }
    }
}

impl miden_parsing::Parse for ast::Module {
    type Parser = grammar::ModuleParser;
    type Error = ParseError;
    type Config = ();
    type Token = Lexed;

    fn root_file_error(source: std::io::Error, path: std::path::PathBuf) -> Self::Error {
        ParseError::FileError { source, path }
    }

    fn parse<S>(
        parser: &Parser,
        diagnostics: &DiagnosticsHandler,
        source: S,
    ) -> Result<Self, Self::Error>
    where
        S: Source,
    {
        let scanner = Scanner::new(source);
        let lexer = Lexer::new(scanner);
        Self::parse_tokens(diagnostics, parser.codemap.clone(), lexer)
    }

    fn parse_tokens<S: IntoIterator<Item = Lexed>>(
        diagnostics: &DiagnosticsHandler,
        codemap: Arc<CodeMap>,
        tokens: S,
    ) -> Result<Self, Self::Error> {
        let mut next_var = 0;
        let result = Self::Parser::new().parse(diagnostics, &codemap, &mut next_var, tokens);
        match result {
            Ok(ast) => {
                if diagnostics.has_errors() {
                    return Err(ParseError::Failed);
                }
                Ok(ast)
            }
            Err(lalrpop_util::ParseError::User { error }) => Err(error),
            Err(err) => Err(err.into()),
        }
    }
}

#[cfg(test)]
mod tests;
