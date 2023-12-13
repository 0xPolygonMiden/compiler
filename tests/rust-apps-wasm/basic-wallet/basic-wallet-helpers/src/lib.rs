#![no_std]

cargo_component_bindings::generate!();

use crate::bindings::exports::miden::basic_wallet_helpers::check_helpers::Guest;
use crate::bindings::miden::base::types::Asset;

struct Component;

impl Guest for Component {
    fn some_asset_check(asset: Asset) -> bool {
        true
    }
}
