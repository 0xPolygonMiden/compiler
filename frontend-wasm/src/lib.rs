//! Performs translation from Wasm to MidenIR

// Coding conventions
#![deny(warnings)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

extern crate alloc;

mod code_translator;
mod component;
mod config;
mod error;
mod intrinsics;
mod miden_abi;
mod module;
mod ssa;
mod translation_utils;

#[cfg(test)]
mod test_utils;

use component::build_ir::translate_component;
use error::WasmResult;
use miden_diagnostics::DiagnosticsHandler;
use module::build_ir::translate_module_as_component;
pub use translation_utils::sanitize_name;

pub use self::{config::*, error::WasmError};

/// Translate a valid Wasm core module or Wasm Component Model binary into Miden
/// IR Component
pub fn translate(
    wasm: &[u8],
    config: &WasmTranslationConfig,
    diagnostics: &DiagnosticsHandler,
) -> WasmResult<midenc_hir::Component> {
    if wasm[4..8] == [0x01, 0x00, 0x00, 0x00] {
        // Wasm core module
        // see https://github.com/WebAssembly/component-model/blob/main/design/mvp/Binary.md#component-definitions
        translate_module_as_component(wasm, config, diagnostics)
    } else {
        translate_component(wasm, config, diagnostics)
    }
}
