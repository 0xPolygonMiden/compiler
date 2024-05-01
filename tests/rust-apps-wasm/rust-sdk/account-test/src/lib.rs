#![no_std]

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate alloc;
use alloc::vec::Vec;

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
        let asset_in = CoreAsset::new([felt!(1), felt!(2), felt!(3), felt!(4)]);
        let asset_out = add_asset(asset_in);
        asset_out.as_word()[0]
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

#[no_mangle]
pub fn test_blake3_hash_1to1(input: [u8; 32]) -> [u8; 32] {
    blake3_hash_1to1(input)
}

#[no_mangle]
pub fn test_blake3_hash_2to1(input1: [u8; 32], input2: [u8; 32]) -> [u8; 32] {
    blake3_hash_2to1(input1, input2)
}

#[no_mangle]
pub fn test_rpo_falcon512_verify(pk: Word, msg: Word) {
    rpo_falcon512_verify(pk, msg)
}

#[no_mangle]
pub fn test_pipe_words_to_memory(num_words: Felt) -> (Word, Vec<Felt>) {
    pipe_words_to_memory(num_words)
}

#[no_mangle]
pub fn test_pipe_double_words_to_memory(num_words: Felt) -> (Word, Vec<Felt>) {
    pipe_double_words_to_memory(num_words)
}

#[no_mangle]
pub fn test_remove_asset(asset: CoreAsset) -> Felt {
    let asset_out = remove_asset(asset);
    asset_out.as_word()[0]
}

#[no_mangle]
pub fn test_create_note(
    asset: CoreAsset,
    tag: Tag,
    note_type: NoteType,
    recipient: Recipient,
) -> NoteId {
    create_note(asset, tag, note_type, recipient)
}
