use midenc_codegen_masm::intrinsics;

use super::*;

/// The code generator may output either a single program,
/// ora  collection of modules, depending on earlier stages.
pub enum Compiled {
    Program(Box<masm::Program>),
    Modules(Vec<Box<masm::Module>>),
}

/// Perform code generation on the possibly-linked output of previous stages
pub struct CodegenStage;
impl Stage for CodegenStage {
    type Input = MaybeLinked;
    type Output = Compiled;

    fn enabled(&self, session: &Session) -> bool {
        session.should_codegen()
    }

    fn run(
        &mut self,
        input: Self::Input,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> CompilerResult<Self::Output> {
        match input {
            MaybeLinked::Linked(program) => {
                let mut convert_to_masm = masm::ConvertHirToMasm::<hir::Program>::default();
                let mut program = convert_to_masm.convert(program, analyses, session)?;
                // Ensure intrinsics modules are linked
                for intrinsics_module in required_intrinsics_modules(session) {
                    program.insert(Box::new(intrinsics_module));
                }
                Ok(Compiled::Program(program))
            }
            MaybeLinked::Unlinked(modules) => {
                let mut convert_to_masm = masm::ConvertHirToMasm::<hir::Module>::default();
                let mut masm_modules = Vec::with_capacity(modules.len());
                // Ensure intrinsics modules are linked
                for intrinsics_module in required_intrinsics_modules(session) {
                    masm_modules.push(Box::new(intrinsics_module));
                }
                for module in modules.into_iter() {
                    let masm_module = convert_to_masm.convert(module, analyses, session)?;
                    masm_modules.push(masm_module);
                }
                Ok(Compiled::Modules(masm_modules))
            }
        }
    }
}

fn required_intrinsics_modules(session: &Session) -> Vec<masm::Module> {
    vec![
        intrinsics::load("intrinsics::mem", &session.codemap).expect("undefined intrinsics module"),
        intrinsics::load("intrinsics::i32", &session.codemap).expect("undefined intrinsics module"),
        intrinsics::load("intrinsics::i64", &session.codemap).expect("undefined intrinsics module"),
    ]
}
