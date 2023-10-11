pub(crate) mod adt;
mod inline_blocks;
mod split_critical_edges;
mod treeify;

pub use self::inline_blocks::InlineBlocks;
pub use self::split_critical_edges::SplitCriticalEdges;
pub use self::treeify::Treeify;

use miden_hir_analysis::FunctionAnalysis;
use miden_hir_pass::Pass;

/// A [RewritePass] is a special kind of [Pass] which is designed to perform some
/// kind of rewrite transformation on a [miden_hir::Function].
///
/// Rewrites require one or more control flow analyses to have been computed, as
/// determined by the requirements of the pass itself. The [FunctionAnalysis]
/// structure is designed for this purpose, allowing one to request specific
/// analysis results, which will be computed on-demand if not yet available.
pub trait RewritePass {
    type Error;

    /// Runs the rewrite on `function` with `analyses`.
    ///
    /// Rewrites should return `Err` to signal that the pass has failed
    /// and compilation should be aborted
    fn run(
        &mut self,
        function: &mut miden_hir::Function,
        analyses: &mut FunctionAnalysis,
    ) -> Result<(), Self::Error>;

    /// Chains two rewrites together to form a new, fused rewrite
    fn chain<P>(self, pass: P) -> RewriteChain<Self, P>
    where
        Self: Sized,
        P: RewritePass<Error = Self::Error>,
    {
        RewriteChain::new(self, pass)
    }
}

/// [RewriteChain] is the equivalent of [miden_hir_pass::Chain] for [RewritePass].
///
/// This is not meant to be constructed or referenced directly, as the type signature gets out
/// of hand quickly when combining multiple rewrites. Instead, you should invoke `chain` on a
/// [RewritePass] implementation, and use it as a trait object. In some cases this may require boxing
/// the `RewriteChain`, depending on how it is being used.
pub struct RewriteChain<A, B> {
    a: A,
    b: B,
}
impl<A, B> RewriteChain<A, B> {
    fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}
impl<A, B> Copy for RewriteChain<A, B>
where
    A: Copy,
    B: Copy,
{
}
impl<A, B> Clone for RewriteChain<A, B>
where
    A: Clone,
    B: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.a.clone(), self.b.clone())
    }
}
impl<A, B, E> RewritePass for RewriteChain<A, B>
where
    A: RewritePass<Error = E>,
    B: RewritePass<Error = E>,
{
    type Error = <B as RewritePass>::Error;

    fn run(
        &mut self,
        function: &mut miden_hir::Function,
        analyses: &mut FunctionAnalysis,
    ) -> Result<(), Self::Error> {
        self.a.run(function, analyses)?;
        self.b.run(function, analyses)
    }
}
impl<A, B, E> Pass for RewriteChain<A, B>
where
    A: RewritePass<Error = E>,
    B: RewritePass<Error = E>,
{
    type Input<'a> = (&'a mut miden_hir::Function, &'a mut FunctionAnalysis);
    type Output<'a> = (&'a mut miden_hir::Function, &'a mut FunctionAnalysis);
    type Error = <B as RewritePass>::Error;

    fn run<'a>(&mut self, input: Self::Input<'a>) -> Result<Self::Output<'a>, Self::Error> {
        let (function, analyses) = input;
        self.a.run(function, analyses)?;
        self.b.run(function, analyses)?;
        Ok((function, analyses))
    }
}
