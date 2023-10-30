//! Performs translation from Wasm to MidenIR

// Coding conventions
#![deny(warnings)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

mod code_translator;
mod config;
mod error;
mod func_translation_state;
mod func_translator;
mod function_builder_ext;
mod module_env;
mod module_translator;
mod sections_translator;
mod ssa;
mod translation_utils;
mod wasm_types;

#[cfg(test)]
mod test_utils;

pub use self::config::WasmTranslationConfig;
pub use self::error::WasmError;
pub use self::module_translator::translate_module;
