//! This module provides traits and associated types for use in compiler pass pipelines.
//use miden_hir_pass::RewritePassRegistration;

// Register rewrite passes for modules
//inventory::collect!(RewritePassRegistration<crate::Module>);

// Register rewrite passes for functions
//inventory::collect!(RewritePassRegistration<crate::Function>);
/*
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
 */

mod analysis;
mod conversion;
mod rewrite;

use midenc_session::Session;

pub use self::{analysis::*, conversion::*, rewrite::*};

/// This trait provides descriptive information about a pass
///
/// This is primarily intended to assist in registering passes with the pass manager
pub trait PassInfo {
    /// The string which should be used as the name of this pass on the command line.
    ///
    /// For example, for the InlineBlocks pass, this is set to `inline-blocks`.
    ///
    /// You can add `#[derive(PassInfo)]` to a pass type, and this will be derived from
    /// the name of the type, and converted to kebab-case, e.g. `inline-blocks`.
    const FLAG: &'static str;
    /// A short, single-line description of what this pass does.
    ///
    /// You can add `#[derive(PassInfo)]` to a pass type, and this will be derived from
    /// the first line of the documentation attached to the type.
    const SUMMARY: &'static str;
    /// A rich, potentially multi-line description of this pass and its configuration.
    ///
    /// You can add `#[derive(PassInfo)]` to a pass type, and this will be derived from
    /// the documentation attached to the type.
    const DESCRIPTION: &'static str;
}

/// The [Pass] trait represents a fallible operation which takes an input of any type, and produces
/// an output of any type. This is intentionally abstract, and is intended as a building block for
/// compiler pipelines.
///
/// [Pass] is in fact so abstract, that it is automatically implemented for any Rust function whose
/// type is representable by `FnMut<I, O, E>(I) -> Result<O, E>`.
///
/// Implementations of [Pass] can be combined via [Pass::chain], which returns an instantiation of
/// the [Chain] type that itself implements [Pass]. This permits any number of passes to be
/// combined/chained together and passed around as a value.
pub trait Pass {
    type Input<'a>;
    type Output<'a>;
    type Error;

    /// Runs the pass on the given input
    ///
    /// Passes should return `Err` to signal that the pass has failed
    /// and compilation should be aborted
    fn run<'a>(
        &mut self,
        input: Self::Input<'a>,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> Result<Self::Output<'a>, Self::Error>;

    /// Chains two passes together to form a new, fused pass
    fn chain<P>(self, pass: P) -> Chain<Self, P>
    where
        Self: Sized,
        P: for<'a> Pass<Input<'a> = Self::Output<'a>, Error = Self::Error>,
    {
        Chain::new(self, pass)
    }
}
impl<P, T, U, E> Pass for &mut P
where
    P: for<'a> Pass<Input<'a> = T, Output<'a> = U, Error = E>,
{
    type Error = E;
    type Input<'a> = T;
    type Output<'a> = U;

    fn run<'a>(
        &mut self,
        input: Self::Input<'a>,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> Result<Self::Output<'a>, Self::Error> {
        (*self).run(input, analyses, session)
    }
}
impl<P, T, U, E> Pass for Box<P>
where
    P: ?Sized + for<'a> Pass<Input<'a> = T, Output<'a> = U, Error = E>,
{
    type Error = E;
    type Input<'a> = T;
    type Output<'a> = U;

    fn run<'a>(
        &mut self,
        input: Self::Input<'a>,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> Result<Self::Output<'a>, Self::Error> {
        (**self).run(input, analyses, session)
    }
}
impl<T, U, E> Pass for dyn FnMut(T, &mut AnalysisManager, &Session) -> Result<U, E> {
    type Error = E;
    type Input<'a> = T;
    type Output<'a> = U;

    #[inline]
    fn run<'a>(
        &mut self,
        input: Self::Input<'a>,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> Result<Self::Output<'a>, Self::Error> {
        self(input, analyses, session)
    }
}

/// [Chain] represents a pipeline of two or more passes whose inputs and outputs are linked
/// together into a "chain". If any pass in the pipeline raises an error, the rest of the
/// pipeline is skipped, and the error is returned.
///
/// This is not meant to be constructed or referenced directly, as the type signature gets out
/// of hand quickly when combining multiple passes. Instead, you should invoke `chain` on a
/// [Pass] implementation, and use it as a trait object. In some cases this may require boxing
/// the `Chain`, depending on how you are using it in your compiler.
pub struct Chain<A, B> {
    a: A,
    b: B,
}
impl<A, B> Chain<A, B> {
    fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}
impl<A, B> Copy for Chain<A, B>
where
    A: Copy,
    B: Copy,
{
}
impl<A, B> Clone for Chain<A, B>
where
    A: Clone,
    B: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.a.clone(), self.b.clone())
    }
}
impl<A, B, E> Pass for Chain<A, B>
where
    A: for<'a> Pass<Error = E>,
    B: for<'a> Pass<Input<'a> = <A as Pass>::Output<'a>, Error = E>,
{
    type Error = <B as Pass>::Error;
    type Input<'a> = <A as Pass>::Input<'a>;
    type Output<'a> = <B as Pass>::Output<'a>;

    fn run<'a>(
        &mut self,
        input: Self::Input<'a>,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> Result<Self::Output<'a>, Self::Error> {
        let output = self.a.run(input, analyses, session)?;
        self.b.run(output, analyses, session)
    }
}
impl<A, B> ConversionPass for Chain<A, B>
where
    A: ConversionPass,
    B: ConversionPass<From = <A as ConversionPass>::To>,
{
    type From = <A as ConversionPass>::From;
    type To = <B as ConversionPass>::To;

    fn convert<'a>(
        &mut self,
        entity: Self::From,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> ConversionResult<Self::To> {
        let output = self.a.convert(entity, analyses, session)?;
        self.b.convert(output, analyses, session)
    }
}
