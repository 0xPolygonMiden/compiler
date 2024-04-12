use miden_prelude::Felt;

use crate::{AccountId, CoreAsset, NoteId, NoteType, Tag};

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

#[link(wasm_import_module = "miden:tx_kernel/tx")]
extern "C" {
    #[link_name = "create_note<0x0000000000000000000000000000000000000000000000000000000000000000>"]
    pub fn extern_tx_create_note(
        asset_f0: Felt,
        asset_f1: Felt,
        asset_f2: Felt,
        asset_f3: Felt,
        tag: Tag,
        note_type: NoteType,
        recipient_f0: Felt,
        recipient_f1: Felt,
        recipient_f2: Felt,
        recipient_f3: Felt,
    ) -> NoteId;
}
