#![no_std]
#![no_main]
// This allows us to abort if the panic handler is invoked, but
// it is gated behind a perma-unstable nightly feature
#![feature(core_intrinsics)]
// Disable the warning triggered by the use of the `core_intrinsics` feature
#![allow(internal_features)]

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    core::intrinsics::abort()
}

#[inline(never)]
#[no_mangle]
pub fn sum_arr(arr: &[u32]) -> u32 {
    arr.iter().sum()
}

#[no_mangle]
pub extern "C" fn __main() -> u32 {
    sum_arr(&[1, 2, 3, 4, 5]) + sum_arr(&[6, 7, 8, 9, 10])
}
