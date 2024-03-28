#![no_std]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod bindings;

use bindings::exports::miden::base::core_types;
use bindings::exports::miden::base::core_types::{AccountId, CoreAsset, Felt};
use bindings::exports::miden::base::types;
use bindings::exports::miden::base::types::Asset;

pub struct Component;

impl core_types::Guest for Component {
    fn account_id_from_felt(felt: Felt) -> AccountId {
        // TODO: assert that felt is a valid account id
        AccountId { inner: felt }
    }
}

impl types::Guest for Component {
    fn from_core_asset(asset: CoreAsset) -> Asset {
        todo!()
    }

    fn to_core_asset(asset: Asset) -> CoreAsset {
        todo!()
    }
}
