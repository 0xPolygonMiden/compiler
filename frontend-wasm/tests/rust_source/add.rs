#![no_std]
#![no_main]

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[inline(never)]
#[no_mangle]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn __main() -> i32 {
    let a = 1;
    let b = 2;
    add(a, b)
}
