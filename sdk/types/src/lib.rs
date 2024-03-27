#![no_std]

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod felt;
mod word;

pub use felt::*;
pub use word::*;
