#![no_std]

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod intrinsics;

pub use intrinsics::{felt::*, word::*};
