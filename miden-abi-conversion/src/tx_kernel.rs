//! Function types and lowering for tx kernel API functions
use std::sync::OnceLock;

use miden_hir_type::{MidenAbiFunctionType, Type::*};
use rustc_hash::FxHashMap;

fn types() -> &'static FxHashMap<String, MidenAbiFunctionType> {
    static TYPES: OnceLock<FxHashMap<String, MidenAbiFunctionType>> = OnceLock::new();
    TYPES.get_or_init(|| {
        let mut m: FxHashMap<String, MidenAbiFunctionType> = Default::default();
        m.insert(
            "miden:tx_kernel/note.get_inputs".to_string(),
            MidenAbiFunctionType::new([I32], [I32, I32]),
        );
        m
    })
}

/// Get the target tx kernel function type for the given function id
#[inline(always)]
pub fn miden_abi_function_type(_function_id: &str) -> MidenAbiFunctionType {
    types().get(_function_id).cloned().expect(
        format!(
            "No Miden ABI function type found for function {}",
            _function_id
        )
        .as_str(),
    )
}
