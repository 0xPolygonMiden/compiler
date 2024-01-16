//! Performs translation from Wasm to MidenIR

// Coding conventions
#![deny(warnings)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

mod component;
mod module;

mod code_translator;
mod config;
mod error;
mod ssa;
mod translation_utils;
mod wasm_types;

#[cfg(test)]
mod test_utils;

pub use self::config::WasmTranslationConfig;
pub use self::error::WasmError;
pub use self::module::translate_module;
