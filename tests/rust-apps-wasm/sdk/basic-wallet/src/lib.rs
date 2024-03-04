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

use bindings::exports::miden::basic_wallet::basic_wallet::{CoreAsset, Guest, Recipient, Tag};
use bindings::miden::base::account::{add_asset, remove_asset};
use bindings::miden::base::tx::create_note;

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
