use miden_hir::cranelift_entity::EntityRef;
use miden_hir::CallConv;
use miden_hir::Function;
use miden_hir::FunctionIdent;
use miden_hir::Ident;
use miden_hir::Linkage;
use miden_hir::Symbol;

use crate::translation_utils::sig_from_funct_type;
use crate::wasm_types::FuncIndex;

use super::ModuleInfo;

/// Environment affecting the translation of a single WebAssembly function.
pub struct FuncEnvironment<'a> {
    pub mod_info: &'a ModuleInfo,
}

impl<'a> FuncEnvironment<'a> {
    pub fn new(mod_info: &'a ModuleInfo) -> Self {
        Self { mod_info }
    }

    /// Set up an external function definition for `func` that can be used to
    /// directly call the function `index`.
    /// The index space covers both imported functions and functions defined in the current module.
    /// The function's signature will only be used for direct calls, even if the module has
    /// indirect calls with the same WebAssembly type.
    pub fn make_direct_func(&mut self, func: &mut Function, index: FuncIndex) -> FunctionIdent {
        let sigidx = self.mod_info.functions[index];
        let func_type = self.mod_info.func_types[sigidx].clone();
        let func_name = self
            .mod_info
            .function_names
            .get(index)
            .cloned()
            .unwrap_or_else(|| format!("func{}", index.index()));
        let fid = Ident::with_empty_span(Symbol::intern(&func_name));
        let sig = sig_from_funct_type(&func_type, CallConv::SystemV, Linkage::External);
        // TODO: handle error
        let imported_fid = func
            .dfg
            .import_function(self.mod_info.id, fid, sig)
            .unwrap();
        imported_fid
    }
}
