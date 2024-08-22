use super::*;
use crate::Type;

/// Represents a region in a [FunctionDeclaration]
#[derive(Default, Spanned)]
pub struct Region {
    #[span]
    pub span: SourceSpan,
    pub id: crate::RegionId,
    pub params: Vec<Span<Type>>,
    pub results: Vec<Span<Type>>,
    pub blocks: Vec<Block>,
}
impl Region {
    pub fn new(
        span: SourceSpan,
        id: crate::RegionId,
        params: Vec<Span<Type>>,
        results: Vec<Span<Type>>,
        blocks: Vec<Block>,
    ) -> Self {
        Self {
            span,
            id,
            params,
            results,
            blocks,
        }
    }
}
impl fmt::Debug for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Region")
            .field("id", &format_args!("{}", self.id))
            .field("params", &self.params)
            .field("results", &self.results)
            .field("blocks", &self.blocks)
            .finish()
    }
}
impl Eq for Region {}
impl PartialEq for Region {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.params == other.params
            && self.results == other.results
            && self.blocks == other.blocks
    }
}
