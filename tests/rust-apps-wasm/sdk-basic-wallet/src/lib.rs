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
use miden::Recipient;
use miden::Tag;

// The code written by user:
// ------------------------------------------------------------------------------------------------

//#[miden::account]
pub struct MyWallet;

//#[miden::account]
impl MyWallet {
    pub fn receive_asset(&self, asset: Asset) {
        account::add_asset(asset);
    }

    pub fn send_asset(&self, asset: Asset, tag: Tag, recipient: Recipient) {
        let asset = account::remove_asset(asset);
        tx::create_note(asset, tag, recipient);
    }
}

// Macros-generated code
// ------------------------------------------------------------------------------------------------

// To be compiled with `cargo build --features build_notes` to build a note's side of the account code.
#[cfg(feature = "build_notes")]
extern "C" {
    #[link_name = "my_wallet::receive_asset"]
    fn receive_asset_extern(
        call_conv: miden::Felt,
        sv2: miden::Felt,
        sv3: miden::Felt,
        sv4: miden::Felt,
    );
}

#[cfg(feature = "build_notes")]
impl MyWallet {
    pub fn receive_asset_note_call(&self, asset: miden::Asset) {
        // TODO: make a struct for all args and serialize it with serde
        // TODO: serialized bytes are packed into felts and are passed via sv1, sv2, ...
        let felts = miden::FeltSerialize::to_felts(&asset);
        if felts.len() <= 15 {
            // TODO: pack arg passing method (1 - 15 felts "on stack", 2 - via advice provider) and total felts count. Packed into single Felt.
            let call_conv = miden::Felt::from(1);
            unsafe {
                receive_asset_extern(call_conv, felts[0], felts[1], 0.into());
            }
        } else {
            todo!("use advice provider");
        }
    }

    pub fn send_asset_note_call(
        &self,
        _asset: miden::Asset,
        _tag: miden::Tag,
        _recipient: miden::Recipient,
    ) {
        todo!("");
    }
}

#[cfg(not(feature = "build_notes"))]
impl MyWallet {
    #[export_name = "my_wallet::receive_asset"]
    pub fn receive_asset_account_export(
        &self,
        call_conv: miden::Felt,
        sv2: miden::Felt,
        sv3: miden::Felt,
        sv4: miden::Felt,
    ) {
        use miden::FeltDeserialize;
        let asset = if call_conv == 1.into() {
            let felts = [sv2, sv3, sv4];
            // TODO: unpack felts into an arg holding struct and get asset from it
            Asset::from_felts(&felts)
        } else {
            todo!("use advice provider");
        };
        self.receive_asset(asset);
    }
}
