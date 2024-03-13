#![no_std]

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate alloc;

use alloc::vec::Vec;
// use miden_sdk_types::Felt;

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
    // #[link_name = "miden_sdk_tx_kernel_get_inputs<mast0x000>(Felt, Word) -> (Word, Felt)"]
    #[link_name = "miden:tx_kernel/note.get_inputs<0x0000000000000000000000000000000000000000000000000000000000000000>"]
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

// Temporary until Felt(f64) is implemented
pub type Felt = i32;

pub struct CoreAsset {
    pub inner: (Felt, Felt, Felt, Felt),
}

pub fn add_assets(asset: CoreAsset) -> CoreAsset {
    unsafe {
        #[repr(align(8))]
        struct RetArea([u8; 32]);
        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
        let CoreAsset { inner: inner0 } = asset;
        let (t1_0, t1_1, t1_2, t1_3) = inner0;
        let ptr6 = ret_area.as_mut_ptr() as i32;
        #[cfg(target_arch = "wasm32")]
        // #[link(wasm_import_module = "miden:base/account@1.0.0")]
        extern "C" {
            #[link_name = "miden:tx_kernel/account.add_asset<0x0000000000000000000000000000000000000000000000000000000000000000>"]
            // #[link_name = "add-asset"]
            fn wit_import(_: Felt, _: Felt, _: Felt, _: Felt, _: i32);
        }

        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: Felt, _: Felt, _: Felt, _: Felt, _: i32) {
            unreachable!()
        }
        wit_import(t1_0, t1_1, t1_2, t1_3, ptr6);
        let l7 = *((ptr6 + 0) as *const i64);
        let l8 = *((ptr6 + 8) as *const i64);
        let l9 = *((ptr6 + 16) as *const i64);
        let l10 = *((ptr6 + 24) as *const i64);
        CoreAsset {
            inner: (l7 as Felt, l8 as Felt, l9 as Felt, l10 as Felt),
        }
    }
}
