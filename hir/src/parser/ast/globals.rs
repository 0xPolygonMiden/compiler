use core::fmt;

use miden_diagnostics::{SourceSpan, Spanned};

use crate::{ConstantData, Ident, Linkage, Type};

/// This represents the declaration of a global variable
#[derive(Spanned)]
pub struct GlobalVarDeclaration {
    #[span]
    pub span: SourceSpan,
    pub id: crate::GlobalVariable,
    pub name: Ident,
    pub ty: Type,
    pub linkage: Linkage,
    pub init: Option<ConstantData>,
}
impl GlobalVarDeclaration {
    pub fn new(
        span: SourceSpan,
        id: crate::GlobalVariable,
        name: Ident,
        ty: Type,
        linkage: Linkage,
        init: Option<ConstantData>,
    ) -> Self {
        Self {
            span,
            id,
            name,
            ty,
            linkage,
            init,
        }
    }
}
impl fmt::Debug for GlobalVarDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("GlobalVarDeclaration")
            .field("id", &format_args!("{}", &self.id))
            .field("name", &self.name.as_symbol())
            .field("ty", &self.ty)
            .field("linkage", &self.linkage)
            .field("init", &DisplayOptionalConstantData(self.init.as_ref()))
            .finish()
    }
}
impl PartialEq for GlobalVarDeclaration {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.ty == other.ty
            && self.linkage == other.linkage
            && self.init == other.init
    }
}

struct DisplayOptionalConstantData<'a>(Option<&'a ConstantData>);
impl<'a> fmt::Debug for DisplayOptionalConstantData<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => f.write_str("None"),
            Some(data) => write!(f, "Some({data})"),
        }
    }
}
