#![no_std]

// #[global_allocator]
// static A: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn function(n: u32) -> u32 {
    let mut a = n;
    while a > 3 {
        a = a / 3;
    }
    a
}
