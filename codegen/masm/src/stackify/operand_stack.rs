use core::{
    fmt,
    ops::{Index, IndexMut},
};

use miden_hir::{Felt, FieldElement, Immediate, Stack, StackElement, Value};

/// Represents a value on the operand stack
#[derive(Copy, Clone)]
pub enum Operand {
    /// The operand is a literal, unassociated with any value in the SSA representation
    Const(Immediate),
    /// The operand is an SSA value
    Value(Value),
}
impl fmt::Debug for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Const(value) => write!(f, "Const({value:?})"),
            Self::Value(value) => write!(f, "Value({value})"),
        }
    }
}
impl Eq for Operand {}
impl PartialEq for Operand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(a), Self::Value(b)) => a == b,
            (Self::Value(_), _) | (_, Self::Value(_)) => false,
            (Self::Const(ref a), Self::Const(ref b)) => a.cmp(b).is_eq(),
        }
    }
}
impl PartialEq<Value> for Operand {
    fn eq(&self, other: &Value) -> bool {
        match self {
            Self::Value(this) => this == other,
            _ => false,
        }
    }
}
impl From<Value> for Operand {
    fn from(value: Value) -> Self {
        Self::Value(value)
    }
}
impl From<bool> for Operand {
    fn from(value: bool) -> Self {
        Self::Const(Immediate::I1(value))
    }
}
impl From<u8> for Operand {
    fn from(value: u8) -> Self {
        Self::Const(Immediate::U8(value))
    }
}
impl From<u16> for Operand {
    fn from(value: u16) -> Self {
        Self::Const(Immediate::U16(value))
    }
}
impl From<u32> for Operand {
    fn from(value: u32) -> Self {
        Self::Const(Immediate::U32(value))
    }
}
impl From<u64> for Operand {
    fn from(value: u64) -> Self {
        Self::Const(Immediate::U64(value))
    }
}
impl From<Felt> for Operand {
    fn from(value: Felt) -> Self {
        Self::Const(Immediate::Felt(value))
    }
}
impl From<Immediate> for Operand {
    fn from(value: Immediate) -> Self {
        Self::Const(value)
    }
}
impl StackElement for Operand {
    /// A value of this type which represents the "zero" value for the type
    const DEFAULT: Self = Self::Const(Immediate::Felt(Felt::ZERO));
}

/// This structure emulates the state of the VM's operand stack while
/// generating code from the SSA representation of a function.
///
/// In order to emit efficient and correct stack manipulation code, we must be able to
/// reason about where values are on the operand stack at a given program point. This
/// structure tracks what SSA values have been pushed on the operand stack, where they are
/// on the stack relative to the top, and whether a given stack slot aliases multiple
/// values.
///
/// In addition to the state tracked, this structure also has an API that mimics the
/// stack manipulation instructions we can emit in the code generator, so that as we
/// emit instructions and modify this structure at the same time, 1:1.
#[derive(Clone, Default)]
pub struct OperandStack {
    stack: Vec<Operand>,
}
impl Stack for OperandStack {
    type Element = Operand;

    #[inline(always)]
    fn stack(&self) -> &Vec<Self::Element> {
        &self.stack
    }

    #[inline(always)]
    fn stack_mut(&mut self) -> &mut Vec<Self::Element> {
        &mut self.stack
    }
}
impl OperandStack {
    /// Renames the `n`th value from the top of the stack to `value`
    pub fn rename(&mut self, n: usize, value: Value) {
        let _ = core::mem::replace(&mut self[n], Operand::Value(value));
    }

    /// Searches for the position on the stack at which `value` resides
    ///
    /// NOTE: This function will panic if `value` is not on the stack
    pub fn find(&self, value: &Value) -> Option<usize> {
        self.stack.iter().rev().position(|v| v == value)
    }
}
impl Index<usize> for OperandStack {
    type Output = Operand;

    fn index(&self, index: usize) -> &Self::Output {
        let len = self.stack.len();
        assert!(
            index < 16,
            "invalid operand stack index ({}), only the top 16 elements are directly accessible",
            index
        );
        assert!(
            index < len,
            "invalid operand stack index ({}), only {} elements are available",
            index,
            len
        );
        &self.stack[len - index - 1]
    }
}
impl IndexMut<usize> for OperandStack {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let len = self.stack.len();
        assert!(
            index < 16,
            "invalid operand stack index ({}), only the top 16 elements are directly accessible",
            index
        );
        assert!(
            index < len,
            "invalid operand stack index ({}), only {} elements are available",
            index,
            len
        );
        &mut self.stack[len - index - 1]
    }
}
