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

static mut G1: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

#[inline(never)]
#[no_mangle]
fn global_var_update() {
    unsafe {
        G1[0] = G1[1] + 1;
    }
}

#[no_mangle]
pub extern "C" fn __main() -> u32 {
    global_var_update();
    unsafe { G1.into_iter().sum::<u8>() as u32 }
}
