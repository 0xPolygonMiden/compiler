use core::fmt;

use crate::{AttributeSet, Ident, Signature};

use super::*;

/// Represents the declaration of a function in a [Module]
#[derive(Spanned)]
pub struct FunctionDeclaration {
    #[span]
    pub span: SourceSpan,
    pub attrs: AttributeSet,
    pub name: Ident,
    pub signature: Signature,
    pub blocks: Vec<Block>,
}
impl FunctionDeclaration {
    pub fn new(
        span: SourceSpan,
        name: Ident,
        signature: Signature,
        blocks: Vec<Block>,
        attrs: AttributeSet,
    ) -> Self {
        Self {
            span,
            attrs,
            name,
            signature,
            blocks,
        }
    }

    /// Returns true if the entry block and signature match for this declaration
    pub fn is_declaration_valid(
        &self,
        diagnostics: &miden_diagnostics::DiagnosticsHandler,
    ) -> bool {
        let entry_block = &self.blocks[0];
        if entry_block.params.len() != self.signature.arity() {
            let num_expected = entry_block.params.len();
            let num_declared = self.signature.arity();
            diagnostics
                .diagnostic(Severity::Error)
                .with_message("invalid function")
                .with_primary_label(
                    entry_block.span,
                    "the parameter list of the entry block does not match the function signature",
                )
                .with_secondary_label(
                    self.span,
                    format!("expected {num_expected} parameters, but got {num_declared}"),
                )
                .emit();
            false
        } else {
            let mut is_valid = true;
            for (expected, declared) in self.signature.params.iter().zip(entry_block.params.iter())
            {
                let expected_ty = &expected.ty;
                let declared_ty = &declared.ty;
                if expected_ty != declared_ty {
                    diagnostics.diagnostic(Severity::Error)
                        .with_message("invalid function")
                        .with_primary_label(entry_block.span, "the parameter list of the entry block does not match the function signature")
                        .with_secondary_label(declared.span, format!("expected a paramter of type {expected_ty}, but got {declared_ty}"))
                        .emit();
                    is_valid = false;
                }
            }

            is_valid
        }
    }

    pub(super) fn populate_block_map(
        &mut self,
        diagnostics: &miden_diagnostics::DiagnosticsHandler,
    ) -> Result<BlocksById, BlocksById> {
        use std::collections::hash_map::Entry;

        let mut blocks_by_id = BlocksById::default();
        let mut is_valid = true;
        for block in core::mem::take(&mut self.blocks).into_iter() {
            match blocks_by_id.entry(block.id) {
                Entry::Vacant(entry) => {
                    entry.insert(block);
                }
                Entry::Occupied(entry) => {
                    diagnostics
                        .diagnostic(Severity::Error)
                        .with_message("invalid block")
                        .with_primary_label(
                            block.span,
                            "a block with the same name has already been declared",
                        )
                        .with_secondary_label(entry.get().span, "previously declared here")
                        .emit();
                    is_valid = false;
                    continue;
                }
            }
        }

        if is_valid {
            Ok(blocks_by_id)
        } else {
            Err(blocks_by_id)
        }
    }
}
impl fmt::Debug for FunctionDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FunctionDeclaration")
            .field("name", &self.name.as_symbol())
            .field("signature", &self.signature)
            .field("blocks", &self.blocks)
            .finish()
    }
}
impl PartialEq for FunctionDeclaration {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.signature == other.signature && self.blocks == other.blocks
    }
}
