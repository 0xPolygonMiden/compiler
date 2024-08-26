use core::{
    any::{Any, TypeId},
    mem,
    ptr::{NonNull, Pointee},
};

use cranelift_entity::{packed_option::ReservedValue, EntityRef};
use downcast_rs::{impl_downcast, Downcast};
use intrusive_collections::{
    container_of, intrusive_adapter,
    linked_list::{LinkOps, LinkedListOps},
    LinkedListLink, UnsafeRef,
};
use smallvec::SmallVec;

use crate::*;

intrusive_adapter!(pub OpAdapter = UnsafeRef<Operation>: Operation { link: LinkedListLink });

pub type OpList = intrusive_collections::LinkedList<InstAdapter>;
pub type OpCursor<'a> = intrusive_collections::linked_list::Cursor<'a, OpAdapter>;
pub type OpCursorMut<'a> = intrusive_collections::linked_list::CursorMut<'a, OpAdapter>;

/// A unique identifier for a given operation instance
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpId(u32);
entity_impl!(OpId, "op");

pub struct OpOperand {
    /// Link in the `uses` list of `value`
    link: LinkedListLink,
    pub value: Value,
}

#[derive(Spanned)]
pub struct Operation {
    /// Link in the `ops` list of `block`
    link: LinkedListLink,
    /// The owning context of this operation
    context: Rc<Context>,
    /// In order to support upcasting from [Operation] to its concrete [Op] type, as well as
    /// casting to any of the operation traits it implements, we need our own vtable that lets
    /// us track the individual vtables for each type and trait we need to cast to for this
    /// instance.
    pub(crate) vtable: traits::MultiTraitVtable,
    /// The unique identifier for this operation
    pub key: OpId,
    #[span]
    pub span: SourceSpan,
    /// Attributes that apply to this operation
    pub attrs: AttributeSet,
    /// The containing block of this operation
    ///
    /// If `Block::is_reserved_value()` returns true, the operation has no containing block
    pub block: Block,
    /// If this operation is contained in a region of another operation, this field is set
    /// to the identifier of the containing operation. This can be used to navigate up the
    /// hierarchy of operations if needed.
    ///
    /// If `OpId::is_reserved_value()` returns true, the operation has no parent
    pub parent: OpId,
    /// The set of operands for this operation
    ///
    /// NOTE: If the op supports immediate operands, the storage for the immediates is handled
    /// by the op, rather than here. Additionally, the semantics of the immediate operands are
    /// determined by the op, e.g. whether the immediate operands are always applied first, or
    /// what they are used for.
    pub operands: SmallVec<[UnsafeRef<OpOperand>; 1]>,
    /// The set of values produced by this operation.
    pub results: SmallVec<[Value; 1]>,
    /// If this operation represents control flow, this field stores the set of successors,
    /// and successor operands.
    pub successors: SmallVec<[Successor; 1]>,
    /// If this operation contains regions, this field stores them
    pub regions: SmallVec<[RegionId; 1]>,
}
impl AsRef<dyn Op> for Operation {
    fn as_ref(&self) -> &dyn Op {
        self.vtable.downcast_trait().unwrap()
    }
}
impl AsMut<dyn Op> for Operation {
    fn as_mut(&mut self) -> &mut dyn Op {
        self.vtable.downcast_trait_mut().unwrap()
    }
}
impl Operation {
    pub fn new<T: Op>(context: Rc<Context>, span: SourceSpan) -> Self {
        use crate::traits::MultiTraitVtable;

        let mut vtable = MultiTraitVtable::new::<T>();
        vtable.register_trait::<T, dyn Op>();

        Self {
            link: LinkedListLink::new(),
            context,
            vtable,
            key: Default::default(),
            span,
            attrs: Default::default(),
            block: Default::default(),
            parent: Default::default(),
            operands: Default::default(),
            results: Default::default(),
            successors: Default::default(),
            regions: Default::default(),
        }
    }
}

/// Traits/Casts
impl Operation {
    /// Returns true if the concrete type of this operation is `T`
    #[inline]
    pub fn is<T: Op>(&self) -> bool {
        self.vtable.is::<T>()
    }

    /// Returns true if this operation implements `Trait`
    #[inline]
    pub fn implements<Trait>(&self) -> bool
    where
        Trait: ?Sized + Pointee<Metadata = *const ()> + 'static,
    {
        self.vtable.implements::<Trait>()
    }

    /// Attempt to downcast to the concrete [Op] type of this operation
    pub fn downcast_ref<T: Op>(&self) -> Option<&T> {
        self.vtable.downcast_ref::<T>()
    }

    /// Attempt to downcast to the concrete [Op] type of this operation
    pub fn downcast_mut<T: Op>(&mut self) -> Option<&mut T> {
        self.vtable.downcast_mut::<T>()
    }
}

/// Attributes
impl Operation {
    /// Return the value associated with attribute `name` for this function
    pub fn get_attribute<Q>(&self, name: &Q) -> Option<&AttributeValue>
    where
        Symbol: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.attrs.get(name)
    }

    /// Return true if this function has an attributed named `name`
    pub fn has_attribute<Q>(&self, name: &Q) -> bool
    where
        Symbol: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.attrs.has(name)
    }

    /// Set the attribute `name` with `value` for this function.
    pub fn set_attribute(&mut self, name: impl Into<Symbol>, value: impl Into<AttributeValue>) {
        self.attrs.insert(name, value);
    }

    /// Remove any attribute with the given name from this function
    pub fn remove_attribute<Q>(&mut self, name: &Q)
    where
        Symbol: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.attrs.remove(name);
    }
}

/// Navigation
impl Operation {
    pub fn prev(&self) -> Option<OpId> {
        unsafe {
            let current = core::ptr::NonNull::new_unchecked(&self.link);
            LinkOps.prev(current).map(Self::link_to_key)
        }
    }

    pub fn next(&self) -> Option<OpId> {
        unsafe {
            let current = core::ptr::NonNull::new_unchecked(&self.link);
            LinkOps.next(current).map(Self::link_to_key)
        }
    }

    #[inline]
    unsafe fn link_to_key(link: NonNull<LinkedListLink>) -> OpId {
        let link = link.as_ref();
        let operation = container_of!(link, Operation, link);
        let key_offset = mem::offset_of!(Operation, key);
        let prev_key = operation.byte_add(key_offset as isize) as *const OpId;
        *prev_key
    }
}

/// Operands
impl Operation {
    pub fn replaces_uses_of_with(&mut self, from: Value, to: Value) {
        if from == to {
            return;
        }

        for operand in self.operands.iter_mut() {
            if operand == &from {
                *operand = to;
            }
        }
    }
}

pub trait Op: Downcast {
    type Id: Copy + PartialEq + Eq + PartialOrd + Ord;

    fn id(&self) -> Self::Id;
    fn name(&self) -> &'static str;
    fn parent(&self) -> Option<OpId> {
        let parent = self.as_operation().parent;
        if parent.is_reserved_value() {
            None
        } else {
            Some(parent)
        }
    }
    fn prev(&self) -> Option<OpId> {
        self.as_operation().prev()
    }
    fn next(&self) -> Option<OpId> {
        self.as_operation().next()
    }
    fn parent_block(&self) -> Option<Block> {
        let block = self.as_operation().block;
        if block.is_reserved_value() {
            None
        } else {
            Some(block)
        }
    }
    fn regions(&self) -> &[RegionId] {
        self.as_operation().regions.as_slice()
    }
    fn operands(&self) -> &ValueList {
        &self.as_operation().operands
    }
    fn results(&self) -> &ValueList {
        &self.as_operation().results
    }
    fn successors(&self) -> &[Successor] {
        self.as_operation().successors.as_slice()
    }
    fn as_operation(&self) -> &Operation;
    fn as_operation_mut(&mut self) -> &mut Operation;
}

impl_downcast!(Op assoc Id where Id: Copy + PartialEq + Eq + PartialOrd + Ord);

impl miden_assembly::Spanned for dyn Op {
    fn span(&self) -> SourceSpan {
        self.as_operation().span
    }
}

pub trait OpExt {
    /// Return the value associated with attribute `name` for this function
    fn get_attribute<Q>(&self, name: &Q) -> Option<&AttributeValue>
    where
        Symbol: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized;

    /// Return true if this function has an attributed named `name`
    fn has_attribute<Q>(&self, name: &Q) -> bool
    where
        Symbol: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized;

    /// Set the attribute `name` with `value` for this function.
    fn set_attribute(&mut self, name: impl Into<Symbol>, value: impl Into<AttributeValue>);

    /// Remove any attribute with the given name from this function
    fn remove_attribute<Q>(&mut self, name: &Q)
    where
        Symbol: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized;
}

impl<T: Op> OpExt for T {
    /// Return the value associated with attribute `name` for this function
    #[inline]
    fn get_attribute<Q>(&self, name: &Q) -> Option<&AttributeValue>
    where
        Symbol: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.as_operation().get_attribute(name)
    }

    /// Return true if this function has an attributed named `name`
    #[inline]
    fn has_attribute<Q>(&self, name: &Q) -> bool
    where
        Symbol: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.as_operation().has_attribute(name)
    }

    /// Set the attribute `name` with `value` for this function.
    #[inline]
    fn set_attribute(&mut self, name: impl Into<Symbol>, value: impl Into<AttributeValue>) {
        self.as_operation_mut().insert(name, value);
    }

    /// Remove any attribute with the given name from this function
    #[inline]
    fn remove_attribute<Q>(&mut self, name: &Q)
    where
        Symbol: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.as_operation_mut().remove(name);
    }
}
