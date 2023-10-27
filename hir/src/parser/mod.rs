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

use std::path::Path;
use std::sync::Arc;

use miden_diagnostics::{CodeMap, DiagnosticsHandler};
use miden_parsing::{Scanner, Source};

pub use self::error::ParseError;
use self::lexer::{Lexed, Lexer};

pub type Parser = miden_parsing::Parser<()>;

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

impl miden_parsing::Parse for crate::Module {
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
                ast.try_into_hir(diagnostics)
            }
            Err(lalrpop_util::ParseError::User { error }) => Err(error),
            Err(err) => Err(err.into()),
        }
    }
}

/// Parses the provided source and returns the AST.
pub fn parse(
    diagnostics: &DiagnosticsHandler,
    codemap: Arc<CodeMap>,
    source: &str,
) -> Result<crate::Module, ParseError> {
    let parser = Parser::new((), codemap);
    match parser.parse_string::<crate::Module, _, _>(diagnostics, source) {
        Ok(ast) => Ok(ast),
        Err(ParseError::Lexer(err)) => {
            diagnostics.emit(err);
            Err(ParseError::Failed)
        }
        Err(err) => Err(err),
    }
}

/// Parses the provided source and returns the AST.
pub fn parse_file<P: AsRef<Path>>(
    diagnostics: &DiagnosticsHandler,
    codemap: Arc<CodeMap>,
    source: P,
) -> Result<crate::Module, ParseError> {
    let parser = Parser::new((), codemap);
    match parser.parse_file::<crate::Module, _, _>(diagnostics, source) {
        Ok(ast) => Ok(ast),
        Err(ParseError::Lexer(err)) => {
            diagnostics.emit(err);
            Err(ParseError::Failed)
        }
        Err(err) => Err(err),
    }
}

/// Parses the provided source string with a default [CodeMap] and [DiagnosticsHandler].
///
/// This is primarily provided for use in tests, you should generally prefer [parse]
pub fn parse_str(source: &str) -> Result<crate::Module, ParseError> {
    use miden_diagnostics::{
        term::termcolor::ColorChoice, DefaultEmitter, DiagnosticsConfig, Verbosity,
    };

    let codemap = Arc::new(CodeMap::new());
    let emitter = Arc::new(DefaultEmitter::new(ColorChoice::Auto));
    let config = DiagnosticsConfig {
        verbosity: Verbosity::Warning,
        warnings_as_errors: true,
        no_warn: false,
        display: Default::default(),
    };
    let diagnostics = DiagnosticsHandler::new(config, codemap.clone(), emitter);
    parse(&diagnostics, codemap, source)
}
