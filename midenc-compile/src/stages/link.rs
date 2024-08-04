use midenc_hir::FunctionIdent;
use midenc_session::diagnostics::Report;

use super::*;

pub enum LinkerInput {
    Hir(Box<hir::Module>),
    Masm(Box<midenc_codegen_masm::Module>),
}

pub struct LinkerOutput {
    /// The possibly-linked HIR program
    pub program: Option<Box<hir::Program>>,
    /// The set of MASM inputs to the linker
    #[allow(clippy::vec_box)]
    pub masm: Vec<Box<midenc_codegen_masm::Module>>,
}

/// Link together one or more HIR modules into an HIR program
pub struct LinkerStage;
impl Stage for LinkerStage {
    type Input = Vec<LinkerInput>;
    type Output = LinkerOutput;

    fn run(
        &mut self,
        inputs: Self::Input,
        _analyses: &mut AnalysisManager,
        session: &Session,
    ) -> CompilerResult<Self::Output> {
        let mut ir = Vec::with_capacity(inputs.len());
        let mut masm = vec![];
        for input in inputs {
            match input {
                LinkerInput::Hir(module) => {
                    ir.push(module);
                }
                LinkerInput::Masm(module) => {
                    masm.push(module);
                }
            }
        }
        let program = if session.should_link() {
            // Find the program entrypoint, if possible
            let entrypoint = match session.options.entrypoint.as_deref() {
                Some(entrypoint) => entrypoint
                    .parse::<FunctionIdent>()
                    .map(Some)
                    .map_err(|err| Report::msg(format!("invalid --entrypoint: {err}")))?,
                None => ir.iter().find_map(|m| m.entrypoint()),
            };
            let builder = hir::ProgramBuilder::new(&session.diagnostics);
            let mut builder = if let Some(entry) = entrypoint {
                builder.with_entrypoint(entry)
            } else {
                builder
            };
            for module in ir.into_iter() {
                builder.add_module(module)?;
            }
            for module in masm.iter() {
                builder
                    .add_extern_module(module.id, module.functions().map(|f| f.name.function))?;
            }
            Some(builder.link()?)
        } else {
            None
        };
        Ok(LinkerOutput { program, masm })
    }
}
