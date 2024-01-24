use std::collections::HashMap;

use miden_hir::cranelift_entity::PrimaryMap;

use crate::{component::Trampoline, module::module_env::ParsedModule};

use super::{
    build_ir::BuildIrComponentInput, ComponentTypes, CoreDef, GlobalInitializer, InstantiateModule,
    LinearComponent, LoweredIndex, RuntimeImportIndex, RuntimeInstanceIndex, StaticModuleIndex,
    TypeFuncIndex,
};

/// A component import
#[derive(Debug)]
pub struct ComponentImport {
    pub runtime_import_index: RuntimeImportIndex,
    pub signature: TypeFuncIndex,
}

pub struct ComponentInstance<'data> {
    pub modules: PrimaryMap<StaticModuleIndex, ParsedModule<'data>>,
    pub module_instances: PrimaryMap<RuntimeInstanceIndex, StaticModuleIndex>,
    pub component: LinearComponent,
    pub component_types: ComponentTypes,
    pub imports: HashMap<StaticModuleIndex, Vec<ComponentImport>>,
}

impl<'data> ComponentInstance<'data> {
    pub fn ensure_module_names(&mut self) {
        for (idx, parsed_module) in self.modules.iter_mut() {
            parsed_module
                .module
                .set_name_fallback(Some(format!("module{}", idx.as_u32())));
        }
    }

    pub fn module(&self, idx: RuntimeInstanceIndex) -> &ParsedModule<'data> {
        &self.modules[self.module_instances[idx]]
    }
}

pub struct ComponentInstanceBuilder<'data> {
    pub input: BuildIrComponentInput<'data>,
}

impl<'data> ComponentInstanceBuilder<'data> {
    pub fn new(input: BuildIrComponentInput<'data>) -> Self {
        Self { input }
    }

    // TODO: make it a function?
    #[allow(unused_variables)]
    pub fn build(self) -> ComponentInstance<'data> {
        let mut module_instances: PrimaryMap<RuntimeInstanceIndex, StaticModuleIndex> =
            PrimaryMap::new();
        let mut lower_imports: HashMap<LoweredIndex, RuntimeImportIndex> = HashMap::new();
        let mut imports: HashMap<StaticModuleIndex, Vec<ComponentImport>> = HashMap::new();
        let component = &self.input.linear_component_translation.component;
        for initializer in &component.initializers {
            match initializer {
                GlobalInitializer::InstantiateModule(m) => {
                    match m {
                        InstantiateModule::Static(static_module_idx, args) => {
                            if module_instances
                                .values()
                                .find(|idx| **idx == *static_module_idx)
                                .is_some()
                            {
                                panic!(
                                    "A module with a static index {} is already instantiated. We don't support multiple instantiations of the same module.",
                                    static_module_idx.as_u32()
                                )
                            }

                            module_instances.push(*static_module_idx);
                            // TODO: store args (imports) for every module instance
                            let mut module_args: Vec<ComponentImport> = Vec::new();
                            for arg in args.iter() {
                                match arg {
                                    CoreDef::Export(_) => todo!(),
                                    CoreDef::InstanceFlags(_) => todo!(),
                                    CoreDef::Trampoline(trampoline_idx) => {
                                        let trampoline =
                                            &self.input.linear_component_translation.trampolines
                                                [*trampoline_idx];
                                        match trampoline {
                                            Trampoline::LowerImport {
                                                index,
                                                lower_ty,
                                                options,
                                            } => {
                                                let import = lower_imports[index];
                                                let import = ComponentImport {
                                                    runtime_import_index: import,
                                                    signature: *lower_ty,
                                                };
                                                module_args.push(import);
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                }
                            }
                            imports.insert(*static_module_idx, module_args);
                        }
                        InstantiateModule::Import(_, _) => todo!(),
                    };
                }
                GlobalInitializer::LowerImport {
                    index: init_lowered_idx,
                    import,
                } => {
                    lower_imports.insert(*init_lowered_idx, *import);
                    // let (import_idx, names) = &component.imports[*import];
                    // assert_eq!(names.len(), 1, "multi-name imports not yet supported");
                    // let func_name = names.first().unwrap();
                    // let (full_interface_name, component_instance_type) =
                    //     component.import_types[*import_idx].clone();
                    // let func_ty: &TypeFuncIndex = self
                    //     .input
                    //     .linear_component_translation
                    //     .trampolines
                    //     .iter()
                    //     .find_map(|(_, t)| match t {
                    //         Trampoline::LowerImport {
                    //             index,
                    //             lower_ty,
                    //             options: _,
                    //         } if index == init_lowered_idx => Some(lower_ty),
                    //         _ => None,
                    //     })
                    //     .unwrap();
                    // let func_type = self.input.component_types[*func_ty].clone();
                    // todo!("get the component function type from the trampoline")
                }
                GlobalInitializer::ExtractMemory(_) => todo!(),
                GlobalInitializer::ExtractRealloc(_) => todo!(),
                GlobalInitializer::ExtractPostReturn(_) => todo!(),
                GlobalInitializer::Resource(_) => todo!(),
            }
        }
        ComponentInstance {
            modules: self.input.modules,
            module_instances,
            component: self.input.linear_component_translation.component,
            component_types: self.input.component_types,
            imports,
        }
    }
}
