use midenc_hir::FunctionType;
use midenc_hir_type::Type::*;

use crate::miden_abi::{FunctionTypeMap, ModuleFunctionTypeMap};

pub const MODULE_ID: &str = "miden:core-import/account@1.0.0";

pub const ADD_ASSET: &str = "add-asset";
pub const REMOVE_ASSET: &str = "remove-asset";
pub const GET_ID: &str = "get-id";

pub(crate) fn signatures() -> ModuleFunctionTypeMap {
    let mut m: ModuleFunctionTypeMap = Default::default();
    let mut account: FunctionTypeMap = Default::default();
    account
        .insert(ADD_ASSET, FunctionType::new([Felt, Felt, Felt, Felt], [Felt, Felt, Felt, Felt]));
    account.insert(
        REMOVE_ASSET,
        FunctionType::new([Felt, Felt, Felt, Felt], [Felt, Felt, Felt, Felt]),
    );
    account.insert(GET_ID, FunctionType::new([], [Felt]));
    m.insert(MODULE_ID, account);
    m
}
