/// Simple macro used in the grammar definition for constructing spans
macro_rules! span {
    ($l:expr, $r:expr) => {
        miden_diagnostics::SourceSpan::new($l, $r)
    };
    ($i:expr) => {
        miden_diagnostics::SourceSpan::new($i, $i)
    };
}

pub mod ast;
mod error;
mod lexer;
#[cfg(test)]
mod tests;

lalrpop_mod!(
    #[allow(clippy::all)]
    grammar,
    "/parser/grammar.rs"
);

use std::{path::Path, sync::Arc};

use miden_diagnostics::SourceFile;
use miden_parsing::{FileMapSource, Scanner, Source};

pub use self::error::ParseError;
use self::{
    ast::ConvertAstToHir,
    lexer::{Lexed, Lexer},
};

pub type ParseResult<T> = Result<T, ParseError>;

/// This is the parser for HIR text format
pub struct Parser<'a> {
    session: &'a midenc_session::Session,
}
impl<'a> Parser<'a> {
    /// Construct a new [Parser]
    pub fn new(session: &'a midenc_session::Session) -> Self {
        Self { session }
    }

    /// Parse a `T` from a source file stored in the current code map
    pub fn parse<T>(&self, source: Arc<SourceFile>) -> ParseResult<T>
    where
        T: Parse,
    {
        <T as Parse>::parse(self, FileMapSource::new(source))
    }

    /// Parse a `T` from a string
    pub fn parse_str<T>(&self, source: impl AsRef<str>) -> ParseResult<T>
    where
        T: Parse,
    {
        let id = self.session.codemap.add("nofile", source.as_ref().to_string());
        let file = self.session.codemap.get(id).unwrap();
        self.parse(file)
    }

    /// Parse a `T` from the given file path
    pub fn parse_file<T>(&self, path: impl AsRef<Path>) -> ParseResult<T>
    where
        T: Parse,
    {
        let path = path.as_ref();
        let id = self
            .session
            .codemap
            .add_file(path)
            .map_err(|err| parse_file_error(err, path.to_owned()))?;
        let file = self.session.codemap.get(id).unwrap();
        self.parse(file)
    }
}

pub trait Parse: Sized {
    type Grammar;

    fn parse(parser: &Parser, source: impl Source) -> ParseResult<Self> {
        let scanner = Scanner::new(source);
        let lexer = Lexer::new(scanner);

        Self::parse_tokens(parser, lexer)
    }

    fn parse_tokens(parser: &Parser, tokens: impl IntoIterator<Item = Lexed>) -> ParseResult<Self>;
}
impl Parse for ast::Module {
    type Grammar = grammar::ModuleParser;

    fn parse_tokens(parser: &Parser, tokens: impl IntoIterator<Item = Lexed>) -> ParseResult<Self> {
        let mut next_var = 0;
        let result = <Self as Parse>::Grammar::new().parse(
            &parser.session.diagnostics,
            &parser.session.codemap,
            &mut next_var,
            tokens,
        );
        match result {
            Ok(ast) => {
                if parser.session.diagnostics.has_errors() {
                    return Err(ParseError::Failed);
                }
                Ok(ast)
            }
            Err(lalrpop_util::ParseError::User { error }) => Err(error),
            Err(err) => Err(err.into()),
        }
    }
}
impl Parse for crate::Module {
    type Grammar = grammar::ModuleParser;

    fn parse_tokens(parser: &Parser, tokens: impl IntoIterator<Item = Lexed>) -> ParseResult<Self> {
        use crate::pass::{AnalysisManager, ConversionError, ConversionPass};

        let mut next_var = 0;
        let result = <Self as Parse>::Grammar::new()
            .parse(&parser.session.diagnostics, &parser.session.codemap, &mut next_var, tokens)
            .map(Box::new);
        match result {
            Ok(ast) => {
                if parser.session.diagnostics.has_errors() {
                    return Err(ParseError::Failed);
                }
                let mut analyses = AnalysisManager::new();
                let mut convert_to_hir = ConvertAstToHir;
                convert_to_hir.convert(ast, &mut analyses, parser.session).map_err(
                    |err| match err {
                        ConversionError::Failed(err) => match err.downcast::<ParseError>() {
                            Ok(err) => err,
                            Err(_) => ParseError::InvalidModule,
                        },
                        _ => ParseError::InvalidModule,
                    },
                )
            }
            Err(lalrpop_util::ParseError::User { error }) => Err(error),
            Err(err) => Err(err.into()),
        }
    }
}

#[inline]
fn parse_file_error(source: std::io::Error, path: std::path::PathBuf) -> ParseError {
    ParseError::FileError { source, path }
}
