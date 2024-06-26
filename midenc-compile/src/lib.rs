mod compiler;
mod stage;
mod stages;

use std::sync::Arc;

use midenc_codegen_masm as masm;
use midenc_hir::pass::AnalysisManager;
use midenc_session::{OutputType, Session};

pub use self::{compiler::Compiler, stages::Compiled};
use self::{stage::Stage, stages::*};

pub type CompilerResult<T> = Result<T, CompilerError>;

#[derive(Debug, thiserror::Error)]
pub enum CompilerError {
    /// An error was raised due to invalid command-line arguments or argument validation
    #[error(transparent)]
    Clap(#[from] clap::Error),
    /// The compilation pipeline was stopped early
    #[error("compilation was canceled by user")]
    Stopped,
    /// An invalid input was given to the compiler
    #[error(transparent)]
    InvalidInput(#[from] midenc_session::InvalidInputError),
    /// An error occurred while parsing/translating a Wasm module from binary
    #[error(transparent)]
    WasmError(#[from] midenc_frontend_wasm::WasmError),
    /// An error occurred while parsing/translating a Wasm module from text
    #[error(transparent)]
    WatError(#[from] wat::Error),
    /// An error occurred while parsing an HIR module
    #[error(transparent)]
    Parsing(#[from] midenc_hir::parser::ParseError),
    /// An error occurred while running an analysis
    #[error(transparent)]
    Analysis(#[from] midenc_hir::pass::AnalysisError),
    /// An error occurred while rewriting an IR entity
    #[error(transparent)]
    Rewriting(#[from] midenc_hir::pass::RewriteError),
    /// An error occurred while converting from one dialect to another
    #[error(transparent)]
    Conversion(#[from] midenc_hir::pass::ConversionError),
    /// An error occurred while linking a program
    #[error(transparent)]
    Linker(#[from] midenc_hir::LinkerError),
    /// An error occurred when reading a file
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// An error occurred while compiling a program
    #[error(transparent)]
    Failed(#[from] anyhow::Error),
    /// An error was emitted as a diagnostic, so we don't need to emit info to stdout
    #[error("exited due to error: see diagnostics for details")]
    Reported,
}
impl From<midenc_hir::ModuleConflictError> for CompilerError {
    fn from(err: midenc_hir::ModuleConflictError) -> CompilerError {
        Self::Linker(midenc_hir::LinkerError::ModuleConflict(err.0))
    }
}

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
    let inputs = vec![session.input.clone()];
    let mut analyses = AnalysisManager::new();
    match compile_inputs(inputs, &mut analyses, &session) {
        Ok(Compiled::Program(ref program)) => {
            if let Some(path) = session.emit_to(OutputType::Mast, None) {
                log::warn!(
                    "skipping emission of MAST to {} as output type is not fully supported yet",
                    path.display()
                );
            }
            if session.should_emit(OutputType::Masm) {
                for module in program.modules() {
                    session.emit(module)?;
                }
            }
        }
        Ok(Compiled::Modules(modules)) => {
            let mut program = masm::Program::empty();
            for module in modules.into_iter() {
                program.insert(module);
            }
            if let Some(path) = session.emit_to(OutputType::Mast, None) {
                log::warn!(
                    "skipping emission of MAST to {} as output type is not fully supported yet",
                    path.display()
                );
            }
            if session.should_emit(OutputType::Masm) {
                for module in program.modules() {
                    session.emit(module)?;
                }
            }
        }
        Err(CompilerError::Stopped) => return Ok(()),
        Err(CompilerError::Reported) => return Err(CompilerError::Reported),
        Err(err) => {
            session.diagnostics.error(err);
            session.diagnostics.abort_if_errors();
        }
    }

    Ok(())
}

/// Same as `compile`, but return compiled artifacts to the caller
pub fn compile_to_memory(session: Arc<Session>) -> CompilerResult<Compiled> {
    let inputs = vec![session.input.clone()];
    let mut analyses = AnalysisManager::new();
    match compile_inputs(inputs, &mut analyses, &session) {
        Ok(output) => Ok(output),
        Err(err) => {
            session.diagnostics.error(err.to_string());
            session.diagnostics.abort_if_errors();
            Err(CompilerError::Reported)
        }
    }
}

fn compile_inputs(
    inputs: Vec<midenc_session::InputFile>,
    analyses: &mut AnalysisManager,
    session: &Session,
) -> CompilerResult<Compiled> {
    let mut stages = ParseStage
        .next(SemanticAnalysisStage)
        .next_optional(ApplyRewritesStage)
        .collect(LinkerStage)
        .next(CodegenStage);

    stages.run(inputs, analyses, session)
}
