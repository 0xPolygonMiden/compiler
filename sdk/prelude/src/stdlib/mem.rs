#![allow(dead_code)]

extern crate alloc;
use alloc::vec::Vec;

use crate::{Felt, Word};

#[link(wasm_import_module = "miden:prelude/std_mem")]
extern "C" {
    #[link_name = "pipe_words_to_memory<0x0000000000000000000000000000000000000000000000000000000000000000>"]
    fn extern_pipe_words_to_memory(num_words: Felt, ptr: i32, out_ptr: i32);
}

/// Reads an arbitrary number of words `num_words` from the advice stack and returns them along with
/// sequantial RPO hash of all read words.
///
/// Cycles:
/// - Even num_words: 48 + 9 * num_words / 2
/// - Odd num_words: 65 + 9 * round_down(num_words / 2)
pub fn pipe_words_to_memory(num_words: Felt) -> (Word, Vec<Felt>) {
    unsafe {
        #[repr(align(8))]
        struct RetArea([Felt; 5]); // 4 felts is for HASH + 1 for  new_ptr
        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
        let out_ptr = ret_area.as_mut_ptr() as i32;
        let mut words_vec: Vec<Felt> = Vec::with_capacity((num_words.as_u64() * 4) as usize);
        extern_pipe_words_to_memory(num_words, words_vec.as_mut_ptr() as i32, out_ptr);
        let f0 = *((out_ptr + 0) as *const Felt);
        let f1 = *((out_ptr + 8) as *const Felt);
        let f2 = *((out_ptr + 16) as *const Felt);
        let f3 = *((out_ptr + 24) as *const Felt);
        // ignore the last element, it's the new ptr
        let rpo_hash = Word::new(f0, f1, f2, f3);
        (rpo_hash, words_vec)
    }
}
