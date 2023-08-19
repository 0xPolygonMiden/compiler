//! Performs translation from Wasm to MidenIR

// Coding conventions
#![deny(warnings)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

mod code_translator;
mod config;
mod environ;
mod error;
mod func_translation_state;
mod func_translator;
mod function_builder_ext;
mod module_translator;
mod sections_translator;
mod ssa;
mod translation_utils;
mod wasm_types;

pub use crate::config::WasmTranslationConfig;
pub use crate::module_translator::translate_module;
