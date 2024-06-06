mod felt;

use std::{collections::HashSet, sync::OnceLock};

use midenc_hir::{FunctionIdent, SourceSpan, Symbol, Value};

use crate::{module::function_builder_ext::FunctionBuilderExt, translation_utils::sanitize_name};

/// Check if the given module is a Miden module that contains intrinsics
pub fn is_miden_intrinsics_module(module_id: Symbol) -> bool {
    modules().contains(module_id.as_str())
}

fn modules() -> &'static HashSet<String> {
    static MODULES: OnceLock<HashSet<String>> = OnceLock::new();
    MODULES.get_or_init(|| {
        let mut s: HashSet<&'static str> = HashSet::default();
        s.insert(felt::PRELUDE_INTRINSICS_FELT_MODULE_NAME);
        s.into_iter().map(sanitize_name).collect()
    })
}

/// Convert a call to a Miden intrinsic function into instruction(s)
pub fn convert_intrinsics_call(
    func_id: FunctionIdent,
    args: &[Value],
    builder: &mut FunctionBuilderExt,
    span: SourceSpan,
) -> Vec<Value> {
    if func_id.module.as_symbol().as_str()
        == sanitize_name(felt::PRELUDE_INTRINSICS_FELT_MODULE_NAME)
    {
        felt::convert_felt_intrinsics(func_id, args, builder, span)
    } else {
        panic!("No intrinsics found for {}", func_id)
    }
}
