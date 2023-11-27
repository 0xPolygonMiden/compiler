#![no_std]
#![no_main]

use basic_wallet;

use miden::account::AccountId;
use miden::Asset;
use miden::FungibleAsset;
use miden::Recipient;
use miden::Tag;
use miden::Word;

// WASI runtime expects the following function to be "main"
#[no_mangle]
pub extern "C" fn __original_main() -> i32 {
    /*
       Expected Miden program:

       use.miden::eoa::basic->auth_tx
       use.miden::wallets::basic->wallet

       begin
           push.{recipient}
           push.{tag}
           push.{asset}
           call.wallet::send_asset drop
           dropw dropw
           call.auth_tx::auth_tx_rpo_falcon512
       end
    */

    // TODO: parse Recipient from the &str?
    let recipient = Recipient::from(Word::from_u64(1, 2, 3, 4));
    let tag = Tag::new(4);
    // TODO: parse Asset from the &str?
    let asset_id = AccountId::new(1234u64);
    let asset: Asset = FungibleAsset::new(asset_id, 100).into();
    // TODO: should be invoked via `call` op
    basic_wallet::MyWallet.send_asset(asset, tag, recipient);
    // TODO: should be invoked via `call` op
    miden::eoa::basic::auth_tx_rpo_falcon512();
    0
}
