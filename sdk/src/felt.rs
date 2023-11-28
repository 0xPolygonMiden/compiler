/// Number of field elements in a word.
pub const WORD_SIZE: usize = 4;

/// A group of four field elements in the Miden base field.
#[repr(transparent)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Word([Felt; WORD_SIZE]);

impl Word {
    pub fn from_u64(f1: u64, f2: u64, f3: u64, f4: u64) -> Self {
        Self([
            Felt::from(f1),
            Felt::from(f2),
            Felt::from(f3),
            Felt::from(f4),
        ])
    }

    pub fn from_felts(f1: Felt, f2: Felt, f3: Felt, f4: Felt) -> Self {
        Self([f1, f2, f3, f4])
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Felt(u64);

impl From<u64> for Felt {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Felt> for u64 {
    fn from(value: Felt) -> Self {
        value.0
    }
}
