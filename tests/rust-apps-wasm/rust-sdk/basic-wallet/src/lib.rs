// Do not link against libstd (i.e. anything defined in `std::`)
#![no_std]

// However, we could still use some standard library types while
// remaining no-std compatible, if we uncommented the following lines:
//
extern crate alloc;
use alloc::vec::Vec;

// Global allocator to use heap memory in no-std environment
#[global_allocator]
static ALLOC: BumpAlloc = miden_sdk::BumpAlloc::new();

// Required for no-std crates
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use miden_sdk::*;

bindings::export!(MyAccount with_types_in bindings);

mod bindings;
use bindings::exports::miden::basic_wallet::basic_wallet::Guest;

struct MyAccount;

impl Guest for MyAccount {
    fn receive_asset(asset: CoreAsset) {
        add_asset(asset);
    }

    fn send_asset(asset: CoreAsset, tag: Tag, note_type: NoteType, recipient: Recipient) {
        let asset = remove_asset(asset);
        create_note(asset, tag, note_type, recipient);
    }

    fn test_felt_intrinsics(a: Felt, b: Felt) -> Felt {
        a + b
    }

    fn test_stdlib(input: Vec<u8>) -> Vec<u8> {
        let input: [u8; 32] = input.try_into().unwrap();
        blake3_hash_1to1(input).to_vec()
    }
}
