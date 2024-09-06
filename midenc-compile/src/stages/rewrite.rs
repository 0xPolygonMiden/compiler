use midenc_hir::RewritePassRegistration;

use super::*;

/// This stage applies all registered (and enabled) module-scoped rewrites to input HIR module(s)
pub struct ApplyRewritesStage;
impl Stage for ApplyRewritesStage {
    type Input = LinkerInput;
    type Output = LinkerInput;

    fn enabled(&self, session: &Session) -> bool {
        !session.analyze_only()
    }

    fn run(
        &mut self,
        input: Self::Input,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> CompilerResult<Self::Output> {
        let output = match input {
            input @ LinkerInput::Masm(_) => {
                log::debug!("skipping rewrites for masm input");
                input
            }
            LinkerInput::Hir(mut input) => {
                log::debug!("applying rewrite passes to '{}'", input.name.as_str());
                // Get all registered module rewrites and apply them in the order they appear
                let mut registered = vec![];
                let matches = session.matches();
                for rewrite in inventory::iter::<RewritePassRegistration<hir::Module>> {
                    log::trace!("checking if flag for rewrite pass '{}' is enabled", rewrite.name);
                    let flag = rewrite.name();
                    if matches.try_contains_id(flag).is_ok() {
                        if let Some(index) = matches.index_of(flag) {
                            let is_enabled = matches.get_flag(flag);
                            if is_enabled {
                                log::debug!(
                                    "rewrite pass '{}' is registered and enabled",
                                    rewrite.name
                                );
                                registered.push((index, rewrite.get()));
                            }
                        }
                    }
                }
                registered.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

                // Populate the set of rewrite passes with default transformations, if there are no
                // specific passes selected.
                let mut rewrites =
                    masm::default_rewrites(registered.into_iter().map(|(_, r)| r), session);
                rewrites.apply(&mut input, analyses, session)?;

                log::debug!("rewrites successful");
                LinkerInput::Hir(input)
            }
        };
        if session.rewrite_only() {
            log::debug!("stopping compiler early (rewrite-only=true)");
            Err(Report::from(CompilerStopped))
        } else {
            Ok(output)
        }
    }
}
