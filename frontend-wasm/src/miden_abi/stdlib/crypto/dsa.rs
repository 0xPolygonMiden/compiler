use midenc_hir::FunctionType;
use midenc_hir_type::Type::*;

use crate::miden_abi::{FunctionTypeMap, ModuleFunctionTypeMap};

pub(crate) const MODULE_ID: &str = "miden:core-import/stdlib-crypto-dsa@1.0.0";

pub(crate) const RPO_FALCON512_VERIFY: &str = "rpo-falcon512-verify";

pub(crate) fn signatures() -> ModuleFunctionTypeMap {
    let mut m: ModuleFunctionTypeMap = Default::default();
    let mut funcs: FunctionTypeMap = Default::default();
    funcs.insert(
        RPO_FALCON512_VERIFY,
        FunctionType::new([Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt], []),
    );
    m.insert(MODULE_ID, funcs);
    m
}
