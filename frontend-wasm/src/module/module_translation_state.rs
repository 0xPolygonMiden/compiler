use miden_core::crypto::hash::RpoDigest;
use miden_diagnostics::DiagnosticsHandler;
use midenc_hir::{AbiParam, CallConv, DataFlowGraph, FunctionIdent, Ident, Linkage, Signature};
use rustc_hash::FxHashMap;

use super::{instance::ModuleArgument, ir_func_type, EntityIndex, FuncIndex, Module, ModuleTypes};
use crate::{
    error::WasmResult,
    intrinsics::is_miden_intrinsics_module,
    miden_abi::{is_miden_abi_module, miden_abi_function_type, parse_import_function_digest},
    translation_utils::sig_from_funct_type,
    WasmError,
};

pub struct ModuleTranslationState {
    /// Imported and local functions
    /// Stores both the function reference and its signature
    functions: FxHashMap<FuncIndex, (FunctionIdent, Signature)>,
    /// Parsed MAST root hash for imported functions for Miden SDK
    digests: FxHashMap<FunctionIdent, RpoDigest>,
    /// Number of imported or aliased functions in the module.
    pub num_imported_funcs: usize,
    // stable_imported_miden_abi_functions: FxHashMap<FunctionIdent, String>,
}

impl ModuleTranslationState {
    pub fn new(module: &Module, mod_types: &ModuleTypes, module_args: Vec<ModuleArgument>) -> Self {
        let mut function_import_subst = FxHashMap::default();
        if module.imports.len() == module_args.len() {
            for (import, arg) in module.imports.iter().zip(module_args) {
                match (import.index, arg) {
                    (EntityIndex::Function(func_idx), ModuleArgument::Function(func_id)) => {
                        // Substitutes the function import with concrete function exported from
                        // another module
                        function_import_subst.insert(func_idx, func_id);
                    }
                    (EntityIndex::Function(_), ModuleArgument::ComponentImport(_)) => {
                        // Do nothing, the local function id will be used
                    }
                    (EntityIndex::Function(_), module_arg) => {
                        panic!(
                            "Unexpected {module_arg:?} module argument for function import \
                             {import:?}"
                        )
                    }
                    (..) => (), // Do nothing, we interested only in function imports
                }
            }
        }
        let mut functions = FxHashMap::default();
        let mut digests = FxHashMap::default();
        for (index, func_type) in &module.functions {
            let wasm_func_type = mod_types[func_type.signature].clone();
            let ir_func_type = ir_func_type(&wasm_func_type).unwrap();
            let sig = sig_from_funct_type(&ir_func_type, CallConv::SystemV, Linkage::External);
            if let Some(subst) = function_import_subst.get(&index) {
                functions.insert(index, (*subst, sig));
            } else if module.is_imported_function(index) {
                assert!((index.as_u32() as usize) < module.num_imported_funcs);
                let import = &module.imports[index.as_u32() as usize];
                if let Ok((func_stable_name, digest)) = parse_import_function_digest(&import.field)
                {
                    let func_id = FunctionIdent {
                        module: Ident::from(import.module.as_str()),
                        function: Ident::from(func_stable_name.as_str()),
                    };
                    functions.insert(index, (func_id, sig));
                    digests.insert(func_id, digest);
                } else {
                    let func_id = FunctionIdent {
                        module: Ident::from(import.module.as_str()),
                        function: Ident::from(import.field.as_str()),
                    };
                    functions.insert(index, (func_id, sig));
                };
            } else {
                let func_name = module.func_name(index);
                let func_id = FunctionIdent {
                    module: module.name(),
                    function: Ident::from(func_name.as_str()),
                };
                functions.insert(index, (func_id, sig));
            };
        }
        Self {
            functions,
            digests,
            num_imported_funcs: module.num_imported_funcs,
        }
    }

    /// Returns an IR function signature converted from Wasm function signature
    /// for the given function index.
    pub fn signature(&self, index: FuncIndex) -> &Signature {
        &self.functions[&index].1
    }

    /// Returns parsed MAST root hash for the given function id (if it is imported and has one)
    pub fn digest(&self, func_id: &FunctionIdent) -> Option<&RpoDigest> {
        self.digests.get(func_id)
    }

    /// Get the `FunctionIdent` that should be used to make a direct call to function
    /// `index`.
    ///
    /// Import the callee into `func`'s DFG if it is not already present.
    pub(crate) fn get_direct_func(
        &mut self,
        dfg: &mut DataFlowGraph,
        index: FuncIndex,
        diagnostics: &DiagnosticsHandler,
    ) -> WasmResult<FunctionIdent> {
        let (func_id, wasm_sig) = self.functions[&index].clone();
        let sig: Signature = if is_miden_abi_module(func_id.module.as_symbol()) {
            let ft =
                miden_abi_function_type(func_id.module.as_symbol(), func_id.function.as_symbol());
            Signature::new(
                ft.params.into_iter().map(AbiParam::new),
                ft.results.into_iter().map(AbiParam::new),
            )
        } else {
            wasm_sig.clone()
        };

        if is_miden_intrinsics_module(func_id.module.as_symbol()) {
            // Exit and do not import intrinsics functions into the DFG
            return Ok(func_id);
        }

        if dfg.get_import(&func_id).is_none() {
            dfg.import_function(func_id.module, func_id.function, sig.clone())
                .map_err(|_e| {
                    let message = format!(
                        "Function with name {} in module {} with signature {sig:?} is already \
                         imported (function call) with a different signature",
                        func_id.function, func_id.module
                    );
                    diagnostics
                        .diagnostic(miden_diagnostics::Severity::Error)
                        .with_message(message.clone())
                        .emit();
                    WasmError::Unexpected(message)
                })?;
        }
        Ok(func_id)
    }
}
