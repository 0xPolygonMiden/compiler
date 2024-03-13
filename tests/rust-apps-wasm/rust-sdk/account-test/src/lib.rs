#![no_std]

extern crate alloc;

use alloc::vec::Vec;

use miden_sdk::{add_assets, get_account_id, get_inputs, CoreAsset, Felt};

pub struct Account;

impl Account {
    // #[no_mangle]
    // pub fn get_wallet_magic_number() -> Felt {
    //     let acc_id = get_account_id();
    //     let magic = Felt::new(42).unwrap();
    //     magic + acc_id.into()
    // }

    #[no_mangle]
    pub fn test_add_asset() {
        let asset_in = CoreAsset {
            inner: (1, 1, 1, 1),
        };
        let asset_out = add_assets(asset_in);
        assert_eq!(asset_out.inner.0, 42);
    }
}

pub struct Note;

impl Note {
    #[no_mangle]
    pub fn note_script() {
        let mut sum = 0;
        for input in get_inputs() {
            sum += input;
        }
        assert_eq!(sum, 42);
    }
}
