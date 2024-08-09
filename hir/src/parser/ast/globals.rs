use core::fmt;

use super::{SourceSpan, Spanned};
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
    pub init: Option<crate::Constant>,
}
impl GlobalVarDeclaration {
    pub fn new(
        span: SourceSpan,
        id: crate::GlobalVariable,
        name: Ident,
        ty: Type,
        linkage: Linkage,
        init: Option<crate::Constant>,
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
        use crate::display::DisplayOptional;

        f.debug_struct("GlobalVarDeclaration")
            .field("id", &format_args!("{}", &self.id))
            .field("name", &self.name.as_symbol())
            .field("ty", &self.ty)
            .field("linkage", &self.linkage)
            .field("init", &DisplayOptional(self.init.as_ref()))
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

/// This represents the declaration of a constant
#[derive(Spanned)]
pub struct ConstantDeclaration {
    #[span]
    pub span: SourceSpan,
    pub id: crate::Constant,
    pub init: ConstantData,
}
impl ConstantDeclaration {
    pub fn new(span: SourceSpan, id: crate::Constant, init: ConstantData) -> Self {
        Self { span, id, init }
    }
}
impl fmt::Debug for ConstantDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ConstantDeclaration")
            .field("id", &format_args!("{}", &self.id))
            .field("init", &format_args!("{}", &self.init))
            .finish()
    }
}
impl PartialEq for ConstantDeclaration {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.init == other.init
    }
}

/// This represents the declaration of a data segment
#[derive(Spanned)]
pub struct DataSegmentDeclaration {
    #[span]
    pub span: SourceSpan,
    pub readonly: bool,
    pub offset: u32,
    pub size: u32,
    pub data: ConstantData,
}
impl DataSegmentDeclaration {
    pub fn new(
        span: SourceSpan,
        offset: u32,
        size: u32,
        readonly: bool,
        data: ConstantData,
    ) -> Self {
        Self {
            span,
            readonly,
            offset,
            size,
            data,
        }
    }
}
impl fmt::Debug for DataSegmentDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DataSegmentDeclaration")
            .field("offset", &self.offset)
            .field("size", &self.size)
            .field("readonly", &self.readonly)
            .field("data", &format_args!("{}", &self.data))
            .finish()
    }
}
impl PartialEq for DataSegmentDeclaration {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset
            && self.size == other.size
            && self.readonly == other.readonly
            && self.data == other.data
    }
}
