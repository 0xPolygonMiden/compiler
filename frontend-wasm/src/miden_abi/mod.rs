pub(crate) mod stdlib;
pub(crate) mod transform;
pub(crate) mod tx_kernel;

use midenc_hir::{FunctionType, Symbol};
use rustc_hash::FxHashMap;

pub(crate) type FunctionTypeMap = FxHashMap<&'static str, FunctionType>;
pub(crate) type ModuleFunctionTypeMap = FxHashMap<&'static str, FunctionTypeMap>;

pub fn is_miden_abi_module(module_id: Symbol) -> bool {
    is_miden_stdlib_module(module_id) || is_miden_sdk_module(module_id)
}

pub fn miden_abi_function_type(module_id: Symbol, function_id: Symbol) -> FunctionType {
    if is_miden_stdlib_module(module_id) {
        miden_stdlib_function_type(module_id, function_id)
    } else {
        miden_sdk_function_type(module_id, function_id)
    }
}

fn is_miden_sdk_module(module_id: Symbol) -> bool {
    tx_kernel::signatures().contains_key(module_id.as_str())
}

/// Get the target Miden ABI tx kernel function type for the given module and function id
pub fn miden_sdk_function_type(module_id: Symbol, function_id: Symbol) -> FunctionType {
    let funcs = tx_kernel::signatures()
        .get(module_id.as_str())
        .unwrap_or_else(|| panic!("No Miden ABI function types found for module {}", module_id));
    funcs.get(function_id.as_str()).cloned().unwrap_or_else(|| {
        panic!(
            "No Miden ABI function type found for function {} in module {}",
            function_id, module_id
        )
    })
}

fn is_miden_stdlib_module(module_id: Symbol) -> bool {
    stdlib::signatures().contains_key(module_id.as_str())
}

/// Get the target Miden ABI stdlib function type for the given module and function id
#[inline(always)]
fn miden_stdlib_function_type(module_id: Symbol, function_id: Symbol) -> FunctionType {
    let funcs = stdlib::signatures()
        .get(module_id.as_str())
        .unwrap_or_else(|| panic!("No Miden ABI function types found for module {}", module_id));
    funcs.get(function_id.as_str()).cloned().unwrap_or_else(|| {
        panic!(
            "No Miden ABI function type found for function {} in module {}",
            function_id, module_id
        )
    })
}
