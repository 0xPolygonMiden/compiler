use crate::account::AccountId;
use crate::felt::Word;

#[derive(derive_more::From, serde::Serialize, serde::Deserialize)]
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

#[repr(C)]
#[derive(serde::Serialize, serde::Deserialize)]
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
#[derive(serde::Serialize, serde::Deserialize)]
pub struct NonFungibleAsset(Word);
