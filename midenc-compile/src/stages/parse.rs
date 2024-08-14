use std::path::Path;

use midenc_session::{
    diagnostics::{IntoDiagnostic, Spanned, WrapErr},
    InputFile,
};
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
                FileType::Mast => Err(Report::msg(
                    "invalid input: mast libraries are not supported as inputs, did you mean to \
                     use '-l'?",
                )),
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
                FileType::Mast => Err(Report::msg(
                    "invalid input: mast libraries are not supported as inputs, did you mean to \
                     use '-l'?",
                )),
            },
        }
    }
}
impl ParseStage {
    fn parse_ast_from_file(&self, path: &Path, session: &Session) -> CompilerResult<ParseOutput> {
        use std::io::Read;

        let mut file = std::fs::File::open(path).into_diagnostic()?;
        let mut bytes = Vec::with_capacity(1024);
        file.read_to_end(&mut bytes).into_diagnostic()?;
        self.parse_ast_from_bytes(&bytes, session)
    }

    fn parse_ast_from_bytes(&self, bytes: &[u8], session: &Session) -> CompilerResult<ParseOutput> {
        use midenc_hir::parser::Parser;

        let source = core::str::from_utf8(bytes)
            .into_diagnostic()
            .wrap_err("input is not valid utf-8")?;
        let parser = Parser::new(session);
        parser.parse_str(source).map(Box::new).map(ParseOutput::Ast)
    }

    fn parse_hir_from_wasm_file(
        &self,
        path: &Path,
        session: &Session,
    ) -> CompilerResult<ParseOutput> {
        use std::io::Read;

        let mut file = std::fs::File::open(path)
            .into_diagnostic()
            .wrap_err("could not open input for reading")?;
        let mut bytes = Vec::with_capacity(1024);
        file.read_to_end(&mut bytes).into_diagnostic()?;
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
        let module = wasm::translate(bytes, config, session)?.unwrap_one_module();

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
        let wasm = wat::parse_file(path).into_diagnostic().wrap_err("failed to parse wat")?;
        let module = wasm::translate(&wasm, &config, session)?.unwrap_one_module();

        Ok(ParseOutput::Hir(module))
    }

    fn parse_hir_from_wat_bytes(
        &self,
        bytes: &[u8],
        session: &Session,
        config: &WasmTranslationConfig,
    ) -> CompilerResult<ParseOutput> {
        let wasm = wat::parse_bytes(bytes).into_diagnostic().wrap_err("failed to parse wat")?;
        let module = wasm::translate(&wasm, config, session)?.unwrap_one_module();

        Ok(ParseOutput::Hir(module))
    }

    fn parse_masm_from_file(&self, path: &Path, session: &Session) -> CompilerResult<ParseOutput> {
        use miden_assembly::{
            ast::{self, Ident, ModuleKind},
            LibraryNamespace, LibraryPath,
        };
        use midenc_codegen_masm as masm;

        // Construct library path for MASM module
        let module_name = Ident::new(path.file_stem().unwrap().to_str().unwrap())
            .into_diagnostic()
            .wrap_err_with(|| {
                format!(
                    "failed to construct valid module identifier from path '{}'",
                    path.display()
                )
            })?;
        let namespace = path
            .parent()
            .map(|dir| {
                LibraryNamespace::User(dir.to_str().unwrap().to_string().into_boxed_str().into())
            })
            .unwrap_or(LibraryNamespace::Anon);
        let name = LibraryPath::new_from_components(namespace, [module_name]);

        // Parse AST
        let mut parser = ast::Module::parser(ModuleKind::Library);
        let ast = parser.parse_file(name, path, &session.source_manager)?;
        let span = ast.span();

        // Convert to MASM IR representation
        Ok(ParseOutput::Masm(Box::new(masm::Module::from_ast(&ast, span))))
    }

    fn parse_masm_from_bytes(
        &self,
        name: &str,
        bytes: &[u8],
        session: &Session,
    ) -> CompilerResult<ParseOutput> {
        use miden_assembly::{
            ast::{self, ModuleKind},
            LibraryPath,
        };
        use midenc_codegen_masm as masm;

        let source = core::str::from_utf8(bytes)
            .into_diagnostic()
            .wrap_err_with(|| format!("input '{name}' contains invalid utf-8"))?;

        // Construct library path for MASM module
        let name = LibraryPath::new(name).into_diagnostic()?;

        // Parse AST
        let mut parser = ast::Module::parser(ModuleKind::Library);
        let ast = parser.parse_str(name, source, &session.source_manager)?;
        let span = ast.span();

        // Convert to MASM IR representation
        Ok(ParseOutput::Masm(Box::new(masm::Module::from_ast(&ast, span))))
    }
}
