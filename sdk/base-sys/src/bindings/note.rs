extern crate alloc;
use alloc::vec::Vec;

use miden_stdlib_sys::Felt;

use super::CoreAsset;

#[link(wasm_import_module = "miden:core-import/note@1.0.0")]
extern "C" {
    #[link_name = "get-inputs"]
    pub fn extern_note_get_inputs(ptr: *mut Felt) -> usize;
}

/// Get the inputs of the currently executing note.
pub fn get_inputs() -> Vec<Felt> {
    const MAX_INPUTS: usize = 256;
    let mut inputs: Vec<Felt> = Vec::with_capacity(MAX_INPUTS);
    let num_inputs = unsafe {
        // Ensure the pointer is a valid Miden pointer
        //
        // NOTE: This relies on the fact that BumpAlloc makes all allocations
        // minimally word-aligned. Each word consists of 4 elements of 4 bytes,
        // so to get a Miden address from a Rust address, we divide by 16 to get
        // the address in words (dividing by 4 gets us an address in elements,
        // and by 4 again we get the word address).
        let ptr = (inputs.as_mut_ptr() as usize) / 16;
        // The MASM for this function is here:
        // https://github.com/0xPolygonMiden/miden-base/blob/3cbe8d59dcf4ccc9c380b7c8417ac6178fc6b86a/miden-lib/asm/miden/note.masm#L69-L102
        // #! Writes the inputs of the currently execute note into memory starting at the specified
        // address. #!
        // #! Inputs: [dest_ptr]
        // #! Outputs: [num_inputs, dest_ptr]
        // #!
        // #! - dest_ptr is the memory address to write the inputs.
        // Compiler generated adapter code at call site will drop the returned dest_ptr
        // and return the number of inputs
        extern_note_get_inputs(ptr as *mut Felt)
    };
    unsafe {
        inputs.set_len(num_inputs);
    }
    inputs
}

pub fn get_assets() -> Vec<CoreAsset> {
    todo!()
}
