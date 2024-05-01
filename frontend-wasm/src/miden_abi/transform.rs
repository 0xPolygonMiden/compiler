use miden_diagnostics::DiagnosticsHandler;
use miden_hir::{FunctionIdent, Immediate, InstBuilder, SourceSpan, Type::*, Value};

use super::{stdlib, tx_kernel};
use crate::module::function_builder_ext::FunctionBuilderExt;

/// The strategy to use for transforming a function call
enum TransformStrategy {
    /// The Miden ABI function returns a length and a pointer and we only want the length
    ListReturn,
    /// The Miden ABI function returns on the stack and we want to return via a pointer argument
    ReturnViaPointer,
    /// No transformation needed
    NoTransform,
}

/// Get the transformation strategy for a function name
fn get_transform_strategy(function_id: &str) -> TransformStrategy {
    match function_id {
        tx_kernel::note::GET_INPUTS => TransformStrategy::ListReturn,
        tx_kernel::account::ADD_ASSET => TransformStrategy::ReturnViaPointer,
        tx_kernel::account::REMOVE_ASSET => TransformStrategy::ReturnViaPointer,
        tx_kernel::account::GET_ID => TransformStrategy::NoTransform,
        tx_kernel::tx::CREATE_NOTE => TransformStrategy::NoTransform,
        stdlib::crypto::hashes::BLAKE3_HASH_1TO1 => TransformStrategy::ReturnViaPointer,
        stdlib::crypto::hashes::BLAKE3_HASH_2TO1 => TransformStrategy::ReturnViaPointer,
        stdlib::crypto::dsa::RPO_FALCON512_VERIFY => TransformStrategy::NoTransform,
        stdlib::mem::PIPE_WORDS_TO_MEMORY => TransformStrategy::ReturnViaPointer,
        stdlib::mem::PIPE_DOUBLE_WORDS_TO_MEMORY => TransformStrategy::ReturnViaPointer,
        _ => panic!("No transform strategy found for function {}", function_id),
    }
}

/// Transform a function call based on the transformation strategy
pub fn transform_miden_abi_call(
    func_id: FunctionIdent,
    args: &[Value],
    builder: &mut FunctionBuilderExt,
    span: SourceSpan,
    diagnostics: &DiagnosticsHandler,
) -> Vec<Value> {
    use TransformStrategy::*;
    match get_transform_strategy(func_id.function.as_symbol().as_str()) {
        ListReturn => list_return(func_id, args, builder, span, diagnostics),
        ReturnViaPointer => return_via_pointer(func_id, args, builder, span, diagnostics),
        NoTransform => no_transform(func_id, args, builder, span, diagnostics),
    }
}

/// No transformation needed
#[inline(always)]
pub fn no_transform(
    func_id: FunctionIdent,
    args: &[Value],
    builder: &mut FunctionBuilderExt,
    span: SourceSpan,
    _diagnostics: &DiagnosticsHandler,
) -> Vec<Value> {
    let call = builder.ins().call(func_id, args, span);
    let results = builder.inst_results(call);
    results.to_vec()
}

/// The Miden ABI function returns a length and a pointer and we only want the length
pub fn list_return(
    func_id: FunctionIdent,
    args: &[Value],
    builder: &mut FunctionBuilderExt,
    span: SourceSpan,
    _diagnostics: &DiagnosticsHandler,
) -> Vec<Value> {
    let call = builder.ins().call(func_id, args, span);
    let results = builder.inst_results(call);
    assert_eq!(results.len(), 2, "List return strategy expects 2 results: length and pointer");
    // Return the first result (length) only
    results[0..1].to_vec()
}

/// The Miden ABI function returns felts on the stack and we want to return via a pointer argument
pub fn return_via_pointer(
    func_id: FunctionIdent,
    args: &[Value],
    builder: &mut FunctionBuilderExt,
    span: SourceSpan,
    _diagnostics: &DiagnosticsHandler,
) -> Vec<Value> {
    // Omit the last argument (pointer)
    let args_wo_pointer = &args[0..args.len() - 1];
    let call = builder.ins().call(func_id, args_wo_pointer, span);
    let results = builder.inst_results(call).to_vec();
    let ptr_arg = *args.last().unwrap();
    let ptr_arg_ty = builder.data_flow_graph().value_type(ptr_arg).clone();
    assert_eq!(ptr_arg_ty, I32);
    let ptr_u32 = builder.ins().cast(ptr_arg, U32, span);
    for (idx, value) in results.iter().enumerate() {
        let value_ty = builder.data_flow_graph().value_type(*value);
        assert_eq!(value_ty, &Felt, "In return_via_pointer, expected only Felt value type returns");
        let eff_ptr = if idx == 0 {
            ptr_u32
        } else {
            // We're storing the stack-returned felts(64-bit) values in the
            // memory that from the Rust "side" point of view is byte-addressed,
            // meaning that for example in array of felts the second felt is
            // expected to be +8 from the first and so on. So we need to
            // multiply the index by 8 so that the subsequent Rust code finds
            // the values in the expected locations.
            let imm = Immediate::I32(idx as i32 * 8);
            builder.ins().add_imm_checked(ptr_u32, imm, span)
        };
        let value_ty = builder.data_flow_graph().value_type(*value).clone();
        let addr = builder.ins().inttoptr(eff_ptr, Ptr(value_ty.into()), span);
        builder.ins().store(addr, *value, span);
    }
    Vec::new()
}
