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

use bindings::exports::miden::base::core_types;
use bindings::exports::miden::base::core_types::{AccountId, CoreAsset, Felt};
use bindings::exports::miden::base::types;
use bindings::exports::miden::base::types::{Asset, FungibleAsset};

pub struct Component;

impl core_types::Guest for Component {
    fn account_id_from_felt(felt: Felt) -> AccountId {
        // TODO: assert that felt is a valid account id
        AccountId { inner: felt }
    }
}

impl types::Guest for Component {
    fn from_core_asset(asset: CoreAsset) -> Asset {
        // TODO: implement
        Asset::Fungible(FungibleAsset {
            asset: AccountId {
                inner: asset.inner.0,
            },
            amount: 0,
        })
    }

    fn to_core_asset(asset: Asset) -> CoreAsset {
        // TODO: implement
        CoreAsset {
            inner: (
                Felt { inner: 0 },
                Felt { inner: 0 },
                Felt { inner: 0 },
                Felt { inner: 0 },
            ),
        }
    }
}
