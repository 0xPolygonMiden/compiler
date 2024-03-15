// TODO: delete this file
#![allow(dead_code)]

use miden_hir::cranelift_entity::PrimaryMap;
use rustc_hash::FxHashMap;

use super::{
    ComponentTypes, CoreDef, GlobalInitializer, InstantiateModule, LinearComponent,
    LinearComponentTranslation, LoweredIndex, RuntimeImportIndex, RuntimeInstanceIndex,
    StaticModuleIndex, TypeFuncIndex,
};
use crate::{
    component::{ExportItem, Trampoline},
    error::WasmResult,
    module::{module_env::ParsedModule, types::EntityIndex},
    WasmError,
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
    pub imports: FxHashMap<StaticModuleIndex, Vec<ComponentImport>>,
}

impl<'data> ComponentInstance<'data> {
    pub fn ensure_module_names(&mut self) {
        for (idx, parsed_module) in self.modules.iter_mut() {
            parsed_module.module.set_name_fallback(format!("module{}", idx.as_u32()).into());
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
        let mut lower_imports: FxHashMap<LoweredIndex, RuntimeImportIndex> = FxHashMap::default();
        let mut imports: FxHashMap<StaticModuleIndex, Vec<ComponentImport>> = FxHashMap::default();
        let component = &self.linear_component_translation.component;
        dbg!(&component.initializers);
        dbg!(&self.linear_component_translation.trampolines);
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
                                    "A module with a static index {} is already instantiated. We \
                                     don't support multiple instantiations of the same module.",
                                    static_module_idx.as_u32()
                                )));
                            }

                            module_instances.push(*static_module_idx);
                            let mut module_args: Vec<ComponentImport> = Vec::new();
                            for arg in args.iter() {
                                match arg {
                                    CoreDef::Export(export) => {
                                        // let static_module_idx =
                                        // module_instances[export.instance];
                                        match export.item {
                                            ExportItem::Index(entity_idx) => match entity_idx {
                                                EntityIndex::Function(_func_idx) => {
                                                    todo!();
                                                    // module_args.push(import);
                                                }
                                                EntityIndex::Table(_) => {
                                                    todo!()
                                                }
                                                EntityIndex::Memory(_) => {
                                                    todo!()
                                                }
                                                EntityIndex::Global(_) => {
                                                    todo!()
                                                }
                                            },
                                            ExportItem::Name(_) => todo!(),
                                        }
                                    }
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
                        InstantiateModule::Import(..) => todo!(),
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
