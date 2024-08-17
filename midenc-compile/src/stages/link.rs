use super::*;

pub enum LinkerInput {
    Hir(Box<hir::Module>),
    Masm(Box<masm::Module>),
}

pub struct LinkerOutput {
    /// The linked HIR program, or the unlinked HIR modules
    pub linked: Either<Box<hir::Program>, hir::ModuleList>,
    /// The set of MASM inputs to the linker
    pub masm: masm::ModuleTree,
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
        let mut ir = hir::ModuleList::default();
        let mut masm = masm::ModuleTree::default();
        for input in inputs {
            match input {
                LinkerInput::Hir(module) => {
                    ir.push_back(module);
                }
                LinkerInput::Masm(module) => {
                    masm.insert(module);
                }
            }
        }
        if session.should_link() {
            log::debug!("linking hir program");

            // Construct a new [Program] builder
            let mut builder = match session.options.entrypoint.as_deref() {
                Some(entrypoint) => {
                    log::debug!("overriding entrypoint with '{entrypoint}'");
                    let entrypoint = entrypoint
                        .parse::<hir::FunctionIdent>()
                        .map_err(|err| Report::msg(format!("invalid --entrypoint: {err}")))?;
                    hir::ProgramBuilder::new(&session.diagnostics).with_entrypoint(entrypoint)
                }
                None => hir::ProgramBuilder::new(&session.diagnostics),
            };

            // Add our HIR modules
            for module in ir.into_iter() {
                log::debug!("adding '{}' to linker inputs", module.name);
                builder.add_module(module)?;
            }

            // Handle linking against ad-hoc MASM sources
            for module in masm.iter() {
                log::debug!("adding external module '{}' to linker inputs", module.name);
                builder
                    .add_extern_module(module.id, module.functions().map(|f| f.name.function))?;
            }

            // Load link libraries now
            for link_lib in session.options.link_libraries.iter() {
                log::debug!(
                    "registering link library '{}' ({}, from {:#?}) with linker",
                    link_lib.name,
                    link_lib.kind,
                    link_lib.path.as_ref()
                );
                builder.add_library(link_lib.load(session)?);
            }

            let linked = Left(builder.link()?);

            if session.options.link_only {
                log::debug!("stopping compiler early (link-only=true)");
                Err(Report::from(CompilerStopped))
            } else {
                Ok(LinkerOutput { linked, masm })
            }
        } else {
            log::debug!("skipping hir linker (should-link=false)");
            Ok(LinkerOutput {
                linked: Right(ir),
                masm,
            })
        }
    }
}
