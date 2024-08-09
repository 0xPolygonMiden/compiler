use alloc::sync::Arc;

use crate::diagnostics::Span;

/// Obtain a [miden_assembly::ast::Ident] from a [crate::Ident], with source span intact.
pub fn translate_ident(id: crate::Ident) -> miden_assembly::ast::Ident {
    let name = Arc::from(id.as_str().to_string().into_boxed_str());
    miden_assembly::ast::Ident::new_unchecked(Span::new(id.span, name))
}
