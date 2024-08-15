use super::*;

/// The artifact produced by the full compiler pipeline.
///
/// The type of artifact depends on what outputs were requested, and what options were specified.
pub enum Artifact {
    /// The user requested MASM outputs, but
    Lowered(masm::ModuleTree),
    Linked(masm::MasmArtifact),
    Assembled(masm::MastArtifact),
}
impl Artifact {
    pub fn unwrap_mast(self) -> masm::MastArtifact {
        match self {
            Self::Assembled(mast) => mast,
            Self::Linked(_) => {
                panic!("expected 'mast' artifact, but got linked 'masm' artifact instead")
            }
            Self::Lowered(_) => {
                panic!("expected 'mast' artifact, but got unlinked 'masm' artifact instead")
            }
        }
    }
}

/// Perform assembly of the generated Miden Assembly, producing MAST
pub struct AssembleStage;
impl Stage for AssembleStage {
    type Input = Either<masm::MasmArtifact, masm::ModuleTree>;
    type Output = Artifact;

    fn run(
        &mut self,
        input: Self::Input,
        _analyses: &mut AnalysisManager,
        session: &Session,
    ) -> CompilerResult<Self::Output> {
        match input {
            Left(masm_artifact) if session.should_assemble() => {
                masm_artifact.assemble(session).map(Artifact::Assembled)
            }
            Left(masm_artifact) => Ok(Artifact::Linked(masm_artifact)),
            Right(_masm_modules) if session.should_assemble() => todo!(), /* Ok(Artifact::Assembled(todo!())), */
            Right(masm_modules) => Ok(Artifact::Lowered(masm_modules)),
        }
    }
}
