use crate::Felt;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Word(Felt, Felt, Felt, Felt);

impl Word {
    pub fn from_u64_unchecked(a: u64, b: u64, c: u64, d: u64) -> Self {
        Self(
            Felt::from_u64_unchecked(a),
            Felt::from_u64_unchecked(b),
            Felt::from_u64_unchecked(c),
            Felt::from_u64_unchecked(d),
        )
    }

    pub fn new(a: Felt, b: Felt, c: Felt, d: Felt) -> Self {
        Word(a, b, c, d)
    }

    pub fn as_tuple(self) -> (Felt, Felt, Felt, Felt) {
        (self.0, self.1, self.2, self.3)
    }
}
