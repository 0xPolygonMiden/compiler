use midenc_hir::{
    cranelift_entity::PrimaryMap, diagnostics::Severity, CanonAbiImport, ComponentBuilder,
    ComponentExport, FunctionIdent, FunctionType, Ident, InterfaceFunctionIdent, InterfaceIdent,
    Symbol,
};
use midenc_hir_type::Abi;
use midenc_session::Session;
use rustc_hash::FxHashMap;

use super::{
    interface_type_to_ir, CanonicalOptions, ComponentTypes, CoreDef, CoreExport, Export,
    ExportItem, GlobalInitializer, InstantiateModule, LinearComponent, LinearComponentTranslation,
    LoweredIndex, RuntimeImportIndex, RuntimeInstanceIndex, RuntimePostReturnIndex,
    RuntimeReallocIndex, StaticModuleIndex, Trampoline, TypeFuncIndex,
};
use crate::{
    component::StringEncoding,
    error::WasmResult,
    module::{
        build_ir::build_ir_module,
        instance::ModuleArgument,
        module_env::ParsedModule,
        module_translation_state::ModuleTranslationState,
        types::{EntityIndex, FuncIndex},
        Module, ModuleImport,
    },
    unsupported_diag, WasmTranslationConfig,
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
    /// The realloc functions used in CanonicalOptions in this component
    reallocs: FxHashMap<RuntimeReallocIndex, FunctionIdent>,
    /// The post return functions used in CanonicalOptions in this component
    post_returns: FxHashMap<RuntimePostReturnIndex, FunctionIdent>,
    session: &'a Session,
}

impl<'a, 'data> ComponentTranslator<'a, 'data> {
    pub fn new(
        component_types: ComponentTypes,
        parsed_modules: PrimaryMap<StaticModuleIndex, ParsedModule<'data>>,
        config: &'a WasmTranslationConfig,
        session: &'a Session,
    ) -> Self {
        Self {
            component_types,
            parsed_modules,
            config,
            session,
            module_instances_source: PrimaryMap::new(),
            lower_imports: FxHashMap::default(),
            reallocs: FxHashMap::default(),
            post_returns: FxHashMap::default(),
        }
    }

    /// Translate the given linearized Wasm component to the Miden IR component
    pub fn translate(
        mut self,
        wasm_translation: LinearComponentTranslation,
    ) -> WasmResult<midenc_hir::Component> {
        let mut component_builder: midenc_hir::ComponentBuilder<'a> =
            midenc_hir::ComponentBuilder::new(&self.session.diagnostics);
        dbg!(&wasm_translation.component.initializers);
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
                GlobalInitializer::ExtractMemory(mem) => {
                    if mem.index.as_u32() > 0 {
                        unsupported_diag!(
                            &self.session.diagnostics,
                            "only one memory is supported in the component"
                        );
                    }
                }
                GlobalInitializer::ExtractRealloc(realloc) => {
                    let func_id = self.func_id_from_core_def(&realloc.def)?;
                    self.reallocs.insert(realloc.index, func_id);
                }
                GlobalInitializer::ExtractPostReturn(post_return) => {
                    let func_id = self.func_id_from_core_def(&post_return.def)?;
                    self.post_returns.insert(post_return.index, func_id);
                }
                GlobalInitializer::Resource(_) => {
                    unsupported_diag!(
                        &self.session.diagnostics,
                        "resource global initializers are not yet supported"
                    );
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
    ) -> WasmResult<()> {
        match instantiate_module {
            InstantiateModule::Static(static_module_idx, args) => {
                if self.module_instances_source.values().any(|idx| *idx == *static_module_idx) {
                    unsupported_diag!(
                        &self.session.diagnostics,
                        "A module with a static index {} is already instantiated. We don't \
                         support multiple instantiations of the same module.",
                        static_module_idx.as_u32()
                    );
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
                            unsupported_diag!(
                                &self.session.diagnostics,
                                "Wasm component instance flags are not supported"
                            );
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
                let mut module_state = ModuleTranslationState::new(
                    module,
                    module_types,
                    module_args,
                    &self.session.diagnostics,
                );
                let ir_module = build_ir_module(
                    self.parsed_modules.get_mut(*static_module_idx).unwrap(),
                    module_types,
                    &mut module_state,
                    self.config,
                    self.session,
                )?;
                component_builder.add_module(ir_module.into()).expect("module is already added");
            }
            InstantiateModule::Import(..) => {
                unsupported_diag!(
                    &self.session.diagnostics,
                    "Imported Wasm core module instantiation is not supported"
                );
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
    ) -> WasmResult<ModuleArgument> {
        match trampoline {
            Trampoline::LowerImport {
                index,
                lower_ty,
                options,
            } => {
                let module_import = module.imports.get(idx).expect("module import not found");
                let runtime_import_idx = self.lower_imports[index];
                let function_id = function_id_from_import(module, module_import);
                let component_import =
                    self.translate_import(runtime_import_idx, *lower_ty, options, wasm_component)?;
                component_builder.add_import(function_id, component_import.clone());
                Ok(ModuleArgument::ComponentImport(component_import))
            }
            _ => unsupported_diag!(
                &self.session.diagnostics,
                "Not yet implemented trampoline type {:?}",
                trampoline
            ),
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
                EntityIndex::Global(_) => unsupported_diag!(
                    &self.session.diagnostics,
                    "Exporting of core module globals are not yet supported"
                ),
            },
            ExportItem::Name(_) => unsupported_diag!(
                &self.session.diagnostics,
                "Named core module exports are not yet supported"
            ),
        }
    }

    /// Translate the given runtime import to the Miden IR component import
    fn translate_import(
        &self,
        runtime_import_index: RuntimeImportIndex,
        signature: TypeFuncIndex,
        options: &CanonicalOptions,
        wasm_component: &LinearComponent,
    ) -> WasmResult<midenc_hir::ComponentImport> {
        let (import_idx, import_names) = &wasm_component.imports[runtime_import_index];
        if import_names.len() != 1 {
            unsupported_diag!(&self.session.diagnostics, "multi-name imports not supported");
        }
        let import_func_name = import_names.first().unwrap();
        let (full_interface_name, _) = wasm_component.import_types[*import_idx].clone();
        let interface_function = InterfaceFunctionIdent {
            interface: InterfaceIdent::from_full_ident(full_interface_name.clone()),
            function: Symbol::intern(import_func_name),
        };
        let Some(import_metadata) = self.config.import_metadata.get(&interface_function) else {
            return Err(self
                .session
                .diagnostics
                .diagnostic(Severity::Error)
                .with_message(format!(
                    "wasm error: import metadata for interface function {interface_function:?} \
                     not found"
                ))
                .into_report());
        };
        let lifted_func_ty = convert_lifted_func_ty(&signature, &self.component_types);

        let component_import = midenc_hir::ComponentImport::CanonAbiImport(CanonAbiImport {
            function_ty: lifted_func_ty,
            interface_function,
            digest: import_metadata.digest,
            options: self.translate_canonical_options(options)?,
        });
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
            Export::ModuleStatic(_) => {
                unsupported_diag!(
                    &self.session.diagnostics,
                    "Static module exports are not supported"
                );
            }
            Export::ModuleImport(_) => unsupported_diag!(
                &self.session.diagnostics,
                "Exporting of an imported module is not supported"
            ),
            Export::Type(_) => {
                // Besides the function exports the individual type are also exported from the
                // component We can ignore them for now
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
        let func_ident = self.func_id_from_core_def(func)?;
        let lifted_func_ty = convert_lifted_func_ty(ty, &self.component_types);
        let export = midenc_hir::ComponentExport {
            function: func_ident,
            function_ty: lifted_func_ty,
            options: self.translate_canonical_options(options)?,
        };
        Ok(export)
    }

    fn func_id_from_core_def(&self, func: &CoreDef) -> WasmResult<FunctionIdent> {
        Ok(match func {
            CoreDef::Export(core_export) => {
                let module =
                    &self.parsed_modules[self.module_instances_source[core_export.instance]].module;
                let from = Ident::from(module.name().as_str());
                let module_name = from;
                let func_name = match core_export.item {
                    ExportItem::Index(idx) => match idx {
                        EntityIndex::Function(func_idx) => module.func_name(func_idx),
                        EntityIndex::Table(_) | EntityIndex::Memory(_) | EntityIndex::Global(_) => {
                            unsupported_diag!(
                                &self.session.diagnostics,
                                "Exporting of non-function entity {:?} is not supported",
                                core_export
                            );
                        }
                    },
                    ExportItem::Name(_) => {
                        unsupported_diag!(
                            &self.session.diagnostics,
                            "Named exports are not yet supported"
                        );
                    }
                };

                midenc_hir::FunctionIdent {
                    module: module_name,
                    function: midenc_hir::Ident::with_empty_span(func_name),
                }
            }
            CoreDef::InstanceFlags(_) => {
                unsupported_diag!(
                    &self.session.diagnostics,
                    "Component instance flags exports are not supported"
                );
            }
            CoreDef::Trampoline(_) => {
                unsupported_diag!(
                    &self.session.diagnostics,
                    "Trampoline core module exports are not supported"
                );
            }
        })
    }

    fn translate_canonical_options(
        &self,
        options: &CanonicalOptions,
    ) -> WasmResult<midenc_hir::CanonicalOptions> {
        if options.string_encoding != StringEncoding::Utf8 {
            unsupported_diag!(
                &self.session.diagnostics,
                "UTF-8 is expected in CanonicalOptions, string transcoding is not yet supported"
            );
        }
        Ok(midenc_hir::CanonicalOptions {
            realloc: options.realloc.map(|idx| self.reallocs[&idx]),
            post_return: options.post_return.map(|idx| self.post_returns[&idx]),
        })
    }
}

/// Get the function id from the given Wasm core module import
fn function_id_from_import(_module: &Module, module_import: &ModuleImport) -> FunctionIdent {
    let function_id = FunctionIdent {
        module: Ident::from(module_import.module.as_str()),
        function: Ident::from(module_import.field.as_str()),
    };
    function_id
}

/// Get the function id from the given Wasm func_idx in the given Wasm core exporting_module
fn function_id_from_export(exporting_module: &Module, func_idx: FuncIndex) -> FunctionIdent {
    let func_name = exporting_module.func_name(func_idx);

    FunctionIdent {
        module: exporting_module.name(),
        function: Ident::with_empty_span(func_name),
    }
}

/// Convert the given Wasm component function type to the Miden IR lifted function type
fn convert_lifted_func_ty(ty: &TypeFuncIndex, component_types: &ComponentTypes) -> FunctionType {
    let type_func = component_types[*ty].clone();
    let params_types = component_types[type_func.params].clone().types;
    let results_types = component_types[type_func.results].clone().types;
    let params = params_types
        .iter()
        .map(|ty| interface_type_to_ir(ty, component_types))
        .collect();
    let results = results_types
        .iter()
        .map(|ty| interface_type_to_ir(ty, component_types))
        .collect();
    FunctionType {
        params,
        results,
        abi: Abi::Wasm,
    }
}
