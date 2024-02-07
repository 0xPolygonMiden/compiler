use miden_diagnostics::DiagnosticsHandler;
use miden_hir::{
    cranelift_entity::PrimaryMap, FunctionIdent, Ident, InterfaceFunctionIdent, InterfaceIdent,
    LiftedFunctionType, Symbol,
};
use wasmparser::WasmFeatures;

use crate::{
    component::{ComponentParser, StringEncoding},
    error::WasmResult,
    module::{build_ir::build_ir_module, module_env::ParsedModule, types::EntityIndex},
    WasmError, WasmTranslationConfig,
};

use super::{
    inline,
    instance::{ComponentImport, ComponentInstance, ComponentInstanceBuilder},
    interface_type_to_ir, CanonicalOptions, ComponentTypes, ComponentTypesBuilder, CoreDef, Export,
    ExportItem, LinearComponent, LinearComponentTranslation, ParsedRootComponent,
    StaticModuleIndex, TypeFuncIndex,
};

/// Translate a Wasm component binary into Miden IR component
pub fn translate_component(
    wasm: &[u8],
    config: &WasmTranslationConfig,
    diagnostics: &DiagnosticsHandler,
) -> WasmResult<miden_hir::Component> {
    let (mut component_types_builder, parsed_component) = parse(config, wasm, diagnostics)?;
    let linearized_component_translation = inline(&mut component_types_builder, &parsed_component)?;
    let component_types = component_types_builder.finish();
    build_ir(
        linearized_component_translation,
        component_types,
        parsed_component.static_modules,
        config,
        diagnostics,
    )
}

fn parse<'data>(
    config: &WasmTranslationConfig,
    wasm: &'data [u8],
    diagnostics: &DiagnosticsHandler,
) -> Result<(ComponentTypesBuilder, ParsedRootComponent<'data>), crate::WasmError> {
    let wasm_features = WasmFeatures::all();
    let mut validator = wasmparser::Validator::new_with_features(wasm_features);
    let mut component_types_builder = Default::default();
    let component_parser =
        ComponentParser::new(config, &mut validator, &mut component_types_builder);
    let parsed_component = component_parser.parse(wasm, diagnostics)?;
    Ok((component_types_builder, parsed_component))
}

fn inline(
    component_types_builder: &mut ComponentTypesBuilder,
    parsed_component: &ParsedRootComponent<'_>,
) -> WasmResult<LinearComponentTranslation> {
    // ... after translation initially finishes the next pass is performed
    // which we're calling "inlining". This will "instantiate" the root
    // component, following nested component instantiations, creating a
    // global list of initializers along the way. This phase uses the simple
    // initializers in each component to track dataflow of host imports and
    // internal references to items throughout a component at translation time.
    // The produce initializers in the final `LinearComponent` are intended to be
    // much simpler than the original component and more efficient for
    // us to process (e.g. no string lookups as
    // most everything is done through indices instead).
    let component_dfg = inline::run(
        component_types_builder,
        &parsed_component.root_component,
        &parsed_component.static_modules,
        &parsed_component.static_components,
    )
    .map_err(|e| crate::WasmError::Unsupported(e.to_string()))?;
    Ok(component_dfg.finish())
}

fn build_ir<'data>(
    linear_component_translation: LinearComponentTranslation,
    component_types: ComponentTypes,
    modules: PrimaryMap<StaticModuleIndex, ParsedModule<'data>>,
    config: &WasmTranslationConfig,
    diagnostics: &DiagnosticsHandler,
) -> WasmResult<miden_hir::Component> {
    let mut cb = miden_hir::ComponentBuilder::new(diagnostics);

    let component_instance_builder =
        ComponentInstanceBuilder::new(linear_component_translation, component_types, modules);
    let mut component_instance = component_instance_builder.build()?;

    component_instance.ensure_module_names();

    // build exports
    for (name, export) in &component_instance.component.exports {
        build_export(export, &component_instance, name, &mut cb, config)?;
    }

    for (static_module_idx, parsed_module) in component_instance.modules {
        let component = &component_instance.component;
        build_import(
            &component_instance.imports[&static_module_idx],
            &component_instance.component_types,
            component,
            &parsed_module,
            &mut cb,
            config,
        )?;

        let module = build_ir_module(
            parsed_module,
            component_instance.component_types.module_types(),
            config,
            diagnostics,
        )?;
        cb.add_module(module.into())
            .expect("module is already added");
    }

    Ok(cb.build())
}

fn build_import(
    component_imports: &[ComponentImport],
    component_types: &ComponentTypes,
    component: &LinearComponent,
    parsed_module: &ParsedModule<'_>,
    cb: &mut miden_hir::ComponentBuilder<'_>,
    config: &WasmTranslationConfig,
) -> WasmResult<()> {
    for import in component_imports {
        let (import_idx, import_names) = &component.imports[import.runtime_import_index];
        if import_names.len() != 1 {
            return Err(crate::WasmError::Unsupported(
                "multi-name imports not supported".to_string(),
            ));
        }
        let import_func_name = import_names.first().unwrap();
        let (full_interface_name, _) = component.import_types[*import_idx].clone();
        let interface_function = InterfaceFunctionIdent {
            interface: InterfaceIdent::from_full_ident(full_interface_name.clone()),
            function: Symbol::intern(import_func_name),
        };
        let Some(import_metadata) = config.import_metadata.get(&interface_function) else {
            return Err(crate::WasmError::MissingImportMetadata(format!(
                "Import metadata for interface function {:?} not found",
                &interface_function,
            )));
        };
        let lifted_func_ty = convert_lifted_func_ty(&import.signature, component_types);

        let component_import = miden_hir::ComponentImport {
            function_ty: lifted_func_ty,
            interface_function,
            invoke_method: import_metadata.invoke_method,
            function_mast_root_hash: import_metadata.function_mast_root_hash.clone(),
        };
        let function_id =
            find_module_import_function(parsed_module, full_interface_name, import_func_name)?;
        cb.add_import(function_id, component_import);
    }
    Ok(())
}

fn find_module_import_function(
    parsed_module: &ParsedModule,
    full_interface_name: String,
    import_func_name: &String,
) -> WasmResult<FunctionIdent> {
    for import in &parsed_module.module.imports {
        if import.module == full_interface_name && &import.field == import_func_name {
            let func_idx = import.index.unwrap_func();
            let func_name = parsed_module.module.func_name(func_idx);
            let module_instance_name = parsed_module.module.name();
            return Ok(FunctionIdent {
                module: Ident::with_empty_span(Symbol::intern(module_instance_name)),
                function: Ident::with_empty_span(Symbol::intern(func_name)),
            });
        }
    }
    Err(WasmError::Unexpected(format!(
        "failed to find module import for interface {} and function {}",
        full_interface_name, import_func_name
    )))
}

fn build_export(
    export: &Export,
    component_instance: &ComponentInstance<'_>,
    name: &String,
    cb: &mut miden_hir::ComponentBuilder<'_>,
    config: &WasmTranslationConfig,
) -> WasmResult<()> {
    match export {
        Export::LiftedFunction { ty, func, options } => {
            build_export_function(component_instance, name, func, ty, options, cb, config)
        }
        Export::Instance(exports) => {
            // We don't support exporting an interface instance, add the interface items to the
            // IR `Component` exports instead
            for (name, export) in exports {
                build_export(export, component_instance, name, cb, config)?;
            }
            Ok(())
        }
        Export::ModuleStatic(_) => todo!(),
        Export::ModuleImport(_) => todo!(),
        Export::Type(_) => todo!(),
    }
}

fn build_export_function(
    component_instance: &ComponentInstance<'_>,
    name: &String,
    func: &CoreDef,
    ty: &TypeFuncIndex,
    options: &CanonicalOptions,
    cb: &mut miden_hir::ComponentBuilder<'_>,
    config: &WasmTranslationConfig,
) -> WasmResult<()> {
    assert_empty_canonical_options(options);
    let func_ident = match func {
        CoreDef::Export(core_export) => {
            let parsed_module = component_instance.module(core_export.instance);
            let module_name = parsed_module.module.name();
            let module_ident = miden_hir::Ident::with_empty_span(Symbol::intern(module_name));
            let func_name = match core_export.item {
                ExportItem::Index(idx) => match idx {
                    EntityIndex::Function(func_idx) => parsed_module.module.func_name(func_idx),
                    EntityIndex::Table(_) => todo!(),
                    EntityIndex::Memory(_) => todo!(),
                    EntityIndex::Global(_) => todo!(),
                },
                ExportItem::Name(_) => todo!(),
            };
            let func_ident = miden_hir::FunctionIdent {
                module: module_ident,
                function: miden_hir::Ident::with_empty_span(Symbol::intern(func_name)),
            };
            func_ident
        }
        CoreDef::InstanceFlags(_) => todo!(),
        CoreDef::Trampoline(_) => todo!(),
    };
    let lifted_func_ty = convert_lifted_func_ty(ty, &component_instance.component_types);
    let export_name = Symbol::intern(name).into();
    let Some(export_metadata) = config.export_metadata.get(&export_name) else {
        return Err(WasmError::MissingExportMetadata(format!(
            "Export metadata for interface function {:?} not found",
            &export_name,
        )));
    };
    let export = miden_hir::ComponentExport {
        function: func_ident,
        function_ty: lifted_func_ty,
        invoke_method: export_metadata.invoke_method,
    };
    cb.add_export(export_name, export);
    Ok(())
}

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

#[cfg(test)]
mod tests {

    use miden_hir::MastRootHash;
    use miden_hir_type::Type;

    use crate::{
        component::StaticModuleIndex,
        config::{ExportMetadata, ImportMetadata},
        test_utils::test_diagnostics,
    };

    use super::*;

    #[test]
    fn translate_simple() {
        let wat = format!(
            r#"
            (component
            (core module (;0;)
                (type (;0;) (func))
                (type (;1;) (func (param i32 i32) (result i32)))
                (func $add (;0;) (type 1) (param i32 i32) (result i32)
                local.get 1
                local.get 0
                i32.add
                )
                (memory (;0;) 17)
                (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
                (export "memory" (memory 0))
                (export "add" (func $add))
            )
            (core instance (;0;) (instantiate 0))
            (alias core export 0 "memory" (core memory (;0;)))
            (type (;0;) (func (param "a" u32) (param "b" u32) (result u32)))
            (alias core export 0 "add" (core func (;0;)))
            (func (;0;) (type 0) (canon lift (core func 0)))
            (export (;1;) "add" (func 0))
            )
        "#,
        );
        let wasm = wat::parse_str(wat).unwrap();
        let diagnostics = test_diagnostics();
        let export_metadata = [(
            Symbol::intern("add").into(),
            ExportMetadata {
                invoke_method: miden_hir::FunctionInvocationMethod::Call,
            },
        )]
        .into_iter()
        .collect();
        let config = WasmTranslationConfig {
            export_metadata,
            ..Default::default()
        };
        let (mut component_types_builder, parsed_component) =
            parse(&config, &wasm, &diagnostics).unwrap();
        let component_translation =
            inline(&mut component_types_builder, &parsed_component).unwrap();

        assert_eq!(parsed_component.static_modules.len(), 1);
        dbg!(&component_translation.component);
        let module = &parsed_component.static_modules[StaticModuleIndex::from_u32(0)].module;
        dbg!(module);
        assert_eq!(module.imports.len(), 0);
        assert_eq!(component_translation.trampolines.len(), 0);
        dbg!(&component_translation.component.initializers);
        assert_eq!(component_translation.component.initializers.len(), 1);
        dbg!(&component_translation.component.exports);
        assert_eq!(component_translation.component.exports.len(), 1);
        let component_types = component_types_builder.finish();
        let ir = build_ir(
            component_translation,
            component_types,
            parsed_component.static_modules,
            &config,
            &diagnostics,
        )
        .unwrap();
        dbg!(&ir.exports());
        assert!(!ir.modules().is_empty());
        assert!(!ir.exports().is_empty());
        let export_name_sym = Symbol::intern("add");
        let export = ir.exports().get(&export_name_sym.into()).unwrap();
        assert_eq!(export.function.function.as_symbol(), export_name_sym);
        let expected_export_func_ty = LiftedFunctionType {
            params: vec![Type::U32, Type::U32],
            results: vec![Type::U32],
        };
        assert_eq!(export.function_ty, expected_export_func_ty);
    }

    #[test]
    fn translate_simple_import() {
        let wat = format!(
            r#"
            (component
            (type (;0;)
                (instance
                (type (;0;) (func (param "a" u32) (param "b" u32) (result u32)))
                (export (;0;) "add" (func (type 0)))
                )
            )
            (import "miden:add/add@1.0.0" (instance (;0;) (type 0)))
            (core module (;0;)
                (type (;0;) (func (param i32 i32) (result i32)))
                (type (;1;) (func))
                (type (;2;) (func (param i32) (result i32)))
                (import "miden:add/add@1.0.0" "add" (func $inc_wasm_component::bindings::miden::add::add::add::wit_import (;0;) (type 0)))
                (func $inc (;1;) (type 2) (param i32) (result i32)
                local.get 0
                i32.const 1
                call $inc_wasm_component::bindings::miden::add::add::add::wit_import
                )
                (memory (;0;) 17)
                (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
                (export "memory" (memory 0))
                (export "inc" (func $inc))
            )
            (alias export 0 "add" (func (;0;)))
            (core func (;0;) (canon lower (func 0)))
            (core instance (;0;)
                (export "add" (func 0))
            )
            (core instance (;1;) (instantiate 0
                (with "miden:add/add@1.0.0" (instance 0))
                )
            )
            (alias core export 1 "memory" (core memory (;0;)))
            (type (;1;) (func (param "a" u32) (result u32)))
            (alias core export 1 "inc" (core func (;1;)))
            (func (;1;) (type 1) (canon lift (core func 1)))
            (export (;1;) "inc" (func 1))
            )
        "#,
        );
        let wasm = wat::parse_str(wat).unwrap();
        let diagnostics = test_diagnostics();
        let interface_function_ident = InterfaceFunctionIdent {
            interface: InterfaceIdent::from_full_ident("miden:add/add@1.0.0".to_string()),
            function: Symbol::intern("add"),
        };
        let import_metadata = [(
            interface_function_ident.clone(),
            ImportMetadata {
                function_mast_root_hash: MastRootHash::ZEROES,
                invoke_method: miden_hir::FunctionInvocationMethod::Call,
            },
        )]
        .into_iter()
        .collect();
        let export_metadata = [(
            Symbol::intern("inc").into(),
            ExportMetadata {
                invoke_method: miden_hir::FunctionInvocationMethod::Call,
            },
        )]
        .into_iter()
        .collect();
        let config = WasmTranslationConfig {
            import_metadata,
            export_metadata,
            ..Default::default()
        };
        let (mut component_types_builder, parsed_component) =
            parse(&config, &wasm, &diagnostics).unwrap();
        let component_translation =
            inline(&mut component_types_builder, &parsed_component).unwrap();
        assert_eq!(parsed_component.static_modules.len(), 1);
        let module = &parsed_component.static_modules[StaticModuleIndex::from_u32(0)].module;

        dbg!(&module.imports);
        assert_eq!(module.imports.len(), 1);

        dbg!(&component_translation.trampolines);
        assert_eq!(component_translation.trampolines.len(), 1);

        dbg!(&component_translation.component.initializers);
        assert_eq!(component_translation.component.initializers.len(), 2);

        dbg!(&component_translation.component.imports);
        assert_eq!(component_translation.component.imports.len(), 1);
        dbg!(&component_translation.component.import_types);
        assert_eq!(component_translation.component.import_types.len(), 1);

        // dbg!(&component_translation.component.exports);
        assert_eq!(component_translation.component.exports.len(), 1);

        let component_types = component_types_builder.finish();

        let ir = build_ir(
            component_translation,
            component_types,
            parsed_component.static_modules,
            &config,
            &diagnostics,
        )
        .unwrap();
        dbg!(&ir.exports());
        assert!(!ir.modules().is_empty());
        assert!(!ir.exports().is_empty());
        assert!(!ir.imports().is_empty());
        let export_name_sym = Symbol::intern("inc");
        let export = ir.exports().get(&export_name_sym.into()).unwrap();
        assert_eq!(export.function.function.as_symbol(), export_name_sym);
        let expected_export_func_ty = LiftedFunctionType {
            params: vec![Type::U32],
            results: vec![Type::U32],
        };
        assert_eq!(export.function_ty, expected_export_func_ty);
        let module = ir.modules().front().get().unwrap();
        dbg!(&module.imports());
        let import_info = module.imports();
        let function_id = import_info
            .imported(&module.name)
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>()
            .first()
            .cloned()
            .unwrap()
            .clone();
        assert_eq!(function_id.module, module.name);
        // assert_eq!(function_id.function, interface_function_ident.function);
        let component_import = ir.imports().get(&function_id).unwrap();
        assert_eq!(
            component_import.interface_function,
            interface_function_ident
        );
        assert!(!component_import.function_ty.params.is_empty());
        let expected_import_func_ty = LiftedFunctionType {
            params: vec![Type::U32, Type::U32],
            results: vec![Type::U32],
        };
        assert_eq!(component_import.function_ty, expected_import_func_ty);
    }
}
