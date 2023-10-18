use miden_hir::{
    self,
    testing::{self, TestContext},
    AbiParam, Felt, FieldElement, Immediate, InstBuilder, OperandStack, ProgramBuilder, Signature,
    SourceSpan, Stack, StarkField, Type,
};
use miden_hir_analysis::FunctionAnalysis;
use std::fmt::Write;

use super::*;

#[cfg(test)]
#[allow(unused_macros)]
macro_rules! assert_masm_output {
    ($lhs:expr, $rhs:expr) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        if lhs != rhs {
            panic!(
                r#"
assertion failed: `(left matches right)`
left: `{}`,
right: `{}`"#,
                lhs, rhs
            );
        }
    }};
}

#[derive(Default)]
struct TestByEmulationHarness {
    context: TestContext,
    emulator: Emulator,
}
impl TestByEmulationHarness {
    #[allow(unused)]
    pub fn with_emulator_config(memory_size: usize, hp: usize, lp: usize) -> Self {
        Self {
            context: TestContext::default(),
            emulator: Emulator::new(
                memory_size.try_into().expect("invalid memory size"),
                hp.try_into().expect("invalid address"),
                lp.try_into().expect("invalid address"),
            ),
        }
    }

    pub fn apply_rewrite_passes(
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

    pub fn stackify(
        &self,
        program: &hir::Program,
        function: &mut hir::Function,
    ) -> anyhow::Result<Box<Function>> {
        use miden_hir_pass::Pass;

        // Analyze function
        let mut analysis = FunctionAnalysis::new(function);

        // Apply pre-codegen transformations
        self.apply_rewrite_passes(function, &mut analysis)?;

        println!("{}", function);

        // Make sure all analyses are available
        analysis.ensure_all(&function);

        // Run stackification
        let mut pass = Stackify::new(program, &analysis);
        pass.run(function)
    }

    pub fn set_cycle_budget(&mut self, budget: usize) {
        self.emulator.set_max_cycles(budget);
    }

    #[allow(unused)]
    pub fn set_breakpoint(&mut self, bp: Breakpoint) {
        self.emulator.set_breakpoint(bp);
    }

    #[allow(unused)]
    pub fn clear_breakpoint(&mut self) {
        self.emulator.clear_breakpoint();
    }

    #[allow(unused)]
    pub fn step(&mut self) {
        match self.resume() {
            Ok(_) | Err(EmulationError::BreakpointHit) => return,
            Err(other) => panic!("unexpected emulation error: {other}"),
        }
    }

    #[allow(unused)]
    pub fn step_with_info(&mut self) {
        match self.emulator.resume() {
            Ok(_) | Err(EmulationError::BreakpointHit) => {
                dbg!(self.emulator.info());
            }
            Err(other) => panic!("unexpected emulation error: {other}"),
        }
    }

    #[allow(unused)]
    pub fn resume(&mut self) -> Result<OperandStack<Felt>, EmulationError> {
        self.emulator.resume()
    }

    pub fn malloc(&mut self, size: usize) -> u32 {
        self.emulator.malloc(size)
    }

    #[inline(always)]
    pub fn store(&mut self, addr: usize, value: Felt) {
        self.emulator.store(addr, value);
    }

    #[allow(unused)]
    pub fn execute(
        &mut self,
        program: Program,
        args: &[Felt],
    ) -> Result<OperandStack<Felt>, EmulationError> {
        let entrypoint = program.entrypoint.expect("cannot execute a library");
        self.emulator
            .load_program(program)
            .expect("failed to load program");
        self.emulator.invoke(entrypoint, args)
    }

    pub fn execute_module(
        &mut self,
        module: Module,
        args: &[Felt],
    ) -> Result<OperandStack<Felt>, EmulationError> {
        let entrypoint = module.entry.expect("cannot execute a library");
        self.emulator
            .load_module(module)
            .expect("failed to load module");
        self.emulator.invoke(entrypoint, args)
    }
}

/// Test the emulator on the fibonacci function
#[test]
fn fib_emulator() {
    let mut harness = TestByEmulationHarness::default();

    // Build a simple program
    let mut builder = ProgramBuilder::new(&harness.context.diagnostics);

    // Build test module with fib function
    let mut mb = builder.module("test");
    let id = testing::fib1(mb.as_mut(), &harness.context);
    mb.build()
        .expect("unexpected error constructing test module");

    // Link the program
    let mut program = builder
        .with_entrypoint("test::fib".parse().unwrap())
        .link()
        .expect("failed to link program");

    // Get the fib function
    let (mut function, imports) = {
        let modules = program.modules_mut();
        let mut test = modules.find_mut("test").remove().expect("undefined module");
        let function = test
            .cursor_mut_at(id.function)
            .remove()
            .expect("undefined function");
        let imports = test.imports();
        modules.insert(test);
        (function, imports)
    };

    let masm = harness
        .stackify(&program, &mut function)
        .expect("stackification failed");

    let mut output = String::with_capacity(1024);
    write!(&mut output, "{}", masm.display(&imports)).expect("formatting failed");

    println!("{}", output.as_str());

    let mut module = Module::new(id.module);
    module.functions.push_back(masm);
    module.entry = Some(id);

    // Test it via the emulator
    let n = Felt::new(10);
    let mut stack = harness
        .execute_module(module, &[n])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop().map(|e| e.as_int()), Some(55));
}

/// Test the [Stackify] pass on a very simple program with a conditional as a sanity check
#[test]
fn stackify_fundamental_if() {
    let mut harness = TestByEmulationHarness::default();

    // Build a simple program
    let mut builder = ProgramBuilder::new(&harness.context.diagnostics);

    // Build test module with function that adds two numbers if the
    // first number is odd, and multiplies them if the first number is even
    let mut mb = builder.module("test");
    let id = {
        let mut fb = mb
            .function(
                "add_odd_mul_even",
                Signature::new(
                    [AbiParam::new(Type::U32), AbiParam::new(Type::U32)],
                    [AbiParam::new(Type::U32)],
                ),
            )
            .expect("unexpected symbol conflict");
        let entry = fb.current_block();
        let (a, b) = {
            let args = fb.block_params(entry);
            (args[0], args[1])
        };
        let is_odd_blk = fb.create_block();
        let is_even_blk = fb.create_block();
        let is_odd = fb.ins().is_odd(a, SourceSpan::UNKNOWN);
        fb.ins().cond_br(
            is_odd,
            is_odd_blk,
            &[],
            is_even_blk,
            &[],
            SourceSpan::UNKNOWN,
        );
        fb.switch_to_block(is_odd_blk);
        fb.ins().add(a, b, SourceSpan::UNKNOWN);
        fb.switch_to_block(is_even_blk);
        fb.ins().mul(a, b, SourceSpan::UNKNOWN);
        fb.build().expect("unexpected error building function")
    };

    mb.build()
        .expect("unexpected error constructing test module");

    // Link the program
    let mut program = builder
        .with_entrypoint(id)
        .link()
        .expect("failed to link program");

    // Get the sum_matrix function
    let mut function = {
        let modules = program.modules_mut();
        let mut test = modules.find_mut("test").remove().expect("undefined module");
        let function = test
            .cursor_mut_at(id.function)
            .remove()
            .expect("undefined function");
        modules.insert(test);
        function
    };

    let masm = harness
        .stackify(&program, &mut function)
        .expect("stackification failed");

    let mut module = Module::new(id.module);
    module.functions.push_back(masm);
    module.entry = Some(id);

    let a = Felt::new(3);
    let b = Felt::new(4);

    let mut stack = harness
        .execute_module(module, &[a, b])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop().map(|e| e.as_int()), Some(12));
}

/// Test the [Stackify] pass on a very simple program with a loop as a sanity check
#[test]
fn stackify_fundamental_loops() {
    let mut harness = TestByEmulationHarness::default();

    // Build a simple program
    let mut builder = ProgramBuilder::new(&harness.context.diagnostics);

    // Build test module with function that increments a number until a
    // given iteration count is reached
    let mut mb = builder.module("test");
    let id = {
        let mut fb = mb
            .function(
                "incr_until",
                Signature::new(
                    [AbiParam::new(Type::U32), AbiParam::new(Type::U32)],
                    [AbiParam::new(Type::U32)],
                ),
            )
            .expect("unexpected symbol conflict");
        let entry = fb.current_block();
        let (a, n) = {
            let args = fb.block_params(entry);
            (args[0], args[1])
        };
        let loop_header_blk = fb.create_block();
        let a1 = fb.append_block_param(loop_header_blk, Type::U32, SourceSpan::UNKNOWN);
        let n1 = fb.append_block_param(loop_header_blk, Type::U32, SourceSpan::UNKNOWN);
        let loop_body_blk = fb.create_block();
        let loop_exit_blk = fb.create_block();
        let result0 = fb.append_block_param(loop_exit_blk, Type::U32, SourceSpan::UNKNOWN);
        fb.ins().br(loop_header_blk, &[a, n], SourceSpan::UNKNOWN);

        fb.switch_to_block(loop_header_blk);
        let is_zero = fb.ins().eq_imm(n1, Immediate::U32(0), SourceSpan::UNKNOWN);
        fb.ins().cond_br(
            is_zero,
            loop_exit_blk,
            &[a1],
            loop_body_blk,
            &[],
            SourceSpan::UNKNOWN,
        );

        fb.switch_to_block(loop_body_blk);
        let a2 = fb.ins().incr(a1, SourceSpan::UNKNOWN);
        let n2 = fb.ins().sub_imm(n1, Immediate::U32(1), SourceSpan::UNKNOWN);
        fb.ins().br(loop_header_blk, &[a2, n2], SourceSpan::UNKNOWN);

        fb.switch_to_block(loop_exit_blk);
        fb.ins().ret(Some(result0), SourceSpan::UNKNOWN);

        fb.build().expect("unexpected error building function")
    };

    mb.build()
        .expect("unexpected error constructing test module");

    // Link the program
    let mut program = builder
        .with_entrypoint(id)
        .link()
        .expect("failed to link program");

    // Get the sum_matrix function
    let (mut function, imports) = {
        let modules = program.modules_mut();
        let mut test = modules.find_mut("test").remove().expect("undefined module");
        let function = test
            .cursor_mut_at(id.function)
            .remove()
            .expect("undefined function");
        let imports = test.imports();
        modules.insert(test);
        (function, imports)
    };

    let masm = harness
        .stackify(&program, &mut function)
        .expect("stackification failed");

    let mut output = String::with_capacity(1024);
    write!(&mut output, "{}", masm.display(&imports)).expect("formatting failed");

    println!("{}", output.as_str());

    let mut module = Module::new(id.module);
    module.functions.push_back(masm);
    module.entry = Some(id);

    let a = Felt::new(3);
    let n = Felt::new(4);

    let mut stack = harness
        .execute_module(module, &[a, n])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop().map(|e| e.as_int()), Some(7));
}

/// Test the [Stackify] pass on a simple program containing [testing::sum_matrix].
#[test]
fn verify_i32_intrinsics_syntax() {
    let mut harness = TestByEmulationHarness::default();

    harness
        .emulator
        .load_module(Module::i32_intrinsics())
        .expect("failed to load intrinsics::i32");
}

/// Test the [Stackify] pass on a simple program containing [testing::sum_matrix].
#[test]
fn stackify_sum_matrix() {
    let mut harness = TestByEmulationHarness::default();

    harness
        .emulator
        .load_module(Module::mem_intrinsics())
        .expect("failed to load intrinsics::mem");

    // Build a simple program
    let mut builder = ProgramBuilder::new(&harness.context.diagnostics);

    // Build test module with fib function
    let mut mb = builder.module("test");
    let id = testing::sum_matrix(mb.as_mut(), &harness.context);
    mb.build()
        .expect("unexpected error constructing test module");

    // Link the program
    let mut program = builder
        .with_entrypoint("test::sum_matrix".parse().unwrap())
        .link()
        .expect("failed to link program");

    // Get the sum_matrix function
    let (mut function, _imports) = {
        let modules = program.modules_mut();
        let mut test = modules.find_mut("test").remove().expect("undefined module");
        let function = test
            .cursor_mut_at(id.function)
            .remove()
            .expect("undefined function");
        let imports = test.imports();
        modules.insert(test);
        (function, imports)
    };

    let masm = harness
        .stackify(&program, &mut function)
        .expect("stackification failed");

    let mut module = Module::new(id.module);
    module.functions.push_back(masm);
    module.entry = Some(id);

    let addr = harness.malloc(core::mem::size_of::<u32>() * 3 * 3);
    let ptr = Felt::new(addr as u64);
    let rows = Felt::new(3);
    let cols = Felt::new(3);

    // [1, 0, 1,
    //  0, 1, 0,
    //  1, 1, 1] == 6
    let addr = addr as usize;
    harness.store(addr, Felt::ONE);
    harness.store(addr + 4, Felt::ZERO);
    harness.store(addr + 8, Felt::ONE);
    harness.store(addr + 12, Felt::ZERO);
    harness.store(addr + 16, Felt::ONE);
    harness.store(addr + 20, Felt::ZERO);
    harness.store(addr + 24, Felt::ONE);
    harness.store(addr + 28, Felt::ONE);
    harness.store(addr + 32, Felt::ONE);

    harness.set_cycle_budget(1000);

    let mut stack = harness
        .execute_module(module, &[ptr, rows, cols])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop().map(|e| e.as_int()), Some(6));
}
