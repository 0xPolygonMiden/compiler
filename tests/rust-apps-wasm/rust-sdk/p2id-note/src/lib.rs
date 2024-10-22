// Do not link against libstd (i.e. anything defined in `std::`)
#![no_std]

// However, we could still use some standard library types while
// remaining no-std compatible, if we uncommented the following lines:
//
// extern crate alloc;
// use alloc::vec::Vec;

// Global allocator to use heap memory in no-std environment
#[global_allocator]
static ALLOC: miden::BumpAlloc = miden::BumpAlloc::new();

// Required for no-std crates
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

bindings::export!(MyNote with_types_in bindings);

mod bindings;

use bindings::{
    exports::miden::base::note_script::Guest, miden::basic_wallet::basic_wallet::receive_asset,
};

struct MyNote;

impl Guest for MyNote {
    fn note_script() {
        let inputs = miden::note::get_inputs();
        let target_account_id_felt = inputs[0];
        let account_id = miden::account::get_id();
        assert_eq!(account_id.as_felt(), target_account_id_felt);
        let assets = miden::note::get_assets();
        for asset in assets {
            receive_asset(asset);
        }
    }
}
