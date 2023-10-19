use std::fmt;

use miden_diagnostics::{SourceSpan, Spanned};

use super::*;

/// This is a type alias used to clarify that an identifier refers to a global variable
pub type GlobalVarId = Identifier;

/// A constant value in the form of a hexadecimal string
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GlobalVarInitializer {
    pub data: Vec<u8>,
}
impl GlobalVarInitializer {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}
impl fmt::Display for GlobalVarInitializer {
    /// Print the constant data in hexadecimal format, e.g. 0x000102030405060708090a0b0c0d0e0f.
    ///
    /// The printed form of the constant renders the bytes in big-endian order, for readability.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.data.is_empty() {
            write!(f, "0x")?;
            for b in self.data.iter().rev() {
                write!(f, "{:02x}", b)?;
            }
        }
        Ok(())
    }
}

/// This represents the declaration of a Miden IR global variable
#[derive(Spanned)]
pub struct GlobalVarDeclaration {
    #[span]
    pub span: SourceSpan,
    pub name: GlobalVarId,
    pub ty: Type,
    pub linkage: Linkage,
    pub init: Option<GlobalVarInitializer>,
}
impl GlobalVarDeclaration {
    /// Constructs a new global variable, with the given span, name, type, linkage, and optinal initializer.
    ///
    pub fn new(span: SourceSpan, name: ModuleId, ty: Type, linkage: Linkage) -> Self {
        Self {
            span: span,
            name: name,
            ty: ty,
            linkage: linkage,
            init: None,
        }
    }

    pub fn with_init(&mut self, init: GlobalVarInitializer) {
        self.init = Some(init)
    }
}
impl fmt::Display for GlobalVarDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.name, self.ty, self.linkage)?;
        if self.init.is_some() {
            write!(f, "= {}", self.init.as_ref().unwrap())?;
        }
        Ok(())
    }
}
/// Represents the intended linkage for a global variable.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Linkage {
    /// Global linkage
    Internal,
    /// "One definition rule" linkage
    Odr,
    /// External linkage
    External,
}
impl fmt::Display for Linkage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Internal => write!(f, "internal"),
            Self::Odr => write!(f, "odr"),
            Self::External => write!(f, "external"),
        }
    }
}
