mod emulator;
mod masm;
mod stackify;
#[cfg(test)]
mod tests;

pub use self::emulator::{Breakpoint, DebugInfo, EmulationError, Emulator, InstructionPointer};
pub use self::masm::*;
pub use self::stackify::Stackify;

use miden_hir as hir;
use midenc_session::Session;

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
/// The [MasmCompiler] is designed to compile a [miden_hir::Program]
///
/// can be used to take a linked [miden_hir::Program] and
/// compile it to MASM IR, an intermediate representation of Miden Assembly
/// used within the compiler.
pub struct MasmCompiler<'a> {
    session: &'a Session,
}
impl<'a> MasmCompiler<'a> {
    pub fn new(session: &'a Session) -> Self {
        Self { session }
    }

    /// Compile an [hir::Program] that has been linked and is ready to be compiled.
    pub fn compile(&mut self, input: &mut hir::Program) -> CompilerResult<Box<Program>> {
        ProgramCompiler::new(input).compile(&self.session)
    }

    /// Compile a single [hir::Module] as a program.
    pub fn compile_module(&mut self, input: Box<hir::Module>) -> CompilerResult<Box<Program>> {
        let mut program = hir::ProgramBuilder::new(&self.session.diagnostics)
            .with_module(input)?
            .link()?;

        self.compile(&mut program)
    }

    /// Compile a set of [hir::Module] as a program.
    pub fn compile_modules<I: IntoIterator<Item = Box<hir::Module>>>(
        &mut self,
        input: I,
    ) -> CompilerResult<Box<Program>> {
        let mut builder = hir::ProgramBuilder::new(&self.session.diagnostics);
        for module in input.into_iter() {
            builder.add_module(module)?;
        }

        let mut program = builder.link()?;

        self.compile(&mut program)
    }
}

struct ProgramCompiler<'a> {
    input: &'a mut hir::Program,
    output: Box<Program>,
}
impl<'a> ProgramCompiler<'a> {
    pub fn new(input: &'a mut hir::Program) -> Self {
        let output = Box::new(Program::from(input as &hir::Program));
        Self { input, output }
    }

    pub fn compile(mut self, session: &Session) -> CompilerResult<Box<Program>> {
        // Remove the set of modules to compile from the program
        let mut modules = self.input.modules_mut().take();

        // For each module in that set:
        //
        // 1. Construct a MASM IR module corresponding to it
        // 2. Rewrite, then compile all functions in the HIR module
        // 3. Put the input module back in the Program
        // 4. Add the compiled MASM IR module to the MASM IR program
        while let Some(mut module) = modules.front_mut().remove() {
            let masm_module = self.compile_module(&mut module, session)?;
            self.input.modules_mut().insert(module);
            self.output.modules.push(masm_module);
        }

        Ok(self.output)
    }

    /// Compile a single [miden_hir::Module] from the current program being compiled.
    ///
    /// For each function, we perform the following steps:
    ///
    /// 1. Detach the function from the module so we can obtain a mutable reference
    /// 2. Construct the function analysis data structure
    /// 3. Prepare the function by applying the rewrite pipeline
    /// 4. Re-attach the function to the module
    /// 5. Run the stackification pass to lower to MASM IR
    /// 6. Add the MASM IR function to the MASM IR module
    ///
    /// Once all functions are compiled from this module, the MASM IR module itself is returned.
    fn compile_module(
        &mut self,
        module: &mut hir::Module,
        session: &Session,
    ) -> CompilerResult<Module> {
        use miden_hir::{
            pass::{Analysis, AnalysisManager, ConversionPass, RewritePass},
            ProgramAnalysisKey,
        };
        use miden_hir_analysis as analysis;
        use miden_hir_transform as transform;

        let mut output = Module::new(module.name);

        // Compute import information for this module
        output.imports = module.imports();

        // If this module contains the program entrypoint, handle that here
        if let Some(entry) = self.input.entrypoint() {
            if entry.module == module.name {
                output.entry = Some(entry);
            }
        }

        // Create new program-wide analysis manager
        let mut analyses = AnalysisManager::new();
        // Register program-wide analyses
        let global_analysis =
            analysis::GlobalVariableAnalysis::analyze(&self.input, &mut analyses, session)?;
        analyses.insert(ProgramAnalysisKey, global_analysis);

        // Removing a function via this cursor will move the cursor to
        // the next function in the module. Once the end of the module
        // is reached, the cursor will point to the null object, and
        // `remove` will return `None`.
        let mut cursor = module.cursor_mut();
        while let Some(mut function) = cursor.remove() {
            // Apply rewrites
            let mut rewrites = transform::SplitCriticalEdges
                .chain(transform::Treeify)
                .chain(transform::InlineBlocks);
            rewrites.apply(&mut function, &mut analyses, session)?;
            // Add the function back to the module
            //
            // We add it before the current position of the cursor
            // to ensure that we don't interfere with our traversal
            // of the module top to bottom
            cursor.insert_before(function);
            // Get a reference to the function again
            //
            // Rather than move the cursor, we use the ability to
            // get a temporary read-only cursor to the previous item
            let function = cursor.peek_prev().get().unwrap();
            // Lower the function
            let mut stackify = Stackify;
            let masm_function = stackify.convert(function, &mut analyses, session)?;
            // Attach to MASM module
            output.functions.push_back(Box::new(masm_function));
        }

        Ok(output)
    }
}
