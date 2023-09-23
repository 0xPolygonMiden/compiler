use intrusive_collections::LinkedList;
use miden_hir::Ident;
use std::collections::BTreeSet;

use super::{FunctionListAdapter, Import};

pub struct Module {
    /// The name of this module, e.g. `std::math::u64`
    pub name: Ident,
    /// The modules to import, along with their local aliases
    pub imports: BTreeSet<Import>,
    /// The functions defined in this module
    pub functions: LinkedList<FunctionListAdapter>,
}

impl Module {
    pub fn new(name: Ident) -> Self {
        Self {
            name,
            imports: Default::default(),
            functions: Default::default(),
        }
    }
}
