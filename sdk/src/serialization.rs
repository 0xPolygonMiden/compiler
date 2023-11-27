use alloc::vec::Vec;

use crate::Felt;

pub trait FeltSerialize {
    fn to_felts(&self) -> Vec<Felt>;
}

pub trait FeltDeserialize {
    fn from_felts(felts: &[Felt]) -> Self;
}
