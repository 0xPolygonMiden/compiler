use midenc_session::OutputType;

use super::*;

/// Perform code generation on the possibly-linked output of previous stages
pub struct CodegenStage;
impl Stage for CodegenStage {
    type Input = LinkerOutput;
    type Output = Either<masm::MasmArtifact, masm::ModuleTree>;

    fn enabled(&self, session: &Session) -> bool {
        session.should_codegen()
    }

    fn run(
        &mut self,
        linker_output: Self::Input,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> CompilerResult<Self::Output> {
        let LinkerOutput {
            linked,
            masm: mut masm_modules,
        } = linker_output;
        match linked {
            Left(program) => {
                let mut convert_to_masm = masm::ConvertHirToMasm::<hir::Program>::default();
                let mut artifact = convert_to_masm.convert(program, analyses, session)?;

                if session.should_emit(OutputType::Masm) {
                    for module in artifact.modules() {
                        session.emit(OutputMode::Text, module).into_diagnostic()?;
                    }
                }

                // Ensure intrinsics modules are linked
                for intrinsics_module in required_intrinsics_modules(session) {
                    artifact.insert(Box::new(intrinsics_module));
                }
                // Link in any MASM inputs provided to the compiler
                for module in masm_modules.into_iter() {
                    artifact.insert(module);
                }

                Ok(Left(artifact))
            }
            Right(ir) => {
                let mut convert_to_masm = masm::ConvertHirToMasm::<hir::Module>::default();
                for module in ir.into_iter() {
                    let masm_module = convert_to_masm.convert(module, analyses, session)?;
                    session
                        .emit(OutputMode::Text, masm_module.as_ref())
                        .into_diagnostic()
                        .wrap_err_with(|| {
                            format!("failed to emit 'masm' output for '{}'", masm_module.id)
                        })?;
                    masm_modules.insert(masm_module);
                }

                Ok(Right(masm_modules))
            }
        }
    }
}

fn required_intrinsics_modules(session: &Session) -> Vec<masm::Module> {
    vec![
        masm::intrinsics::load("intrinsics::mem", &session.source_manager)
            .expect("undefined intrinsics module"),
        masm::intrinsics::load("intrinsics::i32", &session.source_manager)
            .expect("undefined intrinsics module"),
        masm::intrinsics::load("intrinsics::i64", &session.source_manager)
            .expect("undefined intrinsics module"),
    ]
}
