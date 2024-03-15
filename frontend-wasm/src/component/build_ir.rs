use miden_diagnostics::DiagnosticsHandler;
use wasmparser::WasmFeatures;

use super::{
    inline, translator::ComponentTranslator, ComponentTypesBuilder, LinearComponentTranslation,
    ParsedRootComponent,
};
use crate::{component::ComponentParser, error::WasmResult, WasmTranslationConfig};

/// Translate a Wasm component binary into Miden IR component
pub fn translate_component(
    wasm: &[u8],
    config: &WasmTranslationConfig,
    diagnostics: &DiagnosticsHandler,
) -> WasmResult<miden_hir::Component> {
    let (mut component_types_builder, parsed_component) = parse(config, wasm, diagnostics)?;
    let linearized_component_translation = inline(&mut component_types_builder, &parsed_component)?;
    let component_types = component_types_builder.finish();
    let parsed_modules = parsed_component.static_modules;
    let translator = ComponentTranslator::new(component_types, parsed_modules, config, diagnostics);
    translator.translate(linearized_component_translation)
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

#[cfg(test)]
mod tests {
    use miden_core::crypto::hash::RpoDigest;
    use miden_hir::{InterfaceFunctionIdent, InterfaceIdent, LiftedFunctionType, Symbol};
    use miden_hir_type::Type;

    use super::*;
    use crate::{
        component::StaticModuleIndex,
        config::{ExportMetadata, ImportMetadata},
        test_utils::test_diagnostics,
    };

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
        // dbg!(&component_translation.component);
        let module = &parsed_component.static_modules[StaticModuleIndex::from_u32(0)].module;
        // dbg!(module);
        assert_eq!(module.imports.len(), 0);
        assert_eq!(component_translation.trampolines.len(), 0);
        // dbg!(&component_translation.component.initializers);
        assert_eq!(component_translation.component.initializers.len(), 1);
        // dbg!(&component_translation.component.exports);
        assert_eq!(component_translation.component.exports.len(), 1);
        let component_types = component_types_builder.finish();
        let translator = ComponentTranslator::new(
            component_types,
            parsed_component.static_modules,
            &config,
            &diagnostics,
        );
        let ir = translator.translate(component_translation).unwrap();

        // dbg!(&ir.exports());
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
                digest: RpoDigest::default(),
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

        // dbg!(&module.imports);
        assert_eq!(module.imports.len(), 1);

        // dbg!(&component_translation.trampolines);
        assert_eq!(component_translation.trampolines.len(), 1);

        // dbg!(&component_translation.component.initializers);
        assert_eq!(component_translation.component.initializers.len(), 2);

        // dbg!(&component_translation.component.imports);
        assert_eq!(component_translation.component.imports.len(), 1);
        // dbg!(&component_translation.component.import_types);
        assert_eq!(component_translation.component.import_types.len(), 1);

        // dbg!(&component_translation.component.exports);
        assert_eq!(component_translation.component.exports.len(), 1);

        let component_types = component_types_builder.finish();

        let translator = ComponentTranslator::new(
            component_types,
            parsed_component.static_modules,
            &config,
            &diagnostics,
        );
        let ir = translator.translate(component_translation).unwrap();

        // dbg!(&ir.exports());
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
        let module = ir.modules().first().unwrap().1;
        // dbg!(&module.imports());
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
        assert_eq!(component_import.interface_function, interface_function_ident);
        assert!(!component_import.function_ty.params.is_empty());
        let expected_import_func_ty = LiftedFunctionType {
            params: vec![Type::U32, Type::U32],
            results: vec![Type::U32],
        };
        assert_eq!(component_import.function_ty, expected_import_func_ty);
    }
}
