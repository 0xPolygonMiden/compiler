use core::ops::{Index, IndexMut};

use crate::Felt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C, align(32))]
pub struct Word([Felt; 4]);
impl Word {
    pub const fn new(word: [Felt; 4]) -> Self {
        Self(word)
    }
}
impl From<[Felt; 4]> for Word {
    fn from(word: [Felt; 4]) -> Self {
        Self(word)
    }
}
impl From<Word> for [Felt; 4] {
    #[inline(always)]
    fn from(word: Word) -> Self {
        word.0
    }
}
impl Index<usize> for Word {
    type Output = Felt;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}
impl IndexMut<usize> for Word {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}
