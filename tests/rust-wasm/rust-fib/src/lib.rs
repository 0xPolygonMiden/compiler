#![no_std]

#[global_allocator]
static A: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub fn fib(n: u32) -> u32 {
    miden_integration_tests_rust::fib::fib(n)
}
