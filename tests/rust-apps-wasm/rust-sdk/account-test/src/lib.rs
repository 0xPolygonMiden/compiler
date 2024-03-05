#![no_std]

extern crate alloc;

use alloc::vec::Vec;

use miden_sdk::{get_account_id, get_inputs, Felt};

pub struct Account;

impl Account {
    // #[no_mangle]
    // pub fn get_wallet_magic_number() -> Felt {
    //     let acc_id = get_account_id();
    //     let magic = Felt::new(42).unwrap();
    //     magic + acc_id.into()
    // }
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
