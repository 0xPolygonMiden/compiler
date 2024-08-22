use std::collections::VecDeque;

use super::*;
use crate::{diagnostics::DiagnosticsHandler, AttributeSet, Local, Signature};

/// Represents the declaration of a function in a [Module]
#[derive(Spanned)]
pub struct FunctionDeclaration {
    #[span]
    pub span: SourceSpan,
    pub attrs: AttributeSet,
    pub name: Ident,
    pub signature: Signature,
    pub locals: Vec<Span<Local>>,
    pub body: Region,
}
impl FunctionDeclaration {
    pub fn new(
        span: SourceSpan,
        name: Ident,
        signature: Signature,
        locals: Vec<Span<Local>>,
        body: Region,
        attrs: AttributeSet,
    ) -> Self {
        Self {
            span,
            attrs,
            name,
            signature,
            locals,
            body,
        }
    }

    /// Returns true if the entry block and signature match for this declaration
    pub fn is_declaration_valid(&self, diagnostics: &DiagnosticsHandler) -> bool {
        let entry_block = &self.body.blocks[0];
        if entry_block.params.len() != self.body.params.len() {
            let num_expected = entry_block.params.len();
            let num_declared = self.body.params.len();
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
            for (expected_ty, declared) in self.body.params.iter().zip(entry_block.params.iter()) {
                let declared_ty = &declared.ty;
                if expected_ty != declared_ty {
                    diagnostics
                        .diagnostic(Severity::Error)
                        .with_message("invalid function")
                        .with_primary_label(
                            entry_block.span,
                            "the parameter list of the entry block does not match the function \
                             signature",
                        )
                        .with_secondary_label(
                            declared.span,
                            format!(
                                "expected a parameter of type {expected_ty}, but got {declared_ty}"
                            ),
                        )
                        .emit();
                    is_valid = false;
                }
            }

            is_valid
        }
    }

    pub(super) fn populate_region_and_block_maps(
        &mut self,
        diagnostics: &DiagnosticsHandler,
    ) -> Result<(RegionsById, BlocksById), (RegionsById, BlocksById)> {
        use alloc::collections::btree_map::Entry;

        let mut regions_by_id = RegionsById::default();
        let mut blocks_by_id = BlocksById::default();

        regions_by_id.insert(self.body.id, crate::Region::new_empty(self.body.span, self.body.id));

        let mut is_valid = true;
        let mut q = VecDeque::from_iter(core::mem::take(&mut self.body.blocks));
        while let Some(mut block) = q.pop_front() {
            for inst in block.body.iter_mut() {
                match &mut inst.ty {
                    InstType::If {
                        ref mut then_region,
                        ref mut else_region,
                        ..
                    } => {
                        for region_ast in [then_region, else_region] {
                            match regions_by_id.entry(region_ast.id) {
                                Entry::Vacant(entry) => {
                                    let mut region =
                                        crate::Region::new_empty(region_ast.span, region_ast.id);
                                    let results = core::mem::take(&mut region_ast.results);
                                    region
                                        .results
                                        .extend(results.into_iter().map(|t| t.into_inner()));
                                    entry.insert(region);
                                    for block in region_ast.blocks.iter_mut() {
                                        let mut blk = Block::new(
                                            block.span,
                                            block.id,
                                            block.params.clone(),
                                            core::mem::take(&mut block.body),
                                        );
                                        blk.region_id = block.region_id;
                                        q.push_back(blk);
                                    }
                                }
                                Entry::Occupied(entry) => {
                                    diagnostics
                                        .diagnostic(Severity::Error)
                                        .with_message("invalid region")
                                        .with_primary_label(
                                            region_ast.span,
                                            "a region with the same id has already been declared",
                                        )
                                        .with_secondary_label(
                                            entry.get().span,
                                            "previously declared here",
                                        )
                                        .emit();
                                    is_valid = false;
                                    continue;
                                }
                            }
                        }
                    }
                    InstType::While {
                        ref mut before,
                        ref mut body,
                        ..
                    } => {
                        for region_ast in [before, body] {
                            match regions_by_id.entry(region_ast.id) {
                                Entry::Vacant(entry) => {
                                    let mut region =
                                        crate::Region::new_empty(region_ast.span, region_ast.id);
                                    let results = core::mem::take(&mut region_ast.results);
                                    region
                                        .results
                                        .extend(results.into_iter().map(|t| t.into_inner()));
                                    entry.insert(region);
                                    for block in region_ast.blocks.iter_mut() {
                                        let mut blk = Block::new(
                                            block.span,
                                            block.id,
                                            block.params.clone(),
                                            core::mem::take(&mut block.body),
                                        );
                                        blk.region_id = block.region_id;
                                        q.push_back(blk);
                                    }
                                }
                                Entry::Occupied(entry) => {
                                    diagnostics
                                        .diagnostic(Severity::Error)
                                        .with_message("invalid region")
                                        .with_primary_label(
                                            region_ast.span,
                                            "a region with the same id has already been declared",
                                        )
                                        .with_secondary_label(
                                            entry.get().span,
                                            "previously declared here",
                                        )
                                        .emit();
                                    is_valid = false;
                                    continue;
                                }
                            }
                        }
                    }
                    _ => continue,
                }
            }
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
            Ok((regions_by_id, blocks_by_id))
        } else {
            Err((regions_by_id, blocks_by_id))
        }
    }
}
impl fmt::Debug for FunctionDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FunctionDeclaration")
            .field("name", &self.name.as_symbol())
            .field("signature", &self.signature)
            .field("body", &self.body)
            .finish()
    }
}
impl PartialEq for FunctionDeclaration {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.signature == other.signature && self.body == other.body
    }
}
