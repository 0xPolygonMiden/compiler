use midenc_hir::{
    self as hir,
    pass::{RewriteFn, RewriteResult},
    RewritePassRegistration,
};
use midenc_hir_transform as transforms;

use super::*;

/// This stage applies all registered (and enabled) module-scoped rewrites to input HIR module(s)
pub struct ApplyRewritesStage;
impl Stage for ApplyRewritesStage {
    type Input = Box<hir::Module>;
    type Output = Box<hir::Module>;

    fn enabled(&self, session: &Session) -> bool {
        !session.parse_only()
    }

    fn run(
        &mut self,
        mut input: Self::Input,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> CompilerResult<Self::Output> {
        use midenc_hir::pass::{ModuleRewritePassAdapter, RewriteSet};

        // Get all registered module rewrites and apply them in the order they appear
        let mut registered = vec![];
        let matches = session.matches();
        for rewrite in inventory::iter::<RewritePassRegistration<hir::Module>> {
            let flag = rewrite.name();
            if matches.try_contains_id(flag).is_ok() {
                if let Some(index) = matches.index_of(flag) {
                    let is_enabled = matches.get_flag(flag);
                    if is_enabled {
                        registered.push((index, rewrite.get()));
                    }
                }
            }
        }
        registered.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

        // If no rewrites were explicitly enabled, and conversion to Miden Assembly is,
        // then we must ensure that the basic transformation passes are applied.
        //
        // Otherwise, assume that the intent was to skip those rewrites and do not add them
        let mut rewrites = RewriteSet::default();
        if registered.is_empty() {
            if session.should_codegen() {
                rewrites.push(ModuleRewritePassAdapter::new(transforms::SplitCriticalEdges));
                rewrites.push(ModuleRewritePassAdapter::new(transforms::Treeify));
                rewrites.push(ModuleRewritePassAdapter::new(transforms::InlineBlocks));
                // The two spills transformation passes must be applied consecutively
                //
                // We run this transformation after any other significant rewrites, to ensure that
                // the spill placement is as accurate as possible. Block inlining will not disturb
                // spill placement, but we want to run it at least once before this pass to simplify
                // the output of the treeification pass.
                rewrites.push(ModuleRewritePassAdapter::new(transforms::InsertSpills));
                rewrites.push(ModuleRewritePassAdapter::new(transforms::RewriteSpills));
                // If the spills transformation is run, we want to run the block inliner again to
                // clean up the output, but _only_ if there were actually spills, otherwise running
                // the inliner again will have no effect. To avoid that case, we wrap the second run
                // in a closure which will only apply the pass if there were spills
                let maybe_rerun_block_inliner: Box<RewriteFn<hir::Function>> = Box::new(
                    |function: &mut hir::Function,
                     analyses: &mut AnalysisManager,
                     session: &Session|
                     -> RewriteResult {
                        let has_spills = analyses
                            .get::<midenc_hir_analysis::SpillAnalysis>(&function.id)
                            .map(|spills| spills.has_spills())
                            .unwrap_or(false);
                        if has_spills {
                            let mut inliner = transforms::InlineBlocks;
                            inliner.apply(function, analyses, session)
                        } else {
                            Ok(())
                        }
                    },
                );
                rewrites.push(ModuleRewritePassAdapter::new(maybe_rerun_block_inliner));
            }
        } else {
            rewrites.extend(registered.into_iter().map(|(_, r)| r));
        }

        rewrites.apply(&mut input, analyses, session)?;

        Ok(input)
    }
}
