mod block;
mod function;
mod naming;
mod typecheck;

use midenc_hir::{
    diagnostics::{DiagnosticsHandler, Report},
    pass::{Analysis, AnalysisManager, AnalysisResult},
    *,
};
use midenc_session::Session;

use self::{
    block::{BlockValidator, DefsDominateUses},
    function::FunctionValidator,
    naming::NamingConventions,
    typecheck::TypeCheck,
};

inventory::submit! {
    midenc_session::CompileFlag::new("validate")
        .long("no-validate")
        .action(midenc_session::FlagAction::SetFalse)
        .help("If present, disables validation of the IR prior to codegen")
        .help_heading("Analysis")
}

/// A [Rule] validates some specific type of behavior on an item of type `T`
pub trait Rule<T> {
    /// Validate `item`, using `diagnostics` to emit relevant diagnostics.
    fn validate(&mut self, item: &T, diagnostics: &DiagnosticsHandler) -> Result<(), Report>;

    /// Combine two rules into one rule
    fn chain<R>(self, rule: R) -> RuleSet<Self, R>
    where
        Self: Sized,
        R: Rule<T>,
    {
        RuleSet::new(self, rule)
    }
}
impl<R, T> Rule<T> for &mut R
where
    R: Rule<T>,
{
    fn validate(&mut self, item: &T, diagnostics: &DiagnosticsHandler) -> Result<(), Report> {
        (*self).validate(item, diagnostics)
    }
}
impl<R, T> Rule<T> for Box<R>
where
    R: Rule<T>,
{
    fn validate(&mut self, item: &T, diagnostics: &DiagnosticsHandler) -> Result<(), Report> {
        (**self).validate(item, diagnostics)
    }
}
impl<T> Rule<T> for dyn FnMut(&T, &DiagnosticsHandler) -> Result<(), Report> {
    #[inline]
    fn validate(&mut self, item: &T, diagnostics: &DiagnosticsHandler) -> Result<(), Report> {
        self(item, diagnostics)
    }
}

/// A [RuleSet] is a combination of multiple rules into a single [Rule]
pub struct RuleSet<A, B> {
    a: A,
    b: B,
}
impl<A, B> RuleSet<A, B> {
    fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}
impl<A, B> Copy for RuleSet<A, B>
where
    A: Copy,
    B: Copy,
{
}
impl<A, B> Clone for RuleSet<A, B>
where
    A: Clone,
    B: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.a.clone(), self.b.clone())
    }
}
impl<A, B, T> Rule<T> for RuleSet<A, B>
where
    A: Rule<T>,
    B: Rule<T>,
{
    fn validate(&mut self, item: &T, diagnostics: &DiagnosticsHandler) -> Result<(), Report> {
        self.a
            .validate(item, diagnostics)
            .and_then(|_| self.b.validate(item, diagnostics))
    }
}

/// The [ModuleValidationAnalysis] can be used to validate and emit diagnostics for a [Module].
///
/// This validates all rules which apply to items at/within module scope.
#[derive(PassInfo)]
pub struct ModuleValidationAnalysis(Result<(), Report>);
impl Analysis for ModuleValidationAnalysis {
    type Entity = Module;

    fn analyze(
        module: &Self::Entity,
        _analyses: &mut AnalysisManager,
        session: &Session,
    ) -> AnalysisResult<Self> {
        if session.get_flag("validate") {
            return Ok(Self(Ok(())));
        }

        Ok(Self(Self::validate(module, session)))
    }
}
impl ModuleValidationAnalysis {
    fn validate(module: &Module, session: &Session) -> Result<(), Report> {
        // Apply module-scoped rules
        let mut rules = NamingConventions;
        rules.validate(module, &session.diagnostics)?;

        // Apply global-scoped rules
        let mut rules = NamingConventions;
        for global in module.globals().iter() {
            rules.validate(global, &session.diagnostics)?;
        }

        // Apply function-scoped rules
        let mut rules = FunctionValidator::new(module.is_kernel());
        for function in module.functions() {
            rules.validate(function, &session.diagnostics)?;
        }

        Ok(())
    }
}
impl From<ModuleValidationAnalysis> for Result<(), Report> {
    fn from(analysis: ModuleValidationAnalysis) -> Self {
        analysis.0
    }
}

#[cfg(test)]
mod tests {
    use midenc_hir::testing::TestContext;

    use super::*;

    #[test]
    fn module_validator_test() {
        let context = TestContext::default();

        // Define the 'test' module
        let mut builder = ModuleBuilder::new("test");
        builder.with_span(context.current_span());
        testing::sum_matrix(&mut builder, &context);
        let module = builder.build();

        let analysis = ModuleValidationAnalysis::validate(&module, &context.session);
        analysis.expect("module was expected to be valid")
    }
}
