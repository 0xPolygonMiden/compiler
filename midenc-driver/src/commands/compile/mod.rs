mod stage;
mod stages;

use self::stage::Stage;
use self::stages::*;

use miden_hir::pass::AnalysisManager;
use midenc_session::Session;
use std::sync::Arc;

use crate::{DriverError, DriverResult};

pub fn compile(session: Arc<Session>) -> DriverResult<()> {
    let inputs = vec![session.input.clone()];
    let mut analyses = AnalysisManager::new();
    match compile_inputs(inputs, &mut analyses, &session) {
        Ok(_) | Err(DriverError::Stopped) => return Ok(()),
        Err(err) => {
            session.diagnostics.error(err);
            session.diagnostics.abort_if_errors();
        }
    }

    Ok(())
}

fn compile_inputs(
    inputs: Vec<midenc_session::InputFile>,
    analyses: &mut AnalysisManager,
    session: &Session,
) -> DriverResult<()> {
    let mut stages = ParseStage
        .next(SemanticAnalysisStage)
        .next_optional(ApplyRewritesStage)
        .collect(LinkerStage)
        .next(CodegenStage);

    let _compiled = stages.run(inputs, analyses, session)?;

    Ok(())
}
