#![allow(dead_code)]

extern crate alloc;
use alloc::vec::Vec;

use crate::{Felt, Word};

#[link(wasm_import_module = "miden:prelude/std_mem")]
extern "C" {

    /// Moves an arbitrary number of words from the advice stack to memory.
    ///
    /// Input: [num_words, write_ptr, ...]
    /// Output: [HASH, write_ptr', ...]
    ///
    /// Where HASH is the sequential RPO hash of all copied words.
    ///
    /// Cycles:
    /// - Even num_words: 48 + 9 * num_words / 2
    /// - Odd num_words: 65 + 9 * round_down(num_words / 2)
    #[link_name = "pipe_words_to_memory<0x0000000000000000000000000000000000000000000000000000000000000000>"]
    fn extern_pipe_words_to_memory(num_words: Felt, ptr: *mut Felt, out_ptr: *mut Felt);

    /// Moves an even number of words from the advice stack to memory.
    ///
    /// Input: [C, B, A, write_ptr, end_ptr, ...]
    /// Output: [C, B, A, write_ptr, ...]
    ///
    /// Where:
    /// - The words C, B, and A are the RPO hasher state
    /// - A is the capacity
    /// - C, B are the rate portion of the state
    /// - The value num_words = end_ptr - write_ptr must be positive and even
    ///
    /// Cycles: 10 + 9 * num_words / 2
    #[link_name = "pipe_double_words_to_memory<0x0000000000000000000000000000000000000000000000000000000000000000>"]
    fn extern_pipe_double_words_to_memory(
        c0: Felt,
        c1: Felt,
        c2: Felt,
        c3: Felt,
        b0: Felt,
        b1: Felt,
        b2: Felt,
        b3: Felt,
        a0: Felt,
        a1: Felt,
        a2: Felt,
        a3: Felt,
        write_ptr: *mut Felt,
        end_ptr: *mut Felt,
        out_ptr: i32,
    );
}

/// Reads an arbitrary number of words `num_words` from the advice stack and returns them along with
/// sequantial RPO hash of all read words.
///
/// Cycles:
/// - Even num_words: 48 + 9 * num_words / 2
/// - Odd num_words: 65 + 9 * round_down(num_words / 2)
pub fn pipe_words_to_memory(num_words: Felt) -> (Word, Vec<Felt>) {
    unsafe {
        // Place for returned HASH, write_ptr
        let mut ret_area = ::core::mem::MaybeUninit::<[Felt; 4 + 1]>::uninit();
        let out_ptr = ret_area.as_mut_ptr() as i32;
        let mut words_vec: Vec<Felt> = Vec::with_capacity((num_words.as_u64() * 4) as usize);
        extern_pipe_words_to_memory(
            num_words,
            words_vec.as_mut_ptr() as *mut Felt,
            out_ptr as *mut Felt,
        );
        let f0 = *((out_ptr + 0) as *const Felt);
        let f1 = *((out_ptr + 8) as *const Felt);
        let f2 = *((out_ptr + 16) as *const Felt);
        let f3 = *((out_ptr + 24) as *const Felt);
        // ignore the last element, it's the new ptr
        let rpo_hash = [f0, f1, f2, f3];
        (rpo_hash, words_vec)
    }
}

/// Returns an even number of words from the advice stack along with the RPO hash of all read words.
///
/// Cycles: 10 + 9 * num_words / 2
pub fn pipe_double_words_to_memory(num_words: Felt) -> (Word, Vec<Felt>) {
    let num_words_in_felts = num_words.as_u64() * 4;
    let mut words_vec: Vec<Felt> = Vec::with_capacity((num_words_in_felts) as usize);
    let write_ptr = words_vec.as_mut_ptr();
    // we cannot use `write_ptr.add(num_words_in_felts)` because it's get
    // multiplied by 8 (size of Felt in bytes)
    // end_ptr is expected to be write_ptr + num_words (in Felts)
    let end_ptr = write_ptr as u64 + num_words_in_felts;
    // Place for returned C, B, A, write_ptr
    let mut ret_area = ::core::mem::MaybeUninit::<[Felt; 4 + 4 + 4 + 1]>::uninit();
    let zero = Felt::from_u64_unchecked(0);
    unsafe {
        let out_ptr = ret_area.as_mut_ptr() as i32;
        extern_pipe_double_words_to_memory(
            zero,
            zero,
            zero,
            zero,
            zero,
            zero,
            zero,
            zero,
            zero,
            zero,
            zero,
            zero,
            write_ptr,
            end_ptr as *mut Felt,
            out_ptr,
        );
        // B (second) is the hash (see https://github.com/0xPolygonMiden/miden-vm/blob/3a957f7c90176914bda2139f74bff9e5700d59ac/stdlib/asm/crypto/hashes/native.masm#L1-L16 )
        // we're using Felt-sized pointer arithmetic to produce `f64.load` op with
        let f0 = *((out_ptr + 4 * 8) as *const Felt);
        let f1 = *((out_ptr + 5 * 8) as *const Felt);
        let f2 = *((out_ptr + 6 * 8) as *const Felt);
        let f3 = *((out_ptr + 7 * 8) as *const Felt);
        let rpo_hash = [f0, f1, f2, f3];
        (rpo_hash, words_vec)
    }
}
