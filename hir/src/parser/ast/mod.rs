mod block;
mod functions;
mod globals;
mod instruction;

pub use self::block::*;
pub use self::functions::*;
pub use self::globals::*;
pub use self::instruction::*;

use core::fmt;

use miden_diagnostics::{SourceSpan, Spanned};

use crate::{ExternalFunction, Ident};

/// This represents the parsed contents of a single Miden IR module
#[derive(Spanned)]
pub struct Module {
    #[span]
    pub span: SourceSpan,
    pub name: Ident,
    pub global_vars: Vec<GlobalVarDeclaration>,
    pub functions: Vec<FunctionDeclaration>,
    pub externals: Vec<ExternalFunction>,
    pub is_kernel: bool,
}
impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Module")
            .field("name", &self.name.as_symbol())
            .field("global_vars", &self.global_vars)
            .field("functions", &self.functions)
            .field("externals", &self.externals)
            .field("is_kernel", &self.is_kernel)
            .finish()
    }
}
impl Module {
    pub fn new(span: SourceSpan, name: Ident, is_kernel: bool, forms: Vec<Form>) -> Self {
        let mut module = Self {
            span,
            name,
            functions: vec![],
            externals: vec![],
            global_vars: vec![],
            is_kernel,
        };
        for form in forms.into_iter() {
            match form {
                Form::Global(global) => {
                    module.global_vars.push(global);
                }
                Form::Function(function) => {
                    module.functions.push(function);
                }
                Form::ExternalFunction(external) => {
                    module.externals.push(external);
                }
            }
        }
        module
    }
}
impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.is_kernel == other.is_kernel
            && self.global_vars == other.global_vars
            && self.functions == other.functions
            && self.externals == other.externals
    }
}

/// This represents one of the top-level forms which a [Module] can contain
#[derive(Debug)]
pub enum Form {
    Global(GlobalVarDeclaration),
    Function(FunctionDeclaration),
    ExternalFunction(ExternalFunction),
}
