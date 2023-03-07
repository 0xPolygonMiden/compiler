mod options;

pub use self::options::Options;

use std::sync::Arc;
use std::time::Instant;

use miden_diagnostics::term::termcolor::ColorChoice;
use miden_diagnostics::*;

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

    // Track when compilation began
    let start = Instant::now();

    // let files = options.input_files.clone();
    // let artifacts = compile()?;
    // diagnostics.abort_if_errors();

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
