#![no_std]
#![no_main]

use basic_wallet;

// WASI runtime expects the following function to be "main"
#[no_mangle]
pub extern "C" fn __main_void() -> i32 {
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
    0
}
