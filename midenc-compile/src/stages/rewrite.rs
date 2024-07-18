use midenc_hir::{self as hir, RewritePassRegistration};

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

        // Populate the set of rewrite passes with default transformations, if there are no
        // specific passes selected.
        let mut rewrites =
            midenc_codegen_masm::default_rewrites(registered.into_iter().map(|(_, r)| r), session);
        rewrites.apply(&mut input, analyses, session)?;

        Ok(input)
    }
}
