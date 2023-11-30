use crate::felt::Felt;
use crate::felt::Word;

#[repr(transparent)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Recipient(Word);

impl From<Word> for Recipient {
    fn from(value: Word) -> Self {
        Self(value)
    }
}

#[repr(transparent)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Tag(Felt);

impl Tag {
    pub fn new(value: u64) -> Self {
        Self(Felt::from(value))
    }
}

impl From<Felt> for Tag {
    fn from(value: Felt) -> Self {
        Self(value)
    }
}