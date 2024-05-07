use midenc_session::ProjectType;

use super::*;

/// This type is used to represent the fact that depending on
/// flags provided to the compiler, we may or may not perform
/// the link, in which case we will just have a loose collection
/// of modules, not a [Program]
pub enum MaybeLinked {
    Linked(Box<hir::Program>),
    Unlinked(Vec<Box<hir::Module>>),
}

/// Link together one or more HIR modules into an HIR program
pub struct LinkerStage;
impl Stage for LinkerStage {
    type Input = Vec<Box<hir::Module>>;
    type Output = MaybeLinked;

    fn run(
        &mut self,
        input: Self::Input,
        _analyses: &mut AnalysisManager,
        session: &Session,
    ) -> CompilerResult<Self::Output> {
        // Temporary workaround for the issue that backend builds only Program for all
        // OutputType::Masm output types. In case we need a library, we should not link the modules.
        match session.project_type {
            ProjectType::Program => {
                if session.should_link() {
                    let mut builder = hir::ProgramBuilder::new(&session.diagnostics);
                    for module in input.into_iter() {
                        builder.add_module(module)?;
                    }
                    Ok(MaybeLinked::Linked(builder.link()?))
                } else {
                    Ok(MaybeLinked::Unlinked(input.into_iter().collect()))
                }
            }
            ProjectType::Library => Ok(MaybeLinked::Unlinked(input.into_iter().collect())),
        }
    }
}
