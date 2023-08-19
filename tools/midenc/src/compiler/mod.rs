mod options;

pub use self::options::Options;

use anyhow::anyhow;
use std::sync::Arc;
use std::time::Instant;

use miden_diagnostics::term::termcolor::ColorChoice;
use miden_diagnostics::*;
use miden_frontend_wasm::WasmTranslationConfig;

use crate::utils::HumanDuration;

pub fn compile(
    options: Arc<Options>,
    codemap: Arc<CodeMap>,
    emitter: Option<Arc<dyn Emitter>>,
) -> anyhow::Result<()> {
    let diagnostics = Arc::new(DiagnosticsHandler::new(
        DiagnosticsConfig {
            verbosity: options.verbosity,
            warnings_as_errors: options.warnings_as_errors,
            no_warn: options.no_warn,
            display: Default::default(),
        },
        codemap.clone(),
        emitter.unwrap_or_else(|| default_emitter(options.verbosity, ColorChoice::Auto)),
    ));

    if options.input_files.is_empty() {
        diagnostics.fatal("No inputs found!").raise();
    }

    if options.input_files.len() > 1 {
        diagnostics
            .fatal("Multiple Wasm files are not supported!")
            .raise();
    }

    // Track when compilation began
    let start = Instant::now();

    let input_file = options.input_files.first().unwrap();
    let wasm_data = match input_file {
        FileName::Real(path) => match std::fs::read(path) {
            Ok(data) => data,
            Err(e) => diagnostics
                .fatal(format!(
                    "error reading file {}, with error {e}",
                    path.display()
                ))
                .raise(),
        },
        FileName::Virtual(_) => todo!("virtual files are not yet supported"),
    };
    let config = WasmTranslationConfig::default();
    let res = miden_frontend_wasm::translate_module(&wasm_data, &config, &diagnostics);
    let _module = match res {
        Ok(module) => module,
        Err(e) => {
            diagnostics.emit(e);
            return Err(anyhow!("error translating module"));
        }
    };
    diagnostics.abort_if_errors();

    let duration = HumanDuration::since(start);
    diagnostics.success(
        "Finished",
        &format!("built {} in {:#}", options.name, duration),
    );
    Ok(())
}

fn default_emitter(verbosity: Verbosity, color: ColorChoice) -> Arc<dyn Emitter> {
    match verbosity {
        Verbosity::Silent => Arc::new(NullEmitter::new(color)),
        _ => Arc::new(DefaultEmitter::new(color)),
    }
}
