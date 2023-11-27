#![no_std]

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static A: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

extern crate alloc;

use miden::account;
use miden::sat::tx;
use miden::Asset;
use miden::Felt;
use miden::FeltDeserialize;
use miden::FeltSerialize;
use miden::Recipient;
use miden::Tag;

pub struct MyWallet;

// User-defined methods
// --------------------

impl MyWallet {
    #[no_mangle]
    pub fn receive_asset_user(&self, asset: Asset) {
        account::add_asset(asset);
    }

    #[no_mangle]
    pub fn send_asset_user(&self, asset: Asset, tag: Tag, recipient: Recipient) {
        let asset = account::remove_asset(asset);
        tx::create_note(asset, tag, recipient);
    }
}

// Macros-generated code
// --------------------

extern "C" {
    fn receive_asset_arg_ser(call_conv: Felt, sv2: Felt, sv3: Felt, sv4: Felt);
}

// Macros-generated methods
impl MyWallet {
    #[no_mangle]
    pub fn receive_asset(&self, asset: Asset) {
        // TODO: make a struct for all args and serialize it with serde
        // TODO: serialized bytes are packed into felts and are passed via sv1, sv2, ...
        let felts = asset.to_felts();
        if felts.len() <= 15 {
            // TODO: pack arg passing method (1 - 15 felts "on stack", 2 - via advice provider) and total felts count. Packed into single Felt.
            let call_conv = Felt::from(1);
            unsafe {
                receive_asset_arg_ser(call_conv, felts[0], felts[1], 0.into());
            }
        } else {
            todo!("use advice provider");
        }
    }

    #[no_mangle]
    pub fn receive_asset_arg_deser(&self, call_conv: Felt, sv2: Felt, sv3: Felt, sv4: Felt) {
        let asset = if call_conv == 1.into() {
            let felts = [sv2, sv3, sv4];
            // TODO: unpack felts into an arg holding struct and get asset from it
            Asset::from_felts(&felts)
        } else {
            todo!("use advice provider");
        };
        self.receive_asset_user(asset);
    }

    #[no_mangle]
    pub fn send_asset(&self, _asset: Asset, _tag: Tag, _recipient: Recipient) {
        todo!("");
    }
}
