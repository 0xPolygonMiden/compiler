macro_rules! bug {
    ($diagnostics:ident, $msg:literal) => {{
        diagnostic!($diagnostics, Severity::Bug, $msg);
    }};

    ($diagnostics:ident, $msg:literal, $span:expr, $label:expr) => {{
        diagnostic!($diagnostics, Severity::Bug, $msg, $span, $label);
    }};

    ($diagnostics:ident, $msg:literal, $span:expr, $label:expr, $note:expr) => {{
        diagnostic!($diagnostics, Severity::Bug, $msg, $span, $label, $note);
    }};

    ($diagnostics:ident, $msg:literal, $span:expr, $label:expr, $span2:expr, $label2:expr) => {{
        diagnostic!(
            $diagnostics,
            Severity::Bug,
            $msg,
            $span,
            $label,
            $span2,
            $label2
        );
    }};
}

macro_rules! error {
    ($diagnostics:ident, $msg:literal) => {{
        diagnostic!($diagnostics, Severity::Error, $msg);
    }};

    ($diagnostics:ident, $msg:literal, $span:expr, $label:expr) => {{
        diagnostic!($diagnostics, Severity::Error, $msg, $span, $label);
    }};

    ($diagnostics:ident, $msg:literal, $span:expr, $label:expr, $note:expr) => {{
        diagnostic!($diagnostics, Severity::Error, $msg, $span, $label, $note);
    }};

    ($diagnostics:ident, $msg:literal, $span:expr, $label:expr, $span2:expr, $label2:expr) => {{
        diagnostic!(
            $diagnostics,
            Severity::Error,
            $msg,
            $span,
            $label,
            $span2,
            $label2
        );
    }};
}

macro_rules! invalid_instruction {
    ($diagnostics:ident, $inst:expr, $span:expr, $label:expr) => {{
        let span = $span;
        let reason = format!($label);
        bug!($diagnostics, "invalid instruction", span, reason.as_str());
        return Err(crate::validation::ValidationError::InvalidInstruction {
            span,
            inst: $inst,
            reason,
        });
    }};

    ($diagnostics:ident, $inst:expr, $span:expr, $label:expr, $note:expr) => {{
        let span = $span;
        let reason = format!($label);
        bug!(
            $diagnostics,
            "invalid instruction",
            span,
            reason.as_str(),
            $note
        );
        return Err(crate::validation::ValidationError::InvalidInstruction {
            span,
            inst: $inst,
            reason,
        });
    }};
}

macro_rules! invalid_block {
    ($diagnostics:ident, $block:expr, $span:expr, $label:expr) => {{
        let reason = format!($label);
        bug!($diagnostics, "invalid block", $span, reason.as_str());
        return Err(crate::validation::ValidationError::InvalidBlock {
            block: $block,
            reason,
        });
    }};

    ($diagnostics:ident, $block:expr, $span:expr, $label:expr, $note:expr) => {{
        let reason = format!($label);
        bug!($diagnostics, "invalid block", $span, reason.as_str(), $note);
        return Err(crate::validation::ValidationError::InvalidBlock {
            block: $block,
            reason,
        });
    }};
}

macro_rules! invalid_module {
    ($diagnostics:ident, $module:expr, $label:expr) => {{
        invalid_module!($diagnostics, $module, $module.span(), $label);
    }};

    ($diagnostics:ident, $module:expr, $span:expr, $label:expr) => {{
        let span = $span;
        let reason = format!($label);
        error!($diagnostics, "invalid module", span, reason.as_str());
        return Err(crate::validation::ValidationError::InvalidModule {
            module: $module,
            reason,
        });
    }};

    ($diagnostics:ident, $module:expr, $span:expr, $label:expr, $note:expr) => {{
        let span = $span;
        let reason = format!($label);
        error!($diagnostics, "invalid module", span, reason.as_str(), $note);
        return Err(crate::validation::ValidationError::InvalidModule {
            module: $module,
            reason,
        });
    }};
}

macro_rules! invalid_function {
    ($diagnostics:ident, $function:expr, $label:expr) => {{
        invalid_function!($diagnostics, $function, $function.span(), $label);
    }};

    ($diagnostics:ident, $function:expr, $span:expr, $label:expr) => {{
        let span = $span;
        let reason = format!($label);
        error!($diagnostics, "invalid function", span, reason.as_str());
        return Err(crate::validation::ValidationError::InvalidFunction {
            function: $function,
            reason,
        });
    }};

    ($diagnostics:ident, $function:expr, $span:expr, $label:expr, $note:expr) => {{
        let span = $span;
        let reason = format!($label);
        error!(
            $diagnostics,
            "invalid function",
            span,
            reason.as_str(),
            $note
        );
        return Err(crate::validation::ValidationError::InvalidFunction {
            function: $function,
            reason,
        });
    }};

    ($diagnostics:ident, $function:expr, $span:expr, $label:expr, $span2:expr, $label2:expr) => {{
        let span = $span;
        let reason = format!($label);
        error!($diagnostics, "invalid function", span, reason.as_str());
        $diagnostics
            .diagnostic(miden_diagnostics::Severity::Error)
            .with_message("invalid function")
            .with_primary_label(span, reason.as_str())
            .with_secondary_label($span2, $label2)
            .emit();
        return Err(crate::validation::ValidationError::InvalidFunction {
            function: $function,
            reason,
        });
    }};
}

macro_rules! invalid_global {
    ($diagnostics:ident, $name:expr, $label:expr) => {{
        invalid_global!($diagnostics, $name, $name.span(), $label);
    }};

    ($diagnostics:ident, $name:expr, $span:expr, $label:expr) => {{
        let span = $span;
        let reason = format!($label);
        error!(
            $diagnostics,
            "invalid global variable",
            span,
            reason.as_str()
        );
        return Err(crate::validation::ValidationError::InvalidGlobalVariable {
            name: $name,
            reason,
        });
    }};
}

mod block;
mod function;
mod naming;
mod typecheck;

pub use self::typecheck::TypeError;

use miden_diagnostics::{DiagnosticsHandler, SourceSpan};
use miden_hir::*;
use miden_hir_pass::Pass;

use self::block::{BlockValidator, DefsDominateUses};
use self::function::FunctionValidator;
use self::naming::NamingConventions;
use self::typecheck::TypeCheck;

/// This error is produced by validation rules run against the IR
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    /// A validation rule indicates a module is invalid
    #[error("invalid module '{module}': {reason}")]
    InvalidModule { module: Ident, reason: String },
    /// A validation rule indicates a global variable is invalid
    #[error("invalid global variable '{name}': {reason}")]
    InvalidGlobalVariable { name: Ident, reason: String },
    /// A validation rule indicates a function is invalid
    #[error("invalid function '{function}': {reason}")]
    InvalidFunction {
        function: FunctionIdent,
        reason: String,
    },
    /// A validation rule indicates a block is invalid
    #[error("invalid block '{block}': {reason}")]
    InvalidBlock { block: Block, reason: String },
    /// A validation rule indicates an instruction is invalid
    #[error("invalid instruction '{inst}': {reason}")]
    InvalidInstruction {
        span: SourceSpan,
        inst: Inst,
        reason: String,
    },
    /// A type error was found
    #[error("type error: {0}")]
    TypeError(#[from] TypeError),
    /// An unknown validation error occurred
    #[error(transparent)]
    Misc(#[from] anyhow::Error),
}
#[cfg(test)]
impl PartialEq for ValidationError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::InvalidModule {
                    module: am,
                    reason: ar,
                },
                Self::InvalidModule {
                    module: bm,
                    reason: br,
                },
            ) => am == bm && ar == br,
            (
                Self::InvalidGlobalVariable {
                    name: an,
                    reason: ar,
                },
                Self::InvalidGlobalVariable {
                    name: bn,
                    reason: br,
                },
            ) => an == bn && ar == br,
            (
                Self::InvalidFunction {
                    function: af,
                    reason: ar,
                },
                Self::InvalidFunction {
                    function: bf,
                    reason: br,
                },
            ) => af == bf && ar == br,
            (
                Self::InvalidBlock {
                    block: ab,
                    reason: ar,
                },
                Self::InvalidBlock {
                    block: bb,
                    reason: br,
                },
            ) => ab == bb && ar == br,
            (
                Self::InvalidInstruction {
                    inst: ai,
                    reason: ar,
                    ..
                },
                Self::InvalidInstruction {
                    inst: bi,
                    reason: br,
                    ..
                },
            ) => ai == bi && ar == br,
            (Self::TypeError(a), Self::TypeError(b)) => a == b,
            (Self::Misc(a), Self::Misc(b)) => a.to_string() == b.to_string(),
            (_, _) => false,
        }
    }
}

/// A [Rule] validates some specific type of behavior on an item of type `T`
pub trait Rule<T> {
    /// Validate `item`, using `diagnostics` to emit relevant diagnostics.
    fn validate(
        &mut self,
        item: &T,
        diagnostics: &DiagnosticsHandler,
    ) -> Result<(), ValidationError>;

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
    fn validate(
        &mut self,
        item: &T,
        diagnostics: &DiagnosticsHandler,
    ) -> Result<(), ValidationError> {
        (*self).validate(item, diagnostics)
    }
}
impl<R, T> Rule<T> for Box<R>
where
    R: Rule<T>,
{
    fn validate(
        &mut self,
        item: &T,
        diagnostics: &DiagnosticsHandler,
    ) -> Result<(), ValidationError> {
        (**self).validate(item, diagnostics)
    }
}
impl<T> Rule<T> for dyn FnMut(&T, &DiagnosticsHandler) -> Result<(), ValidationError> {
    #[inline]
    fn validate(
        &mut self,
        item: &T,
        diagnostics: &DiagnosticsHandler,
    ) -> Result<(), ValidationError> {
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
    fn validate(
        &mut self,
        item: &T,
        diagnostics: &DiagnosticsHandler,
    ) -> Result<(), ValidationError> {
        self.a
            .validate(item, diagnostics)
            .and_then(|_| self.b.validate(item, diagnostics))
    }
}

/// The [ModuleValidator] can be used to validate and emit diagnostics for a [Module].
///
/// It implements [miden_hir_pass::Pass], so can be used as part of a pass pipeline.
///
/// This validates all rules which apply to items at/within module scope.
pub struct ModuleValidator<'a> {
    diagnostics: &'a DiagnosticsHandler,
}
impl<'a> ModuleValidator<'a> {
    pub fn new(diagnostics: &'a DiagnosticsHandler) -> Self {
        Self { diagnostics }
    }

    pub fn validate(&mut self, module: &Module) -> Result<(), ValidationError> {
        self.run(module)
    }
}
impl<'p> Pass for ModuleValidator<'p> {
    type Input<'a> = &'a Module;
    type Output<'a> = ();
    type Error = ValidationError;

    fn run<'a>(&mut self, input: Self::Input<'a>) -> Result<Self::Output<'a>, Self::Error> {
        // Apply module-scoped rules
        let mut rules = NamingConventions;
        rules.validate(input, self.diagnostics)?;

        // Apply global-scoped rules
        let mut rules = NamingConventions;
        for global in input.globals() {
            rules.validate(global, self.diagnostics)?;
        }

        // Apply function-scoped rules
        let mut rules = FunctionValidator::new(input.is_kernel());
        for function in input.functions() {
            rules.validate(function, self.diagnostics)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use miden_hir::{
        testing::{self, TestContext},
        ModuleBuilder,
    };

    use super::*;

    #[test]
    fn module_validator_test() {
        let context = TestContext::default();

        // Define the 'test' module
        let mut builder = ModuleBuilder::new("test");
        builder.with_span(context.current_span());
        testing::sum_matrix(&mut builder, &context);
        let module = builder.build();

        let mut validator = ModuleValidator::new(&context.diagnostics);
        assert_eq!(validator.validate(&module), Ok(()));
    }
}
