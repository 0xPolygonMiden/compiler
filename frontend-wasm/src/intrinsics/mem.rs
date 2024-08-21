use midenc_hir::{AbiParam, FunctionIdent, InstBuilder, Signature, SourceSpan, Type, Value};

use crate::module::function_builder_ext::FunctionBuilderExt;

/// Convert a call to a memory intrinsic function
pub(crate) fn convert_mem_intrinsics(
    func_id: FunctionIdent,
    args: &[Value],
    builder: &mut FunctionBuilderExt<'_, '_, '_>,
    span: SourceSpan,
) -> Vec<Value> {
    match func_id.function.as_symbol().as_str() {
        "heap_base" => {
            assert_eq!(args.len(), 0, "{} takes no arguments", func_id);
            if builder
                .data_flow_graph()
                .get_import_by_name(func_id.module, func_id.function)
                .is_none()
            {
                let signature = Signature::new([], [AbiParam::new(Type::U32)]);
                let _ = builder.data_flow_graph_mut().import_function(
                    func_id.module,
                    func_id.function,
                    signature,
                );
            }
            let call = builder.ins().call(func_id, &[], span);
            let value = builder.data_flow_graph().first_result(call);
            vec![value]
        }
        _ => panic!("No allowed memory intrinsics found for {}", func_id),
    }
}
