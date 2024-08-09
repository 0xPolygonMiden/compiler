use midenc_hir::FunctionType;
use midenc_hir_type::Type::*;

use crate::miden_abi::{FunctionTypeMap, ModuleFunctionTypeMap};

pub(crate) const BLAKE3_HASH_1TO1: &str = "hash_1to1";
pub(crate) const BLAKE3_HASH_2TO1: &str = "hash_2to1";

pub(crate) fn signatures() -> ModuleFunctionTypeMap {
    let mut m: ModuleFunctionTypeMap = Default::default();
    let mut blake3: FunctionTypeMap = Default::default();
    blake3.insert(
        BLAKE3_HASH_1TO1,
        //Accepts and returns a 8 Felt elements
        FunctionType::new(
            [I32, I32, I32, I32, I32, I32, I32, I32],
            [I32, I32, I32, I32, I32, I32, I32, I32],
        ),
    );
    blake3.insert(
        BLAKE3_HASH_2TO1,
        // Accepts 16 and returns a 8 Felt elements
        FunctionType::new(
            [I32, I32, I32, I32, I32, I32, I32, I32, I32, I32, I32, I32, I32, I32, I32, I32],
            [I32, I32, I32, I32, I32, I32, I32, I32],
        ),
    );
    m.insert("std::crypto::hashes::blake3", blake3);
    m
}
