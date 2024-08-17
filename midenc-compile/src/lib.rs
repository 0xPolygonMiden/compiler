mod compiler;
mod stage;
mod stages;

use std::rc::Rc;

use either::Either::{self, Left, Right};
use midenc_codegen_masm::{self as masm, MasmArtifact};
use midenc_hir::{
    diagnostics::{miette, Diagnostic, IntoDiagnostic, Report, WrapErr},
    pass::AnalysisManager,
};
use midenc_session::{OutputMode, Session};

pub use self::compiler::Compiler;
use self::{stage::Stage, stages::*};

pub type CompilerResult<T> = Result<T, Report>;

/// The compilation pipeline was stopped early
#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("compilation was canceled by user")]
#[diagnostic()]
pub struct CompilerStopped;

/// Run the compiler using the provided [Session]
pub fn compile(session: Rc<Session>) -> CompilerResult<()> {
    use midenc_hir::formatter::DisplayHex;
    let mut analyses = AnalysisManager::new();
    log::info!("starting compilation session");
    match compile_inputs(session.inputs.clone(), &mut analyses, &session)? {
        Artifact::Assembled(ref mast) => {
            log::info!(
                "succesfully assembled mast package '{}' with digest {}",
                mast.name,
                DisplayHex::new(&mast.digest.as_bytes())
            );
            session
                .emit(OutputMode::Text, mast)
                .into_diagnostic()
                .wrap_err("failed to pretty print 'mast' artifact")?;
            session
                .emit(OutputMode::Binary, mast)
                .into_diagnostic()
                .wrap_err("failed to serialize 'mast' artifact")
        }
        Artifact::Linked(_) => {
            log::debug!("no outputs requested by user: pipeline stopped after linking");
            Ok(())
        }
        Artifact::Lowered(_) => {
            log::debug!("no outputs requested by user: pipeline stopped before linking");
            Ok(())
        }
    }
}

/// Same as `compile`, but return compiled artifacts to the caller
pub fn compile_to_memory(session: Rc<Session>) -> CompilerResult<Artifact> {
    let mut analyses = AnalysisManager::new();
    compile_inputs(session.inputs.clone(), &mut analyses, &session)
}

/// Same as `compile_to_memory`, but allows registering a callback which will be used as an extra
/// compiler stage immediately after code generation and prior to assembly, if the linker was run.
pub fn compile_to_memory_with_pre_assembly_stage<F>(
    session: Rc<Session>,
    stage: &mut F,
) -> CompilerResult<Artifact>
where
    F: FnMut(MasmArtifact, &mut AnalysisManager, &Session) -> CompilerResult<MasmArtifact>,
{
    type AssemblyInput = Either<MasmArtifact, masm::ModuleTree>;

    let mut analyses = AnalysisManager::new();

    let mut pre_assembly_stage = move |output: AssemblyInput,
                                       analysis: &mut AnalysisManager,
                                       session: &Session| {
        match output {
            Left(artifact) => stage(artifact, analysis, session).map(Left),
            right @ Right(_) => Ok(right),
        }
    };
    let mut stages = ParseStage
        .next(SemanticAnalysisStage)
        .next_optional(ApplyRewritesStage)
        .collect(LinkerStage)
        .next(CodegenStage)
        .next(
            &mut pre_assembly_stage
                as &mut (dyn FnMut(
                    AssemblyInput,
                    &mut AnalysisManager,
                    &Session,
                ) -> CompilerResult<AssemblyInput>
                          + '_),
        )
        .next(AssembleStage);

    stages.run(session.inputs.clone(), &mut analyses, &session)
}

fn compile_inputs(
    inputs: Vec<midenc_session::InputFile>,
    analyses: &mut AnalysisManager,
    session: &Session,
) -> CompilerResult<Artifact> {
    let mut stages = ParseStage
        .next(SemanticAnalysisStage)
        .next_optional(ApplyRewritesStage)
        .collect(LinkerStage)
        .next(CodegenStage)
        .next(AssembleStage);

    stages.run(inputs, analyses, session)
}
