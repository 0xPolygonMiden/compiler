use super::*;
use crate::{ArgumentExtension, ArgumentPurpose, CallConv, FunctionIdent, Type};

/// The possible visibilities of a function
#[derive(PartialEq, Debug)]
pub enum Visibility {
    /// (Module) private visibility
    Private,
    /// Public visibility
    Public,
}

/// A single parameter to a function.
/// Parameter names are defined in the entry block for the function.
#[derive(PartialEq, Debug)]
pub struct FunctionParameter {
    /// The purpose of the parameter (default or struct return)
    pub purpose: ArgumentPurpose,
    /// The bit extension for the parameter
    pub extension: ArgumentExtension,
    /// The type of the parameter
    pub ty: Type,
}
impl FunctionParameter {
    pub fn new(purpose: ArgumentPurpose, extension: ArgumentExtension, ty: Type) -> Self {
        Self {
            purpose,
            extension,
            ty,
        }
    }
}
impl fmt::Display for FunctionParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.purpose {
            ArgumentPurpose::Default => Ok(()),
            ArgumentPurpose::StructReturn => f.write_str("sret "),
        }?;
        match self.extension {
            ArgumentExtension::None => Ok(()),
            ArgumentExtension::Zext => f.write_str("zext "),
            ArgumentExtension::Sext => f.write_str("sext "),
        }?;
        write!(f, "{}", self.ty)
    }
}

/// A single return value from a function.
#[derive(PartialEq, Debug)]
pub struct FunctionReturn {
    /// The bit extension for the parameter
    pub extension: ArgumentExtension,
    /// The type of the parameter
    pub ty: Type,
}
impl FunctionReturn {
    pub fn new(extension: ArgumentExtension, ty: Type) -> Self {
        Self { extension, ty }
    }
}
impl fmt::Display for FunctionReturn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.extension {
            ArgumentExtension::None => Ok(()),
            ArgumentExtension::Zext => f.write_str("zext "),
            ArgumentExtension::Sext => f.write_str("sext "),
        }?;
        write!(f, "{}", self.ty)
    }
}

/// Represents the type signature of a function
#[derive(Spanned, Debug)]
pub struct FunctionSignature {
    #[span]
    pub span: SourceSpan,
    pub visibility: Visibility,
    pub call_convention: CallConv,
    pub name: FunctionIdent,
    pub params: Vec<FunctionParameter>,
    pub returns: Vec<FunctionReturn>,
}
impl FunctionSignature {
    pub fn new(
        span: SourceSpan,
        visibility: Visibility,
        call_convention: CallConv,
        name: FunctionIdent,
        params: Vec<FunctionParameter>,
        returns: Vec<FunctionReturn>,
    ) -> Self {
        Self {
            span,
            visibility,
            call_convention,
            name,
            params,
            returns,
        }
    }
}
impl PartialEq for FunctionSignature {
    fn eq(&self, other: &Self) -> bool {
        self.visibility == other.visibility
            && self.call_convention == other.call_convention
            && self.name == other.name
            && self.params == other.params
            && self.returns == other.returns
    }
}
impl fmt::Display for FunctionSignature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.visibility {
            Visibility::Private => Ok(()),
            Visibility::Public => f.write_str("pub "),
        }?;
        match self.call_convention {
            CallConv::SystemV => Ok(()),
            CallConv::Kernel => f.write_str("kernel "),
            CallConv::Fast => f.write_str("fast "),
        }?;
        write!(f, "{}(", self.name)?;
        for (i, param) in self.params.iter().enumerate() {
            if i != 0 {
                write!(f, ", {}", param)?;
            } else {
                write!(f, "{}", param)?;
            }
        }
        f.write_str(")")?;
        for (i, ret) in self.returns.iter().enumerate() {
            if i != 0 {
                write!(f, ", {}", ret)?;
            } else {
                write!(f, "{}", ret)?;
            }
        }
        Ok(())
    }
}

/// Represents the declaration of a function
#[derive(Spanned, Debug)]
pub struct FunctionDeclaration {
    #[span]
    pub span: SourceSpan,
    pub signature: FunctionSignature,
    pub blocks: Vec<Block>,
}
impl FunctionDeclaration {
    pub fn new(span: SourceSpan, signature: FunctionSignature, blocks: Vec<Block>) -> Self {
        Self {
            span,
            signature,
            blocks,
        }
    }
}
impl PartialEq for FunctionDeclaration {
    fn eq(&self, other: &Self) -> bool {
        self.signature == other.signature
            && self.blocks == other.blocks
    }
}
impl fmt::Display for FunctionDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.signature)?;
        f.write_str("{{\n")?;
        for block in self.blocks.iter() {
            write!(f, "{}", block)?;
        }
        f.write_str("}}\n")
    }
}
