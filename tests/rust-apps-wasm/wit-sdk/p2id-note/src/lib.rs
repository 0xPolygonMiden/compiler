#![no_std]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

bindings::export!(Component with_types_in bindings);

#[allow(dead_code)]
mod bindings;

use bindings::{
    exports::miden::base::note_script::Guest,
    miden::{
        base::{
            account::get_id,
            core_types::account_id_from_felt,
            note::{get_assets, get_inputs},
        },
        basic_wallet::basic_wallet::receive_asset,
    },
};

pub struct Component;

impl Guest for Component {
    fn note_script() {
        let inputs = get_inputs();
        let target_account_id_felt = inputs[0];
        let target_account_id = account_id_from_felt(target_account_id_felt);
        let account_id = get_id();
        assert_eq!(account_id, target_account_id);
        let assets = get_assets();
        for asset in assets {
            receive_asset(asset);
        }
    }
}
