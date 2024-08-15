use super::*;

/// This stage of compilation takes the output of the parsing
/// stage and loads it into an HIR module for later stages.
///
/// This may involve additional validation/semantic analysis, hence the name.
pub struct SemanticAnalysisStage;
impl Stage for SemanticAnalysisStage {
    type Input = ParseOutput;
    type Output = LinkerInput;

    fn run(
        &mut self,
        input: Self::Input,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> CompilerResult<Self::Output> {
        let parse_only = session.parse_only();
        let output = match input {
            ParseOutput::Ast(ast) if parse_only => {
                session.emit(OutputMode::Text, &ast).into_diagnostic()?;
                return Err(CompilerStopped.into());
            }
            ParseOutput::Ast(ast) => {
                session.emit(OutputMode::Text, &ast).into_diagnostic()?;
                let mut convert_to_hir = ast::ConvertAstToHir;
                let module = Box::new(convert_to_hir.convert(ast, analyses, session)?);
                LinkerInput::Hir(module)
            }
            ParseOutput::Hir(module) if parse_only => {
                session.emit(OutputMode::Text, &module).into_diagnostic()?;
                return Err(CompilerStopped.into());
            }
            ParseOutput::Hir(module) => LinkerInput::Hir(module),
            ParseOutput::Masm(masm) if parse_only => {
                session.emit(OutputMode::Text, &masm).into_diagnostic()?;
                return Err(CompilerStopped.into());
            }
            ParseOutput::Masm(masm) => LinkerInput::Masm(masm),
        };
        if session.analyze_only() {
            Err(CompilerStopped.into())
        } else {
            Ok(output)
        }
    }
}
