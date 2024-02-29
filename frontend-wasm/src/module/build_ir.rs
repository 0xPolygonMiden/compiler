use core::mem;

use miden_diagnostics::{DiagnosticsHandler, SourceSpan};
use miden_hir::{CallConv, ConstantData, Linkage, ModuleBuilder};
use wasmparser::{Validator, WasmFeatures};

use super::Module;
use crate::{
    error::WasmResult,
    module::{
        func_env::FuncEnvironment,
        func_translator::FuncTranslator,
        module_env::{FunctionBodyData, ModuleEnvironment, ParsedModule},
        types::{ir_func_sig, ir_func_type, ir_type, ModuleTypes},
    },
    WasmError, WasmTranslationConfig,
};

/// Translate a valid Wasm core module binary into Miden IR module
pub fn translate_module(
    wasm: &[u8],
    config: &WasmTranslationConfig,
    diagnostics: &DiagnosticsHandler,
) -> WasmResult<miden_hir::Module> {
    let wasm_features = WasmFeatures::default();
    let mut validator = Validator::new_with_features(wasm_features);
    let parser = wasmparser::Parser::new(0);
    let mut module_types_builder = Default::default();
    let mut parsed_module = ModuleEnvironment::new(
        config,
        &mut validator,
        &mut module_types_builder,
    )
    .parse(parser, wasm, diagnostics)?;
    parsed_module.module.set_name_fallback(config.source_name.clone());
    if let Some(name_override) = config.override_name.as_ref() {
        parsed_module.module.set_name_override(name_override.clone());
    }
    let module_types = module_types_builder.finish();

    let func_env = FuncEnvironment::new(&parsed_module.module, &module_types, vec![]);
    build_ir_module(&mut parsed_module, &module_types, func_env, config, diagnostics)
}

pub fn build_ir_module(
    parsed_module: &mut ParsedModule,
    module_types: &ModuleTypes,
    func_env: FuncEnvironment,
    _config: &WasmTranslationConfig,
    diagnostics: &DiagnosticsHandler,
) -> WasmResult<miden_hir::Module> {
    let name = parsed_module.module.name();
    let mut module_builder = ModuleBuilder::new(name.clone().as_str());
    build_globals(&parsed_module.module, &mut module_builder, diagnostics)?;
    build_data_segments(parsed_module, &mut module_builder, diagnostics)?;
    let mut func_translator = FuncTranslator::new();
    // Although this renders this parsed module invalid(without functiong
    // bodies), we don't support multiple module instances. Thus, this
    // ParseModule will not be used again to make another module instance.
    let func_body_inputs = mem::take(&mut parsed_module.function_body_inputs);
    for (defined_func_idx, body_data) in func_body_inputs {
        let func_index = &parsed_module.module.func_index(defined_func_idx);
        let func_type = &parsed_module.module.functions[*func_index];
        let func_name = &parsed_module.module.func_name(*func_index);
        let wasm_func_type = module_types[func_type.signature].clone();
        let ir_func_type = ir_func_type(&wasm_func_type)?;
        let sig = ir_func_sig(&ir_func_type, CallConv::SystemV, Linkage::External);
        let mut module_func_builder = module_builder.function(func_name.as_str(), sig.clone())?;
        let FunctionBodyData { validator, body } = body_data;
        let mut func_validator = validator.into_validator(Default::default());
        func_translator.translate_body(
            &body,
            &mut module_func_builder,
            &parsed_module.module,
            &module_types,
            &func_env,
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

fn build_globals(
    wasm_module: &Module,
    module_builder: &mut ModuleBuilder,
    diagnostics: &DiagnosticsHandler,
) -> Result<(), WasmError> {
    Ok(for (global_idx, global) in &wasm_module.globals {
        let global_name = wasm_module
            .name_section
            .globals_names
            .get(&global_idx)
            .cloned()
            .unwrap_or(format!("gv{}", global_idx.as_u32()));
        let global_init = wasm_module.try_global_initializer(global_idx, diagnostics)?;
        let init = ConstantData::from(global_init.to_le_bytes(&wasm_module, diagnostics)?);
        if let Err(e) = module_builder.declare_global_variable(
            &global_name,
            ir_type(global.ty.clone())?,
            Linkage::External,
            Some(init.clone()),
            SourceSpan::default(),
        ) {
            let message = format!(
                "Failed to declare global variable '{global_name}' with initializer '{init}' with \
                 error: {:?}",
                e
            );
            diagnostics
                .diagnostic(miden_diagnostics::Severity::Error)
                .with_message(message.clone())
                .emit();
            return Err(WasmError::Unexpected(message));
        }
    })
}

fn build_data_segments(
    translation: &ParsedModule,
    module_builder: &mut ModuleBuilder,
    diagnostics: &DiagnosticsHandler,
) -> Result<(), WasmError> {
    for (data_segment_idx, data_segment) in &translation.data_segments {
        let data_segment_name =
            translation.module.name_section.data_segment_names[&data_segment_idx].clone();
        let readonly = data_segment_name.contains(".rodata");
        let init = ConstantData::from(data_segment.data);
        let offset = data_segment.offset.as_i32(&translation.module, diagnostics)? as u32;
        let size = init.len() as u32;
        if let Err(e) = module_builder.declare_data_segment(offset, size, init, readonly) {
            let message = format!(
                "Failed to declare data segment '{data_segment_name}' with size '{size}' at \
                 '{offset}' with error: {:?}",
                e
            );
            diagnostics
                .diagnostic(miden_diagnostics::Severity::Error)
                .with_message(message.clone())
                .emit();
            return Err(WasmError::Unexpected(message));
        }
    }
    Ok(())
}
