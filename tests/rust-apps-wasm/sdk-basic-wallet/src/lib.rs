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
#[cfg(not(feature = "build_notes"))]
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
        sv1: miden::Felt,
        sv2: miden::Felt,
        sv3: miden::Felt,
        sv4: miden::Felt,
        sv5: miden::Felt,
        sv6: miden::Felt,
        sv7: miden::Felt,
        sv8: miden::Felt,
        sv9: miden::Felt,
        sv10: miden::Felt,
        sv11: miden::Felt,
        sv12: miden::Felt,
        sv13: miden::Felt,
        sv14: miden::Felt,
        sv15: miden::Felt,
    );
}

// Substitution for the user's code for the note's code
#[cfg(feature = "build_notes")]
impl MyWallet {
    pub fn receive_asset(&self, asset: miden::Asset) {
        let felts = {
            #[derive(serde::Serialize, serde::Deserialize)]
            struct Args {
                asset: miden::Asset,
            }
            let args_bytes = postcard::to_allocvec(&Args { asset }).unwrap();
            miden::bytes_to_felts(args_bytes)
        };
        if felts.len() <= 15 {
            // TODO: pack arg passing method (1 - 15 felts "on stack", 2 - via advice provider) and total felts count. Packed into single Felt.
            let call_conv = miden::Felt::from(1);
            unsafe {
                receive_asset_extern(
                    call_conv,
                    felts[0],
                    felts[1],
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                );
            }
        } else {
            todo!("use advice provider");
        }
    }

    pub fn send_asset(&self, _asset: miden::Asset, _tag: miden::Tag, _recipient: miden::Recipient) {
        todo!("");
    }
}

#[cfg(not(feature = "build_notes"))]
impl MyWallet {
    #[export_name = "my_wallet::receive_asset"]
    pub fn receive_asset_account_export(
        &self,
        call_conv: miden::Felt,
        sv1: miden::Felt,
        sv2: miden::Felt,
        sv3: miden::Felt,
        sv4: miden::Felt,
        sv5: miden::Felt,
        sv6: miden::Felt,
        sv7: miden::Felt,
        sv8: miden::Felt,
        sv9: miden::Felt,
        sv10: miden::Felt,
        sv11: miden::Felt,
        sv12: miden::Felt,
        sv13: miden::Felt,
        sv14: miden::Felt,
        sv15: miden::Felt,
    ) {
        let asset = if call_conv == 1.into() {
            let felts = [sv1, sv2, sv3];
            let bytes = miden::felts_to_bytes(felts.to_vec());

            #[derive(serde::Serialize, serde::Deserialize)]
            struct Args {
                asset: miden::Asset,
            }
            let args: Args = postcard::from_bytes(&bytes).unwrap();
            args.asset
        } else {
            todo!("use advice provider");
        };
        self.receive_asset(asset);
    }
}
