mod emulator;
mod masm;
mod stackify;
#[cfg(test)]
mod tests;

pub use self::emulator::{Breakpoint, DebugInfo, EmulationError, Emulator};
pub use self::masm::*;
pub use self::stackify::Stackify;

use miden_diagnostics::DiagnosticsHandler;
use miden_hir as hir;
use miden_hir_analysis::FunctionAnalysis;

/// This struct implements a compiler pass that emits an intermediate form
/// of Miden Assembly corresponding to a given [miden_hir::Program].
///
/// This pass assumes that the input [miden_hir::Program] was constructed
/// and linked without running any transformation passes, i.e. it assumes
/// that it must do so. Running this pass when those transformations have
/// already been applied shouldn't be a problem, however it is quite expensive
/// to run many of those passes, so it should be avoided.
///
/// The resulting [Program] in MASM IR can then be used to:
///
/// * Emit textual Miden Assembly via the [core::fmt::Display] trait
/// * Execute the program or a function via [Emulator]
pub struct HirToMasm<'a> {
    diagnostics: &'a DiagnosticsHandler,
}
impl<'p> miden_hir_pass::Pass for HirToMasm<'p> {
    type Input<'a> = &'a mut hir::Program;
    type Output<'a> = Program;
    type Error = anyhow::Error;

    fn run<'a>(&mut self, input: Self::Input<'a>) -> Result<Self::Output<'a>, Self::Error> {
        let mut compiler = MasmCompiler::new(self.diagnostics);
        compiler.compile(input)
    }
}

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
    diagnostics: &'a DiagnosticsHandler,
}
impl<'a> MasmCompiler<'a> {
    pub fn new(diagnostics: &'a DiagnosticsHandler) -> Self {
        Self { diagnostics }
    }

    /// Compile an [hir::Program] that has been linked and is ready to be compiled.
    pub fn compile(&mut self, input: &mut hir::Program) -> anyhow::Result<Program> {
        ProgramCompiler::new(input, self.diagnostics).compile()
    }

    /// Compile a single [hir::Module] as a program.
    pub fn compile_module(&mut self, input: Box<hir::Module>) -> anyhow::Result<Program> {
        let mut program = hir::ProgramBuilder::new(self.diagnostics)
            .with_module(input)?
            .link()?;

        self.compile(&mut program)
    }

    /// Compile a set of [hir::Module] as a program.
    pub fn compile_modules<I: IntoIterator<Item = Box<hir::Module>>>(
        &mut self,
        input: I,
    ) -> anyhow::Result<Program> {
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
    diagnostics: &'a DiagnosticsHandler,
    input: &'a mut hir::Program,
    output: Program,
}
impl<'a> ProgramCompiler<'a> {
    pub fn new(input: &'a mut hir::Program, diagnostics: &'a DiagnosticsHandler) -> Self {
        let output = Program::from(input as &hir::Program);
        Self {
            diagnostics,
            input,
            output,
        }
    }

    pub fn compile(mut self) -> anyhow::Result<Program> {
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

        // Ensure intrinsics modules are linked
        self.output
            .modules
            .push(Module::load_intrinsic("intrinsics::mem").expect("parsing failed"));
        self.output
            .modules
            .push(Module::load_intrinsic("intrinsics::i32").expect("parsing failed"));

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

        // If this module makes use of any intrinsics modules, add them to the program
        for import in output
            .imports
            .iter()
            .filter(|import| import.name.as_str().starts_with("intrinsics::"))
        {
            if self.output.contains(import.name) {
                continue;
            }
            match Module::load_intrinsic(import.name.as_str()) {
                Some(loaded) => {
                    self.output.modules.push(loaded);
                }
                None => unimplemented!("unrecognized intrinsic module: '{}'", &import.name),
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
