use miden_hir::{CallConv, FunctionIdent, Ident, Linkage, Signature, Symbol};
use rustc_hash::FxHashMap;

use crate::{module::EntityIndex, translation_utils::sig_from_funct_type};

use super::{instance::ModuleArgument, ir_func_type, FuncIndex, Module, ModuleTypes};

pub struct FuncEnvironment {
    /// A translated IR function ids indexed by the Wasm function index.
    function_ids: FxHashMap<FuncIndex, FunctionIdent>,
    /// A translated IR function signatures, indexed by the Wasm function index.
    signatures: FxHashMap<FuncIndex, Signature>,
}

impl FuncEnvironment {
    pub fn new(module: &Module, mod_types: &ModuleTypes, module_args: Vec<ModuleArgument>) -> Self {
        assert_eq!(
            module.imports.len(),
            module_args.len(),
            "Mismatched module imports and arguments"
        );
        let mut subst_imports = FxHashMap::default();
        for (import, arg) in module.imports.iter().zip(module_args) {
            match (import.index, arg) {
                (EntityIndex::Function(func_idx), ModuleArgument::Function(func_id)) => {
                    subst_imports.insert(func_idx, func_id);
                }
                (EntityIndex::Function(_), ModuleArgument::ComponentImport(_)) => {
                    // Do nothing, the local function name will be used
                    ()
                }
                (EntityIndex::Table(_), ModuleArgument::Table) => {
                    // TODO: implement table imports
                    ()
                }
                (import, arg) => {
                    panic!("Mismatched import and argument: {:?} {:?}", import, arg);
                }
            }
        }
        let mut function_ids = FxHashMap::default();
        let mut signatures = FxHashMap::default();
        for (index, _func) in &module.functions {
            let func_type_idx = module.functions[index].clone();
            let func_type = mod_types[func_type_idx.signature].clone();
            let ir_func_type = ir_func_type(&func_type).unwrap();
            let sig = sig_from_funct_type(&ir_func_type, CallConv::SystemV, Linkage::External);
            signatures.insert(index, sig);
            if let Some(subst) = subst_imports.get(&index) {
                function_ids.insert(index, subst.clone());
            } else {
                let func_name = module.func_name(index);
                let func_id = FunctionIdent {
                    module: Ident::with_empty_span(Symbol::intern(module.name())),
                    function: Ident::with_empty_span(Symbol::intern(func_name)),
                };
                function_ids.insert(index, func_id);
            };
        }
        Self {
            function_ids,
            signatures,
        }
    }

    pub fn function_id(&self, function_idx: FuncIndex) -> &FunctionIdent {
        &self.function_ids[&function_idx]
    }

    pub fn signature(&self, function_idx: FuncIndex) -> &Signature {
        &self.signatures[&function_idx]
    }
}
