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

enum Op {
    Add,
    Sub,
    Mul,
}

#[inline(never)]
#[no_mangle]
fn match_enum(a: u32, b: u32, foo: Op) -> u32 {
    match foo {
        Op::Add => a + b,
        Op::Sub => a - b,
        Op::Mul => a * b,
    }
}

#[no_mangle]
pub extern "C" fn __main() -> u32 {
    match_enum(3, 5, Op::Add) + match_enum(3, 5, Op::Sub) + match_enum(3, 5, Op::Mul)
}
