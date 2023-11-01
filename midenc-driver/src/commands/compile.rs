use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use miden_codegen_masm as masm;
use midenc_session::{FileType, InputType, Session};

use crate::DriverError;

/// Run the compiler with the given [Session].
pub fn compile(session: Arc<Session>) -> Result<(), DriverError> {
    // Track when compilation began
    let file_type = session.input.file_type();
    match &session.input.file {
        InputType::Real(ref path) => match file_type {
            FileType::Hir => compile_hir_from_file(path.as_ref(), &session)?,
            FileType::Wasm => compile_wasm_from_file(path.as_ref(), &session)?,
            unsupported => unreachable!("unsupported file type: {}"),
        },
        InputType::Stdin { ref content } => match file_type {
            FileType::Hir => compile_hir_from_bytes(&content, &session)?,
            FileType::Wasm => compile_wasm_from_bytes(&content, &session)?,
            unsupported => unreachable!("unsupported file type: {}"),
        },
    }

    let duration = session.elapsed();
    diagnostics.success(
        "Finished",
        &format!("built {} in {:#}", &session.options.name, duration),
    );

    Ok(())
}

fn compile_hir_from_file(path: &Path, session: &Session) -> Result<(), DriverError> {
    let mut file = std::fs::File::open(path)?;
    let mut bytes = Vec::with_capacity(1024);
    file.read_to_end(&mut bytes)?;
    compile_hir_from_bytes(&bytes, session)
}

fn compile_hir_from_bytes(bytes: &[u8], session: &Session) -> Result<(), DriverError> {
    use miden_codegen_masm::MasmCompiler;
    use miden_hir::parser;

    let mut source = core::str::from_utf8(bytes).map_err(|_| {
        DriverError::InvalidInput(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "input is not valid utf-8",
        ))
    })?;
    let ast = parser::parse_ast(&session.diagnostics, session.codemap.clone(), source)?;
    session.emit(&ast)?;

    // If we're only parsing input, then we're done
    if session.parse_only() {
        return Ok(());
    }

    // Get the HIR module
    let module = Box::new(ast.try_into_hir(&session.diagnostics)?);
    if session.options.print_ir_after_all {
        session.emit(&module)?;
    }

    // If we need to link a program, then compile using the MasmCompiler
    if session.should_link() {
        let mut masm_compiler = MasmCompiler::new(session);
        let _masm_program = masm_compiler.compile_module(module)?;
    }

    Ok(())
}

fn compile_wasm_from_file(path: &Path, session: &Session) -> Result<(), DriverError> {
    let mut file = std::fs::File::open(path)?;
    let mut bytes = Vec::with_capacity(1024);
    file.read_to_end(&mut bytes)?;
    compile_wasm_from_bytes(&bytes, session)
}

fn compile_wasm_from_bytes(bytes: &[u8], session: &Session) -> Result<(), DriverError> {
    use miden_codegen_masm::MasmCompiler;
    use miden_frontend_wasm::{self as wasm, WasmTranslationConfig};

    let config = WasmTranslationConfig::default();
    let module = wasm::translate_module(bytes, &config, &diagnostics)?;

    // If we're only parsing input, then we're done
    if session.parse_only() {
        return Ok(());
    }

    // Emit hir artifact
    session.emit(&module)?;

    // If we need to link a program, then compile using the MasmCompiler
    if session.should_link() {
        let mut masm_compiler = MasmCompiler::new(session);
        let _masm_program = masm_compiler.compile_module(Box::new(module))?;
    }

    Ok(())
}
