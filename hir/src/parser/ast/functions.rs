use core::fmt;

use crate::{Ident, Signature};

use super::*;

/// Represents the declaration of a function in a [Module]
#[derive(Spanned)]
pub struct FunctionDeclaration {
    #[span]
    pub span: SourceSpan,
    pub name: Ident,
    pub signature: Signature,
    pub blocks: Vec<Block>,
}
impl FunctionDeclaration {
    pub fn new(span: SourceSpan, name: Ident, signature: Signature, blocks: Vec<Block>) -> Self {
        Self {
            span,
            name,
            signature,
            blocks,
        }
    }
}
impl fmt::Debug for FunctionDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FunctionDeclaration")
            .field("name", &self.name.as_symbol())
            .field("signature", &self.signature)
            .field("blocks", &self.blocks)
            .finish()
    }
}
impl PartialEq for FunctionDeclaration {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.signature == other.signature && self.blocks == other.blocks
    }
}
