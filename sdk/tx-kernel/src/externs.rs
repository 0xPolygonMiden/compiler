use miden_prelude::Felt;

use crate::{AccountId, CoreAsset};

#[link(wasm_import_module = "miden:tx_kernel/account")]
extern "C" {
    #[link_name = "get_id<0x0000000000000000000000000000000000000000000000000000000000000000>"]
    pub fn extern_account_get_id() -> AccountId;
    #[link_name = "add_asset<0x0000000000000000000000000000000000000000000000000000000000000000>"]
    pub fn extern_account_add_asset(_: Felt, _: Felt, _: Felt, _: Felt, ptr: *mut CoreAsset);
    #[link_name = "remove_asset<0x0000000000000000000000000000000000000000000000000000000000000000>"]
    pub fn extern_account_remove_asset(_: Felt, _: Felt, _: Felt, _: Felt, ptr: *mut CoreAsset);
}

#[link(wasm_import_module = "miden:tx_kernel/note")]
extern "C" {
    #[link_name = "get_inputs<0x0000000000000000000000000000000000000000000000000000000000000000>"]
    pub fn extern_note_get_inputs(ptr: *mut Felt) -> usize;
}
