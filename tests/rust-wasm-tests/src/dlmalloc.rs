extern crate alloc;
use alloc::vec::Vec;

#[no_mangle]
pub fn vec_alloc() {
    let mut v = Vec::new();
    if let Err(_) = v.try_reserve(4) {
        unreachable!();
    }
    if let Err(_) = v.push_within_capacity(1) {
        unreachable!();
    }
    v.pop();
}
