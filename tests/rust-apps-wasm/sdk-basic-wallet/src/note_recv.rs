#![no_std]

// #[panic_handler]
// fn my_panic(_info: &core::panic::PanicInfo) -> ! {
//     loop {}
// }

use basic_wallet;

fn main() {
    /*
    Expected Miden program:

    use.miden::sat::note
    use.miden::wallets::basic->wallet

    # add the asset
    begin
        dropw
        exec.note::get_assets drop
        mem_loadw
        call.wallet::receive_asset
        dropw
    end
     */

    let asset = miden::sat::note::get_assets();
    // TODO: should be invoked via `call` op
    basic_wallet::MyWallet.receive_asset(asset);
}
