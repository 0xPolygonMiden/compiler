#![no_std]

mod externs;
use externs::*;

mod types;
pub use types::*;

extern crate alloc;

use alloc::vec::Vec;

use miden_prelude::Felt;

#[inline(always)]
pub fn get_id() -> AccountId {
    unsafe { extern_account_get_id() }
}

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
