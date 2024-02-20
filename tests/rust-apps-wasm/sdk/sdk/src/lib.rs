#![no_std]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod bindings;

use bindings::exports::miden::base::core_types::Guest;
use bindings::exports::miden::base::core_types::{AccountId, Felt};

pub struct Component;

impl Guest for Component {
    fn account_id_from_felt(felt: Felt) -> AccountId {
        // TODO: assert that felt is a valid account id
        AccountId { inner: felt }
    }
}
