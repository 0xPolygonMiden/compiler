use std::path::Path;

use midenc_session::InputFile;
use wasm::WasmTranslationConfig;

use super::*;

/// This represents the output of the parser, depending on the type
/// of input that was parsed/loaded.
pub enum ParseOutput {
    /// We parsed HIR into the AST from text
    Ast(Box<ast::Module>),
    /// We parsed HIR from a Wasm module or other binary format
    Hir(Box<hir::Module>),
}

/// This stage of compilation is where we parse input files into the
/// earliest representation supported by the input file type. Later
/// stages will handle lowering as needed.
pub struct ParseStage;
impl Stage for ParseStage {
    type Input = InputFile;
    type Output = ParseOutput;

    fn run(
        &mut self,
        input: Self::Input,
        _analyses: &mut AnalysisManager,
        session: &Session,
    ) -> CompilerResult<Self::Output> {
        use midenc_session::{FileType, InputType};
        // Track when compilation began
        let file_type = input.file_type();
        match &input.file {
            InputType::Real(ref path) => match file_type {
                FileType::Hir => self.parse_ast_from_file(path.as_ref(), session),
                FileType::Wasm => self.parse_hir_from_wasm_file(path.as_ref(), session),
                FileType::Wat => self.parse_hir_from_wat_file(path.as_ref(), session),
                unsupported => unreachable!("unsupported file type: {unsupported}"),
            },
            InputType::Stdin { name, ref input } => match file_type {
                FileType::Hir => self.parse_ast_from_bytes(input, session),
                FileType::Wasm => self.parse_hir_from_wasm_bytes(
                    input,
                    session,
                    &WasmTranslationConfig {
                        source_name: name.as_str().unwrap().to_string().into(),
                        ..Default::default()
                    },
                ),
                FileType::Wat => self.parse_hir_from_wat_bytes(
                    input,
                    session,
                    &WasmTranslationConfig {
                        source_name: name.as_str().unwrap().to_string().into(),
                        ..Default::default()
                    },
                ),
                unsupported => unreachable!("unsupported file type: {unsupported}"),
            },
        }
    }
}
impl ParseStage {
    fn parse_ast_from_file(&self, path: &Path, session: &Session) -> CompilerResult<ParseOutput> {
        use std::io::Read;

        let mut file = std::fs::File::open(path)?;
        let mut bytes = Vec::with_capacity(1024);
        file.read_to_end(&mut bytes)?;
        self.parse_ast_from_bytes(&bytes, session)
    }

    fn parse_ast_from_bytes(&self, bytes: &[u8], session: &Session) -> CompilerResult<ParseOutput> {
        use std::io::{Error, ErrorKind};

        use midenc_hir::parser::Parser;

        let source = core::str::from_utf8(bytes).map_err(|_| {
            CompilerError::Io(Error::new(ErrorKind::InvalidInput, "input is not valid utf-8"))
        })?;
        let parser = Parser::new(session);
        match parser.parse_str(source).map(Box::new) {
            Ok(ast) => {
                session.emit(&ast)?;
                Ok(ParseOutput::Ast(ast))
            }
            Err(err) => {
                session.diagnostics.emit(err);
                Err(CompilerError::Reported)
            }
        }
    }

    fn parse_hir_from_wasm_file(
        &self,
        path: &Path,
        session: &Session,
    ) -> CompilerResult<ParseOutput> {
        use std::io::Read;

        let mut file = std::fs::File::open(path)?;
        let mut bytes = Vec::with_capacity(1024);
        file.read_to_end(&mut bytes)?;
        let file_name = path.file_stem().unwrap().to_str().unwrap().to_owned();
        let config = wasm::WasmTranslationConfig {
            source_name: file_name.into(),
            ..Default::default()
        };
        self.parse_hir_from_wasm_bytes(&bytes, session, &config)
    }

    fn parse_hir_from_wasm_bytes(
        &self,
        bytes: &[u8],
        session: &Session,
        config: &WasmTranslationConfig,
    ) -> CompilerResult<ParseOutput> {
        let module = wasm::translate(bytes, config, &session.diagnostics)?.unwrap_one_module();

        Ok(ParseOutput::Hir(module))
    }

    fn parse_hir_from_wat_file(
        &self,
        path: &Path,
        session: &Session,
    ) -> CompilerResult<ParseOutput> {
        let file_name = path.file_stem().unwrap().to_str().unwrap().to_owned();
        let config = WasmTranslationConfig {
            source_name: file_name.into(),
            ..Default::default()
        };
        let wasm = wat::parse_file(path)?;
        let module = wasm::translate(&wasm, &config, &session.diagnostics)?.unwrap_one_module();

        Ok(ParseOutput::Hir(module))
    }

    fn parse_hir_from_wat_bytes(
        &self,
        bytes: &[u8],
        session: &Session,
        config: &WasmTranslationConfig,
    ) -> CompilerResult<ParseOutput> {
        let wasm = wat::parse_bytes(bytes)?;
        let module = wasm::translate(&wasm, config, &session.diagnostics)?.unwrap_one_module();

        Ok(ParseOutput::Hir(module))
    }
}
