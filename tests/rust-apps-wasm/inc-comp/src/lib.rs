#![no_std]
// This allows us to abort if the panic handler is invoked, but
// it is gated behind a perma-unstable nightly feature
#![feature(core_intrinsics)]
// Disable the warning triggered by the use of the `core_intrinsics` feature
#![allow(internal_features)]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    core::intrinsics::abort()
}

mod bindings;

use crate::bindings::miden::add_package::add_interface::add;
use crate::bindings::Guest;

struct Component;

impl Guest for Component {
    fn inc(a: u32) -> u32 {
        add(a, 1)
    }
}
