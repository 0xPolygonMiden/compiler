use super::*;

/// This stage of compilation takes the output of the parsing
/// stage and loads it into an HIR module for later stages.
///
/// This may involve additional validation/semantic analysis, hence the name.
pub struct SemanticAnalysisStage;
impl Stage for SemanticAnalysisStage {
    type Input = ParseOutput;
    type Output = Box<hir::Module>;

    fn enabled(&self, session: &Session) -> bool {
        !session.parse_only()
    }

    fn run(
        &mut self,
        input: Self::Input,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> CompilerResult<Self::Output> {
        match input {
            ParseOutput::Ast(ast) => {
                let mut convert_to_hir = ast::ConvertAstToHir;
                let module = Box::new(convert_to_hir.convert(ast, analyses, session)?);
                session.emit(&module)?;
                Ok(module)
            }
            ParseOutput::Hir(module) => Ok(module),
        }
    }
}
