use std::sync::Arc;

use miden_assembly::library::Library as CompiledLibrary;
use midenc_hir::Symbol;
use midenc_session::{Emit, OutputMode, OutputType, Session};

/// The artifact produced by lowering a [Program] to a Merkelized Abstract Syntax Tree
///
/// This type is used in compilation pipelines to abstract over the type of output requested.
pub enum MastArtifact {
    /// A MAST artifact which can be executed by the VM directly
    Executable(Arc<miden_core::Program>),
    /// A MAST artifact which can be used as a dependency by a [miden_core::Program]
    Library(Arc<CompiledLibrary>),
}

impl MastArtifact {
    pub fn unwrap_program(self) -> Arc<miden_core::Program> {
        match self {
            Self::Executable(prog) => prog,
            Self::Library(_) => panic!("attempted to unwrap 'mast' library as program"),
        }
    }
}

impl Emit for MastArtifact {
    fn name(&self) -> Option<Symbol> {
        None
    }

    fn output_type(&self, mode: OutputMode) -> OutputType {
        match mode {
            OutputMode::Text => OutputType::Mast,
            OutputMode::Binary => OutputType::Masl,
        }
    }

    fn write_to<W: std::io::Write>(
        &self,
        writer: W,
        mode: OutputMode,
        session: &Session,
    ) -> std::io::Result<()> {
        match self {
            Self::Executable(ref prog) => {
                if matches!(mode, OutputMode::Binary) {
                    log::warn!(
                        "unable to write 'masl' output type for miden_core::Program: skipping.."
                    );
                }
                prog.write_to(writer, mode, session)
            }
            Self::Library(ref lib) => lib.write_to(writer, mode, session),
        }
    }
}
