use std::any::Any;
use std::sync::Arc;

use midenc_session::Session;

use super::{AnalysisError, AnalysisManager, Chain, PassInfo};

/// A [Dialect] refers to a specific intermediate representation in the compiler,
/// typically suited for some specific purpose. These dialects are named, and have
/// one or more passes which are applied to operations in that dialect.
pub trait Dialect {
    /// The name used to refer to this dialect, e.g. "hir"
    fn name(&self) -> &'static str;

    /// The file type extension used by this dialect when stored on disk
    fn extension(&self) -> &'static str;
}

/// A trait applied to all operations of a [Dialect] providing a means by which
/// to express passes which abstract over operations.
pub trait Op: Sized + Any {
    fn dialect(&self) -> Arc<dyn Dialect>;
    /// Returns true if this operation is of concrete type `T`
    fn is<T>(&self) -> bool
    where
        T: Op,
    {
        (self as &dyn Any).is::<T>()
    }
    /// Returns true if this operation is of concrete type `T`
    fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Op,
    {
        (self as &dyn Any).downcast_ref::<T>()
    }
    /// Returns true if this operation is of concrete type `T`
    fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Op,
    {
        (self as &mut dyn Any).downcast_mut::<T>()
    }
}

/// This error is produced when a [ConversionPass] fails
#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    /// Conversion failed due to an analysis error
    #[error(transparent)]
    Analysis(#[from] AnalysisError),
    /// An unexpected error occurred during conversion
    #[error(transparent)]
    Failed(#[from] anyhow::Error),
}

/// A convenient type alias for `Result<T, ConversionError>`
pub type ConversionResult<T> = Result<T, ConversionError>;

/// This is a marker trait for [ConversionPass] impls which also implement [PassInfo]
///
/// It is automatically implemented for you.
pub trait ConversionPassInfo: PassInfo + ConversionPass {}
impl<P> ConversionPassInfo for P where P: PassInfo + ConversionPass {}

/// A [ConversionPass] is a pass which applies a change in representation to some compiler entity.
///
/// Specifically, this is used to convert between intermediate representations/dialects in the compiler.
///
/// For example, a conversion pass would be used to lower a `miden_hir::parser::ast::Module`
/// to a `miden_hir::Module`. Each conversion between dialects like this can be thought of
/// as delineating compilation phases (e.g. parsing, semantic analysis, elaboration, optimization,
/// etc.).
pub trait ConversionPass {
    type From<'a>;
    type To;

    /// Apply this conversion to `entity`
    fn convert<'a>(
        &mut self,
        entity: Self::From<'a>,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> ConversionResult<Self::To>;

    /// Chains two conversions together to form a new, fused conversion
    fn chain<P>(self, next: P) -> Chain<Self, P>
    where
        Self: Sized,
        P: for<'a> ConversionPass<From<'a> = Self::To>,
    {
        Chain::new(self, next)
    }
}
impl<P, T, U> ConversionPass for Box<P>
where
    P: ?Sized + for<'a> ConversionPass<From<'a> = T, To = U>,
{
    type From<'a> = T;
    type To = U;

    fn convert<'a>(
        &mut self,
        entity: Self::From<'a>,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> ConversionResult<Self::To> {
        (**self).convert(entity, analyses, session)
    }
}
