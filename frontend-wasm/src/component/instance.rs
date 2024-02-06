use std::collections::HashMap;

use miden_hir::cranelift_entity::PrimaryMap;

use crate::{
    component::Trampoline, error::WasmResult, module::module_env::ParsedModule, WasmError,
};

use super::{
    ComponentTypes, CoreDef, GlobalInitializer, InstantiateModule, LinearComponent,
    LinearComponentTranslation, LoweredIndex, RuntimeImportIndex, RuntimeInstanceIndex,
    StaticModuleIndex, TypeFuncIndex,
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
                .set_name_fallback(format!("module{}", idx.as_u32()));
        }
    }

    pub fn module(&self, idx: RuntimeInstanceIndex) -> &ParsedModule<'data> {
        &self.modules[self.module_instances[idx]]
    }
}

pub struct ComponentInstanceBuilder<'data> {
    linear_component_translation: LinearComponentTranslation,
    component_types: ComponentTypes,
    modules: PrimaryMap<StaticModuleIndex, ParsedModule<'data>>,
}

impl<'data> ComponentInstanceBuilder<'data> {
    pub fn new(
        linear_component_translation: LinearComponentTranslation,
        component_types: ComponentTypes,
        modules: PrimaryMap<StaticModuleIndex, ParsedModule<'data>>,
    ) -> Self {
        Self {
            linear_component_translation,
            component_types,
            modules,
        }
    }

    pub fn build(self) -> WasmResult<ComponentInstance<'data>> {
        let mut module_instances: PrimaryMap<RuntimeInstanceIndex, StaticModuleIndex> =
            PrimaryMap::new();
        let mut lower_imports: HashMap<LoweredIndex, RuntimeImportIndex> = HashMap::new();
        let mut imports: HashMap<StaticModuleIndex, Vec<ComponentImport>> = HashMap::new();
        let component = &self.linear_component_translation.component;
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
                                return Err(WasmError::Unsupported(format!(
                                    "A module with a static index {} is already instantiated. We don't support multiple instantiations of the same module.",
                                    static_module_idx.as_u32()
                                )));
                            }

                            module_instances.push(*static_module_idx);
                            let mut module_args: Vec<ComponentImport> = Vec::new();
                            for arg in args.iter() {
                                match arg {
                                    CoreDef::Export(_) => todo!(),
                                    CoreDef::InstanceFlags(_) => todo!(),
                                    CoreDef::Trampoline(trampoline_idx) => {
                                        let trampoline = &self
                                            .linear_component_translation
                                            .trampolines[*trampoline_idx];
                                        match trampoline {
                                            Trampoline::LowerImport {
                                                index,
                                                lower_ty,
                                                options: _,
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
                }
                GlobalInitializer::ExtractMemory(_) => todo!(),
                GlobalInitializer::ExtractRealloc(_) => todo!(),
                GlobalInitializer::ExtractPostReturn(_) => todo!(),
                GlobalInitializer::Resource(_) => todo!(),
            }
        }
        Ok(ComponentInstance {
            modules: self.modules,
            module_instances,
            component: self.linear_component_translation.component,
            component_types: self.component_types,
            imports,
        })
    }
}
