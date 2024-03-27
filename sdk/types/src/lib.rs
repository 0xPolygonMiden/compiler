#![no_std]

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Temporary until Felt(f64) is implemented
pub type Felt = u64;

pub type Word = [Felt; 4];
