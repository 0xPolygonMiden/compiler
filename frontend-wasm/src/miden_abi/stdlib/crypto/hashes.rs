use miden_hir::FunctionType;
use miden_hir_type::Type::*;

use crate::miden_abi::{FunctionTypeMap, ModuleFunctionTypeMap};

pub(crate) const BLAKE3_HASH_1TO1: &str = "blake3_hash_1to1";
pub(crate) const BLAKE3_HASH_2TO1: &str = "blake3_hash_2to1";

pub(crate) fn signatures() -> ModuleFunctionTypeMap {
    let mut m: ModuleFunctionTypeMap = Default::default();
    let mut crypto: FunctionTypeMap = Default::default();
    crypto.insert(
        BLAKE3_HASH_1TO1,
        //Accepts and returns a 8 Felt elements
        FunctionType::new_miden(
            [Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt],
            [Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt],
        ),
    );
    crypto.insert(
        BLAKE3_HASH_2TO1,
        // Accepts 16 and returns a 8 Felt elements
        FunctionType::new_miden(
            [
                Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt,
                Felt, Felt,
            ],
            [Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt],
        ),
    );
    m.insert("miden:prelude/std_crypto_hashes", crypto);
    m
}
