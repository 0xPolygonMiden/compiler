#![no_std]

cargo_component_bindings::generate!();

use bindings::miden::base::tx_kernel::get_assets;
use bindings::miden::base::tx_kernel::get_id;
use bindings::miden::base::tx_kernel::get_inputs;
use bindings::miden::basic_wallet::basic_wallet::receive_asset;

fn main() {
    let inputs = get_inputs();
    let target_account_id = inputs.0;
    let account_id = get_id();
    assert_eq!(account_id, target_account_id);
    let assets = get_assets();
    for asset in assets {
        // should be invoked via `call` op
        receive_asset(asset);
    }
}
