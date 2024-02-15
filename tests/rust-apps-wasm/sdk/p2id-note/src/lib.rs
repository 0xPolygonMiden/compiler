#![no_std]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[allow(dead_code)]
mod bindings;

use bindings::miden::base::tx_kernel::{get_assets, get_id, get_inputs};
use bindings::miden::basic_wallet::basic_wallet::receive_asset;

use bindings::exports::miden::base::note::Guest;

pub struct Component;

impl Guest for Component {
    fn note_script() {
        let inputs = get_inputs();
        let target_account_id = inputs[0];
        let account_id = get_id();
        assert_eq!(account_id, target_account_id);
        let assets = get_assets();
        for asset in assets {
            receive_asset(asset);
        }
    }
}
