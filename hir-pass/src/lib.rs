//! This module provides traits and associated types for use in compiler pass pipelines.

/// The [Pass] trait represents a fallible operation which takes an input of any type, and produces an
/// output of any type. This is intentionally abstract, and is intended as a building block for compiler
/// pipelines.
///
/// [Pass] is in fact so abstract, that it is automatically implemented for any Rust function whose type
/// is representable by `FnMut<I, O, E>(I) -> Result<O, E>`.
///
/// Implementations of [Pass] can be combined via [Pass::chain], which returns an instantiation of the
/// [Chain] type that itself implements [Pass]. This permits any number of passes to be combined/chained
/// together and passed around as a value.
pub trait Pass {
    type Input<'a>;
    type Output<'a>;
    type Error;

    /// Runs the pass on the given input
    ///
    /// Passes should return `Err` to signal that the pass has failed
    /// and compilation should be aborted
    fn run<'a>(&mut self, input: Self::Input<'a>) -> Result<Self::Output<'a>, Self::Error>;

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
    type Input<'a> = T;
    type Output<'a> = U;
    type Error = E;

    fn run<'a>(&mut self, input: Self::Input<'a>) -> Result<Self::Output<'a>, Self::Error> {
        (*self).run(input)
    }
}
impl<P, T, U, E> Pass for Box<P>
where
    P: ?Sized + for<'a> Pass<Input<'a> = T, Output<'a> = U, Error = E>,
{
    type Input<'a> = T;
    type Output<'a> = U;
    type Error = E;

    fn run<'a>(&mut self, input: Self::Input<'a>) -> Result<Self::Output<'a>, Self::Error> {
        (**self).run(input)
    }
}
impl<T, U, E> Pass for dyn FnMut(T) -> Result<U, E> {
    type Input<'a> = T;
    type Output<'a> = U;
    type Error = E;

    #[inline]
    fn run<'a>(&mut self, input: Self::Input<'a>) -> Result<Self::Output<'a>, Self::Error> {
        self(input)
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
    type Input<'a> = <A as Pass>::Input<'a>;
    type Output<'a> = <B as Pass>::Output<'a>;
    type Error = <B as Pass>::Error;

    fn run<'a>(&mut self, input: Self::Input<'a>) -> Result<Self::Output<'a>, Self::Error> {
        let u = self.a.run(input)?;
        self.b.run(u)
    }
}
