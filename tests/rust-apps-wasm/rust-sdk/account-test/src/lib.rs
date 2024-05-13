#![no_std]

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use miden_sdk::*;

pub struct Account;

impl Account {
    #[no_mangle]
    pub fn receive_asset(asset: CoreAsset) {
        add_asset(asset);
    }

    #[no_mangle]
    pub fn send_asset(asset: CoreAsset, tag: Tag, note_type: NoteType, recipient: Recipient) {
        let asset = remove_asset(asset);
        create_note(asset, tag, note_type, recipient);
    }
}
