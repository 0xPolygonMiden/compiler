use miden_ir::cranelift_entity::EntityRef;
use miden_ir::hir::FuncRef;
use miden_ir::hir::Function;
use miden_ir::hir::Signature;
use miden_ir::hir::Visibility;

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
    pub fn make_direct_func(&mut self, func: &mut Function, index: FuncIndex) -> FuncRef {
        let sigidx = self.mod_info.functions[index];
        let func_type = self.mod_info.func_types[sigidx].clone();
        let name = self
            .mod_info
            .function_names
            .get(index)
            .cloned()
            .unwrap_or_else(|| format!("func{}", index.index()));
        let sig = Signature {
            visibility: Visibility::PUBLIC,
            name: name.clone(),
            ty: func_type,
        };
        let fref = func.dfg.register_callee(name, sig);
        fref
    }
}
