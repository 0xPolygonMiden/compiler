#![feature(array_windows)]
#![feature(is_sorted)]

mod codegen;
mod convert;
mod emulator;
mod masm;
#[cfg(test)]
mod tests;

use midenc_hir::{
    self as hir,
    pass::{RewritePass, RewriteSet},
};
use midenc_session::Session;

pub use self::{
    convert::ConvertHirToMasm,
    emulator::{
        Breakpoint, BreakpointEvent, CallFrame, DebugInfo, DebugInfoWithStack, EmulationError,
        Emulator, EmulatorEvent, InstructionPointer, WatchMode, Watchpoint, WatchpointId,
    },
    masm::*,
};

/// This error type represents all of the errors produced by [MasmCompiler]
#[derive(Debug, thiserror::Error)]
pub enum CompilerError {
    /// Two or more modules conflict with each other
    #[error(transparent)]
    ModuleConflict(#[from] hir::ModuleConflictError),
    /// An error occurred at link-time
    #[error(transparent)]
    Linker(#[from] hir::LinkerError),
    /// An error occurred during analysis
    #[error(transparent)]
    Analysis(#[from] hir::pass::AnalysisError),
    /// An error occurred during application of a rewrite
    #[error(transparent)]
    Rewrite(#[from] hir::pass::RewriteError),
    /// An error occurred during application of a conversion
    #[error(transparent)]
    Conversion(#[from] hir::pass::ConversionError),
}

pub type CompilerResult<T> = Result<T, CompilerError>;

/// [MasmCompiler] is a compiler from Miden IR to MASM IR, an intermediate representation
/// of Miden Assembly which is used within the Miden compiler framework for various purposes,
/// and can be emitted directly to textual Miden Assembly.
///
/// The [MasmCompiler] is designed to compile a [midenc_hir::Program]
///
/// can be used to take a linked [midenc_hir::Program] and
/// compile it to MASM IR, an intermediate representation of Miden Assembly
/// used within the compiler.
pub struct MasmCompiler<'a> {
    session: &'a Session,
    analyses: hir::pass::AnalysisManager,
}
impl<'a> MasmCompiler<'a> {
    pub fn new(session: &'a Session) -> Self {
        Self {
            session,
            analyses: hir::pass::AnalysisManager::new(),
        }
    }

    /// Compile an [hir::Program] that has been linked and is ready to be compiled.
    pub fn compile(&mut self, mut input: Box<hir::Program>) -> CompilerResult<Box<Program>> {
        use midenc_hir::pass::ConversionPass;

        let mut rewrites = default_rewrites([], self.session);

        let modules = input.modules_mut().take();
        for mut module in modules.into_iter() {
            rewrites.apply(&mut module, &mut self.analyses, self.session)?;
            input.modules_mut().insert(module);
        }

        let mut convert_to_masm = ConvertHirToMasm::<hir::Program>::default();
        let mut program = convert_to_masm.convert(input, &mut self.analyses, self.session)?;

        // Ensure standard library is linked
        for module in intrinsics::load_stdlib(&self.session.codemap) {
            program.insert(Box::new(module.clone()));
        }

        // Ensure intrinsics modules are linked
        program.insert(Box::new(
            intrinsics::load("intrinsics::mem", &self.session.codemap)
                .expect("undefined intrinsics module"),
        ));
        program.insert(Box::new(
            intrinsics::load("intrinsics::i32", &self.session.codemap)
                .expect("undefined intrinsics module"),
        ));
        program.insert(Box::new(
            intrinsics::load("intrinsics::i64", &self.session.codemap)
                .expect("undefined intrinsics module"),
        ));

        Ok(program)
    }

    /// Compile a single [hir::Module] as a program.
    ///
    /// It is assumed that the given module has been validated, and that all necessary
    /// rewrites have been applied. If one of these invariants is not upheld, compilation
    /// may fail.
    pub fn compile_module(&mut self, input: Box<hir::Module>) -> CompilerResult<Box<Program>> {
        let program =
            hir::ProgramBuilder::new(&self.session.diagnostics).with_module(input)?.link()?;

        self.compile(program)
    }

    /// Compile a set of [hir::Module] as a program.
    ///
    /// It is assumed that the given modules have been validated, and that all necessary
    /// rewrites have been applied. If one of these invariants is not upheld, compilation
    /// may fail.
    pub fn compile_modules<I: IntoIterator<Item = Box<hir::Module>>>(
        &mut self,
        input: I,
    ) -> CompilerResult<Box<Program>> {
        let mut builder = hir::ProgramBuilder::new(&self.session.diagnostics);
        for module in input.into_iter() {
            builder.add_module(module)?;
        }

        let program = builder.link()?;

        self.compile(program)
    }
}

pub fn default_rewrites<P>(registered: P, session: &Session) -> RewriteSet<hir::Module>
where
    P: IntoIterator<Item = Box<dyn RewritePass<Entity = hir::Module>>>,
    <P as IntoIterator>::IntoIter: ExactSizeIterator,
{
    use midenc_hir::pass::ModuleRewritePassAdapter;

    let registered = registered.into_iter();

    // If no rewrites were explicitly enabled, and conversion to Miden Assembly is,
    // then we must ensure that the basic transformation passes are applied.
    //
    // Otherwise, assume that the intent was to skip those rewrites and do not add them
    let mut rewrites = RewriteSet::default();
    if registered.len() == 0 {
        if dbg!(session.should_codegen()) {
            let fn_rewrites = default_function_rewrites(session);
            for rewrite in fn_rewrites {
                rewrites.push(ModuleRewritePassAdapter::new(rewrite));
            }
        }
    } else {
        rewrites.extend(registered);
    }

    rewrites
}

pub fn default_function_rewrites(session: &Session) -> RewriteSet<hir::Function> {
    use midenc_hir::pass::{AnalysisManager, RewriteFn, RewriteResult};
    use midenc_hir_transform as transforms;

    // If no rewrites were explicitly enabled, and conversion to Miden Assembly is,
    // then we must ensure that the basic transformation passes are applied.
    //
    // Otherwise, assume that the intent was to skip those rewrites and do not add them
    let mut rewrites = RewriteSet::default();
    if dbg!(session.should_codegen()) {
        rewrites.push(transforms::SplitCriticalEdges);
        rewrites.push(transforms::Treeify);
        rewrites.push(transforms::InlineBlocks);
        // The two spills transformation passes must be applied consecutively
        //
        // We run this transformation after any other significant rewrites, to ensure that
        // the spill placement is as accurate as possible. Block inlining will not disturb
        // spill placement, but we want to run it at least once before this pass to simplify
        // the output of the treeification pass.
        rewrites.push(transforms::InsertSpills);
        rewrites.push(transforms::RewriteSpills);
        // If the spills transformation is run, we want to run the block inliner again to
        // clean up the output, but _only_ if there were actually spills, otherwise running
        // the inliner again will have no effect. To avoid that case, we wrap the second run
        // in a closure which will only apply the pass if there were spills
        let maybe_rerun_block_inliner: Box<RewriteFn<hir::Function>> = Box::new(
            |function: &mut hir::Function,
             analyses: &mut AnalysisManager,
             session: &Session|
             -> RewriteResult {
                let has_spills = analyses
                    .get::<midenc_hir_analysis::SpillAnalysis>(&function.id)
                    .map(|spills| spills.has_spills())
                    .unwrap_or(false);
                if has_spills {
                    let mut inliner = transforms::InlineBlocks;
                    inliner.apply(function, analyses, session)
                } else {
                    Ok(())
                }
            },
        );
        rewrites.push(maybe_rerun_block_inliner);
    }

    rewrites
}
