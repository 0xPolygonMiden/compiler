mod block;
mod functions;
mod globals;
mod instruction;
mod types;

pub use self::block::*;
pub use self::functions::*;
pub use self::globals::*;
pub use self::instruction::*;
pub use self::types::*;

use std::fmt;

use miden_diagnostics::{SourceSpan, Span, Spanned};

use crate::Symbol;

/// This represents a fully parsed Miden IR program.
#[derive(Spanned)]
pub struct Program {
    #[span]
    pub span: SourceSpan,
    /// The set of modules in the program
    pub modules: Vec<Module>,
    /// The name of the function that acts as the entry point for the program
    pub entry_point: Option<FunctionIdentifier>,
    /// The global variables declared in this program
    pub global_vars: Vec<GlobalVarDeclaration>,
}
impl Program {
    /// Creates a new [Program].
    pub fn new(span: SourceSpan, modules: Vec<Module>, globals: Vec<GlobalVarDeclaration>) -> Self {
        Self {
            span: span,
            modules: modules,
            entry_point: None,
            global_vars: globals,
        }
    }

    pub fn with_entry_point(&mut self, name: FunctionIdentifier) {
        self.entry_point = Some(name)
    }
}
impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for module in self.modules.iter() {
            writeln!(f, "{}", module)?;
        }
        writeln!(f)?;
        if self.entry_point.is_some() {
            writeln!(f, "{}", self.entry_point.as_ref().unwrap())?;
        }
        for global in self.global_vars.iter() {
            writeln!(f, "{}", global)?;
        }
        Ok(())
    }
}

/// This is a type alias used to clarify that an identifier refers to a module
pub type ModuleId = Identifier;

#[derive(Copy, Clone, PartialEq, Eq)]
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
#[derive(Spanned)]
pub struct Module {
    #[span]
    pub span: SourceSpan,
    pub name: ModuleId,
    pub ty: ModuleType,
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
        functions: Vec<FunctionDeclaration>,
        externals: Vec<FunctionSignature>,
    ) -> Self {
        Self {
            span,
            name,
            ty,
            functions,
            externals,
        }
    }
}
impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} {}", self.ty, self.name)?;
        for func in self.functions.iter() {
            writeln!(f, "{}", func)?;
        }
        for ext in self.externals.iter() {
            writeln!(f, "{};", ext)?;
        }
        Ok(())
    }
}

/// Represents any type of identifier in Miden IR.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Spanned)]
pub struct Identifier(Span<Symbol>);
impl Identifier {
    pub fn new(span: SourceSpan, name: Symbol) -> Self {
        Self(Span::new(span, name))
    }

    /// Returns the underlying symbol of the identifier.
    pub fn name(&self) -> Symbol {
        self.0.item
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}
