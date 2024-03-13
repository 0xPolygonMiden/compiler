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
    functions: FxHashMap<FuncIndex, (FunctionIdent, Signature)>,

    stable_imported_miden_abi_functions: FxHashMap<FunctionIdent, String>,
}

impl ModuleTranslationState {
    pub(crate) fn new() -> Self {
        Self {
            functions: FxHashMap::default(),
            stable_imported_miden_abi_functions: FxHashMap::default(),
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
                let sig = func_env.signature(index);
                if !func_env.is_imported_function(index) {
                    (func_id, sig);
                }
                let sig: Signature = match self.stable_imported_miden_abi_functions.entry(*func_id)
                {
                    Occupied(entry) => {
                        let stable_name = entry.get().clone();
                        miden_abi_function_type(&stable_name).into()
                    }
                    Vacant(entry) => {
                        if let Ok((stable_name, _)) =
                            parse_import_function_digest(func_id.function.as_symbol().as_str())
                        {
                            entry.insert(stable_name.clone());
                            miden_abi_function_type(&stable_name).into()
                        } else {
                            // This is imported but not a "well-known" Miden ABI function
                            sig.clone()
                        }
                    }
                };
                entry.insert((*func_id, sig.clone())).clone()
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

    pub(crate) fn get_stable_imported_miden_abi_function(
        &self,
        func_id: &FunctionIdent,
    ) -> Option<&String> {
        self.stable_imported_miden_abi_functions.get(func_id)
    }
}
