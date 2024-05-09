use std::sync::Arc;

use miden_diagnostics::{
    term::termcolor::ColorChoice, CodeMap, DiagnosticsConfig, DiagnosticsHandler, Emitter,
    NullEmitter, Verbosity,
};

pub fn default_emitter(color: ColorChoice) -> Arc<dyn Emitter> {
    Arc::new(NullEmitter::new(color))
}

pub fn test_diagnostics() -> DiagnosticsHandler {
    let codemap = Arc::new(CodeMap::new());

    DiagnosticsHandler::new(
        DiagnosticsConfig {
            verbosity: Verbosity::Debug,
            warnings_as_errors: false,
            no_warn: false,
            display: Default::default(),
        },
        codemap,
        default_emitter(ColorChoice::Auto),
    )
}
