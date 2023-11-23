/// Number of field elements in a word.
pub const WORD_SIZE: usize = 4;

/// A group of four field elements in the Miden base field.
pub type Word = [Felt; WORD_SIZE];

#[repr(transparent)]
pub struct Felt(u64);

impl From<u64> for Felt {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
