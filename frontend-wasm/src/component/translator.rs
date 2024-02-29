use miden_diagnostics::DiagnosticsHandler;
use rustc_hash::FxHashMap;

use miden_hir::{
    cranelift_entity::PrimaryMap, ComponentBuilder, ComponentExport, FunctionIdent, Ident,
    InterfaceFunctionIdent, InterfaceIdent, Symbol,
};
use miden_hir_type::LiftedFunctionType;

use crate::{
    component::StringEncoding,
    error::WasmResult,
    module::{
        build_ir::build_ir_module,
        func_env::FuncEnvironment,
        instance::ModuleArgument,
        module_env::ParsedModule,
        types::{EntityIndex, FuncIndex},
        Module, ModuleImport,
    },
    WasmError, WasmTranslationConfig,
};

use super::{
    interface_type_to_ir, CanonicalOptions, ComponentTypes, CoreDef, CoreExport, Export,
    ExportItem, GlobalInitializer, InstantiateModule, LinearComponent, LinearComponentTranslation,
    LoweredIndex, RuntimeImportIndex, RuntimeInstanceIndex, StaticModuleIndex, Trampoline,
    TypeFuncIndex,
};

/// A translator from the linearized Wasm component model to the Miden IR component
pub struct ComponentTranslator<'a, 'data> {
    /// The Wasm component types
    component_types: ComponentTypes,
    /// The parsed static modules of the Wasm component
    parsed_modules: PrimaryMap<StaticModuleIndex, ParsedModule<'data>>,
    /// The translation configuration
    config: &'a WasmTranslationConfig,
    /// The runtime module instances index mapped to the static module index
    module_instances_source: PrimaryMap<RuntimeInstanceIndex, StaticModuleIndex>,
    /// The lower imports index mapped to the runtime import index
    lower_imports: FxHashMap<LoweredIndex, RuntimeImportIndex>,
    diagnostics: &'a DiagnosticsHandler,
}

impl<'a, 'data> ComponentTranslator<'a, 'data> {
    pub fn new(
        component_types: ComponentTypes,
        parsed_modules: PrimaryMap<StaticModuleIndex, ParsedModule<'data>>,
        config: &'a WasmTranslationConfig,
        diagnostics: &'a DiagnosticsHandler,
    ) -> Self {
        Self {
            component_types,
            parsed_modules,
            config,
            diagnostics,
            module_instances_source: PrimaryMap::new(),
            lower_imports: FxHashMap::default(),
        }
    }

    /// Translate the given linearized Wasm component to the Miden IR component
    pub fn translate(
        mut self,
        wasm_translation: LinearComponentTranslation,
    ) -> WasmResult<miden_hir::Component> {
        let mut component_builder: miden_hir::ComponentBuilder<'a> =
            miden_hir::ComponentBuilder::new(self.diagnostics);
        for initializer in &wasm_translation.component.initializers {
            match initializer {
                GlobalInitializer::InstantiateModule(instantiate_module) => {
                    self.translate_module_instance(
                        instantiate_module,
                        &mut component_builder,
                        &wasm_translation,
                    )?;
                }
                GlobalInitializer::LowerImport {
                    index: init_lowered_idx,
                    import,
                } => {
                    self.lower_imports.insert(*init_lowered_idx, *import);
                }
                GlobalInitializer::ExtractMemory(_) => {
                    // Do nothing, there is only one memory address space in Miden IR
                }
                GlobalInitializer::ExtractRealloc(_) => {
                    return Err(WasmError::Unsupported(
                        "Realloc function pointer global initializer is not yet supported"
                            .to_string(),
                    ))
                }
                GlobalInitializer::ExtractPostReturn(_) => {
                    return Err(WasmError::Unsupported(
                        "Post return function pointer global initializer is not yet supported"
                            .to_string(),
                    ))
                }
                GlobalInitializer::Resource(_) => {
                    return Err(WasmError::Unsupported(
                        "Resource global initializers are not yet supported".to_string(),
                    ))
                }
            }
        }
        for (name, export) in &wasm_translation.component.exports {
            self.build_export(export, name, &mut component_builder)?;
        }
        Ok(component_builder.build())
    }

    /// Translate the given Wasm core module instantiotion to the Miden IR component
    fn translate_module_instance(
        &mut self,
        instantiate_module: &InstantiateModule,
        component_builder: &mut ComponentBuilder<'_>,
        wasm_translation: &LinearComponentTranslation,
    ) -> Result<(), WasmError> {
        match instantiate_module {
            InstantiateModule::Static(static_module_idx, args) => {
                if self
                    .module_instances_source
                    .values()
                    .find(|idx| **idx == *static_module_idx)
                    .is_some()
                {
                    return Err(WasmError::Unsupported(format!(
                        "A module with a static index {} is already instantiated. We don't support multiple instantiations of the same module.",
                        static_module_idx.as_u32()
                    )));
                }
                self.module_instances_source.push(*static_module_idx);
                // TODO: create and init module instance tables
                // see https://github.com/0xPolygonMiden/compiler/issues/133
                let module = &self.parsed_modules[*static_module_idx].module;
                let mut module_args: Vec<ModuleArgument> = Vec::new();
                for (idx, arg) in args.iter().enumerate() {
                    match arg {
                        CoreDef::Export(export) => {
                            module_args.push(self.module_arg_from_export(export)?);
                        }
                        CoreDef::InstanceFlags(_) => {
                            return Err(WasmError::Unsupported(
                                "Wasm component instance flags are not supported".to_string(),
                            ))
                        }
                        CoreDef::Trampoline(trampoline_idx) => {
                            let trampoline = &wasm_translation.trampolines[*trampoline_idx];
                            let arg = self.module_arg_from_trampoline(
                                trampoline,
                                module,
                                idx,
                                &wasm_translation.component,
                                component_builder,
                            )?;
                            module_args.push(arg);
                        }
                    }
                }
                let module_types = self.component_types.module_types();
                let func_env = FuncEnvironment::new(module, module_types, module_args);
                let ir_module = build_ir_module(
                    self.parsed_modules.get_mut(*static_module_idx).unwrap(),
                    module_types,
                    func_env,
                    self.config,
                    self.diagnostics,
                )?;
                component_builder
                    .add_module(ir_module.into())
                    .expect("module is already added");
            }
            InstantiateModule::Import(_, _) => {
                return Err(WasmError::Unsupported(
                    "Imported Wasm core module instantiation is not supported".to_string(),
                ))
            }
        };
        Ok(())
    }

    /// Build a Wasm core module argument from the given trampoline (component import)
    fn module_arg_from_trampoline(
        &self,
        trampoline: &Trampoline,
        module: &Module,
        idx: usize,
        wasm_component: &LinearComponent,
        component_builder: &mut ComponentBuilder<'_>,
    ) -> Result<ModuleArgument, WasmError> {
        match trampoline {
            Trampoline::LowerImport {
                index,
                lower_ty,
                options: _,
            } => {
                let module_import = module.imports.get(idx).expect("module import not found");
                let runtime_import_idx = self.lower_imports[index];
                let function_id = function_id_from_import(module, module_import);
                let component_import =
                    self.translate_import(runtime_import_idx, *lower_ty, wasm_component)?;
                component_builder.add_import(function_id, component_import.clone());
                Ok(ModuleArgument::ComponentImport(component_import))
            }
            _ => Err(WasmError::Unsupported(format!(
                "Not yet implemented trampoline type {trampoline:?}"
            ))),
        }
    }

    /// Build a module argument from the given module export
    fn module_arg_from_export(
        &self,
        export: &CoreExport<EntityIndex>,
    ) -> WasmResult<ModuleArgument> {
        match export.item {
            ExportItem::Index(entity_idx) => match entity_idx {
                EntityIndex::Function(func_idx) => {
                    let exporting_module_id = self.module_instances_source[export.instance];
                    let function_id = function_id_from_export(
                        &self.parsed_modules[exporting_module_id].module,
                        func_idx,
                    );
                    Ok(ModuleArgument::Function(function_id))
                }
                EntityIndex::Table(_idx) => {
                    // TODO: init the exported table with this module's table initialization values
                    // see https://github.com/0xPolygonMiden/compiler/issues/133
                    Ok(ModuleArgument::Table)
                }
                EntityIndex::Memory(_) => {
                    unreachable!("Attempt to export memory from a module instance. ")
                }
                EntityIndex::Global(_) => Err(WasmError::Unsupported(
                    "Exporting of core module globals are not yet supported".to_string(),
                )),
            },
            ExportItem::Name(_) => Err(WasmError::Unsupported(
                "Named core module exports are not yet supported".to_string(),
            )),
        }
    }

    /// Translate the given runtime import to the Miden IR component import
    fn translate_import(
        &self,
        runtime_import_index: RuntimeImportIndex,
        signature: TypeFuncIndex,
        wasm_component: &LinearComponent,
    ) -> WasmResult<miden_hir::ComponentImport> {
        let (import_idx, import_names) = &wasm_component.imports[runtime_import_index];
        if import_names.len() != 1 {
            return Err(crate::WasmError::Unsupported(
                "multi-name imports not supported".to_string(),
            ));
        }
        let import_func_name = import_names.first().unwrap();
        let (full_interface_name, _) = wasm_component.import_types[*import_idx].clone();
        let interface_function = InterfaceFunctionIdent {
            interface: InterfaceIdent::from_full_ident(full_interface_name.clone()),
            function: Symbol::intern(import_func_name),
        };
        let Some(import_metadata) = self.config.import_metadata.get(&interface_function) else {
            return Err(crate::WasmError::MissingImportMetadata(format!(
                "Import metadata for interface function {:?} not found",
                &interface_function,
            )));
        };
        let lifted_func_ty = convert_lifted_func_ty(&signature, &self.component_types);

        let component_import = miden_hir::ComponentImport {
            function_ty: lifted_func_ty,
            interface_function,
            digest: import_metadata.digest.clone(),
        };
        Ok(component_import)
    }

    /// Build an IR Component export from the given Wasm component export
    fn build_export(
        &self,
        export: &Export,
        name: &String,
        component_builder: &mut ComponentBuilder,
    ) -> WasmResult<()> {
        match export {
            Export::LiftedFunction { ty, func, options } => {
                let export_name = Symbol::intern(name).into();
                let export = self.build_export_lifted_function(func, ty, options)?;
                component_builder.add_export(export_name, export);
                Ok(())
            }
            Export::Instance(exports) => {
                // Flatten any(nested) interface instance exports into the IR `Component` exports
                for (name, export) in exports {
                    self.build_export(export, name, component_builder)?;
                }
                Ok(())
            }
            Export::ModuleStatic(_) => Err(WasmError::Unsupported(
                "Static module exports are not supported".to_string(),
            )),
            Export::ModuleImport(_) => Err(WasmError::Unsupported(
                "Exporting of an imported module is not supported".to_string(),
            )),
            Export::Type(_) => {
                // Besides the function exports the individual type are also exported from the component
                // We can ignore them for now
                Ok(())
            }
        }
    }

    /// Build an IR Component export from the given lifted Wasm core module function export
    fn build_export_lifted_function(
        &self,
        func: &CoreDef,
        ty: &TypeFuncIndex,
        options: &CanonicalOptions,
    ) -> WasmResult<ComponentExport> {
        assert_empty_canonical_options(options);
        let func_ident = match func {
            CoreDef::Export(core_export) => {
                let module =
                    &self.parsed_modules[self.module_instances_source[core_export.instance]].module;
                let module_name = module.name();
                let module_ident = miden_hir::Ident::with_empty_span(Symbol::intern(module_name));
                let func_name = match core_export.item {
                    ExportItem::Index(idx) => match idx {
                        EntityIndex::Function(func_idx) => module.func_name(func_idx),
                        EntityIndex::Table(_) | EntityIndex::Memory(_) | EntityIndex::Global(_) => {
                            return Err(WasmError::Unsupported(format!(
                                "Exporting of non-function entity {core_export:?} is not supported"
                            )));
                        }
                    },
                    ExportItem::Name(_) => {
                        return Err(WasmError::Unsupported(
                            "Named exports are not yet supported".to_string(),
                        ))
                    }
                };
                let func_ident = miden_hir::FunctionIdent {
                    module: module_ident,
                    function: miden_hir::Ident::with_empty_span(Symbol::intern(func_name)),
                };
                func_ident
            }
            CoreDef::InstanceFlags(_) => {
                return Err(WasmError::Unsupported(
                    "Component instance flags exports are not supported".to_string(),
                ))
            }
            CoreDef::Trampoline(_) => {
                return Err(WasmError::Unsupported(
                    "Trampoline core module exports are not supported".to_string(),
                ))
            }
        };
        let lifted_func_ty = convert_lifted_func_ty(ty, &self.component_types);
        let export = miden_hir::ComponentExport {
            function: func_ident,
            function_ty: lifted_func_ty,
        };
        Ok(export)
    }
}

/// Get the function id from the given Wasm core module import
fn function_id_from_import(module: &Module, module_import: &ModuleImport) -> FunctionIdent {
    let func_name = module.func_name(module_import.index.unwrap_func());
    let module_name = module.name();
    let function_id = FunctionIdent {
        module: Ident::with_empty_span(Symbol::intern(module_name)),
        function: Ident::with_empty_span(Symbol::intern(func_name)),
    };
    function_id
}

/// Get the function id from the given Wasm func_idx in the given Wasm core exporting_module
fn function_id_from_export(exporting_module: &Module, func_idx: FuncIndex) -> FunctionIdent {
    let func_name = exporting_module.func_name(func_idx);
    let module_name = exporting_module.name();
    let function_id = FunctionIdent {
        module: Ident::with_empty_span(Symbol::intern(module_name)),
        function: Ident::with_empty_span(Symbol::intern(func_name)),
    };
    function_id
}

/// Convert the given Wasm component function type to the Miden IR lifted function type
fn convert_lifted_func_ty(
    ty: &TypeFuncIndex,
    component_types: &ComponentTypes,
) -> LiftedFunctionType {
    let type_func = component_types[*ty].clone();
    let params_types = component_types[type_func.params].clone().types;
    let results_types = component_types[type_func.results].clone().types;
    let params = params_types
        .into_iter()
        .map(|ty| interface_type_to_ir(ty, component_types))
        .collect();
    let results = results_types
        .into_iter()
        .map(|ty| interface_type_to_ir(ty, component_types))
        .collect();
    LiftedFunctionType { params, results }
}

fn assert_empty_canonical_options(options: &CanonicalOptions) {
    assert_eq!(
        options.string_encoding,
        StringEncoding::Utf8,
        "UTF-8 is expected in CanonicalOptions, string transcoding is not yet supported"
    );
    assert!(
        options.realloc.is_none(),
        "realloc in CanonicalOptions is not yet supported"
    );
    assert!(
        options.post_return.is_none(),
        "post_return in CanonicalOptions is not yet supported"
    );
    assert!(
        options.memory.is_none(),
        "memory in CanonicalOptions is not yet supported"
    );
}
