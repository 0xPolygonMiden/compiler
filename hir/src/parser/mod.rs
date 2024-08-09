/// Simple macro used in the grammar definition for constructing spans
macro_rules! span {
    ($source_id:expr, $l:expr, $r:expr) => {
        SourceSpan::new($source_id, $l..$r)
    };
    ($source_id:expr, $i:expr) => {
        SourceSpan::at($source_id, $i)
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

use alloc::sync::Arc;
use std::path::Path;

pub use self::error::ParseError;
use self::{
    ast::ConvertAstToHir,
    lexer::{Lexed, Lexer},
};
use crate::diagnostics::{Report, SourceFile, SourceManagerExt};

pub type ParseResult<T> = Result<T, Report>;

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
        <T as Parse>::parse(self, source)
    }

    /// Parse a `T` from a string
    pub fn parse_str<T>(&self, source: impl AsRef<str>) -> ParseResult<T>
    where
        T: Parse,
    {
        let file = self.session.source_manager.load("nofile", source.as_ref().to_string());
        self.parse(file)
    }

    /// Parse a `T` from the given file path
    pub fn parse_file<T>(&self, path: impl AsRef<Path>) -> ParseResult<T>
    where
        T: Parse,
    {
        let path = path.as_ref();
        let file = self.session.source_manager.load_file(path).map_err(|err| {
            Report::msg(err).wrap_err(format!("failed to load '{}' from disk", path.display()))
        })?;
        self.parse(file)
    }
}

pub trait Parse: Sized {
    type Grammar;

    fn parse(parser: &Parser, source: Arc<SourceFile>) -> ParseResult<Self> {
        let lexer = Lexer::new(source.id(), source.as_str());

        Self::parse_tokens(parser, source.clone(), lexer)
    }

    fn parse_tokens(
        parser: &Parser,
        source: Arc<SourceFile>,
        tokens: impl IntoIterator<Item = Lexed>,
    ) -> ParseResult<Self>;
}
impl Parse for ast::Module {
    type Grammar = grammar::ModuleParser;

    fn parse_tokens(
        _parser: &Parser,
        source: Arc<SourceFile>,
        tokens: impl IntoIterator<Item = Lexed>,
    ) -> ParseResult<Self> {
        let source_id = source.id();
        let mut next_var = 0;
        let result = <Self as Parse>::Grammar::new().parse(source_id, &mut next_var, tokens);
        match result {
            Ok(ast) => Ok(ast),
            Err(lalrpop_util::ParseError::User { error }) => {
                Err(Report::from(error).with_source_code(source))
            }
            Err(err) => {
                let error = ParseError::from(err);
                Err(Report::from(error).with_source_code(source))
            }
        }
    }
}
impl Parse for crate::Module {
    type Grammar = grammar::ModuleParser;

    fn parse_tokens(
        parser: &Parser,
        source: Arc<SourceFile>,
        tokens: impl IntoIterator<Item = Lexed>,
    ) -> ParseResult<Self> {
        use crate::pass::{AnalysisManager, ConversionPass};

        let source_id = source.id();
        let mut next_var = 0;
        let result = <Self as Parse>::Grammar::new()
            .parse(source_id, &mut next_var, tokens)
            .map(Box::new);
        match result {
            Ok(ast) => {
                let mut analyses = AnalysisManager::new();
                let mut convert_to_hir = ConvertAstToHir;
                convert_to_hir
                    .convert(ast, &mut analyses, parser.session)
                    .map_err(|err| err.with_source_code(source))
            }
            Err(lalrpop_util::ParseError::User { error }) => {
                Err(Report::from(error).with_source_code(source))
            }
            Err(err) => {
                let error = ParseError::from(err);
                Err(Report::from(error).with_source_code(source))
            }
        }
    }
}
