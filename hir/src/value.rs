use cranelift_entity::{self as entity, entity_impl};

use miden_diagnostics::SourceSpan;

use super::{Block, Inst, Type};

pub type ValueList = entity::EntityList<Value>;
pub type ValueListPool = entity::ListPool<Value>;

/// A handle to a single SSA value
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Value(u32);
entity_impl!(Value, "v");
impl Default for Value {
    #[inline]
    fn default() -> Self {
        use cranelift_entity::packed_option::ReservedValue;

        Self::reserved_value()
    }
}

/// Data associated with a `Value`.
///
/// Values are either block arguments or instructions, and
/// in addition to being linked to a `Inst` or a `Block`, they
/// have an associated type, position, and in some cases, a `SourceSpan`.
#[derive(Debug, Clone)]
pub enum ValueData {
    Inst {
        ty: Type,
        num: u16,
        inst: Inst,
    },
    Param {
        ty: Type,
        num: u16,
        block: Block,
        span: SourceSpan,
    },
}
impl ValueData {
    pub fn ty(&self) -> Type {
        match self {
            Self::Inst { ty, .. } | Self::Param { ty, .. } => ty.clone(),
        }
    }

    pub fn set_type(&mut self, ty: Type) {
        match self {
            Self::Inst {
                ty: ref mut prev_ty,
                ..
            } => *prev_ty = ty,
            Self::Param {
                ty: ref mut prev_ty,
                ..
            } => *prev_ty = ty,
        }
    }
}

pub struct Values<'a> {
    pub(super) inner: entity::Iter<'a, Value, ValueData>,
}
impl<'a> Iterator for Values<'a> {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.by_ref().next().map(|kv| kv.0)
    }
}
