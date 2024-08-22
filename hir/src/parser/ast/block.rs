use super::*;

/// Represents a basic block in a [FunctionDeclaration]
#[derive(Spanned)]
pub struct Block {
    #[span]
    pub span: SourceSpan,
    pub id: crate::Block,
    pub region_id: crate::RegionId,
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
            region_id: Default::default(),
            params,
            body,
        }
    }
}
impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Block")
            .field("id", &format_args!("{}", self.id))
            .field("region_id", &format_args!("{}", self.region_id))
            .field("params", &self.params)
            .field("body", &self.body)
            .finish()
    }
}
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.region_id == other.region_id
            && self.params == other.params
            && self.body == other.body
    }
}
