use core::any::Any;

use super::*;

pub trait Op: Any + OpVerifier {
    /// The name of this operation's opcode
    ///
    /// The opcode must be distinct from all other opcodes in the same dialect
    fn name(&self) -> OperationName;
    fn as_operation(&self) -> &Operation;
    fn as_operation_mut(&mut self) -> &mut Operation;

    fn parent(&self) -> Option<BlockRef> {
        self.as_operation().parent()
    }
    fn parent_region(&self) -> Option<RegionRef> {
        self.as_operation().parent_region()
    }
    fn parent_op(&self) -> Option<OperationRef> {
        self.as_operation().parent_op()
    }
    fn regions(&self) -> &RegionList {
        self.as_operation().regions()
    }
    fn regions_mut(&mut self) -> &mut RegionList {
        self.as_operation_mut().regions_mut()
    }
    fn region(&self, index: usize) -> EntityRef<'_, Region> {
        self.as_operation().region(index)
    }
    fn region_mut(&mut self, index: usize) -> EntityMut<'_, Region> {
        self.as_operation_mut().region_mut(index)
    }
    fn has_operands(&self) -> bool {
        self.as_operation().has_operands()
    }
    fn num_operands(&self) -> usize {
        self.as_operation().num_operands()
    }
    fn operands(&self) -> &OpOperandStorage {
        self.as_operation().operands()
    }
    fn operands_mut(&mut self) -> &mut OpOperandStorage {
        self.as_operation_mut().operands_mut()
    }
    fn results(&self) -> &[OpResultRef] {
        self.as_operation().results()
    }
    fn results_mut(&mut self) -> &mut [OpResultRef] {
        self.as_operation_mut().results_mut()
    }
    fn successors(&self) -> &[OpSuccessor] {
        self.as_operation().successors()
    }
    fn successors_mut(&mut self) -> &mut [OpSuccessor] {
        self.as_operation_mut().successors_mut()
    }
}

impl Spanned for dyn Op {
    fn span(&self) -> SourceSpan {
        self.as_operation().span
    }
}

pub trait OpExt {
    /// Return the value associated with attribute `name` for this function
    fn get_attribute<Q>(&self, name: &Q) -> Option<&dyn AttributeValue>
    where
        interner::Symbol: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized;

    /// Return true if this function has an attributed named `name`
    fn has_attribute<Q>(&self, name: &Q) -> bool
    where
        interner::Symbol: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized;

    /// Set the attribute `name` with `value` for this function.
    fn set_attribute(
        &mut self,
        name: impl Into<interner::Symbol>,
        value: Option<impl AttributeValue>,
    );

    /// Remove any attribute with the given name from this function
    fn remove_attribute<Q>(&mut self, name: &Q)
    where
        interner::Symbol: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized;

    /// Returns a handle to the nearest containing [Operation] of type `T` for this operation, if it
    /// is attached to one
    fn nearest_parent_op<T: Op>(&self) -> Option<UnsafeIntrusiveEntityRef<T>>;
}

impl<T: ?Sized + Op> OpExt for T {
    #[inline]
    fn get_attribute<Q>(&self, name: &Q) -> Option<&dyn AttributeValue>
    where
        interner::Symbol: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.as_operation().get_attribute(name)
    }

    #[inline]
    fn has_attribute<Q>(&self, name: &Q) -> bool
    where
        interner::Symbol: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.as_operation().has_attribute(name)
    }

    #[inline]
    fn set_attribute(
        &mut self,
        name: impl Into<interner::Symbol>,
        value: Option<impl AttributeValue>,
    ) {
        self.as_operation_mut().set_attribute(name, value);
    }

    #[inline]
    fn remove_attribute<Q>(&mut self, name: &Q)
    where
        interner::Symbol: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.as_operation_mut().remove_attribute(name);
    }

    #[inline]
    fn nearest_parent_op<U: Op>(&self) -> Option<UnsafeIntrusiveEntityRef<U>> {
        self.as_operation().nearest_parent_op()
    }
}