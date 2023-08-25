#![no_std]
#![no_main]

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[inline(never)]
#[no_mangle]
pub fn fib(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }
    a
}

#[no_mangle]
pub extern "C" fn __main() -> u32 {
    fib(25)
}
