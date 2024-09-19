use midenc_hir::FunctionType;
use midenc_hir_type::Type::*;

use crate::miden_abi::{FunctionTypeMap, ModuleFunctionTypeMap};

pub const MODULE_ID: &str = "miden:core-import/stdlib-crypto-hashes@1.0.0";

pub(crate) const BLAKE3_HASH_1TO1: &str = "blake3-hash-one-to-one";
pub(crate) const BLAKE3_HASH_2TO1: &str = "hash_2to1";

pub(crate) fn signatures() -> ModuleFunctionTypeMap {
    let mut m: ModuleFunctionTypeMap = Default::default();
    let mut blake3: FunctionTypeMap = Default::default();
    blake3.insert(
        BLAKE3_HASH_1TO1,
        FunctionType::new(
            [I32, I32, I32, I32, I32, I32, I32, I32],
            [I32, I32, I32, I32, I32, I32, I32, I32],
        ),
    );
    blake3.insert(
        BLAKE3_HASH_2TO1,
        FunctionType::new(
            [I32, I32, I32, I32, I32, I32, I32, I32, I32, I32, I32, I32, I32, I32, I32, I32],
            [I32, I32, I32, I32, I32, I32, I32, I32],
        ),
    );
    m.insert(MODULE_ID, blake3);
    m
}
