use crate::account::AccountId;
use crate::felt::Word;

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
