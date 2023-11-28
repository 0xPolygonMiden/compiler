use crate::asset::Asset;
use crate::felt::Felt;

extern "C" {
    #[link_name = "miden::sat::account::add_asset"]
    pub fn add_asset_inner(asset: Asset) -> Asset;
    #[link_name = "miden::sat::account::remove_asset"]
    pub fn remove_asset_inner(asset: Asset) -> Asset;
}

pub fn add_asset(asset: Asset) -> Asset {
    unsafe { add_asset_inner(asset) }
}

pub fn remove_asset(asset: Asset) -> Asset {
    unsafe { remove_asset_inner(asset) }
}

#[repr(transparent)]
#[derive(Clone, derive_more::Into, serde::Serialize, serde::Deserialize)]
pub struct AccountId(Felt);

impl AccountId {
    pub fn new(value: u64) -> Self {
        Self(Felt::from(value))
    }
}
