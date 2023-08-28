use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;
use std::rc::Rc;

use cranelift_entity::{entity_impl, PrimaryMap};

use miden_diagnostics::{SourceSpan, Spanned};

use crate::types::{FunctionType, Type};

use super::*;

/// A handle that refers to a function definition/declaration
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FuncRef(u32);
entity_impl!(FuncRef, "fn");

bitflags::bitflags! {
    pub struct Visibility: u8 {
        /// The function is private
        const PRIVATE = 1;
        /// The function is public
        const PUBLIC = 1 << 1;
        /// The function is defined externally, but referenced locally
        const EXTERN = 1 << 2;
    }
}
impl Visibility {
    pub fn is_externally_defined(&self) -> bool {
        self.contains(Self::EXTERN)
    }

    pub fn is_public(&self) -> bool {
        self.contains(Self::PUBLIC)
    }
}
impl Default for Visibility {
    fn default() -> Self {
        Self::PRIVATE
    }
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub visibility: Visibility,
    pub name: String,
    pub ty: FunctionType,
}
impl Signature {
    /// Returns a slice of the parameter types for this function
    pub fn params(&self) -> &[Type] {
        self.ty.params()
    }

    /// Returns the parameter type of the argument at `index`, if present
    #[inline]
    pub fn param(&self, index: usize) -> Option<&Type> {
        self.ty.params().get(index)
    }

    pub fn results(&self) -> &[Type] {
        match self.ty.results() {
            [Type::Unit] => &[],
            [Type::Never] => &[],
            results => results,
        }
    }
}
impl Eq for Signature {}
impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.params().len() == other.params().len()
            && self.results().len() == other.results().len()
    }
}

/// Represents the dataflow structure of a function definition
#[derive(Spanned)]
pub struct Function {
    pub id: FuncRef,
    #[span]
    pub span: SourceSpan,
    pub signature: Signature,
    pub dfg: DataFlowGraph,
}
impl Function {
    pub fn new(
        id: FuncRef,
        span: SourceSpan,
        signature: Signature,
        signatures: Rc<RefCell<PrimaryMap<FuncRef, Signature>>>,
        callees: Rc<RefCell<BTreeMap<String, FuncRef>>>,
    ) -> Self {
        let dfg = DataFlowGraph::new(signatures, callees);
        Self {
            id,
            span,
            signature,
            dfg,
        }
    }
}
impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Function")
            .field("id", &self.id)
            .field("span", &self.span)
            .field("signature", &self.signature)
            .finish()
    }
}
