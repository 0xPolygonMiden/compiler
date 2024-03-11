use miden_diagnostics::{DiagnosticsHandler, Severity, Spanned};
use miden_hir::*;

use super::{Rule, ValidationError};

/// This validation rule ensures that all identifiers adhere to the rules of their respective items.
pub struct NamingConventions;
impl Rule<Module> for NamingConventions {
    fn validate(
        &mut self,
        module: &Module,
        diagnostics: &DiagnosticsHandler,
    ) -> Result<(), ValidationError> {
        // Make sure all functions in this module have the same module name in their id
        for function in module.functions() {
            let id = function.id;
            if id.module != module.name {
                let expected_name = FunctionIdent {
                    module: module.name,
                    function: id.function,
                };
                invalid_function!(
                    diagnostics,
                    function.id,
                    function.id.span(),
                    "the fully-qualified name of this function is '{id}'",
                    module.name.span(),
                    format!("but we expected '{expected_name}' because it belongs to this module")
                );
            }
        }

        // 1. Must not be empty
        let name = module.name.as_str();
        if name.is_empty() {
            invalid_module!(diagnostics, module.name, "module name cannot be empty");
        }

        // 2. Must begin with a lowercase ASCII alphabetic character
        if !name.starts_with(is_lower_ascii_alphabetic) {
            invalid_module!(
                diagnostics,
                module.name,
                "module name must start with a lowercase, ascii-alphabetic character"
            );
        }

        // 3. May otherwise consist of any number of characters of the following classes:
        //   * `A-Z`
        //   * `a-z`
        //   * `0-9`
        //   * `_-+$@`
        // 4. May only contain `:` when used via the namespacing operator, e.g. `std::math`
        let mut char_indices = name.char_indices().peekable();
        let mut is_namespaced = false;
        while let Some((offset, c)) = char_indices.next() {
            match c {
                c if c.is_ascii_alphanumeric() => continue,
                '_' | '-' | '+' | '$' | '@' => continue,
                ':' => match char_indices.peek() {
                    Some((_, ':')) => {
                        char_indices.next();
                        is_namespaced = true;
                        continue;
                    }
                    _ => {
                        let pos = module.name.span().start() + offset;
                        let span = SourceSpan::new(pos, pos);
                        invalid_module!(
                            diagnostics,
                            module.name,
                            span,
                            "module name contains invalid character ':'",
                            "Did you mean to use the namespacing operator '::'?"
                        );
                    }
                },
                c if c.is_whitespace() => {
                    invalid_module!(
                        diagnostics,
                        module.name,
                        "module names may not contain whitespace"
                    );
                }
                c => {
                    let pos = module.name.span().start() + offset;
                    let span = SourceSpan::new(pos, pos);
                    invalid_module!(
                        diagnostics,
                        module.name,
                        span,
                        "{c} is not valid in module names"
                    );
                }
            }
        }

        // 5. The namespacing operator may only appear between two valid module identifiers
        // 6. Namespaced module names must adhere to the above rules in each submodule identifier
        if is_namespaced {
            let mut offset = 0;
            for component in name.split("::") {
                let len = component.as_bytes().len();
                let start = module.name.span().start() + offset;
                let span = SourceSpan::new(start, start + len);
                if component.is_empty() {
                    invalid_module!(
                        diagnostics,
                        module.name,
                        span,
                        "submodule names cannot be empty"
                    );
                }

                if !name.starts_with(is_lower_ascii_alphabetic) {
                    invalid_module!(
                        diagnostics,
                        module.name,
                        span,
                        "submodule name must start with a lowercase, ascii-alphabetic character"
                    );
                }

                offset += len + 2;
            }
        }

        Ok(())
    }
}
impl Rule<Function> for NamingConventions {
    fn validate(
        &mut self,
        function: &Function,
        diagnostics: &DiagnosticsHandler,
    ) -> Result<(), ValidationError> {
        let name = function.id.function.as_str();
        let span = function.id.function.span();

        // 1. Must not be empty
        if name.is_empty() {
            invalid_function!(diagnostics, function.id, "function names cannot be empty");
        }

        // 2. Must start with an ASCII-alphabetic character, underscore, `$` or `@`
        fn name_starts_with(c: char) -> bool {
            c.is_ascii_alphabetic() || matches!(c, '_' | '$' | '@')
        }

        // 3. Otherwise, no restrictions, but may not contain whitespace
        if let Err((offset, c)) = is_valid_identifier(name, name_starts_with, char::is_whitespace) {
            if c.is_whitespace() {
                let pos = span.start() + offset;
                let span = SourceSpan::new(pos, pos);
                invalid_function!(
                    diagnostics,
                    function.id,
                    span,
                    "function names may not contain whitespace"
                );
            } else {
                debug_assert_eq!(offset, 0);
                let span = SourceSpan::new(span.start(), span.start());
                invalid_function!(
                    diagnostics,
                    function.id,
                    span,
                    "function names must start with an ascii-alphabetic character, '_', '$', or \
                     '@'"
                );
            }
        }

        Ok(())
    }
}
impl Rule<GlobalVariableData> for NamingConventions {
    fn validate(
        &mut self,
        global: &GlobalVariableData,
        diagnostics: &DiagnosticsHandler,
    ) -> Result<(), ValidationError> {
        let span = global.name.span();
        let name = global.name.as_str();

        // 1. Must not be empty
        if name.is_empty() {
            invalid_global!(diagnostics, global.name, "global variable names cannot be empty");
        }

        // 2. Must start with an ASCII-alphabetic character, underscore, `.`, `$` or `@`
        fn name_starts_with(c: char) -> bool {
            c.is_ascii_alphabetic() || matches!(c, '_' | '.' | '$' | '@')
        }

        // 3. Otherwise, no restrictions, but may not contain whitespace
        if let Err((offset, c)) = is_valid_identifier(name, name_starts_with, char::is_whitespace) {
            if c.is_whitespace() {
                let pos = span.start() + offset;
                let span = SourceSpan::new(pos, pos);
                invalid_global!(
                    diagnostics,
                    global.name,
                    span,
                    "global variable names may not contain whitespace"
                );
            } else {
                debug_assert_eq!(offset, 0);
                let span = SourceSpan::new(span.start(), span.start());
                invalid_global!(
                    diagnostics,
                    global.name,
                    span,
                    "global variable names must start with an ascii-alphabetic character, '_', \
                     '.', '$', or '@'"
                );
            }
        }

        Ok(())
    }
}

#[inline(always)]
fn is_lower_ascii_alphabetic(c: char) -> bool {
    c.is_ascii_alphabetic() && c.is_ascii_lowercase()
}

/// This is necessary until [std::str::Pattern] is stabilized
trait Pattern {
    fn matches(&self, c: char) -> bool;
}
impl Pattern for char {
    #[inline(always)]
    fn matches(&self, c: char) -> bool {
        *self == c
    }
}
impl<F> Pattern for F
where
    F: Fn(char) -> bool,
{
    #[inline(always)]
    fn matches(&self, c: char) -> bool {
        self(c)
    }
}

#[inline]
fn is_valid_identifier<P1, P2>(id: &str, start_with: P1, forbidden: P2) -> Result<(), (usize, char)>
where
    P1: Pattern,
    P2: Pattern,
{
    for (offset, c) in id.char_indices() {
        if offset == 0 && !start_with.matches(c) {
            return Err((offset, c));
        }

        if forbidden.matches(c) {
            return Err((offset, c));
        }
    }

    Ok(())
}
