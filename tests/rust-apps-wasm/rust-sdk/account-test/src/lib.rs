#![no_std]

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

use miden_sdk::{add_assets, get_id, get_inputs, CoreAsset, Felt, Word};

pub struct Account;

impl Account {
    #[no_mangle]
    pub fn get_wallet_magic_number() -> Felt {
        let acc_id = get_id();
        let magic = Felt::new(42).unwrap();
        magic + acc_id.into()
    }

    #[no_mangle]
    pub fn test_add_asset() -> Felt {
        let asset_in = CoreAsset::new(Word::from_u64_unchecked(1, 2, 3, 4));
        let asset_out = add_assets(asset_in);
        asset_out.as_word().as_tuple().0
    }
}

// pub struct Note;

// impl Note {
//     #[no_mangle]
//     pub fn note_script() -> Felt {
//         let mut sum = Felt::new(0).unwrap();
//         for input in get_inputs() {
//             sum = sum + input;
//         }
//         sum
//     }
// }
