use miden_stdlib_sys::Felt;

use crate::bindings::tx::{AccountId, CoreAsset, NoteId, NoteType, Tag};

#[link(wasm_import_module = "miden:core-import/account@1.0.0")]
extern "C" {
    #[link_name = "get-id"]
    pub fn extern_account_get_id() -> AccountId;
    #[link_name = "add-asset"]
    pub fn extern_account_add_asset(_: Felt, _: Felt, _: Felt, _: Felt, ptr: *mut CoreAsset);
    #[link_name = "remove-asset"]
    pub fn extern_account_remove_asset(_: Felt, _: Felt, _: Felt, _: Felt, ptr: *mut CoreAsset);
}

#[link(wasm_import_module = "miden:core-import/note@1.0.0")]
extern "C" {
    #[link_name = "get-inputs"]
    pub fn extern_note_get_inputs(ptr: *mut Felt) -> usize;
}

#[link(wasm_import_module = "miden:core-import/tx@1.0.0")]
extern "C" {
    #[link_name = "create-note"]
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
