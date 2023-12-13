#![no_std]

cargo_component_bindings::generate!();

use bindings::exports::miden::basic_wallet::basic_wallet::{Asset, Guest, Recipient, Tag};
use bindings::miden::base::tx_kernel::{add_asset, create_note, remove_asset};

struct Component;

impl Guest for Component {
    fn receive_asset(asset: Asset) {
        add_asset(asset);
    }

    fn send_asset(asset: Asset, tag: Tag, recipient: Recipient) {
        let asset = remove_asset(asset);
        create_note(asset, tag, recipient);
    }
}
