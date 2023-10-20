use super::*;

/// Represents an identifier that represents a function name.
///
/// A function identifier is a non-empty sequence of identifiers, separated by double
/// colons ("::"). The last identifier in the sequence denotes the name of the function
/// itself. The other identifiers denote the module that the function can be found in. If
/// the function identifier only consists of a single identifier, then the function must
/// be found in the current module.
#[derive(Spanned)]
pub struct FunctionIdentifier {
    #[span]
    span: SourceSpan,
    names: Vec<Identifier>,
}
impl FunctionIdentifier {
    pub fn new(span: SourceSpan, names: Vec<Identifier>) -> Self {
        Self { span, names }
    }

    pub fn id(&self) -> &Identifier {
        self.names.last().unwrap()
    }
}
impl fmt::Display for FunctionIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, id) in self.names.iter().enumerate() {
            if i > 0 {
                f.write_str("::")?;
            }
            write!(f, "{}", id)?;
        }
        Ok(())
    }
}

/// The possible visibilities of a function
pub enum Visibility {
    /// (Module) private visibility
    Private,
    /// Public visibility
    Public,
}

/// The possible calling convetions of a function
pub enum CallConvention {
    /// Default call convention
    Default,
    /// Kernel call convention
    Kernel,
    /// Fast call convention
    Fast,
}

/// The possible purposes of a function parameter
pub enum ParameterPurpose {
    /// Standard parameter
    Standard,
    /// Parameter for struct return
    Sret,
}

/// The possible extensions of a function parameter when filling up a word
pub enum ParameterExtension {
    /// No extension
    None,
    /// 0 extended
    Zero,
    /// Sign extended
    Signed,
}

/// A single parameter to a function.
/// Parameter names are defined in the entry block for the function.
pub struct FunctionParameter {
    /// The purpose of the parameter (default or struct return)
    pub purpose: ParameterPurpose,
    /// The bit extension for the parameter
    pub extension: ParameterExtension,
    /// The type of the parameter
    pub ty: Type,
}
impl FunctionParameter {
    pub fn new(purpose: ParameterPurpose, extension: ParameterExtension, ty: Type) -> Self {
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
            ParameterPurpose::Standard => Ok(()),
            ParameterPurpose::Sret => f.write_str("sret "),
        }?;
        match self.extension {
            ParameterExtension::None => Ok(()),
            ParameterExtension::Zero => f.write_str("zext "),
            ParameterExtension::Signed => f.write_str("sext "),
        }?;
        write!(f, "{}", self.ty)
    }
}

/// A single return value from a function.
pub struct FunctionReturn {
    /// The bit extension for the parameter
    pub extension: ParameterExtension,
    /// The type of the parameter
    pub ty: Type,
}
impl FunctionReturn {
    pub fn new(extension: ParameterExtension, ty: Type) -> Self {
        Self { extension, ty }
    }
}
impl fmt::Display for FunctionReturn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.extension {
            ParameterExtension::None => Ok(()),
            ParameterExtension::Zero => f.write_str("zext "),
            ParameterExtension::Signed => f.write_str("sext "),
        }?;
        write!(f, "{}", self.ty)
    }
}

/// Represents the type signature of a function
#[derive(Spanned)]
pub struct FunctionSignature {
    #[span]
    pub span: SourceSpan,
    pub visibility: Visibility,
    pub call_convention: CallConvention,
    pub name: FunctionIdentifier,
    pub params: Vec<FunctionParameter>,
    pub returns: Vec<FunctionReturn>,
}
impl FunctionSignature {
    pub fn new(
        span: SourceSpan,
        visibility: Visibility,
        call_convention: CallConvention,
        name: FunctionIdentifier,
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
impl fmt::Display for FunctionSignature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.visibility {
            Visibility::Private => Ok(()),
            Visibility::Public => f.write_str("pub "),
        }?;
        match self.call_convention {
            CallConvention::Default => Ok(()),
            CallConvention::Kernel => f.write_str("kernel "),
            CallConvention::Fast => f.write_str("fast "),
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
#[derive(Spanned)]
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
