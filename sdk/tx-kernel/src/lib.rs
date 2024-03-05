#![no_std]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate alloc;

use alloc::vec::Vec;
use miden_sdk_types::Felt;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct AccountId(Felt);

impl From<AccountId> for Felt {
    fn from(account_id: AccountId) -> Felt {
        account_id.0
    }
}

extern "C" {
    #[link_name = "miden_sdk_tx_kernel_get_account_id_mast_0x000000000000000000"]
    fn extern_get_account_id() -> AccountId;
}

#[inline(always)]
pub fn get_account_id() -> AccountId {
    unsafe { extern_get_account_id() }
}

// Temporary use u64 instead of Felt until https://github.com/0xPolygonMiden/compiler/issues/118#issuecomment-1978388977 is resolved

extern "C" {
    #[link_name = "miden_sdk_tx_kernel_get_inputs_mast_0x000000000000000000"]
    fn extern_get_inputs(ptr: i32) -> i32;
}

const MAX_INPUTS: usize = 256;

#[inline(always)]
pub fn get_inputs() -> Vec<u64> {
    // The MASM for this function is here:
    // https://github.com/0xPolygonMiden/miden-base/blob/3cbe8d59dcf4ccc9c380b7c8417ac6178fc6b86a/miden-lib/asm/miden/note.masm#L69-L102
    // #! Writes the inputs of the currently execute note into memory starting at the specified address.
    // #!
    // #! Inputs: [dest_ptr]
    // #! Outputs: [num_inputs, dest_ptr]
    // #!
    // #! - dest_ptr is the memory address to write the inputs.
    unsafe {
        struct RetArea([u64; MAX_INPUTS]);
        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr = ret_area.as_mut_ptr() as i32;
        // Compiler generated adapter function will drop the returned dest_ptr
        // and return the number of inputs
        let num_inputs = extern_get_inputs(ptr);
        Vec::from_raw_parts(ptr as *mut u64, num_inputs as usize, num_inputs as usize)
    }
}
