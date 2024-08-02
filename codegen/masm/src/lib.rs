#![feature(array_windows)]
#![feature(iter_array_chunks)]
#![feature(is_sorted)]

mod codegen;
mod convert;
mod emulator;
mod masm;
#[cfg(test)]
mod tests;

use midenc_hir::{
    self as hir,
    diagnostics::Report,
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

pub type CompilerResult<T> = Result<T, Report>;

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
        for module in intrinsics::load_stdlib() {
            program.insert(Box::new(module.clone()));
        }

        // Ensure intrinsics modules are linked
        program.insert(Box::new(
            intrinsics::load("intrinsics::mem", &self.session.source_manager)
                .expect("undefined intrinsics module"),
        ));
        program.insert(Box::new(
            intrinsics::load("intrinsics::i32", &self.session.source_manager)
                .expect("undefined intrinsics module"),
        ));
        program.insert(Box::new(
            intrinsics::load("intrinsics::i64", &self.session.source_manager)
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
        if session.should_codegen() {
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
    use midenc_hir_transform as transforms;

    // If no rewrites were explicitly enabled, and conversion to Miden Assembly is,
    // then we must ensure that the basic transformation passes are applied.
    //
    // Otherwise, assume that the intent was to skip those rewrites and do not add them
    let mut rewrites = RewriteSet::default();
    if session.should_codegen() {
        rewrites.push(transforms::SplitCriticalEdges);
        rewrites.push(transforms::Treeify);
        rewrites.push(transforms::InlineBlocks);
        rewrites.push(transforms::ApplySpills);
    }

    rewrites
}
