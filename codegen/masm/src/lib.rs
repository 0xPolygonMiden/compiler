mod emulator;
mod masm;
mod stackify;
#[cfg(test)]
mod tests;

pub use self::emulator::{Breakpoint, DebugInfo, EmulationError, Emulator, InstructionPointer};
pub use self::masm::*;
pub use self::stackify::Stackify;

use std::sync::Arc;

use miden_diagnostics::DiagnosticsHandler;
use miden_hir as hir;
use miden_hir_analysis::FunctionAnalysis;
use midenc_session::Options;

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
    options: Arc<Options>,
    diagnostics: &'a DiagnosticsHandler,
}
impl<'a> MasmCompiler<'a> {
    pub fn new(options: Arc<Options>, diagnostics: &'a DiagnosticsHandler) -> Self {
        Self {
            options,
            diagnostics,
        }
    }

    /// Compile an [hir::Program] that has been linked and is ready to be compiled.
    pub fn compile(&mut self, input: &mut hir::Program) -> anyhow::Result<Box<Program>> {
        ProgramCompiler::new(input, &self.options, self.diagnostics).compile()
    }

    /// Compile a single [hir::Module] as a program.
    pub fn compile_module(&mut self, input: Box<hir::Module>) -> anyhow::Result<Box<Program>> {
        let mut program = hir::ProgramBuilder::new(self.diagnostics)
            .with_module(input)?
            .link()?;

        self.compile(&mut program)
    }

    /// Compile a set of [hir::Module] as a program.
    pub fn compile_modules<I: IntoIterator<Item = Box<hir::Module>>>(
        &mut self,
        input: I,
    ) -> anyhow::Result<Box<Program>> {
        let mut builder = hir::ProgramBuilder::new(self.diagnostics);
        for module in input.into_iter() {
            builder.add_module(module)?;
        }

        let mut program = builder.link()?;

        self.compile(&mut program)
    }
}

struct ProgramCompiler<'a> {
    #[allow(unused)]
    options: &'a Options,
    #[allow(unused)]
    diagnostics: &'a DiagnosticsHandler,
    input: &'a mut hir::Program,
    output: Box<Program>,
}
impl<'a> ProgramCompiler<'a> {
    pub fn new(
        input: &'a mut hir::Program,
        options: &'a Options,
        diagnostics: &'a DiagnosticsHandler,
    ) -> Self {
        let output = Box::new(Program::from(input as &hir::Program));
        Self {
            options,
            diagnostics,
            input,
            output,
        }
    }

    pub fn compile(mut self) -> anyhow::Result<Box<Program>> {
        // Remove the set of modules to compile from the program
        let mut modules = self.input.modules_mut().take();

        // For each module in that set:
        //
        // 1. Construct a MASM IR module corresponding to it
        // 2. Rewrite, then compile all functions in the HIR module
        // 3. Put the input module back in the Program
        // 4. Add the compiled MASM IR module to the MASM IR program
        while let Some(mut module) = modules.front_mut().remove() {
            let masm_module = self.compile_module(&mut module)?;
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
    fn compile_module(&mut self, module: &mut hir::Module) -> anyhow::Result<Module> {
        use miden_hir_pass::Pass;

        let mut output = Module::new(module.name);

        // Compute import information for this module
        output.imports = module.imports();

        // If this module contains the program entrypoint, handle that here
        if let Some(entry) = self.input.entrypoint() {
            if entry.module == module.name {
                output.entry = Some(entry);
            }
        }

        // Removing a function via this cursor will move the cursor to
        // the next function in the module. Once the end of the module
        // is reached, the cursor will point to the null object, and
        // `remove` will return `None`.
        let mut cursor = module.cursor_mut();
        while let Some(mut function) = cursor.remove() {
            let mut analysis = FunctionAnalysis::new(&function);
            // Apply rewrites
            self.rewrite_function(&mut function, &mut analysis)?;
            // Make sure all analyses are available
            analysis.ensure_all(&function);
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
            let mut pass = Stackify::new(self.input, &analysis);
            let masm_function = pass.run(function)?;
            // Attach to MASM module
            output.functions.push_back(masm_function);
        }

        Ok(output)
    }

    fn rewrite_function(
        &self,
        function: &mut hir::Function,
        analysis: &mut FunctionAnalysis,
    ) -> anyhow::Result<()> {
        use miden_hir_transform::{self as transform, RewritePass};

        let mut rewrites = transform::SplitCriticalEdges
            .chain(transform::Treeify)
            .chain(transform::InlineBlocks);
        rewrites.run(function, analysis)
    }
}
