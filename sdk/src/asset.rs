use crate::felt::Word;

#[repr(transparent)]
pub struct Asset(Word);

impl From<Word> for Asset {
    fn from(value: Word) -> Self {
        Self(value)
    }
}
