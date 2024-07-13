#![feature(array_windows)]
#![feature(is_sorted)]

mod codegen;
mod convert;
mod emulator;
mod masm;
#[cfg(test)]
mod tests;

use midenc_hir as hir;
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
        use midenc_hir::pass::{ConversionPass, ModuleRewritePassAdapter, RewritePass, RewriteSet};
        use midenc_hir_transform as transforms;

        let mut rewrites = RewriteSet::default();
        rewrites.push(ModuleRewritePassAdapter::new(transforms::SplitCriticalEdges));
        rewrites.push(ModuleRewritePassAdapter::new(transforms::Treeify));
        rewrites.push(ModuleRewritePassAdapter::new(transforms::InlineBlocks));

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
