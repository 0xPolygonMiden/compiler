//! Performs translation from Wasm to MidenIR

// Coding conventions
#![deny(warnings)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
// TODO: remove this once everything is implemented
#![allow(unused_variables)]
#![allow(dead_code)]

mod component;
mod module;

mod code_translator;
mod config;
mod error;
mod ssa;
mod translation_utils;

#[cfg(test)]
mod test_utils;

pub use self::component::translate::translate_component;
pub use self::config::WasmTranslationConfig;
pub use self::error::WasmError;
pub use self::module::translate::translate_module;
