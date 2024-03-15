//! Function types and lowering for tx kernel API functions
#![allow(missing_docs)]

use std::sync::OnceLock;

use miden_hir_type::{MidenAbiFunctionType, Type::*};
use rustc_hash::FxHashMap;

pub const ACCOUNT_MODULE_NAME: &str = "miden:tx_kernel/account";
pub const NOTE_MODULE_NAME: &str = "miden:tx_kernel/note";

pub const NOTE_GET_INPUTS: &str = "get_inputs";
pub const ACCOUNT_ADD_ASSET: &str = "add_asset";
pub const ACCOUNT_GET_ID: &str = "get_id";

type FunctionTypeMap = FxHashMap<String, MidenAbiFunctionType>;
type ModuleFunctionTypeMap = FxHashMap<String, FunctionTypeMap>;

fn types() -> &'static ModuleFunctionTypeMap {
    static TYPES: OnceLock<ModuleFunctionTypeMap> = OnceLock::new();
    TYPES.get_or_init(|| {
        let mut m: ModuleFunctionTypeMap = Default::default();
        let mut note: FunctionTypeMap = Default::default();
        let mut account: FunctionTypeMap = Default::default();
        note.insert(
            NOTE_GET_INPUTS.to_string(),
            MidenAbiFunctionType::new([I32], [I32, I32]),
        );
        account.insert(
            ACCOUNT_ADD_ASSET.to_string(),
            // Accepts and returns word
            MidenAbiFunctionType::new([I32, I32, I32, I32], [I32, I32, I32, I32]),
        );
        account.insert(
            ACCOUNT_GET_ID.to_string(),
            MidenAbiFunctionType::new([], [I32]),
        );
        m.insert(NOTE_MODULE_NAME.to_string(), note);
        m.insert(ACCOUNT_MODULE_NAME.to_string(), account);
        m
    })
}

/// Get the target tx kernel function type for the given function id
#[inline(always)]
pub fn miden_abi_function_type(module_id: &str, function_id: &str) -> MidenAbiFunctionType {
    let funcs = types()
        .get(module_id)
        .expect(format!("No Miden ABI function types found for module {}", module_id).as_str());
    funcs.get(function_id).cloned().expect(
        format!(
            "No Miden ABI function type found for function {} in module {}",
            function_id, module_id
        )
        .as_str(),
    )
}
