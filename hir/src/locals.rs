use alloc::alloc::Layout;
use core::fmt;

use super::Type;

/// A strongly typed identifier for referencing locals associated with a function
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalId(u16);
impl LocalId {
    /// Create a new instance from a `u16`.
    #[inline]
    pub fn from_u16(x: u16) -> Self {
        debug_assert!(x < u16::MAX, "invalid raw local id");
        Self(x)
    }

    /// Return the underlying index value as a `usize`.
    #[inline]
    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}
impl cranelift_entity::EntityRef for LocalId {
    #[inline]
    fn new(index: usize) -> Self {
        debug_assert!(index < (u16::MAX as usize));
        Self(index as u16)
    }

    #[inline]
    fn index(self) -> usize {
        self.0 as usize
    }
}
impl cranelift_entity::packed_option::ReservedValue for LocalId {
    #[inline]
    fn reserved_value() -> LocalId {
        Self(u16::MAX)
    }

    #[inline]
    fn is_reserved_value(&self) -> bool {
        self.0 == u16::MAX
    }
}
impl fmt::Display for LocalId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "local{}", self.0)
    }
}
impl fmt::Debug for LocalId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
impl From<LocalId> for u16 {
    #[inline(always)]
    fn from(id: LocalId) -> Self {
        id.0
    }
}
impl From<LocalId> for miden_assembly::ast::Immediate<u16> {
    #[inline(always)]
    fn from(id: LocalId) -> Self {
        miden_assembly::ast::Immediate::Value(miden_assembly::Span::unknown(id.0))
    }
}

/// Represents a local allocated on the heap statically
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Local {
    /// The unique identifier associated with this local
    ///
    /// It also represents the offset in the set of locals of a function
    /// where this local will be allocated.
    ///
    /// NOTE: If a local's size is larger than a word, multiple consecutive
    /// local allocations may be made to ensure there is enough memory starting
    /// at the offset represented by `id` to hold the entire value
    pub id: LocalId,
    /// The type of the value stored in this local
    pub ty: Type,
}
impl Local {
    /// Returns the [Layout] for this local in memory
    pub fn layout(&self) -> Layout {
        self.ty.layout()
    }

    /// Returns the size in bytes for this local, including necessary alignment padding
    pub fn size_in_bytes(&self) -> usize {
        self.ty.size_in_bytes()
    }

    /// Returns the size in words for this local, including necessary alignment padding
    pub fn size_in_words(&self) -> usize {
        self.ty.size_in_words()
    }
}
