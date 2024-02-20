#![no_std]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[allow(dead_code)]
mod bindings;

use bindings::exports::miden::basic_wallet::basic_wallet::{CoreAsset, Guest, Recipient, Tag};
use bindings::miden::base::tx_kernel::{add_asset, create_note, remove_asset};

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
