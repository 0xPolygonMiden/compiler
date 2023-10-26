use core::fmt;

use super::*;

/// Represents a basic block in a [FunctionDeclaration]
#[derive(Spanned)]
pub struct Block {
    #[span]
    pub span: SourceSpan,
    pub id: crate::Block,
    pub params: Vec<TypedValue>,
    pub body: Vec<Inst>,
}
impl Block {
    pub fn new(
        span: SourceSpan,
        id: crate::Block,
        params: Vec<TypedValue>,
        body: Vec<Inst>,
    ) -> Self {
        Self {
            span,
            id,
            params,
            body,
        }
    }
}
impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Block")
            .field("id", &format_args!("{}", self.id))
            .field("params", &self.params)
            .field("body", &self.body)
            .finish()
    }
}
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.params == other.params && self.body == other.body
    }
}
