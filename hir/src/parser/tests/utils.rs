use std::path::Path;
use std::sync::Arc;

use miden_diagnostics::{Emitter, Verbosity};
use midenc_session::{Options, Warnings};
use pretty_assertions::assert_eq;

use crate::{
    parser::ast::Module,
    parser::{ParseError, Parser},
    testing::TestContext,
};

struct SplitEmitter {
    capture: miden_diagnostics::CaptureEmitter,
    default: miden_diagnostics::DefaultEmitter,
}
impl SplitEmitter {
    #[inline]
    pub fn new() -> Self {
        use miden_diagnostics::term::termcolor::ColorChoice;

        Self {
            capture: Default::default(),
            default: miden_diagnostics::DefaultEmitter::new(ColorChoice::Auto),
        }
    }

    #[allow(unused)]
    pub fn captured(&self) -> String {
        self.capture.captured()
    }
}
impl Emitter for SplitEmitter {
    #[inline]
    fn buffer(&self) -> miden_diagnostics::term::termcolor::Buffer {
        self.capture.buffer()
    }

    #[inline]
    fn print(&self, buffer: miden_diagnostics::term::termcolor::Buffer) -> std::io::Result<()> {
        use std::io::Write;

        let mut copy = self.capture.buffer();
        copy.write_all(buffer.as_slice())?;
        self.capture.print(buffer)?;
        self.default.print(copy)
    }
}

/// [ParseTest] is a container for the data required to run parser tests. Used to build an AST from
/// the given source string and asserts that executing the test will result in the expected AST.
///
/// # Errors:
/// - ScanError test: check that the source provided contains valid characters and keywords.
/// - ParseError test: check that the parsed values are valid.
///   * InvalidInt: This error is returned if the parsed number is not a valid u64.
pub struct ParseTest {
    context: TestContext,
    emitter: Arc<SplitEmitter>,
}

impl ParseTest {
    /// Creates a new test, from the source string.
    pub fn new() -> Self {
        let emitter = Arc::new(SplitEmitter::new());
        let options = Options::new(std::env::current_dir().unwrap())
            .with_verbosity(Verbosity::Warning)
            .with_warnings(Warnings::Error);
        let context = TestContext::default_with_opts_and_emitter(options, Some(emitter.clone()));
        Self { context, emitter }
    }

    /// This adds a new in-memory file to the [CodeMap] for this test.
    ///
    /// This is used when we want to write a test with imports, without having to place files on disk
    #[allow(unused)]
    pub fn add_virtual_file<P: AsRef<Path>>(&self, name: P, content: String) {
        self.context.session.codemap.add(name.as_ref(), content);
    }

    pub fn parse_module_ast_from_file<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<Module, ParseError> {
        let parser = Parser::new(&self.context.session);
        parser.parse_file::<Module>(path)
    }

    pub fn parse_module_ast(&self, source: &str) -> Result<Module, ParseError> {
        let parser = Parser::new(&self.context.session);
        parser.parse_str::<Module>(source)
    }

    pub fn parse_module_from_file<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<crate::Module, ParseError> {
        let parser = Parser::new(&self.context.session);
        parser.parse_file::<crate::Module>(path)
    }

    pub fn parse_module(&self, source: &str) -> Result<crate::Module, ParseError> {
        let parser = Parser::new(&self.context.session);
        parser.parse_str::<crate::Module>(source)
    }

    #[track_caller]
    #[allow(unused)]
    pub fn expect_module_diagnostic(&self, source: &str, expected: &str) {
        if let Err(err) = self.parse_module(source) {
            self.context.session.diagnostics.emit(err);
            assert!(
                self.emitter.captured().contains(expected),
                "expected diagnostic output to contain the string: '{}'",
                expected
            );
        } else {
            panic!("expected parsing to fail, but it succeeded");
        }
    }

    /// Parses a [Module] from the given source string and asserts that executing the test will result
    /// in the expected AST.
    #[track_caller]
    pub fn expect_module(&self, source: &str, expected: &crate::Module) {
        match self.parse_module(source) {
            Err(err) => {
                self.context.session.diagnostics.emit(err);
                panic!("expected parsing to succeed, see diagnostics for details");
            }
            Ok(parsed) => {
                assert_eq!(&parsed, expected);
            }
        }
    }

    /// Parses a [Module] from the given source string and asserts that executing the test will result
    /// in the expected AST.
    #[track_caller]
    #[allow(unused)]
    pub fn expect_module_ast(&self, source: &str, expected: Module) {
        match self.parse_module_ast(source) {
            Err(err) => {
                self.context.session.diagnostics.emit(err);
                panic!("expected parsing to succeed, see diagnostics for details");
            }
            Ok(ast) => assert_eq!(ast, expected),
        }
    }

    /// Parses a [Module] from the given source path and asserts that executing the test will result
    /// in the expected AST.
    #[track_caller]
    pub fn expect_module_ast_from_file(&self, path: &str, expected: Module) {
        match self.parse_module_ast_from_file(path) {
            Err(err) => {
                self.context.session.diagnostics.emit(err);
                panic!("expected parsing to succeed, see diagnostics for details");
            }
            Ok(ast) => assert_eq!(ast, expected),
        }
    }
}
