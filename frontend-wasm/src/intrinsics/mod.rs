mod felt;
mod mem;

use std::{collections::HashSet, sync::OnceLock};

use midenc_hir::{FunctionIdent, SourceSpan, Symbol, Value};

use crate::module::function_builder_ext::FunctionBuilderExt;

/// Check if the given module is a Miden module that contains intrinsics
pub fn is_miden_intrinsics_module(module_id: Symbol) -> bool {
    modules().contains(module_id.as_str())
}

fn modules() -> &'static HashSet<&'static str> {
    static MODULES: OnceLock<HashSet<&'static str>> = OnceLock::new();
    MODULES.get_or_init(|| {
        let mut s = HashSet::default();
        s.insert("intrinsics::mem");
        s.insert(felt::INTRINSICS_FELT_MODULE_NAME);
        s
    })
}

/// Convert a call to a Miden intrinsic function into instruction(s)
pub fn convert_intrinsics_call(
    func_id: FunctionIdent,
    args: &[Value],
    builder: &mut FunctionBuilderExt,
    span: SourceSpan,
) -> Vec<Value> {
    match func_id.module.as_symbol().as_str() {
        "intrinsics::mem" => mem::convert_mem_intrinsics(func_id, args, builder, span),
        felt::INTRINSICS_FELT_MODULE_NAME => {
            felt::convert_felt_intrinsics(func_id, args, builder, span)
        }
        _ => panic!("No intrinsics found for {}", func_id),
    }
}
