use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use miden_codegen_masm as masm;
use miden_diagnostics::{CodeMap, DiagnosticsHandler, Emitter};
use midenc_session::{FileType, HumanDuration, Input, InvalidFileTypeError, Options, OutputTypes};

use crate::DriverError;

/// Run the compiler with the given [Options].
///
/// If the current compilation options do not result in compiling to Miden Assembly,
/// successful compilation will return `Ok(None)`, otherwise it returns the compiled
/// [miden_codegen_masm::Program] which can be used to run the program via the emulator
/// or Miden VM.
///
/// This will automatically configure a [miden_diagnostics::CodeMap] and diagnostics
/// configuration, based on the provided options.
///
/// See [compile_with_opts] or [compile_with_diagnostics] if you'd like to control
/// some or all of how those items are set up and configured.
pub fn compile(options: Arc<Options>) -> Result<Option<Box<masm::Program>>, DriverError> {
    let codemap = Arc::new(CodeMap::new());
    compile_with_opts(options, codemap, None)
}

/// Run the compiler with the given [Options] and [miden_diagnostics::CodeMap].
///
/// If the current compilation options do not result in compiling to Miden Assembly,
/// successful compilation will return `Ok(None)`, otherwise it returns the compiled
/// [miden_codegen_masm::Program] which can be used to run the program via the emulator
/// or Miden VM.
///
/// This function allows you to provide a custom [miden_diagnostics::Emitter] to
/// use when constructing the [miden_diagnostics::DiagnosticsHandler] from the
/// provided options.
///
/// See [compile_with_diagnostics] if you'd like full control over how the
/// [miden_diagnostics::DiagnosticsHandler] is constructed, or if you already
/// have one that you'd like to use.
pub fn compile_with_opts(
    options: Arc<Options>,
    codemap: Arc<CodeMap>,
    emitter: Option<Arc<dyn Emitter>>,
) -> Result<Option<Box<masm::Program>>, DriverError> {
    let diagnostics = Arc::new(DiagnosticsHandler::new(
        options.diagnostics.clone(),
        codemap.clone(),
        emitter.unwrap_or_else(|| options.default_emitter()),
    ));
    compile_with_diagnostics(options, codemap, &diagnostics)
}

/// Run the compiler with the provided [Options], [miden_diagnostics::CodeMap],
/// and [miden_diagnostics::DiagnosticsHandler].
///
/// If the current compilation options do not result in compiling to Miden Assembly,
/// successful compilation will return `Ok(None)`, otherwise it returns the compiled
/// [miden_codegen_masm::Program] which can be used to run the program via the emulator
/// or Miden VM.
pub fn compile_with_diagnostics(
    options: Arc<Options>,
    codemap: Arc<CodeMap>,
    diagnostics: &DiagnosticsHandler,
) -> Result<Option<Box<masm::Program>>, DriverError> {
    // Track when compilation began
    let start = Instant::now();
    let name = options.name.clone();

    let program = match options.inputs.len() {
        0 => return Err(DriverError::NoInputs),
        1 => match &options.inputs[0] {
            Input::File(ref path) => match FileType::try_from(path.as_ref())? {
                FileType::Hir => compile_hir(options, codemap, diagnostics)?,
                FileType::Wasm => {
                    compile_wasm_from_file(path.as_ref(), options.clone(), codemap, diagnostics)
                        .map(Some)?
                }
                FileType::Masm | FileType::Masl | FileType::Wat => {
                    return Err(DriverError::InvalidFileType(
                        InvalidFileTypeError::Unsupported(path.as_ref().to_path_buf()),
                    ));
                }
            },
            Input::Stdin(ref filename, ref content) => match FileType::detect(content)? {
                FileType::Hir => compile_hir(options, codemap, diagnostics)?,
                FileType::Wasm => {
                    compile_wasm_from_bytes(&content, options.clone(), codemap, diagnostics)
                        .map(Some)?
                }
                FileType::Masm | FileType::Masl | FileType::Wat => {
                    return Err(DriverError::InvalidFileType(
                        InvalidFileTypeError::Unsupported(filename.as_ref().to_path_buf()),
                    ));
                }
            },
        },
        _ => compile_hir(options, codemap, diagnostics)?,
    };

    let duration = HumanDuration::since(start);
    diagnostics.success("Finished", &format!("built {} in {:#}", name, duration));

    Ok(program)
}

fn compile_hir(
    options: Arc<Options>,
    codemap: Arc<CodeMap>,
    diagnostics: &DiagnosticsHandler,
) -> Result<Option<Box<masm::Program>>, DriverError> {
    use miden_codegen_masm::MasmCompiler;
    use miden_hir::parser;

    let mut modules = Vec::with_capacity(options.inputs.len());

    for input in options.inputs.iter() {
        match input {
            Input::Stdin(filename, bytes) => {
                let content = core::str::from_utf8(&bytes).map_err(|_| {
                    DriverError::InvalidInput(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("{} is not valid utf-8", filename),
                    ))
                })?;
                let ast = parser::parse_ast(diagnostics, codemap.clone(), content)?;
                options.emit(&ast)?;
                modules.push(Box::new(ast.try_into_hir(diagnostics)?));
            }
            Input::File(filename) => {
                let ast = parser::parse_file_ast(diagnostics, codemap.clone(), filename)?;
                options.emit(&ast)?;
                modules.push(Box::new(ast.try_into_hir(diagnostics)?));
            }
        }
    }

    // If we're not compiling to Miden Assembly, we're expecting some kind of pass pipeline to run
    if !options.output_types.contains(OutputTypes::MASM) {
        todo!()
    } else {
        // Compile to Miden Assembly using the standard pass pipeline
        let mut masm_compiler = MasmCompiler::new(options, diagnostics);
        Ok(Some(masm_compiler.compile_modules(modules)?))
    }
}

fn compile_wasm_from_file(
    path: &Path,
    options: Arc<Options>,
    codemap: Arc<CodeMap>,
    diagnostics: &DiagnosticsHandler,
) -> Result<Box<masm::Program>, DriverError> {
    let mut file = std::fs::File::open(path)?;
    let mut bytes = Vec::with_capacity(1024);
    file.read_to_end(&mut bytes)?;
    compile_wasm_from_bytes(&bytes, options, codemap, diagnostics)
}

fn compile_wasm_from_bytes(
    bytes: &[u8],
    options: Arc<Options>,
    _codemap: Arc<CodeMap>,
    diagnostics: &DiagnosticsHandler,
) -> Result<Box<masm::Program>, DriverError> {
    use miden_codegen_masm::MasmCompiler;
    use miden_frontend_wasm::{self as wasm, WasmTranslationConfig};

    let config = WasmTranslationConfig::default();
    let module = wasm::translate_module(bytes, &config, &diagnostics)?;
    // Emit hir artifact
    options.emit(&module)?;
    // Compile to MASM IR
    let mut masm_compiler = MasmCompiler::new(options, diagnostics);
    Ok(masm_compiler.compile_module(Box::new(module))?)
}
