macro_rules! register_function_rewrite {
    ($name:literal, $ty:ty) => {
        impl $ty {
            fn new(
                _options: std::sync::Arc<midenc_session::Options>,
                _diagnostics: std::sync::Arc<miden_diagnostics::DiagnosticsHandler>,
            ) -> Box<dyn crate::ModuleRewritePass> {
                Box::new(crate::ModuleRewritePassAdapter(Self))
            }
        }
        inventory::submit! {
            crate::ModuleRewritePassRegistration::new($name, <$ty>::new)
        }
    };
}

pub(crate) mod adt;
mod inline_blocks;
mod split_critical_edges;
mod treeify;

pub use self::inline_blocks::InlineBlocks;
pub use self::split_critical_edges::SplitCriticalEdges;
pub use self::treeify::Treeify;

use std::sync::Arc;

use miden_diagnostics::DiagnosticsHandler;
use miden_hir_analysis::FunctionAnalysis;
use midenc_session::Options;

pub struct ModuleRewritePassRegistration {
    name: &'static str,
    ctor: fn(Arc<Options>, Arc<DiagnosticsHandler>) -> Box<dyn ModuleRewritePass>,
}
impl ModuleRewritePassRegistration {
    pub const fn new(
        name: &'static str,
        ctor: fn(Arc<Options>, Arc<DiagnosticsHandler>) -> Box<dyn ModuleRewritePass>,
    ) -> Self {
        Self { name, ctor }
    }

    /// Get the name of the registered pass
    #[inline]
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Get an instance of the registered pass
    #[inline]
    pub fn get(
        &self,
        options: Arc<Options>,
        diagnostics: Arc<DiagnosticsHandler>,
    ) -> Box<dyn ModuleRewritePass> {
        (self.ctor)(options, diagnostics)
    }
}

inventory::collect!(ModuleRewritePassRegistration);

pub struct ModuleRewritePassAdapter<R>(R);
impl<R> RewritePass for ModuleRewritePassAdapter<R>
where
    R: RewritePass<Input = miden_hir::Function, Analysis = FunctionAnalysis>,
{
    type Input = miden_hir::Module;
    type Analysis = ();

    fn run(
        &mut self,
        module: &mut Self::Input,
        _analysis: &mut Self::Analysis,
    ) -> anyhow::Result<()> {
        // Removing a function via this cursor will move the cursor to
        // the next function in the module. Once the end of the module
        // is reached, the cursor will point to the null object, and
        // `remove` will return `None`.
        let mut cursor = module.cursor_mut();
        while let Some(mut function) = cursor.remove() {
            let mut analysis = FunctionAnalysis::new(&function);
            // Apply rewrites
            self.0.run(&mut function, &mut analysis)?;
            // Add the function back to the module
            //
            // We add it before the current position of the cursor
            // to ensure that we don't interfere with our traversal
            // of the module top to bottom
            cursor.insert_before(function);
        }

        Ok(())
    }
}

pub trait ModuleRewritePass: RewritePass<Input = miden_hir::Module, Analysis = ()> {}
impl<R: RewritePass<Input = miden_hir::Module, Analysis = ()>> ModuleRewritePass for R {}

pub trait FunctionRewritePass:
    RewritePass<Input = miden_hir::Function, Analysis = FunctionAnalysis>
{
}
impl<R: RewritePass<Input = miden_hir::Function, Analysis = FunctionAnalysis>> FunctionRewritePass
    for R
{
}

/// A [RewritePass] is a special kind of [Pass] which is designed to perform some
/// kind of rewrite transformation on a [miden_hir::Function].
///
/// Rewrites require one or more control flow analyses to have been computed, as
/// determined by the requirements of the pass itself. The [FunctionAnalysis]
/// structure is designed for this purpose, allowing one to request specific
/// analysis results, which will be computed on-demand if not yet available.
pub trait RewritePass {
    type Input;
    type Analysis;

    /// Runs the rewrite on `item` with `analyses`.
    ///
    /// Rewrites should return `Err` to signal that the pass has failed
    /// and compilation should be aborted
    fn run(&mut self, item: &mut Self::Input, analyses: &mut Self::Analysis) -> anyhow::Result<()>;

    /// Chains two rewrites together to form a new, fused rewrite
    fn chain<P>(self, pass: P) -> RewriteChain<Self, P>
    where
        Self: Sized,
        P: RewritePass<
            Input = <Self as RewritePass>::Input,
            Analysis = <Self as RewritePass>::Analysis,
        >,
    {
        RewriteChain::new(self, pass)
    }
}
impl<F: RewritePass> RewritePass for Box<F> {
    type Input = <F as RewritePass>::Input;
    type Analysis = <F as RewritePass>::Analysis;

    #[inline]
    fn run(&mut self, item: &mut Self::Input, analysis: &mut Self::Analysis) -> anyhow::Result<()> {
        (**self).run(item, analysis)
    }
}
impl<I, A> RewritePass for dyn Fn(&mut I, &mut A) -> anyhow::Result<()> {
    type Input = I;
    type Analysis = A;

    #[inline]
    fn run(&mut self, item: &mut Self::Input, analysis: &mut Self::Analysis) -> anyhow::Result<()> {
        (*self)(item, analysis)
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
impl<T, U, I, A> RewritePass for RewriteChain<T, U>
where
    T: RewritePass<Input = I, Analysis = A>,
    U: RewritePass<Input = I, Analysis = A>,
{
    type Input = I;
    type Analysis = A;

    fn run(&mut self, item: &mut Self::Input, analyses: &mut Self::Analysis) -> anyhow::Result<()> {
        self.a.run(item, analyses)?;
        self.b.run(item, analyses)
    }
}
