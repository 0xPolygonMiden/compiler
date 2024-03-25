#![no_std]

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

use miden_sdk::*;

pub struct Account;

impl Account {
    #[no_mangle]
    pub fn get_wallet_magic_number() -> Felt {
        let acc_id = get_id();
        let magic = felt!(42);
        magic + acc_id.into()
    }

    #[no_mangle]
    pub fn test_add_asset() -> Felt {
        let asset_in = CoreAsset::new(Word::from_u64_unchecked(1, 2, 3, 4));
        let asset_out = add_assets(asset_in);
        asset_out.as_word().as_tuple().0
    }

    #[no_mangle]
    pub fn test_felt_ops_smoke(a: Felt, b: Felt) -> Felt {
        let d = a.as_u64();
        if a > b {
            a.inv() + b
        } else if a < b {
            a.exp(b) - b
        } else if a <= b {
            a.pow2() * b
        } else if a >= b {
            b / a
        } else if a == b {
            assert_eq(a, b);
            a + Felt::from_u64_unchecked(d)
        } else if a != b {
            -a
        } else if b.is_odd() {
            assert(a);
            b
        } else {
            assertz(b);
            a
        }
    }
}

pub struct Note;

impl Note {
    #[no_mangle]
    pub fn note_script() -> Felt {
        let mut sum = Felt::new(0).unwrap();
        for input in get_inputs() {
            sum = sum + input;
        }
        sum
    }
}
