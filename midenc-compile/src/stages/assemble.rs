use super::*;

/// The artifact produced by the full compiler pipeline.
///
/// The type of artifact depends on what outputs were requested, and what options were specified.
pub enum Artifact {
    /// The user requested MASM outputs, but
    Lowered(masm::ModuleTree),
    Linked(masm::MasmArtifact),
    Assembled(masm::Package),
}
impl Artifact {
    pub fn unwrap_mast(self) -> masm::Package {
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
        use midenc_hir::formatter::DisplayHex;

        match input {
            Left(masm_artifact) if session.should_assemble() => {
                let mast = masm_artifact.assemble(session)?;
                log::debug!(
                    "successfully assembled mast artifact with digest {}",
                    DisplayHex::new(&mast.digest().as_bytes())
                );
                Ok(Artifact::Assembled(masm::Package::new(mast, &masm_artifact, session)))
            }
            Left(masm_artifact) => {
                log::debug!(
                    "skipping assembly of mast package from masm artifact (should-assemble=false)"
                );
                Ok(Artifact::Linked(masm_artifact))
            }
            Right(_masm_modules) if session.should_assemble() => todo!(), /* Ok(Artifact::Assembled(todo!())), */
            Right(masm_modules) => {
                log::debug!(
                    "skipping assembly of mast package from unlinked modules \
                     (should-assemble=false)"
                );
                Ok(Artifact::Lowered(masm_modules))
            }
        }
    }
}
