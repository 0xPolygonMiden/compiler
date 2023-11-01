use midenc_session::Session;

use super::{AnalysisError, AnalysisKey, AnalysisManager, PassInfo};

/// This error is produced when an error occurs when applying a rewrite rule
#[derive(Debug, thiserror::Error)]
pub enum RewriteError {
    /// The rewrite failed due to an analysis error
    #[error(transparent)]
    Analysis(#[from] AnalysisError),
    /// An unexpected error occurred during this rewrite
    #[error(transparent)]
    Failed(#[from] anyhow::Error),
}

/// A convenient type alias for `Result<(), RewriteError>`
pub type RewriteResult = Result<(), RewriteError>;

/// This is a marker trait for [RewritePass] impls which also implement [PassInfo]
///
/// It is automatically implemented for you.
pub trait RewritePassInfo: PassInfo + RewritePass {}
impl<P> RewritePassInfo for P where P: PassInfo + RewritePass {}

/// A [RewritePass] is a pass which transforms/rewrites an entity without converting it to a
/// new representation. For conversions, see [crate::ConversionPass].
///
/// For example, a rewrite rule which applies a mangling scheme to function names, does not
/// change the representation of a function, it simply changes things about the existing
/// representation (e.g. the name in this example).
///
/// A rewrite is given access to the current [AnalysisManager], which can be used to obtain
/// the results of some [Analysis] needed to perform the rewrite, as well as indicate to the
/// [AnalysisManager] which analyses are preserved by the rewrite, if any.
///
/// Additionally, the current [midenc_session::Session] is provided, which can be used as a
/// source of configuration for the rewrite, if needed.
pub trait RewritePass {
    /// The entity type to which this rewrite applies
    type Entity: AnalysisKey;

    /// Returns true if this rewrite should be applied to `entity`
    fn should_apply(&self, _entity: &Self::Entity, _session: &Session) -> bool {
        true
    }

    /// Apply this rewrite to `entity`
    fn apply(
        &mut self,
        entity: &mut Self::Entity,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> RewriteResult;

    /// Apply this rewrite, then `next` as a pipeline of rewrites
    fn chain<R>(self, next: R) -> RewriteSet<Self::Entity>
    where
        Self: Sized + 'static,
        R: RewritePass<Entity = Self::Entity> + 'static,
    {
        RewriteSet::pair(self, next)
    }
}

impl<P, T> RewritePass for Box<P>
where
    T: AnalysisKey,
    P: RewritePass<Entity = T>,
{
    type Entity = T;

    fn should_apply(&self, entity: &Self::Entity, session: &Session) -> bool {
        (**self).should_apply(entity, session)
    }

    fn apply(
        &mut self,
        entity: &mut Self::Entity,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> RewriteResult {
        (**self).apply(entity, analyses, session)
    }

    fn chain<R>(self, next: R) -> RewriteSet<Self::Entity>
    where
        Self: Sized + 'static,
        R: RewritePass<Entity = Self::Entity> + 'static,
    {
        let mut rewrites = RewriteSet::from(self);
        rewrites.push(next);
        rewrites
    }
}
impl<T> RewritePass for dyn FnMut(&mut T, &mut AnalysisManager, &Session) -> RewriteResult
where
    T: AnalysisKey,
{
    type Entity = T;

    #[inline]
    fn apply(
        &mut self,
        entity: &mut Self::Entity,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> RewriteResult {
        self(entity, analyses, session)
    }
}

/// A [RewriteSet] is used to compose two or more [RewritePass] impls for the same entity type,
/// to be applied as a single, fused [RewritePass].
pub struct RewriteSet<T> {
    rewrites: Vec<Box<dyn RewritePass<Entity = T>>>,
}
impl<T> RewriteSet<T>
where
    T: AnalysisKey,
{
    pub fn pair<A, B>(a: A, b: B) -> Self
    where
        A: RewritePass<Entity = T> + 'static,
        B: RewritePass<Entity = T> + 'static,
    {
        Self {
            rewrites: vec![Box::new(a), Box::new(b)],
        }
    }

    pub fn push<R>(&mut self, rewrite: R)
    where
        R: RewritePass<Entity = T> + 'static,
    {
        self.rewrites.push(Box::new(rewrite));
    }

    pub fn append(&mut self, other: &mut Self) {
        self.rewrites.append(&mut other.rewrites);
    }
}
impl<T> From<Box<dyn RewritePass<Entity = T>>> for RewriteSet<T>
where
    T: AnalysisKey,
{
    fn from(rewrite: Box<dyn RewritePass<Entity = T>>) -> Self {
        Self {
            rewrites: vec![rewrite],
        }
    }
}
impl<T, R: RewritePass<Entity = T> + 'static> From<Box<R>> for RewriteSet<T>
where
    T: AnalysisKey,
{
    fn from(rewrite: Box<R>) -> Self {
        Self {
            rewrites: vec![rewrite],
        }
    }
}
impl<T> RewritePass for RewriteSet<T>
where
    T: AnalysisKey,
{
    type Entity = T;

    fn apply(
        &mut self,
        entity: &mut Self::Entity,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> RewriteResult {
        for pass in self.rewrites.iter_mut() {
            // Skip the rewrite if it shouldn't be applied
            if !pass.should_apply(entity, session) {
                continue;
            }

            // Apply the rewrite
            pass.apply(entity, analyses, session)?;
            // Invalidate all analyses that were not marked preserved by `pass`
            analyses.invalidate::<T>(&entity.key());
        }

        Ok(())
    }

    fn chain<R>(mut self, next: R) -> RewriteSet<Self::Entity>
    where
        Self: Sized + 'static,
        R: RewritePass<Entity = Self::Entity> + 'static,
    {
        self.push(next);
        self
    }
}

#[doc(hidden)]
pub struct RewritePassRegistration<T> {
    pub name: &'static str,
    pub help: &'static str,
    ctor: fn() -> Box<dyn RewritePass<Entity = T>>,
}
impl<T> RewritePassRegistration<T> {
    pub const fn new(
        name: &'static str,
        help: &'static str,
        ctor: fn() -> Box<dyn RewritePass<Entity = T>>,
    ) -> Self {
        Self { name, help, ctor }
    }

    /// Get the name of the registered pass
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Get the help documentation associated with the registered pass
    #[inline]
    pub const fn help(&self) -> &'static str {
        self.help
    }

    /// Get an instance of the registered pass
    #[inline]
    pub fn get(&self) -> Box<dyn RewritePass<Entity = T>> {
        (self.ctor)()
    }
}
