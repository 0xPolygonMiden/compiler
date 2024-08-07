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
    exports::miden::basic_wallet::basic_wallet::{CoreAsset, Guest, Recipient, Tag},
    miden::base::{
        account::{add_asset, remove_asset},
        tx::create_note,
    },
};

struct Component;

impl Guest for Component {
    fn receive_asset(asset: CoreAsset) {
        add_asset(asset);
    }

    fn send_asset(asset: CoreAsset, tag: Tag, recipient: Recipient) {
        let asset = remove_asset(asset);
        create_note(asset, tag, recipient);
    }
}
