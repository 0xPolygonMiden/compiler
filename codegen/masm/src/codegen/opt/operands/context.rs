use std::{collections::BTreeMap, num::NonZeroU8};

use miden_hir as hir;

use super::{SolverError, Stack, ValueOrAlias};
use crate::codegen::Constraint;

/// The context associated with an instance of [OperandMovementConstraintSolver].
///
/// Contained in this context is the current state of the stack, the expected operands,
/// the constraints on those operands, and metadata about copied operands.
#[derive(Debug)]
pub struct SolverContext {
    stack: Stack,
    expected: Stack,
    copies: CopyInfo,
}
impl SolverContext {
    pub fn new(
        expected: &[hir::Value],
        constraints: &[Constraint],
        stack: &crate::codegen::OperandStack,
    ) -> Result<Self, SolverError> {
        use std::collections::btree_map::Entry;

        // Compute the expected output on the stack, as well as alias/copy information
        let mut stack = Stack::from(stack);
        let mut expected_output = Stack::default();
        let mut copies = CopyInfo::default();
        for (value, constraint) in expected.iter().rev().zip(constraints.iter().rev()) {
            let value = ValueOrAlias::from(*value);
            match constraint {
                // If we observe a value with move semantics, then it is
                // always referencing the original value
                Constraint::Move => {
                    expected_output.push(value);
                }
                // If we observe a value with copy semantics, then it is
                // always referencing an alias, because the original would
                // need to be preserved
                Constraint::Copy => {
                    expected_output.push(copies.push(value));
                }
            }
        }

        // Rename multiple occurrences of the same value on the operand stack, if present
        let mut renamed = BTreeMap::<ValueOrAlias, u8>::default();
        for operand in stack.iter_mut().rev() {
            match renamed.entry(operand.value) {
                Entry::Vacant(entry) => {
                    entry.insert(0);
                }
                Entry::Occupied(mut entry) => {
                    let next_id = entry.get_mut();
                    *next_id += 1;
                    operand.value.set_alias(NonZeroU8::new(*next_id).unwrap());
                }
            }
        }

        // Determine if the stack is already in the desired order
        //
        // If copies are required we can't consider the stack in order even if
        // the operands we want are in the desired order, because we must make
        // copies of them anyway.
        let requires_copies = !copies.is_empty();
        let is_solved = !requires_copies
            && expected_output.iter().rev().all(|op| &stack[op.pos as usize] == op);
        if is_solved {
            return Err(SolverError::AlreadySolved);
        }

        Ok(Self {
            stack,
            expected: expected_output,
            copies,
        })
    }

    /// Returns the number of operands expected by the current instruction
    #[inline]
    pub fn arity(&self) -> usize {
        self.expected.len()
    }

    /// Get a reference to the copy analysis results
    #[inline(always)]
    pub fn copies(&self) -> &CopyInfo {
        &self.copies
    }

    /// Get a reference to the state of the stack at the current program point
    #[inline(always)]
    pub fn stack(&self) -> &Stack {
        &self.stack
    }

    /// Get a [Stack] representing the state of the stack for a valid solution.
    ///
    /// NOTE: The returned stack only contains the expected operands, not the full stack
    #[inline(always)]
    pub fn expected(&self) -> &Stack {
        &self.expected
    }

    /// Return true if the given stack matches what is expected
    /// if a solution was correctly found.
    pub fn is_solved(&self, pending: &Stack) -> bool {
        debug_assert!(pending.len() >= self.expected.len());
        self.expected.iter().all(|o| pending.contains(o))
    }
}

#[derive(Debug, Default)]
pub struct CopyInfo {
    copies: BTreeMap<ValueOrAlias, u8>,
    num_copies: u8,
}
impl CopyInfo {
    /// Returns the number of copies recorded in this structure
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.num_copies as usize
    }

    /// Returns true if there are no copied values
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.num_copies == 0
    }

    /// Push a new copy of `value`, returning an alias of that value
    pub fn push(&mut self, value: ValueOrAlias) -> ValueOrAlias {
        use std::collections::btree_map::Entry;

        self.num_copies += 1;
        match self.copies.entry(value) {
            Entry::Vacant(entry) => {
                entry.insert(0);
                value.copy(unsafe { NonZeroU8::new_unchecked(1) })
            }
            Entry::Occupied(mut entry) => {
                let next_id = entry.get_mut();
                *next_id += 1;
                value.copy(NonZeroU8::new(*next_id).unwrap())
            }
        }
    }

    pub fn has_copies(&self, value: &ValueOrAlias) -> bool {
        self.copies.contains_key(value)
    }
}
