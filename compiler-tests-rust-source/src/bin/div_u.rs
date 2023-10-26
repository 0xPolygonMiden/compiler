#![no_main]
#![no_std]

use miden_compiler_tests_rust_source::div_u::div_u;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn __main(a: u32, b: u32) -> u32 {
    div_u(a, b)
}
