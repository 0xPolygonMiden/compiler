use midenc_codegen_masm::intrinsics;

use super::*;

/// Perform code generation on the possibly-linked output of previous stages
pub struct CodegenStage;
impl Stage for CodegenStage {
    type Input = LinkerOutput;
    type Output = Option<Box<masm::Program>>;

    fn enabled(&self, session: &Session) -> bool {
        session.should_codegen()
    }

    fn run(
        &mut self,
        linker_output: Self::Input,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> CompilerResult<Self::Output> {
        let MaybeLinked::Linked(program) = linker_output.program else {
            return Ok(None);
        };
        let mut convert_to_masm = masm::ConvertHirToMasm::<hir::Program>::default();
        let mut program = convert_to_masm.convert(program, analyses, session)?;
        // Ensure intrinsics modules are linked
        for intrinsics_module in required_intrinsics_modules(session) {
            program.insert(Box::new(intrinsics_module));
        }
        // Link in any MASM inputs provided to the compiler
        for module in linker_output.masm {
            program.insert(module);
        }

        Ok(Some(program))
    }
}

fn required_intrinsics_modules(session: &Session) -> Vec<masm::Module> {
    vec![
        intrinsics::load("intrinsics::mem", &session.codemap).expect("undefined intrinsics module"),
        intrinsics::load("intrinsics::i32", &session.codemap).expect("undefined intrinsics module"),
        intrinsics::load("intrinsics::i64", &session.codemap).expect("undefined intrinsics module"),
    ]
}
