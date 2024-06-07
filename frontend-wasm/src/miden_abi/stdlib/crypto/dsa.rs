use midenc_hir::FunctionType;
use midenc_hir_type::Type::*;

use crate::miden_abi::{FunctionTypeMap, ModuleFunctionTypeMap};

pub(crate) const RPO_FALCON512_VERIFY: &str = "rpo_falcon512_verify";

pub(crate) fn signatures() -> ModuleFunctionTypeMap {
    let mut m: ModuleFunctionTypeMap = Default::default();
    let mut funcs: FunctionTypeMap = Default::default();
    funcs.insert(
        RPO_FALCON512_VERIFY,
        FunctionType::new([Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt], []),
    );
    m.insert("miden:prelude/std_crypto_dsa".to_string(), funcs);
    m
}
