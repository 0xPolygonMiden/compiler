use std::{path::Path, sync::Arc};

use midenc_session::{Options, Verbosity, Warnings};
use pretty_assertions::assert_eq;

use crate::{
    diagnostics::{self, Buffer, CaptureEmitter, DefaultEmitter, Emitter, IntoDiagnostic, Report},
    parser::{ast::Module, Parser},
    testing::TestContext,
};

struct SplitEmitter {
    capture: CaptureEmitter,
    default: DefaultEmitter,
}
impl SplitEmitter {
    #[inline]
    pub fn new() -> Self {
        use midenc_session::ColorChoice;

        Self {
            capture: Default::default(),
            default: DefaultEmitter::new(ColorChoice::Auto),
        }
    }

    #[allow(unused)]
    pub fn captured(&self) -> String {
        self.capture.captured()
    }
}
impl Emitter for SplitEmitter {
    #[inline]
    fn buffer(&self) -> Buffer {
        self.capture.buffer()
    }

    #[inline]
    fn print(&self, buffer: Buffer) -> Result<(), Report> {
        use std::io::Write;

        let mut copy = self.capture.buffer();
        copy.write_all(buffer.as_slice()).into_diagnostic()?;
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
        use midenc_session::{ProjectType, TargetEnv};

        let emitter = Arc::new(SplitEmitter::new());
        let options = Options::new(
            None,
            TargetEnv::Base,
            ProjectType::Library,
            std::env::current_dir().unwrap(),
            None,
        )
        .with_verbosity(Verbosity::Warning)
        .with_warnings(Warnings::Error);
        let context = TestContext::default_with_opts_and_emitter(options, Some(emitter.clone()));
        Self { context, emitter }
    }

    /// This adds a new in-memory file to the [CodeMap] for this test.
    ///
    /// This is used when we want to write a test with imports, without having to place files on
    /// disk
    #[allow(unused)]
    pub fn add_virtual_file<P: AsRef<Path>>(&self, name: P, content: String) {
        use diagnostics::SourceManager;

        let name = name.as_ref().to_str().unwrap();
        self.context.session.source_manager.load(name, content);
    }

    pub fn parse_module_ast_from_file<P: AsRef<Path>>(&self, path: P) -> Result<Module, Report> {
        let parser = Parser::new(&self.context.session);
        parser.parse_file::<Module>(path)
    }

    pub fn parse_module_ast(&self, source: &str) -> Result<Module, Report> {
        let parser = Parser::new(&self.context.session);
        parser.parse_str::<Module>(source)
    }

    pub fn parse_module_from_file<P: AsRef<Path>>(&self, path: P) -> Result<crate::Module, Report> {
        let parser = Parser::new(&self.context.session);
        parser.parse_file::<crate::Module>(path)
    }

    pub fn parse_module(&self, source: &str) -> Result<crate::Module, Report> {
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

    /// Parses a [Module] from the given source string and asserts that executing the test will
    /// result in the expected IR.
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

    /// Parses a [Module] from the given source string and asserts that executing the test will
    /// result in the expected AST.
    #[track_caller]
    #[allow(unused)]
    pub fn expect_module_ast(&self, source: &str, expected: &Module) {
        match self.parse_module_ast(source) {
            Err(err) => {
                self.context.session.diagnostics.emit(err);
                panic!("expected parsing to succeed, see diagnostics for details");
            }
            Ok(ref ast) => assert_eq!(ast, expected),
        }
    }

    /// Parses a [Module] from the given source path and asserts that executing the test will result
    /// in the expected AST.
    #[track_caller]
    pub fn expect_module_ast_from_file(&self, path: &str, expected: &Module) {
        match self.parse_module_ast_from_file(path) {
            Err(err) => {
                self.context.session.diagnostics.emit(err);
                panic!("expected parsing to succeed, see diagnostics for details");
            }
            Ok(ref ast) => assert_eq!(ast, expected),
        }
    }
}
