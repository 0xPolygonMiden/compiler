#![no_std]

cargo_component_bindings::generate!();

use bindings::miden::base::tx_kernel::{get_assets, get_id, get_inputs};
use bindings::miden::basic_wallet::basic_wallet::receive_asset;
use bindings::miden::basic_wallet_helpers::check_helpers::some_asset_check;

fn main() {
    let inputs = get_inputs();
    let target_account_id = inputs.0;
    let account_id = get_id();
    assert_eq!(account_id, target_account_id);
    let assets = get_assets();
    for asset in assets {
        // using some helper function from the basic_wallet_helpers component
        if some_asset_check(&asset) {
            receive_asset(asset);
        }
    }
}
