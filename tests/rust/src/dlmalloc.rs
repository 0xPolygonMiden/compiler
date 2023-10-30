extern crate alloc;
use alloc::vec::Vec;

#[no_mangle]
pub fn vec_alloc() -> u32 {
    let mut v = Vec::new();
    v.push(1);
    v.pop().unwrap()
}
