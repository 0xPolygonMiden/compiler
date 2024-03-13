use miden_abi_conversion::tx_kernel::miden_abi_function_type;
use miden_diagnostics::DiagnosticsHandler;
use miden_hir::{DataFlowGraph, FunctionIdent, Signature};
use rustc_hash::FxHashMap;
use std::collections::{hash_map::Entry::Occupied, hash_map::Entry::Vacant};

use crate::{error::WasmResult, miden_abi::parse_import_function_digest, WasmError};

use super::{func_env::FuncEnvironment, FuncIndex};

pub struct ModuleTranslationState {
    // Imported and local functions
    // Stores both the function reference and its signature
    pub functions: FxHashMap<FuncIndex, (FunctionIdent, Signature)>,
}

impl ModuleTranslationState {
    pub(crate) fn new() -> Self {
        Self {
            functions: FxHashMap::default(),
        }
    }

    /// Get the `FunctionIdent` that should be used to make a direct call to function
    /// `index`. Also return the number of WebAssembly arguments in the signature.
    ///
    /// Import the callee into `func`'s DFG if it is not already present.
    pub(crate) fn get_direct_func(
        &mut self,
        dfg: &mut DataFlowGraph,
        index: FuncIndex,
        func_env: &FuncEnvironment,
        diagnostics: &DiagnosticsHandler,
    ) -> WasmResult<(FunctionIdent, usize)> {
        let (func_id, sig) = match self.functions.entry(index) {
            Occupied(entry) => entry.get().clone(),
            Vacant(entry) => {
                let func_id = func_env.function_id(index);
                let wasm_sig = func_env.signature(index);
                // TODO: don't parse on every call. Cache it?
                let sig = if let Ok((func_name, _)) =
                    parse_import_function_digest(func_id.function.as_symbol().as_str())
                {
                    // TODO: hard fail if the Miden ABI function type is not found
                    if let Some(miden_abi) = miden_abi_function_type(&func_name) {
                        miden_abi.into()
                    } else {
                        wasm_sig.clone()
                    }
                } else {
                    wasm_sig.clone()
                };
                entry.insert((*func_id, sig)).clone()
            }
        };
        if dfg.get_import(&func_id).is_none() {
            dfg.import_function(func_id.module, func_id.function, sig.clone()).map_err(|_e| {
                let message = format!("Function with name {} in module {} with signature {sig:?} is already imported (function call) with a different signature", func_id.function, func_id.module);
                diagnostics
                    .diagnostic(miden_diagnostics::Severity::Error)
                    .with_message(message.clone())
                    .emit();
                WasmError::Unexpected(message)
            })?;
        }
        Ok((func_id, sig.params.len()))
    }
}
