mod context;
mod solver;
mod stack;
mod tactics;

use self::context::SolverContext;
pub use self::solver::{OperandMovementConstraintSolver, SolverError};
use self::stack::Stack;

use std::fmt;
use std::num::NonZeroU8;

use miden_hir as hir;

/// This represents a specific action that should be taken by
/// the code generator with regard to an operand on the stack.
///
/// The output of the optimizer is a sequence of these actions,
/// the effect of which is to place all of the current instruction's
/// operands exactly where they need to be, just when they are
/// needed.
#[derive(Debug, Copy, Clone)]
pub enum Action {
    /// Copy the operand at the given index to the top of the stack
    Copy(u8),
    /// Swap the operand at the given index with the one on top of the stack
    Swap(u8),
    /// Move the operand at the given index to the top of the stack
    MoveUp(u8),
    /// Move the operand at the top of the stack to the given index
    MoveDown(u8),
}

/// This is a [miden_hir::Value], but with a modified encoding that lets
/// us uniquely identify aliases of a value on the operand stack during
/// analysis.
///
/// Aliases of a value are treated as unique values for purposes of operand
/// stack management, but are associated with multiple copies of a value
/// on the stack.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Value(u32);
impl Value {
    const ALIAS_MASK: u32 = (u8::MAX as u32) << 23;

    /// Create a new [Value] with the given numeric identifier.
    ///
    /// The given identifier must have the upper 8 bits zeroed, or this function will panic.
    pub fn new(id: u32) -> Self {
        assert_eq!(id & Self::ALIAS_MASK, 0);
        Self(id)
    }

    /// Create an aliased copy of this value, using `id` to uniquely identify the alias.
    ///
    /// NOTE: You must ensure that each alias of the same value gets a unique identifier,
    /// or you may observe strange behavior due to two aliases that should be distinct,
    /// being treated as if they have the same identity.
    pub fn copy(self, id: NonZeroU8) -> Self {
        Self(self.id() | ((id.get() as u32) << 23))
    }

    /// Get an un-aliased copy of this value
    pub fn unaliased(self) -> Self {
        Self(self.id())
    }

    /// Convert this value into an alias, using `id` to uniquely identify the alias.
    ///
    /// NOTE: You must ensure that each alias of the same value gets a unique identifier,
    /// or you may observe strange behavior due to two aliases that should be distinct,
    /// being treated as if they have the same identity.
    pub fn set_alias(&mut self, id: NonZeroU8) {
        self.0 = self.id() | ((id.get() as u32) << 23);
    }

    /// Get the raw u32 value of the original [miden_hir::Value]
    pub fn id(self) -> u32 {
        self.0 & !Self::ALIAS_MASK
    }

    /// Get the unique alias identifier for this value, if this value is an alias
    pub fn alias(self) -> Option<NonZeroU8> {
        NonZeroU8::new(((self.0 & Self::ALIAS_MASK) >> 23) as u8)
    }

    /// Get the unique alias identifier for this value, if this value is an alias
    pub fn unwrap_alias(self) -> NonZeroU8 {
        NonZeroU8::new(((self.0 & Self::ALIAS_MASK) >> 23) as u8)
            .unwrap_or_else(|| panic!("expected {self:?} to be an alias"))
    }

    /// Returns true if this value is an alias
    pub fn is_alias(&self) -> bool {
        self.0 & Self::ALIAS_MASK != 0
    }
}
impl Ord for Value {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.id().cmp(&other.id()).then_with(|| {
            let self_alias = self.alias().map(|nz| nz.get()).unwrap_or(0);
            let other_alias = self.alias().map(|nz| nz.get()).unwrap_or(0);
            self_alias.cmp(&other_alias)
        })
    }
}
impl PartialEq<hir::Value> for Value {
    fn eq(&self, other: &hir::Value) -> bool {
        self.id() == other.as_u32()
    }
}
impl PartialOrd for Value {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl From<hir::Value> for Value {
    #[inline]
    fn from(value: hir::Value) -> Self {
        Self::new(value.as_u32())
    }
}
impl From<Value> for hir::Value {
    #[inline]
    fn from(value: Value) -> Self {
        Self::from_u32(value.id())
    }
}
impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.alias() {
            None => write!(f, "v{}", self.id()),
            Some(alias) => write!(f, "v{}.{alias}", self.id()),
        }
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Value {
    type Parameters = ();
    type Strategy = proptest::strategy::Map<proptest::arbitrary::StrategyFor<u8>, fn(u8) -> Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        use proptest::strategy::Strategy;
        proptest::arbitrary::any::<u8>().prop_map(|id| Value(id as u32))
    }
}

/// This is an simple representation of an operand on the operand stack
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Operand {
    /// The position of this operand on the corresponding stack
    pub pos: u8,
    /// The value this operand corresponds to
    pub value: Value,
}
impl From<(usize, Value)> for Operand {
    #[inline(always)]
    fn from(pair: (usize, Value)) -> Self {
        Self {
            pos: pair.0 as u8,
            value: pair.1,
        }
    }
}
impl PartialEq<Value> for Operand {
    #[inline(always)]
    fn eq(&self, other: &Value) -> bool {
        self.value.eq(other)
    }
}
