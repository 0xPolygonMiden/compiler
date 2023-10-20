pub mod ast;
mod lexer;
mod parser;

pub use self::parser::{ParseError, Parser};
use super::Symbol;

use std::path::Path;
use std::sync::Arc;

use miden_diagnostics::{CodeMap, DiagnosticsHandler};

/// Parses the provided source and returns the AST.
pub fn parse(
    diagnostics: &DiagnosticsHandler,
    codemap: Arc<CodeMap>,
    source: &str,
) -> Result<ast::Module, ParseError> {
    let parser = Parser::new((), codemap);
    match parser.parse_string::<ast::Module, _, _>(diagnostics, source) {
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
) -> Result<ast::Module, ParseError> {
    let parser = Parser::new((), codemap);
    match parser.parse_file::<ast::Module, _, _>(diagnostics, source) {
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
pub fn parse_str(source: &str) -> Result<ast::Module, ParseError> {
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

/// Parses a [Module] from the given path.
///
/// This is primarily intended for use in the import resolution phase.
pub(crate) fn parse_module_from_file<P: AsRef<Path>>(
    diagnostics: &DiagnosticsHandler,
    codemap: Arc<CodeMap>,
    path: P,
) -> Result<ast::Module, ParseError> {
    let parser = Parser::new((), codemap);
    match parser.parse_file::<ast::Module, _, _>(diagnostics, path) {
        ok @ Ok(_) => ok,
        Err(ParseError::Lexer(err)) => {
            diagnostics.emit(err);
            Err(ParseError::Failed)
        }
        err @ Err(_) => err,
    }
}

/// Parses a [Module] from a file already in the codemap
///
/// This is primarily intended for use in the import resolution phase.
pub(crate) fn parse_module(
    diagnostics: &DiagnosticsHandler,
    codemap: Arc<CodeMap>,
    source: Arc<miden_diagnostics::SourceFile>,
) -> Result<ast::Module, ParseError> {
    let parser = Parser::new((), codemap);
    match parser.parse::<ast::Module, _>(diagnostics, source) {
        ok @ Ok(_) => ok,
        Err(ParseError::Lexer(err)) => {
            diagnostics.emit(err);
            Err(ParseError::Failed)
        }
        err @ Err(_) => err,
    }
}
