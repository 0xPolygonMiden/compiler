//! Module translation environment

use crate::error::{WasmError, WasmResult};
use crate::func_translator::FuncTranslator;
use crate::translation_utils::sig_from_funct_type;
use crate::wasm_types::{
    DefinedFuncIndex, FuncIndex, Global, GlobalInit, Memory, MemoryIndex, TypeIndex,
};
use miden_diagnostics::{DiagnosticsHandler, SourceSpan};
use miden_hir::cranelift_entity::{EntityRef, PrimaryMap, SecondaryMap};
use miden_hir::{CallConv, Ident, Linkage, Module, ModuleBuilder, Symbol};
use miden_hir_type::FunctionType;
use std::string::String;
use std::vec::Vec;
use wasmparser::{FunctionBody, Validator};

use super::FuncEnvironment;

/// The main state belonging to a `ModuleEnvironment`. This is split out from
/// `ModuleEnvironment` to allow it to be borrowed separately from the
/// `FuncTranslator` field.
pub struct ModuleInfo {
    /// Module name
    pub name: Ident,

    /// Function types
    pub func_types: PrimaryMap<TypeIndex, FunctionType>,

    /// Functions, imported and local.
    pub functions: PrimaryMap<FuncIndex, TypeIndex>,

    /// Function names.
    pub function_names: SecondaryMap<FuncIndex, String>,

    /// Module and field names of imported functions as provided by `declare_func_import`.
    pub imported_funcs: Vec<(String, String)>,

    /// Memories as provided by `declare_memory`.
    pub memories: PrimaryMap<MemoryIndex, Memory>,

    /// The start function.
    pub start_func: Option<FuncIndex>,
}

impl ModuleInfo {
    pub fn new(id: Ident) -> Self {
        Self {
            name: id,
            func_types: PrimaryMap::new(),
            imported_funcs: Vec::new(),
            functions: PrimaryMap::new(),
            memories: PrimaryMap::new(),
            start_func: None,
            function_names: SecondaryMap::new(),
        }
    }
}

pub struct ModuleEnvironment<'a> {
    /// Module information.
    pub info: ModuleInfo,

    /// Function translation.
    pub trans: FuncTranslator,

    /// Unparsed function bodies (bytes).
    pub function_bodies: PrimaryMap<DefinedFuncIndex, FunctionBody<'a>>,
}

impl<'a> ModuleEnvironment<'a> {
    /// Creates a new `ModuleEnvironment` instance.
    pub fn new() -> Self {
        Self {
            info: ModuleInfo::new(Ident::with_empty_span(Symbol::intern("noname"))),
            trans: FuncTranslator::new(),
            function_bodies: PrimaryMap::new(),
        }
    }

    /// Get the type for the function at the given index.
    pub fn get_func_type(&self, func_index: FuncIndex) -> TypeIndex {
        self.info.functions[func_index]
    }

    /// Return the number of imported functions within this `ModuleEnvironment`.
    pub fn get_num_func_imports(&self) -> usize {
        self.info.imported_funcs.len()
    }

    /// Return the name of the function, if a name for the function with
    /// the corresponding index exists.
    pub fn get_func_name(&self, func_index: FuncIndex) -> Option<&str> {
        self.info.function_names.get(func_index).map(String::as_ref)
    }

    pub fn build(
        mut self,
        diagnostics: &DiagnosticsHandler,
        validator: &mut Validator,
    ) -> WasmResult<Module> {
        let mut module_builder = ModuleBuilder::new(self.info.name.as_str());
        let get_num_func_imports = self.get_num_func_imports();
        for (def_func_index, body) in &self.function_bodies {
            let func_index = FuncIndex::new(get_num_func_imports + def_func_index.index());
            let sig_type_idx = self.get_func_type(func_index);
            let func_ty = &self.info.func_types[sig_type_idx];
            let func_name = self
                .get_func_name(func_index)
                .unwrap_or(&format!("func{}", func_index.index()))
                .to_string();
            let sig = sig_from_funct_type(func_ty, CallConv::SystemV, Linkage::External);
            let mut module_func_builder =
                module_builder.build_function(func_name, sig.clone(), SourceSpan::default())?;
            let func_builder = module_func_builder.func_builder();
            let mut func_environ = FuncEnvironment::new(&self.info);
            let mut func_validator = validator
                .code_section_entry(&body)?
                .into_validator(Default::default());
            self.trans.translate_body(
                body,
                func_builder,
                &mut func_environ,
                diagnostics,
                &mut func_validator,
            )?;
            module_func_builder
                .build(diagnostics)
                .map_err(|_| WasmError::InvalidFunctionError)?;
        }
        let module = module_builder.build();
        Ok(*module)
    }

    /// Declares a function signature to the environment.
    pub fn declare_type_func(&mut self, func_type: FunctionType) {
        self.info.func_types.push(func_type);
    }

    /// Declares a function import to the environment.
    pub fn declare_func_import(&mut self, index: TypeIndex, module: &'a str, field: &'a str) {
        assert_eq!(
            self.info.functions.len(),
            self.info.imported_funcs.len(),
            "Imported functions must be declared first"
        );
        self.info.functions.push(index);
        self.info
            .imported_funcs
            .push((String::from(module), String::from(field)));
    }

    /// Declares the type (signature) of a local function in the module.
    pub fn declare_func_type(&mut self, index: TypeIndex) {
        self.info.functions.push(index);
    }

    /// Declares a global to the environment.
    pub fn declare_global(&mut self, _global: Global, _init: GlobalInit) {
        // TODO: store global and global inits
    }

    /// Declares a memory to the environment
    pub fn declare_memory(&mut self, memory: Memory) {
        self.info.memories.push(memory);
    }

    /// Declares the optional start function.
    pub fn declare_start_func(&mut self, func_index: FuncIndex) {
        debug_assert!(self.info.start_func.is_none());
        self.info.start_func = Some(func_index);
    }

    /// Provides the contents of a function body.
    pub fn define_function_body(&mut self, body: FunctionBody<'a>) {
        self.function_bodies.push(body);
    }

    /// Declares the name of a module to the environment.
    pub fn declare_module_name(&mut self, name: &'a str) {
        self.info.name = Ident::with_empty_span(Symbol::intern(name));
    }

    /// Declares the name of a function to the environment.
    pub fn declare_func_name(&mut self, func_index: FuncIndex, name: &'a str) {
        self.info.function_names[func_index] = String::from(name);
    }

    /// Indicates that a custom section has been found in the wasm file
    pub fn custom_section(&mut self, _name: &'a str, _data: &'a [u8]) {
        // Do we need to support custom sections?
    }

    /// Declares the name of a function's local to the environment.
    pub fn declare_local_name(
        &mut self,
        _func_index: FuncIndex,
        _local_index: u32,
        _name: &'a str,
    ) {
        // Do we need a local's name?
    }
}