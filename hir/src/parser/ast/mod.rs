mod block;
mod functions;
mod globals;
mod instruction;

pub use self::block::*;
pub use self::functions::*;
pub use self::globals::*;
pub use self::instruction::*;

use std::fmt;

use miden_diagnostics::{SourceSpan, Spanned};

use crate::Ident;

/// This is a type alias used to clarify that an identifier refers to a module
pub type ModuleId = Ident;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ModuleType {
    /// Kernel context module
    Kernel,
    /// User context module
    Module,
}
impl fmt::Display for ModuleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Kernel => f.write_str("kernel"),
            Self::Module => f.write_str("module"),
        }
    }
}

/// This represents the parsed contents of a single Miden IR module
///
#[derive(Spanned, Debug)]
pub struct Module {
    #[span]
    pub span: SourceSpan,
    pub name: ModuleId,
    pub ty: ModuleType,
    pub global_vars: Vec<GlobalVarDeclaration>,
    pub functions: Vec<FunctionDeclaration>,
    pub externals: Vec<FunctionSignature>,
}
impl Module {
    /// Constructs a new module of the specified type, with the given span, name, functions and exports (externals).
    ///
    pub fn new(
        span: SourceSpan,
        ty: ModuleType,
        name: ModuleId,
        global_vars: Vec<GlobalVarDeclaration>,
        functions: Vec<FunctionDeclaration>,
        externals: Vec<FunctionSignature>,
    ) -> Self {
        Self {
            span,
            name,
            ty,
            functions,
            externals,
            global_vars,
        }
    }
}
impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.ty == other.ty
            && self.global_vars == other.global_vars
            && self.functions == other.functions
            && self.externals == other.externals
    }
}
impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} {}", self.ty, self.name)?;
        for gvar in self.global_vars.iter() {
            writeln!(f, "{}", gvar)?;
        }
        for func in self.functions.iter() {
            writeln!(f, "{}", func)?;
        }
        for ext in self.externals.iter() {
            writeln!(f, "{};", ext)?;
        }
        Ok(())
    }
}
