use core::fmt;

use intrusive_collections::LinkedList;
use miden_hir::{FunctionIdent, Ident};

use super::{Function, FunctionListAdapter, ModuleImportInfo, Op};

pub struct Module {
    /// The name of this module, e.g. `std::math::u64`
    pub name: Ident,
    /// If this module contains a program entrypoint, this is the
    /// function identifier which should be used for that purpose.
    pub entry: Option<FunctionIdent>,
    /// The modules to import, along with their local aliases
    pub imports: ModuleImportInfo,
    /// The functions defined in this module
    pub functions: LinkedList<FunctionListAdapter>,
}

impl Module {
    pub fn new(name: Ident) -> Self {
        Self {
            name,
            entry: None,
            imports: Default::default(),
            functions: Default::default(),
        }
    }

    /// Write this module as Miden Assembly text to `out`
    pub fn emit(&self, _out: &mut dyn std::io::Write) -> std::io::Result<()> {
        todo!()
    }
}
