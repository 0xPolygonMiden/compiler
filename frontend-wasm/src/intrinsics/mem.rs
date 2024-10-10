use midenc_hir::{
    AbiParam, FunctionIdent, FunctionType, InstBuilder, Signature, SourceSpan, Type, Value,
};

use crate::module::function_builder_ext::FunctionBuilderExt;

pub const MODULE_ID: &str = "intrinsics::mem";

pub const HEAP_BASE: &str = "heap_base";

const HEAP_BASE_FUNC: ([Type; 0], [Type; 1]) = ([], [Type::U32]);

pub fn function_type(func_id: &FunctionIdent) -> FunctionType {
    match func_id.function.as_symbol().as_str() {
        HEAP_BASE => FunctionType::new(HEAP_BASE_FUNC.0, HEAP_BASE_FUNC.1),
        _ => panic!("No memory intrinsics FunctionType found for {}", func_id),
    }
}

fn signature(func_id: &FunctionIdent) -> Signature {
    match func_id.function.as_symbol().as_str() {
        HEAP_BASE => {
            Signature::new(HEAP_BASE_FUNC.0.map(AbiParam::new), HEAP_BASE_FUNC.1.map(AbiParam::new))
        }
        _ => panic!("No memory intrinsics Signature found for {}", func_id),
    }
}

/// Convert a call to a memory intrinsic function
pub(crate) fn convert_mem_intrinsics(
    func_id: FunctionIdent,
    args: &[Value],
    builder: &mut FunctionBuilderExt<'_, '_, '_>,
    span: SourceSpan,
) -> Vec<Value> {
    match func_id.function.as_symbol().as_str() {
        HEAP_BASE => {
            assert_eq!(args.len(), 0, "{} takes no arguments", func_id);
            if builder
                .data_flow_graph()
                .get_import_by_name(func_id.module, func_id.function)
                .is_none()
            {
                let signature = signature(&func_id);
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
