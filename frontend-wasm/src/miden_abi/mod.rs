#![allow(dead_code)]
#![allow(unused_variables)]

use miden_abi_conversion::tx_kernel;
use miden_core::crypto::hash::RpoDigest;
use miden_diagnostics::DiagnosticsHandler;
use miden_hir::{FunctionIdent, InstBuilder, SourceSpan};
use thiserror::Error;

use crate::{
    module::{
        func_translation_state::FuncTranslationState, function_builder_ext::FunctionBuilderExt,
        module_tratnslation_state::ModuleTranslationState,
    },
    WasmError,
};

pub fn parse_import_function_digest(import_name: &str) -> Result<(String, RpoDigest), String> {
    // parse the hex encoded digest from the function name in the angle brackets
    // and the function name (before the angle brackets) example:
    // "miden:tx_kernel/note.get_inputs<0x0000000000000000000000000000>"
    let mut parts = import_name.split('<');
    let function_name = parts.next().unwrap();
    let digest = parts
        .next()
        .and_then(|s| s.strip_suffix('>'))
        .ok_or_else(|| "Import name parsing error: missing closing angle bracket in import name")?;
    Ok((
        function_name.to_string(),
        RpoDigest::try_from(digest).map_err(|e| e.to_string())?,
    ))
}

#[derive(Error, Debug)]
pub enum AdapterError {}

pub fn generate_adapter(
    func_id: FunctionIdent,
    module_state: &ModuleTranslationState,
    state: &mut FuncTranslationState,
    builder: &mut FunctionBuilderExt,
    span: SourceSpan,
    diagnostics: &DiagnosticsHandler,
) -> Result<bool, AdapterError> {
    if let Some(stable_import_func_name) =
        module_state.get_stable_imported_miden_abi_function(&func_id)
    {
        // The function is a stable imported Miden ABI function and might need a special adapter
        let func_ty = tx_kernel::miden_abi_function_type(&stable_import_func_name);
        let num_args = func_ty.params.len();
        // TODO: pop args number according to the original wasm function type
        let args = state.peekn_mut(num_args);
        let call = builder.ins().call(func_id, &args, span);
        let inst_results = builder.inst_results(call);
        state.popn(num_args);
        // TODO: push results according to the original wasm function type (number of results and types)
        // state.pushn(inst_results);
        // TODO: eww! fix this
        state.push1(inst_results[1]);
        // panic!("Adapter generation not implemented");
        Ok(true)
    } else {
        Ok(false)
    }
}

impl From<AdapterError> for WasmError {
    fn from(value: AdapterError) -> Self {
        WasmError::Unexpected(format!("Adapter generation error: {}", value))
    }
}
