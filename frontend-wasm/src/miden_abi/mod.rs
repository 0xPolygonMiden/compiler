#![allow(unused_variables)]

use miden_abi_conversion::tx_kernel;
use miden_core::crypto::hash::RpoDigest;
use miden_diagnostics::DiagnosticsHandler;
use miden_hir::{FunctionIdent, InstBuilder, SourceSpan, Type::*, Value};
use thiserror::Error;

use crate::{
    module::{
        function_builder_ext::FunctionBuilderExt, module_tratnslation_state::ModuleTranslationState,
    },
    WasmError,
};

pub fn parse_import_function_digest(import_name: &str) -> Result<(String, RpoDigest), String> {
    // parse the hex encoded digest from the function name in the angle brackets
    // and the function name (before the angle brackets) example:
    // "miden:tx_kernel/note.get_inputs<0x0000000000000000000000000000>"
    let mut parts = import_name.split('<');
    let function_name = parts.next().unwrap();
    let digest = parts
        .next()
        .and_then(|s| s.strip_suffix('>'))
        .ok_or_else(|| "Import name parsing error: missing closing angle bracket in import name")?;
    Ok((
        function_name.to_string(),
        RpoDigest::try_from(digest).map_err(|e| e.to_string())?,
    ))
}

#[derive(Error, Debug)]
pub enum AdapterError {}

impl From<AdapterError> for WasmError {
    fn from(value: AdapterError) -> Self {
        WasmError::Unexpected(format!("Adapter generation error: {}", value))
    }
}

// TODO: never fails?
pub fn adapt_call<'a, 'b, 'c: 'b, 'd>(
    func_id: FunctionIdent,
    args: &[Value],
    module_state: &ModuleTranslationState,
    // state: &'a FuncTranslationState,
    builder: &'d mut FunctionBuilderExt<'a, 'b, 'c>,
    span: SourceSpan,
    diagnostics: &DiagnosticsHandler,
) -> Result<&'d [Value], AdapterError> {
    if let Some(stable_import_func_name) =
        module_state.get_stable_imported_miden_abi_function(&func_id)
    {
        Ok(transform_call(
            func_id,
            args,
            &stable_import_func_name,
            builder,
            span,
            diagnostics,
        ))
    } else {
        // no transformation needed
        let call = builder.ins().call(func_id, &args, span);
        let inst_results = builder.inst_results(call);
        Ok(inst_results)
    }
}

enum TransformStrategy {
    ListReturn,
    ReturnViaPointer,
    NoTransform,
}

fn get_transform_strategy(function_id: &str) -> TransformStrategy {
    match function_id {
        tx_kernel::NOTE_GET_INPUTS => TransformStrategy::ListReturn,
        tx_kernel::ACCOUNT_ADD_ASSET => TransformStrategy::ReturnViaPointer,
        tx_kernel::ACCOUNT_GET_ID => TransformStrategy::NoTransform,
        _ => panic!("No transform strategy found for function {}", function_id),
    }
}

pub fn transform_call<'a, 'b, 'c: 'b, 'd>(
    func_id: FunctionIdent,
    args: &[Value],
    stable_name: &str,
    builder: &'d mut FunctionBuilderExt<'a, 'b, 'c>,
    span: SourceSpan,
    diagnostics: &DiagnosticsHandler,
) -> &'d [Value] {
    use TransformStrategy::*;
    match get_transform_strategy(stable_name) {
        ListReturn => list_return(func_id, args, builder, span, diagnostics),
        ReturnViaPointer => return_via_pointer(func_id, args, builder, span, diagnostics),
        NoTransform => no_transform(func_id, args, builder, span, diagnostics),
    }
}

pub fn no_transform<'a, 'b, 'c: 'b, 'd>(
    func_id: FunctionIdent,
    args: &[Value],
    builder: &'d mut FunctionBuilderExt<'a, 'b, 'c>,
    span: SourceSpan,
    diagnostics: &DiagnosticsHandler,
) -> &'d [Value] {
    let call = builder.ins().call(func_id, args, span);
    let results = builder.inst_results(call);
    results
}

pub fn list_return<'a, 'b, 'c: 'b, 'd>(
    func_id: FunctionIdent,
    args: &[Value],
    builder: &'d mut FunctionBuilderExt<'a, 'b, 'c>,
    span: SourceSpan,
    diagnostics: &DiagnosticsHandler,
) -> &'d [Value] {
    let call = builder.ins().call(func_id, args, span);
    let results = builder.inst_results(call);
    assert_eq!(
        results.len(),
        2,
        "List return strategy expects 2 results: length and pointer"
    );
    // Return the first result (length) only
    results[0..1].as_ref()
}

pub fn return_via_pointer<'a, 'b, 'c: 'b, 'd>(
    func_id: FunctionIdent,
    args: &[Value],
    builder: &'d mut FunctionBuilderExt<'a, 'b, 'c>,
    span: SourceSpan,
    diagnostics: &DiagnosticsHandler,
) -> &'d [Value] {
    // Omit the last argument (pointer)
    let args_wo_pointer = &args[0..args.len() - 1];
    let call = builder.ins().call(func_id, args_wo_pointer, span);
    let results = builder.inst_results(call).to_vec();
    let ptr = *args.last().unwrap();
    let ptr_ty = builder.data_flow_graph().value_type(ptr).clone();
    assert_eq!(ptr_ty, I32, "Expected pointer type to be i32");
    let ptr_u32 = builder.ins().cast(ptr, U32, span);
    for (idx, value) in results.iter().enumerate() {
        let eff_ptr = if idx == 0 {
            ptr_u32
        } else {
            builder
                .ins()
                .add_imm_checked(ptr_u32, miden_hir::Immediate::I32(idx as i32), span)
        };
        let addr = builder
            .ins()
            .inttoptr(eff_ptr, Ptr(ptr_ty.clone().into()), span);
        builder.ins().store(addr, *value, span);
    }
    &[]
}
