#![no_std]

// #[panic_handler]
// fn my_panic(_info: &core::panic::PanicInfo) -> ! {
//     loop {}
// }

use basic_wallet;

use miden::account::AccountId;
use miden::asset::Asset;
use miden::asset::FungibleAsset;
use miden::felt::Word;
use miden::note::Recipient;
use miden::note::Tag;

fn main() {
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
}
