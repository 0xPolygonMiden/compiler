#![no_main]
#![no_std]

extern crate dlmalloc;

use crate::dlmalloc::vec_alloc;

#[global_allocator]
static A: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn __main() -> u32 {
    vec_alloc()
}
