#![no_main]
#![no_std]

use miden_integration_tests_rust::fib::div_u;

#[global_allocator]
static A: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn __main(a: u32, b: u32) -> u32 {
    div_u(a, b)
}
