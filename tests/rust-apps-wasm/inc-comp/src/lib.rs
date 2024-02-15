#![no_std]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod bindings;

use crate::bindings::miden::add::add::add;
use crate::bindings::Guest;

struct Component;

impl Guest for Component {
    fn inc(a: u32) -> u32 {
        add(a, 1)
    }
}
