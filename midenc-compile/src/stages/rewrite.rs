use miden_hir::RewritePassRegistration;
use miden_hir_transform as transforms;

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
        use miden_hir::pass::{ModuleRewritePassAdapter, RewriteSet};

        // Get all registered module rewrites and apply them in the order they appear
        let mut registered = vec![];
        let matches = session.matches();
        for rewrite in inventory::iter::<RewritePassRegistration<hir::Module>> {
            let flag = rewrite.name();
            if let Some(index) = matches.index_of(flag) {
                let is_enabled = matches.get_flag(flag);
                if is_enabled {
                    registered.push((index, rewrite.get()));
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
            let convert_to_masm_flag = <masm::ConvertHirToMasm<hir::Module> as PassInfo>::FLAG;
            let convert_to_masm = matches.get_flag(convert_to_masm_flag);

            if convert_to_masm {
                rewrites.push(ModuleRewritePassAdapter::new(
                    transforms::SplitCriticalEdges,
                ));
                rewrites.push(ModuleRewritePassAdapter::new(transforms::Treeify));
                rewrites.push(ModuleRewritePassAdapter::new(transforms::InlineBlocks));
            }
        } else {
            rewrites.extend(registered.into_iter().map(|(_, r)| r));
        }

        rewrites.apply(&mut input, analyses, session)?;

        Ok(input)
    }
}
