//! Function types and lowered signatures for the Miden stdlib API functions
#![allow(missing_docs)]

use std::sync::OnceLock;

use miden_hir::FunctionType;
use miden_hir_type::Type::*;

use super::ModuleFunctionTypeMap;
use crate::miden_abi::FunctionTypeMap;

pub const CRYPTO_HASHES_MODULE_NAME: &str = "miden:stdlib/crypto_hashes";
pub const BLAKE3_HASH_1TO1: &str = "blake3_hash_1to1";
pub const BLAKE3_HASH_2TO1: &str = "blake3_hash_2to1";

pub(crate) fn types() -> &'static ModuleFunctionTypeMap {
    static TYPES: OnceLock<ModuleFunctionTypeMap> = OnceLock::new();
    TYPES.get_or_init(|| {
        let mut m: ModuleFunctionTypeMap = Default::default();
        let mut crypto: FunctionTypeMap = Default::default();
        crypto.insert(
            BLAKE3_HASH_1TO1,
            // Accepts and returns a 8 Felt elements
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
                    Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt,
                    Felt, Felt, Felt,
                ],
                [Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt],
            ),
        );
        m.insert(CRYPTO_HASHES_MODULE_NAME, crypto);

        m
    })
}
