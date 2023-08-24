//! Translation skeleton that traverses the whole WebAssembly module and call helper functions
//! to deal with each part of it.

use crate::environ::ModuleEnvironment;
use crate::error::WasmResult;
use crate::sections_translator::{
    parse_data_section, parse_element_section, parse_function_section, parse_global_section,
    parse_import_section, parse_memory_section, parse_name_section, parse_type_section,
};
use crate::wasm_types::FuncIndex;
use crate::{unsupported_diag, WasmTranslationConfig};
use miden_diagnostics::DiagnosticsHandler;
use miden_ir::hir::Module;
use std::prelude::v1::*;
use wasmparser::{NameSectionReader, Parser, Payload, Validator, WasmFeatures};

#[cfg(test)]
mod tests;

/// Translate a sequence of bytes forming a valid Wasm binary into Miden IR
pub fn translate_module(
    wasm: &[u8],
    _config: &WasmTranslationConfig,
    diagnostics: &DiagnosticsHandler,
) -> WasmResult<Module> {
    let mut module_env = ModuleEnvironment::new();
    let env = &mut module_env;
    let wasm_features = WasmFeatures::default();
    let mut validator = Validator::new_with_features(wasm_features);
    for payload in Parser::new(0).parse_all(wasm) {
        match payload? {
            Payload::Version {
                num,
                encoding,
                range,
            } => {
                validator.version(num, encoding, &range)?;
            }
            Payload::End(offset) => {
                validator.end(offset)?;
            }

            Payload::TypeSection(types) => {
                validator.type_section(&types)?;
                parse_type_section(types, env, diagnostics)?;
            }

            Payload::ImportSection(imports) => {
                validator.import_section(&imports)?;
                parse_import_section(imports, env, diagnostics)?;
            }

            Payload::FunctionSection(functions) => {
                validator.function_section(&functions)?;
                parse_function_section(functions, env)?;
            }

            Payload::TableSection(tables) => {
                validator.table_section(&tables)?;
                // skip the table section
            }

            Payload::MemorySection(memories) => {
                validator.memory_section(&memories)?;
                parse_memory_section(memories, env)?;
            }

            Payload::TagSection(tags) => {
                validator.tag_section(&tags)?;
                unsupported_diag!(diagnostics, "Tag sections are not supported");
            }

            Payload::GlobalSection(globals) => {
                validator.global_section(&globals)?;
                parse_global_section(globals, env, diagnostics)?;
            }

            Payload::ExportSection(exports) => {
                validator.export_section(&exports)?;
                // skip the export section
            }

            Payload::StartSection { func, range } => {
                validator.start_section(func, &range)?;
                env.declare_start_func(FuncIndex::from_u32(func));
            }

            Payload::ElementSection(elements) => {
                validator.element_section(&elements)?;
                parse_element_section(elements, env, diagnostics)?;
            }

            Payload::CodeSectionStart { count, range, .. } => {
                validator.code_section_start(count, &range)?;
            }

            Payload::CodeSectionEntry(body) => {
                let func_validator = validator
                    .code_section_entry(&body)?
                    .into_validator(Default::default());
                env.define_function_body(func_validator, body);
            }

            Payload::DataSection(data) => {
                validator.data_section(&data)?;
                parse_data_section(data, env, diagnostics)?;
            }

            Payload::DataCountSection { count, range } => {
                validator.data_count_section(count, &range)?;
            }

            Payload::CustomSection(s) if s.name() == "name" => {
                let result =
                    parse_name_section(NameSectionReader::new(s.data(), s.data_offset()), env);
                if let Err(e) = result {
                    log::warn!("failed to parse name section {:?}", e);
                }
            }

            Payload::CustomSection(s) => env.custom_section(s.name(), s.data()),

            other => {
                validator.payload(&other)?;
                panic!("unimplemented section {:?}", other);
            }
        }
    }
    Ok(module_env.build(diagnostics)?)
}
