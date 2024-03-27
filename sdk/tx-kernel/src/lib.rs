#![no_std]
#![allow(dead_code)]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate alloc;

use alloc::vec::Vec;

use miden_sdk_types::{Felt, Word};

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct AccountId(Felt);

impl From<AccountId> for Felt {
    fn from(account_id: AccountId) -> Felt {
        account_id.0
    }
}

#[link(wasm_import_module = "miden:tx_kernel/account")]
extern "C" {
    #[link_name = "get_id<0x0000000000000000000000000000000000000000000000000000000000000000>"]
    fn extern_account_get_id() -> AccountId;
    #[link_name = "add_asset<0x0000000000000000000000000000000000000000000000000000000000000000>"]
    fn extern_account_add_asset(_: Felt, _: Felt, _: Felt, _: Felt, ptr: *mut CoreAsset);
}

#[link(wasm_import_module = "miden:tx_kernel/note")]
extern "C" {
    #[link_name = "get_inputs<0x0000000000000000000000000000000000000000000000000000000000000000>"]
    fn extern_note_get_inputs(ptr: *mut Felt) -> usize;
}

#[inline(always)]
pub fn get_id() -> AccountId {
    unsafe { extern_account_get_id() }
}

const MAX_INPUTS: usize = 256;

#[inline(always)]
pub fn get_inputs() -> Vec<Felt> {
    // The MASM for this function is here:
    // https://github.com/0xPolygonMiden/miden-base/blob/3cbe8d59dcf4ccc9c380b7c8417ac6178fc6b86a/miden-lib/asm/miden/note.masm#L69-L102
    // #! Writes the inputs of the currently execute note into memory starting at the specified
    // address. #!
    // #! Inputs: [dest_ptr]
    // #! Outputs: [num_inputs, dest_ptr]
    // #!
    // #! - dest_ptr is the memory address to write the inputs.
    unsafe {
        #[repr(transparent)]
        struct RetArea([Felt; MAX_INPUTS]);
        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr = ret_area.as_mut_ptr();
        let num_inputs = extern_note_get_inputs(ptr as *mut Felt);
        // Compiler generated adapter function will drop the returned dest_ptr
        // and return the number of inputs
        Vec::from_raw_parts(ptr as *mut Felt, num_inputs, num_inputs)
    }
}

#[repr(transparent)]
pub struct CoreAsset {
    inner: Word,
}

impl CoreAsset {
    pub fn new(word: Word) -> Self {
        CoreAsset { inner: word }
    }

    pub fn as_word(&self) -> Word {
        self.inner
    }
}

pub fn add_assets(asset: CoreAsset) -> CoreAsset {
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
