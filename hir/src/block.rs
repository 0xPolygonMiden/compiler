use cranelift_entity::entity_impl;
use intrusive_collections::linked_list::{Cursor, CursorMut};

use super::*;

/// A handle to a single function block
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Block(u32);
entity_impl!(Block, "block");
impl Default for Block {
    #[inline]
    fn default() -> Self {
        use cranelift_entity::packed_option::ReservedValue;

        Self::reserved_value()
    }
}

/// Data associated with a `Block`.
///
/// Blocks have arguments, and consist of a sequence of instructions.
pub struct BlockData {
    pub id: Block,
    pub params: ValueList,
    pub insts: InstructionList,
}
impl Drop for BlockData {
    fn drop(&mut self) {
        self.insts.fast_clear();
    }
}
impl Clone for BlockData {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            params: self.params,
            insts: Default::default(),
        }
    }
}
impl BlockData {
    pub(crate) fn new(id: Block) -> Self {
        Self {
            id,
            params: ValueList::new(),
            insts: Default::default(),
        }
    }

    #[inline]
    pub fn arity(&self, pool: &ValueListPool) -> usize {
        self.params.len(pool)
    }

    #[inline]
    pub fn params<'a, 'b: 'a>(&'b self, pool: &'a ValueListPool) -> &'a [Value] {
        self.params.as_slice(pool)
    }

    pub fn insts(&self) -> impl Iterator<Item = Inst> + '_ {
        Insts {
            cursor: self.insts.front(),
        }
    }

    #[inline(always)]
    pub fn prepend(&mut self, inst: UnsafeRef<InstNode>) {
        self.insts.push_front(inst);
    }

    #[inline(always)]
    pub fn append(&mut self, inst: UnsafeRef<InstNode>) {
        self.insts.push_back(inst);
    }

    #[inline(always)]
    pub fn cursor_mut<'a, 'b: 'a>(&'b mut self) -> CursorMut<'a, InstAdapter> {
        self.insts.front_mut()
    }

    pub fn cursor_mut_at<'a, 'b: 'a>(&'b mut self, index: usize) -> CursorMut<'a, InstAdapter> {
        let mut cursor = self.insts.front_mut();
        for _ in 0..index {
            cursor.move_next();
            assert!(!cursor.is_null(), "index out of bounds");
        }
        cursor
    }

    #[inline]
    pub fn insert_after(&mut self, index: usize, inst: UnsafeRef<InstNode>) {
        let mut cursor = self.cursor_mut_at(index);
        cursor.insert_after(inst);
    }

    #[inline]
    pub fn insert_before(&mut self, index: usize, inst: UnsafeRef<InstNode>) {
        let mut cursor = self.cursor_mut_at(index);
        cursor.insert_before(inst);
    }

    pub fn first(&self) -> Option<Inst> {
        self.insts.front().get().map(|data| data.key)
    }

    pub fn last(&self) -> Option<Inst> {
        self.insts.back().get().map(|data| data.key)
    }

    pub fn is_empty(&self) -> bool {
        self.insts.is_empty()
    }

    pub fn len(&self) -> usize {
        self.insts.iter().count()
    }
}

struct Insts<'f> {
    cursor: Cursor<'f, InstAdapter>,
}
impl<'f> Iterator for Insts<'f> {
    type Item = Inst;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.is_null() {
            return None;
        }
        let next = self.cursor.get().map(|data| data.key);
        self.cursor.move_next();
        next
    }
}
