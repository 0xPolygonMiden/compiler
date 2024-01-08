use miden_diagnostics::{DiagnosticsHandler, SourceSpan};
use miden_hir::{CallConv, ConstantData, Linkage, ModuleBuilder};
use wasmparser::{Validator, WasmFeatures};

use crate::{
    error::WasmResult,
    module::func_translator::FuncTranslator,
    module::module_env::{FunctionBodyData, ModuleEnvironment, ModuleTranslation},
    module::types::{ir_func_sig, ir_func_type, ir_type, ModuleTypes},
    WasmError, WasmTranslationConfig,
};

use super::Module;

// TODO: rename to `translate_core_module`?
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
    let translation = ModuleEnvironment::new(config, &mut validator, &mut module_types_builder)
        .translate(parser, wasm)?;
    let module_types = module_types_builder.finish();
    build_ir_module(translation, module_types, config, diagnostics)
}

fn build_ir_module(
    mut translation: ModuleTranslation,
    module_types: ModuleTypes,
    config: &WasmTranslationConfig,
    diagnostics: &DiagnosticsHandler,
) -> WasmResult<miden_hir::Module> {
    if translation.module.name_section.module_name.is_none() {
        translation.module.name_section.module_name = Some(config.module_name_fallback.clone());
    }
    let wasm_module = translation.module;
    let name = wasm_module
        .name_section
        .module_name
        .clone()
        .expect("Module name should be set by this point");
    let mut module_builder = ModuleBuilder::new(name.as_str());
    build_globals(&wasm_module, &mut module_builder, diagnostics)?;
    // TODO: add support for data segments
    // build_data_segments(&wasm_module, &mut module_builder, diagnostics)?;
    let get_num_func_imports = wasm_module.num_imported_funcs;
    let mut func_translator = FuncTranslator::new();
    for (defined_func_idx, body_data) in translation.function_body_inputs {
        let func_index = wasm_module.func_index(defined_func_idx);
        let func_type = wasm_module.functions[func_index];
        let func_name = wasm_module
            .name_section
            .func_names
            .get(&func_index)
            .cloned()
            .unwrap_or(format!("func{}", func_index.as_u32()));
        let wasm_func_type = module_types[func_type.signature].clone();
        let ir_func_type = ir_func_type(&wasm_func_type)?;
        let sig = ir_func_sig(&ir_func_type, CallConv::SystemV, Linkage::External);
        let mut module_func_builder = module_builder.function(func_name.as_str(), sig.clone())?;
        let FunctionBodyData { validator, body } = body_data;
        let mut func_validator = validator.into_validator(Default::default());
        func_translator.translate_body(
            &body,
            &mut module_func_builder,
            &wasm_module,
            &module_types,
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
        let defined_global_idx = wasm_module
            .defined_global_index(global_idx)
            .expect("No initializer for imported global variable");
        let global_name = wasm_module
            .name_section
            .globals_names
            .get(&global_idx)
            .cloned()
            .unwrap_or(format!("gv{}", global_idx.as_u32()));
        let global_init = wasm_module.global_initializers[defined_global_idx];
        let init = ConstantData::from(global_init.to_le_bytes(&wasm_module));
        if let Err(e) = module_builder.declare_global_variable(
            &global_name,
            ir_type(global.ty.clone())?,
            Linkage::External,
            Some(init.clone()),
            SourceSpan::default(),
        ) {
            let message = format!("Failed to declare global variable '{global_name}' with initializer '{init}' with error: {:?}", e);
            diagnostics
                .diagnostic(miden_diagnostics::Severity::Error)
                .with_message(message.clone())
                .emit();
            return Err(WasmError::Unexpected(message));
        }
    })
}

fn build_data_segments(
    wasm_module: &Module,
    module_builder: &mut ModuleBuilder,
    diagnostics: &DiagnosticsHandler,
) -> Result<(), WasmError> {
    todo!("data segments are not supported yet");
    // TODO: enable frontend tests for data segments (rust_array, rust_static_mut)
    // for (data_segment_idx, data_segment) in &self.data_segments {
    //     let data_segment_name = self.data_segment_names[data_segment_idx].clone();
    //     let readonly = data_segment_name.contains(".rodata");
    //     let init = ConstantData::from(data_segment.data);
    //     let offset = data_segment
    //         .offset
    //         .as_i32(&self.info.globals, diagnostics)? as u32;
    //     let size = init.len() as u32;
    //     if let Err(e) = module_builder.declare_data_segment(offset, size, init, readonly) {
    //         let message = format!("Failed to declare data segment '{data_segment_name}' with size '{size}' at '{offset}' with error: {:?}", e);
    //         diagnostics
    //             .diagnostic(miden_diagnostics::Severity::Error)
    //             .with_message(message.clone())
    //             .emit();
    //         return Err(WasmError::Unexpected(message));
    //     }
    // }
    // Ok(())
}
