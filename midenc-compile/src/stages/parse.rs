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
    /// We parsed MASM from a Miden Assembly module or other binary format
    Masm(Box<midenc_codegen_masm::Module>),
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
                FileType::Masm => self.parse_masm_from_file(path.as_ref(), session),
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
                FileType::Masm => {
                    self.parse_masm_from_bytes(name.as_str().unwrap(), input, session)
                }
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
        let module = wasm::translate(bytes, config, &session.codemap, &session.diagnostics)?
            .unwrap_one_module();

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
        let module = wasm::translate(&wasm, &config, &session.codemap, &session.diagnostics)?
            .unwrap_one_module();

        Ok(ParseOutput::Hir(module))
    }

    fn parse_hir_from_wat_bytes(
        &self,
        bytes: &[u8],
        session: &Session,
        config: &WasmTranslationConfig,
    ) -> CompilerResult<ParseOutput> {
        let wasm = wat::parse_bytes(bytes)?;
        let module = wasm::translate(&wasm, config, &session.codemap, &session.diagnostics)?
            .unwrap_one_module();

        Ok(ParseOutput::Hir(module))
    }

    fn parse_masm_from_file(&self, path: &Path, session: &Session) -> CompilerResult<ParseOutput> {
        use miden_assembly::{
            ast::{self, Ident, ModuleKind},
            LibraryNamespace, LibraryPath,
        };
        use midenc_codegen_masm as masm;

        // Construct library path for MASM module
        let module_name = match Ident::new(path.file_stem().unwrap().to_str().unwrap()) {
            Ok(id) => id,
            Err(err) => {
                session.diagnostics.error(err);
                return Err(CompilerError::Reported);
            }
        };
        let namespace = path
            .parent()
            .map(|dir| {
                LibraryNamespace::User(dir.to_str().unwrap().to_string().into_boxed_str().into())
            })
            .unwrap_or(LibraryNamespace::Anon);
        let name = LibraryPath::new_from_components(namespace, [module_name]);

        // Make sure sources are in codemap for error reporting
        let source_id = session.codemap.add_file(path).map_err(CompilerError::Io)?;
        let span = session.codemap.source_span(source_id).unwrap();

        // Parse AST, then convert to IR representation
        let ast = ast::Module::parse_file(name, ModuleKind::Library, path)
            .map_err(miden_assembly::diagnostics::RelatedError::new)
            .map_err(CompilerError::Report)?;
        Ok(ParseOutput::Masm(Box::new(masm::Module::from_ast(&ast, span))))
    }

    fn parse_masm_from_bytes(
        &self,
        name: &str,
        bytes: &[u8],
        session: &Session,
    ) -> CompilerResult<ParseOutput> {
        use std::io::{Error, ErrorKind};

        use miden_assembly::{
            ast::{self, ModuleKind},
            LibraryPath,
        };
        use miden_diagnostics::FileName;
        use midenc_codegen_masm as masm;

        // Make sure sources are in codemap for error reporting
        let source = core::str::from_utf8(bytes).map_err(|_| {
            CompilerError::Io(Error::new(ErrorKind::InvalidInput, "input is not valid utf-8"))
        })?;
        let source_id = session
            .codemap
            .add(FileName::Virtual(name.to_string().into()), source.to_string());
        let span = session.codemap.source_span(source_id).unwrap();

        // Construct library path for MASM module
        let name = LibraryPath::new(name).map_err(|err| {
            session.diagnostics.error(err);
            CompilerError::Reported
        })?;

        // Parse AST, then convert to IR representation
        let ast = ast::Module::parse_str(name, ModuleKind::Library, source)
            .map_err(miden_assembly::diagnostics::RelatedError::new)
            .map_err(CompilerError::Report)?;
        Ok(ParseOutput::Masm(Box::new(masm::Module::from_ast(&ast, span))))
    }
}
