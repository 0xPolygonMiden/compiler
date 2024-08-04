mod compiler;
mod stage;
mod stages;

use std::sync::Arc;

use midenc_codegen_masm as masm;
use midenc_hir::{
    diagnostics::{miette, Diagnostic, IntoDiagnostic, Report, WrapErr},
    pass::AnalysisManager,
};
use midenc_session::{OutputType, Session};

pub use self::compiler::{Compiler, CompilerOptions};
use self::{stage::Stage, stages::*};

pub type CompilerResult<T> = Result<T, Report>;

/// The compilation pipeline was stopped early
#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("compilation was canceled by user")]
#[diagnostic()]
pub struct CompilerStopped;

/// Register dynamic flags to be shown via `midenc help compile`
pub fn register_flags(cmd: clap::Command) -> clap::Command {
    use midenc_hir::RewritePassRegistration;
    use midenc_session::CompileFlag;

    let cmd = inventory::iter::<CompileFlag>.into_iter().fold(cmd, |cmd, flag| {
        let arg = clap::Arg::new(flag.name)
            .long(flag.long.unwrap_or(flag.name))
            .action(clap::ArgAction::from(flag.action));
        let arg = if let Some(help) = flag.help {
            arg.help(help)
        } else {
            arg
        };
        let arg = if let Some(help_heading) = flag.help_heading {
            arg.help_heading(help_heading)
        } else {
            arg
        };
        let arg = if let Some(short) = flag.short {
            arg.short(short)
        } else {
            arg
        };
        let arg = if let Some(env) = flag.env {
            arg.env(env)
        } else {
            arg
        };
        let arg = if let Some(value) = flag.default_missing_value {
            arg.default_missing_value(value)
        } else {
            arg
        };
        let arg = if let Some(value) = flag.default_value {
            arg.default_value(value)
        } else {
            arg
        };
        cmd.arg(arg)
    });

    inventory::iter::<RewritePassRegistration<midenc_hir::Module>>.into_iter().fold(
        cmd,
        |cmd, rewrite| {
            let name = rewrite.name();
            let arg = clap::Arg::new(name)
                .long(name)
                .action(clap::ArgAction::SetTrue)
                .help(rewrite.summary())
                .help_heading("Transformations");
            cmd.arg(arg)
        },
    )
}

/// Run the compiler using the provided [Session]
pub fn compile(session: Arc<Session>) -> CompilerResult<()> {
    let mut analyses = AnalysisManager::new();
    match compile_inputs(session.inputs.clone(), &mut analyses, &session)? {
        // No outputs, generally due to skipping codegen
        None => return Ok(()),
        Some(output) => {
            if let Some(path) = session.emit_to(OutputType::Mast, None) {
                match output {
                    masm::MasmArtifact::Executable(_) => {
                        log::warn!(
                            "skipping emission of MAST to {} as output type is not fully \
                             supported yet",
                            path.display()
                        );
                    }
                    masm::MasmArtifact::Library(ref library) => {
                        let mast = library.assemble(&session)?;
                        mast.write_to_file(
                            path.clone(),
                            miden_assembly::ast::AstSerdeOptions {
                                debug_info: session.options.emit_debug_decorators(),
                                ..Default::default()
                            },
                        )
                        .into_diagnostic()
                        .wrap_err_with(|| {
                            format!("failed to write MAST to '{}'", path.display())
                        })?;
                    }
                }
            }
            if session.should_emit(OutputType::Masm) {
                for module in output.modules() {
                    session.emit(module).into_diagnostic()?;
                }
            }
        }
    }

    Ok(())
}

/// Same as `compile`, but return compiled artifacts to the caller
pub fn compile_to_memory(session: Arc<Session>) -> CompilerResult<Option<masm::MasmArtifact>> {
    let mut analyses = AnalysisManager::new();
    compile_inputs(session.inputs.clone(), &mut analyses, &session)
}

fn compile_inputs(
    inputs: Vec<midenc_session::InputFile>,
    analyses: &mut AnalysisManager,
    session: &Session,
) -> CompilerResult<Option<masm::MasmArtifact>> {
    let mut stages = ParseStage
        .next(SemanticAnalysisStage)
        .next_optional(ApplyRewritesStage)
        .collect(LinkerStage)
        .next(CodegenStage);

    stages.run(inputs, analyses, session)
}
