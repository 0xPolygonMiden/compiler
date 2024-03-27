use std::{collections::HashSet, sync::OnceLock, vec};

use miden_hir::{FunctionIdent, InstBuilder, SourceSpan, Symbol, Type::*, Value};

use crate::module::function_builder_ext::FunctionBuilderExt;

pub const TYPES_FELT_MODULE_NAME: &str = "miden:types/felt";
pub const TYPES_FELT_ADD: &str = "add";
pub const TYPES_FELT_FROM_U64_UNCHECKED: &str = "from_u64_unchecked";

/// Check if the given module is a Miden module that contains intrinsics
pub fn is_miden_intrinsics_module(module_id: Symbol) -> bool {
    modules().contains(module_id.as_str())
}

fn modules() -> &'static HashSet<&'static str> {
    static MODULES: OnceLock<HashSet<&'static str>> = OnceLock::new();
    MODULES.get_or_init(|| {
        let mut s = HashSet::default();
        s.insert(TYPES_FELT_MODULE_NAME);
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
        TYPES_FELT_MODULE_NAME => match func_id.function.as_symbol().as_str() {
            TYPES_FELT_ADD => {
                assert_eq!(args.len(), 2, "add takes exactly two arguments");
                let inst = builder.ins().add_unchecked(args[0], args[1], span);
                vec![inst]
            }
            TYPES_FELT_FROM_U64_UNCHECKED => {
                assert_eq!(args.len(), 1, "from_u64_unchecked takes exactly one argument");
                let inst = builder.ins().cast(args[0], Felt, span);
                vec![inst]
            }
            _ => panic!("No intrinsics found for {}", func_id),
        },
        _ => panic!("No intrinsics found for {}", func_id),
    }
}
