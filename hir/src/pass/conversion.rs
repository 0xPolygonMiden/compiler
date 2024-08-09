use midenc_session::Session;

use super::{AnalysisManager, Chain, PassInfo};
use crate::diagnostics::Report;

/// A convenient type alias for `Result<T, ConversionError>`
pub type ConversionResult<T> = Result<T, Report>;

/// This is a marker trait for [ConversionPass] impls which also implement [PassInfo]
///
/// It is automatically implemented for you.
pub trait ConversionPassInfo: PassInfo + ConversionPass {}
impl<P> ConversionPassInfo for P where P: PassInfo + ConversionPass {}

/// A [ConversionPass] is a pass which applies a change in representation to some compiler entity.
///
/// Specifically, this is used to convert between intermediate representations/dialects in the
/// compiler.
///
/// For example, a conversion pass would be used to lower a `midenc_hir::parser::ast::Module`
/// to a `midenc_hir::Module`. Each conversion between dialects like this can be thought of
/// as delineating compilation phases (e.g. parsing, semantic analysis, elaboration, optimization,
/// etc.).
pub trait ConversionPass {
    type From;
    type To;

    /// Apply this conversion to `entity`
    fn convert(
        &mut self,
        entity: Self::From,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> ConversionResult<Self::To>;

    /// Chains two conversions together to form a new, fused conversion
    fn chain<P>(self, next: P) -> Chain<Self, P>
    where
        Self: Sized,
        P: ConversionPass<From = Self::To>,
    {
        Chain::new(self, next)
    }
}
impl<P, T, U> ConversionPass for Box<P>
where
    P: ?Sized + ConversionPass<From = T, To = U>,
{
    type From = T;
    type To = U;

    fn convert(
        &mut self,
        entity: Self::From,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> ConversionResult<Self::To> {
        (**self).convert(entity, analyses, session)
    }
}

type ConversionPassCtor<T, U> = fn() -> Box<dyn ConversionPass<From = T, To = U>>;

#[doc(hidden)]
pub struct ConversionPassRegistration<T, U> {
    pub name: &'static str,
    pub summary: &'static str,
    pub description: &'static str,
    ctor: ConversionPassCtor<T, U>,
}
impl<T, U> ConversionPassRegistration<T, U> {
    pub const fn new<P>() -> Self
    where
        P: ConversionPass<From = T, To = U> + PassInfo + Default + 'static,
    {
        Self {
            name: <P as PassInfo>::FLAG,
            summary: <P as PassInfo>::SUMMARY,
            description: <P as PassInfo>::DESCRIPTION,
            ctor: dyn_conversion_pass_ctor::<P, T, U>,
        }
    }

    /// Get the name of the registered pass
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Get a summary of the registered pass
    #[inline]
    pub const fn summary(&self) -> &'static str {
        self.summary
    }

    /// Get a rich description of the registered pass
    #[inline]
    pub const fn description(&self) -> &'static str {
        self.description
    }

    /// Get an instance of the registered pass
    #[inline]
    pub fn get(&self) -> Box<dyn ConversionPass<From = T, To = U>> {
        (self.ctor)()
    }
}

fn dyn_conversion_pass_ctor<P, T, U>() -> Box<dyn ConversionPass<From = T, To = U>>
where
    P: Default + ConversionPass<From = T, To = U> + 'static,
{
    Box::<P>::default()
}
