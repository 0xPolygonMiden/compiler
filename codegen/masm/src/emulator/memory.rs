use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use miden_core::FieldElement;
use midenc_hir::Felt;
use rustc_hash::FxHashSet;

const EMPTY_WORD: [Felt; 4] = [Felt::ZERO; 4];

pub struct Memory {
    memory: Vec<[Felt; 4]>,
    set_memory_addrs: FxHashSet<usize>,
}

impl Memory {
    pub fn new(memory_size: usize) -> Self {
        Self {
            memory: vec![EMPTY_WORD; memory_size],
            set_memory_addrs: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.memory.len()
    }

    pub fn reset(&mut self) {
        for addr in self.set_memory_addrs.iter() {
            self.memory[*addr] = EMPTY_WORD;
        }
        self.set_memory_addrs = Default::default();
    }
}

impl Index<usize> for Memory {
    type Output = [Felt; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.memory[index]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.set_memory_addrs.insert(index);
        &mut self.memory[index]
    }
}

impl Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for addr in self.set_memory_addrs.iter() {
            write!(f, "{}: {:?}, ", addr, self[*addr])?;
        }
        Ok(())
    }
}
