use std::sync::Arc;

use miden_diagnostics::term::termcolor::ColorChoice;
use miden_diagnostics::CodeMap;
use miden_diagnostics::DiagnosticsConfig;
use miden_diagnostics::DiagnosticsHandler;
use miden_diagnostics::Emitter;
use miden_diagnostics::NullEmitter;
use miden_diagnostics::Verbosity;
use miden_hir::MastRootHash;
use miden_hir::MAST_ROOT_HASH_SIZE_BYTES;

pub fn default_emitter(verbosity: Verbosity, color: ColorChoice) -> Arc<dyn Emitter> {
    match verbosity {
        _ => Arc::new(NullEmitter::new(color)),
    }
}

pub fn test_diagnostics() -> DiagnosticsHandler {
    let codemap = Arc::new(CodeMap::new());
    let diagnostics = DiagnosticsHandler::new(
        DiagnosticsConfig {
            verbosity: Verbosity::Debug,
            warnings_as_errors: false,
            no_warn: false,
            display: Default::default(),
        },
        codemap,
        default_emitter(Verbosity::Debug, ColorChoice::Auto),
    );
    diagnostics
}

/// Intentionally invalid MAST root hash to be used only in tests.
pub fn invalid_mast_root_hash() -> MastRootHash {
    MastRootHash::new([0u8; MAST_ROOT_HASH_SIZE_BYTES])
}
