use alloc::vec::Vec;

use crate::account::AccountId;
use crate::felt::Word;
use crate::Felt;
use crate::FeltDeserialize;
use crate::FeltSerialize;

#[derive(derive_more::From)]
#[repr(C)]
pub enum Asset {
    Fungible(FungibleAsset),
    NonFungible(NonFungibleAsset),
}

impl From<Word> for Asset {
    fn from(_value: Word) -> Self {
        todo!()
    }
}

impl FeltSerialize for Asset {
    fn to_felts(&self) -> Vec<Felt> {
        match self {
            Asset::Fungible(asset) => {
                let mut felts: Vec<Felt> = Vec::with_capacity(2);
                felts.push(asset.asset.clone().into());
                felts.push(asset.amount.into());
                felts
            }
            Asset::NonFungible(asset) => asset.0.to_felts(),
        }
    }
}

impl FeltDeserialize for Asset {
    fn from_felts(_felts: &[Felt]) -> Self {
        todo!()
    }
}

#[repr(C)]
pub struct FungibleAsset {
    pub asset: AccountId,
    pub amount: u64,
}

impl FungibleAsset {
    pub fn new(asset: AccountId, amount: u64) -> Self {
        Self { asset, amount }
    }
}

#[repr(transparent)]
pub struct NonFungibleAsset(Word);
