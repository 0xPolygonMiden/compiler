#![no_std]

mod externs;
use externs::*;

mod types;
pub use types::*;

extern crate alloc;

use alloc::vec::Vec;

use miden_stdlib_sys::Felt;

/// Get the account ID of the currently executing note account.
pub fn get_id() -> AccountId {
    unsafe { extern_account_get_id() }
}

/// Get the inputs of the currently executing note.
pub fn get_inputs() -> Vec<Felt> {
    const MAX_INPUTS: usize = 256;
    let mut inputs: Vec<Felt> = Vec::with_capacity(MAX_INPUTS);
    let num_inputs = unsafe {
        // The MASM for this function is here:
        // https://github.com/0xPolygonMiden/miden-base/blob/3cbe8d59dcf4ccc9c380b7c8417ac6178fc6b86a/miden-lib/asm/miden/note.masm#L69-L102
        // #! Writes the inputs of the currently execute note into memory starting at the specified
        // address. #!
        // #! Inputs: [dest_ptr]
        // #! Outputs: [num_inputs, dest_ptr]
        // #!
        // #! - dest_ptr is the memory address to write the inputs.
        // Compiler generated adapter code at call site will drop the returned dest_ptr
        // and return the number of inputs
        extern_note_get_inputs(inputs.as_mut_ptr())
    };
    inputs.truncate(num_inputs);
    inputs
}

/// Add the specified asset to the vault.
/// Returns the final asset in the account vault defined as follows: If asset is
/// a non-fungible asset, then returns the same as asset. If asset is a
/// fungible asset, then returns the total fungible asset in the account
/// vault after asset was added to it.
///
/// Panics:
/// - If the asset is not valid.
/// - If the total value of two fungible assets is greater than or equal to 2^63.
/// - If the vault already contains the same non-fungible asset.
pub fn add_asset(asset: CoreAsset) -> CoreAsset {
    unsafe {
        let mut ret_area = ::core::mem::MaybeUninit::<CoreAsset>::uninit();
        extern_account_add_asset(
            asset.inner[0],
            asset.inner[1],
            asset.inner[2],
            asset.inner[3],
            ret_area.as_mut_ptr(),
        );
        ret_area.assume_init()
    }
}

/// Remove the specified asset from the vault.
///
/// Panics:
/// - The fungible asset is not found in the vault.
/// - The amount of the fungible asset in the vault is less than the amount to be removed.
/// - The non-fungible asset is not found in the vault.
pub fn remove_asset(asset: CoreAsset) -> CoreAsset {
    unsafe {
        let mut ret_area = ::core::mem::MaybeUninit::<CoreAsset>::uninit();
        extern_account_remove_asset(
            asset.inner[0],
            asset.inner[1],
            asset.inner[2],
            asset.inner[3],
            ret_area.as_mut_ptr(),
        );
        ret_area.assume_init()
    }
}

/// Creates a new note.  asset is the asset to be included in the note.  tag is
/// the tag to be included in the note.  recipient is the recipient of the note.
/// Returns the id of the created note.
pub fn create_note(
    asset: CoreAsset,
    tag: Tag,
    note_type: NoteType,
    recipient: Recipient,
) -> NoteId {
    unsafe {
        extern_tx_create_note(
            asset.inner[0],
            asset.inner[1],
            asset.inner[2],
            asset.inner[3],
            tag,
            note_type,
            recipient.0[0],
            recipient.0[1],
            recipient.0[2],
            recipient.0[3],
        )
    }
}
