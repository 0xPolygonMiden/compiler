#![no_std]
// This allows us to abort if the panic handler is invoked, but
// it is gated behind a perma-unstable nightly feature
#![feature(core_intrinsics)]
// Disable the warning triggered by the use of the `core_intrinsics` feature
#![allow(internal_features)]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    core::intrinsics::abort()
}

#[allow(dead_code)]
mod bindings;

use bindings::miden::base::account::get_id;
use bindings::miden::base::core_types::account_id_from_felt;
use bindings::miden::base::note::{get_assets, get_inputs};
use bindings::miden::basic_wallet::basic_wallet::receive_asset;

use bindings::exports::miden::base::note_script::Guest;

pub struct Component;

impl Guest for Component {
    fn note_script() {
        let inputs = get_inputs();
        let target_account_id_felt = inputs[0];
        let target_account_id = account_id_from_felt(target_account_id_felt);
        let account_id = get_id();
        // assert_eq!(account_id, target_account_id);
        let assets = get_assets();
        for asset in assets {
            receive_asset(asset);
        }
    }
}
