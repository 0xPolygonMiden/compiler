use miden_hir::{
    self,
    pass::{AnalysisManager, ConversionPass},
    testing::{self, TestContext},
    AbiParam, Felt, FieldElement, FunctionIdent, Immediate, InstBuilder, OperandStack,
    ProgramBuilder, Signature, SourceSpan, Stack, StarkField, Type,
};
use std::sync::Arc;

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
        let mut harness = Self {
            context: TestContext::default(),
            emulator: Emulator::new(
                memory_size.try_into().expect("invalid memory size"),
                hp.try_into().expect("invalid address"),
                lp.try_into().expect("invalid address"),
            ),
        };
        harness.set_cycle_budget(2000);
        harness
    }

    pub fn stackify(
        &self,
        program: &hir::Program,
        function: &mut hir::Function,
    ) -> CompilerResult<Box<Function>> {
        use miden_hir::{
            pass::{Analysis, RewritePass},
            ProgramAnalysisKey,
        };
        use miden_hir_analysis as analysis;
        use miden_hir_transform as transform;

        // Analyze function
        let mut analyses = AnalysisManager::new();

        // Register program-wide analyses
        let global_analysis = analysis::GlobalVariableAnalysis::<hir::Program>::analyze(
            program,
            &mut analyses,
            &self.context.session,
        )?;
        analyses.insert(ProgramAnalysisKey, global_analysis);

        // Apply pre-codegen transformations
        let mut rewrites = transform::SplitCriticalEdges
            .chain(transform::Treeify)
            .chain(transform::InlineBlocks);
        rewrites.apply(function, &mut analyses, &self.context.session)?;

        println!("{}", function);

        // Run stackification
        let mut convert_to_masm = ConvertHirToMasm::<&hir::Function>::default();
        convert_to_masm
            .convert(function, &mut analyses, &self.context.session)
            .map(Box::new)
            .map_err(CompilerError::Conversion)
    }

    pub fn set_cycle_budget(&mut self, budget: usize) {
        self.emulator.set_max_cycles(budget);
    }

    #[allow(unused)]
    pub fn set_breakpoint(&mut self, bp: Breakpoint) {
        self.emulator.set_breakpoint(bp);
    }

    #[allow(unused)]
    pub fn clear_breakpoint(&mut self, bp: Breakpoint) {
        self.emulator.clear_breakpoint(bp);
    }

    #[allow(unused)]
    pub fn resume_until_break(&mut self) {
        match self.emulator.resume() {
            Ok(_) | Err(EmulationError::BreakpointHit(_)) => return,
            Err(other) => panic!("unexpected emulation error: {other}"),
        }
    }

    #[allow(unused)]
    pub fn resume_until_break_with_info(&mut self) {
        match self.emulator.resume() {
            Ok(_) | Err(EmulationError::BreakpointHit(_)) => {
                dbg!(self.emulator.info());
            }
            Err(other) => panic!("unexpected emulation error: {other}"),
        }
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
        program: Arc<Program>,
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
        module: Arc<Module>,
        args: &[Felt],
    ) -> Result<OperandStack<Felt>, EmulationError> {
        let entrypoint = module.entry.expect("cannot execute a library");
        self.emulator
            .load_module(module)
            .expect("failed to load module");
        self.emulator.invoke(entrypoint, args)
    }

    pub fn execute_program(
        &mut self,
        program: Arc<Program>,
        args: &[Felt],
    ) -> Result<OperandStack<Felt>, EmulationError> {
        let entrypoint = program.entrypoint.expect("cannot execute a library");
        self.emulator
            .load_program(program)
            .expect("failed to load program");
        self.emulator.invoke(entrypoint, args)
    }

    #[inline]
    pub fn invoke(
        &mut self,
        entrypoint: FunctionIdent,
        args: &[Felt],
    ) -> Result<OperandStack<Felt>, EmulationError> {
        self.emulator.invoke(entrypoint, args)
    }

    #[inline]
    pub fn enter(&mut self, entrypoint: FunctionIdent, args: &[Felt]) {
        match self.emulator.enter(entrypoint, args) {
            Ok(_) | Err(EmulationError::BreakpointHit(_)) => {
                dbg!(self.emulator.info());
            }
            Err(other) => panic!("unexpected emulation error: {other}"),
        }
    }

    pub fn step_with_info(&mut self) {
        match self.emulator.step() {
            Ok(_) | Err(EmulationError::BreakpointHit(_)) => {
                dbg!(self.emulator.info());
            }
            Err(other) => panic!("unexpected emulation error: {other}"),
        }
    }

    #[inline]
    pub fn step(&mut self) -> Result<EmulatorEvent, EmulationError> {
        self.emulator.step()
    }

    #[inline]
    pub fn step_over(&mut self) -> Result<EmulatorEvent, EmulationError> {
        self.emulator.step_over()
    }
}

/// Test the emulator on the fibonacci function
#[test]
fn fib_emulator() {
    let mut harness = TestByEmulationHarness::default();

    // Build a simple program
    let mut builder = ProgramBuilder::new(&harness.context.session.diagnostics);

    // Build test module with fib function
    let mut mb = builder.module("test");
    testing::fib1(mb.as_mut(), &harness.context);
    mb.build()
        .expect("unexpected error constructing test module");

    // Link the program
    let program = builder
        .with_entrypoint("test::fib".parse().unwrap())
        .link()
        .expect("failed to link program");

    let mut compiler = MasmCompiler::new(&harness.context.session);
    let program = compiler.compile(program).expect("compilation failed");

    println!("{}", program.get("test").unwrap());

    // Test it via the emulator
    let n = Felt::new(10);
    let mut stack = harness
        .execute_program(program.freeze(), &[n])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop().map(|e| e.as_int()), Some(55));
}

/// Test the [Stackify] pass on a very simple program with a conditional as a sanity check
#[test]
fn stackify_fundamental_if() {
    let mut harness = TestByEmulationHarness::default();

    // Build a simple program
    let mut builder = ProgramBuilder::new(&harness.context.session.diagnostics);

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
        fb.ins().add_checked(a, b, SourceSpan::UNKNOWN);
        fb.switch_to_block(is_even_blk);
        fb.ins().mul_checked(a, b, SourceSpan::UNKNOWN);
        fb.build().expect("unexpected error building function")
    };

    mb.build()
        .expect("unexpected error constructing test module");

    // Link the program
    let program = builder
        .with_entrypoint(id)
        .link()
        .expect("failed to link program");

    let mut compiler = MasmCompiler::new(&harness.context.session);
    let program = compiler.compile(program).expect("compilation failed");

    let a = Felt::new(3);
    let b = Felt::new(4);

    let mut stack = harness
        .execute_program(program.freeze(), &[a, b])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop().map(|e| e.as_int()), Some(12));
}

/// Test the [Stackify] pass on a very simple program with a loop as a sanity check
#[test]
fn stackify_fundamental_loops() {
    let mut harness = TestByEmulationHarness::default();

    // Build a simple program
    let mut builder = ProgramBuilder::new(&harness.context.session.diagnostics);

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
        let a2 = fb.ins().incr_checked(a1, SourceSpan::UNKNOWN);
        let n2 = fb
            .ins()
            .sub_imm_checked(n1, Immediate::U32(1), SourceSpan::UNKNOWN);
        fb.ins().br(loop_header_blk, &[a2, n2], SourceSpan::UNKNOWN);

        fb.switch_to_block(loop_exit_blk);
        fb.ins().ret(Some(result0), SourceSpan::UNKNOWN);

        fb.build().expect("unexpected error building function")
    };

    mb.build()
        .expect("unexpected error constructing test module");

    // Link the program
    let program = builder
        .with_entrypoint(id)
        .link()
        .expect("failed to link program");

    let mut compiler = MasmCompiler::new(&harness.context.session);
    let program = compiler.compile(program).expect("compilation failed");

    let a = Felt::new(3);
    let n = Felt::new(4);

    let mut stack = harness
        .execute_program(program.freeze(), &[a, n])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop().map(|e| e.as_int()), Some(7));
}

/// Test the [Stackify] pass on a simple program containing [testing::sum_matrix].
#[test]
fn stackify_sum_matrix() {
    let mut harness = TestByEmulationHarness::default();

    // Build a simple program
    let mut builder = ProgramBuilder::new(&harness.context.session.diagnostics);

    // Build test module with fib function
    let mut mb = builder.module("test");
    testing::sum_matrix(mb.as_mut(), &harness.context);
    mb.build()
        .expect("unexpected error constructing test module");

    // Link the program
    let program = builder
        .with_entrypoint("test::sum_matrix".parse().unwrap())
        .link()
        .expect("failed to link program");

    // Compile
    let mut compiler = MasmCompiler::new(&harness.context.session);
    let program = compiler.compile(program).expect("compilation failed");

    // Prep emulator
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

    // Execute test::sum_matrix
    let mut stack = harness
        .execute_program(program.freeze(), &[ptr, rows, cols])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop().map(|e| e.as_int()), Some(6));
}

#[test]
fn i32_icmp() {
    let mut harness = TestByEmulationHarness::default();

    harness
        .emulator
        .load_module(
            Box::new(
                Module::load_intrinsic("intrinsics::i32", &harness.context.session.codemap)
                    .expect("parsing failed"),
            )
            .freeze(),
        )
        .expect("failed to load intrinsics::i32");

    let zero = Felt::new(0i32 as u32 as u64);
    let neg_one = Felt::new(-1i32 as u32 as u64);
    let one = Felt::new(1i32 as u32 as u64);
    let max = Felt::new(i32::MAX as u32 as u64);
    let min = Felt::new(i32::MIN as u32 as u64);

    // NOTE: arguments are passed in reverse, i.e. [b, a] not [a, b]
    let icmp = "intrinsics::i32::icmp".parse().unwrap();
    // 0.cmp(1)
    let mut stack = harness
        .invoke(icmp, &[one, zero])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(neg_one));

    // Reset emulator
    harness.emulator.stop();
    // 1.cmp(0)
    let mut stack = harness
        .invoke(icmp, &[zero, one])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(one));

    harness.emulator.stop();
    // 0.cmp(0)
    let mut stack = harness
        .invoke(icmp, &[zero, zero])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(zero));

    harness.emulator.stop();
    // -1.cmp(0)
    let mut stack = harness
        .invoke(icmp, &[zero, neg_one])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(neg_one));

    harness.emulator.stop();
    // 0.cmp(-1)
    let mut stack = harness
        .invoke(icmp, &[neg_one, zero])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(one));

    harness.emulator.stop();
    // -1.cmp(i32::MAX)
    let mut stack = harness
        .invoke(icmp, &[max, neg_one])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(neg_one));

    harness.emulator.stop();
    // i32::MAX.cmp(-1)
    let mut stack = harness
        .invoke(icmp, &[neg_one, max])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(one));

    harness.emulator.stop();
    // -1.cmp(i32::MIN)
    let mut stack = harness
        .invoke(icmp, &[min, neg_one])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(one));

    harness.emulator.stop();
    // i32::MIN.cmp(-1)
    let mut stack = harness
        .invoke(icmp, &[neg_one, min])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(neg_one));

    harness.emulator.stop();
    // i32::MIN.cmp(i32::MIN)
    let mut stack = harness.invoke(icmp, &[min, min]).expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(zero));

    harness.emulator.stop();
    // i32::MAX.cmp(i32::MIN)
    let mut stack = harness.invoke(icmp, &[min, max]).expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(one));

    harness.emulator.stop();
    // i32::MAX.cmp(1)
    let mut stack = harness.invoke(icmp, &[one, max]).expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(one));
}

#[test]
fn i32_is_gt() {
    let mut harness = TestByEmulationHarness::default();

    harness
        .emulator
        .load_module(
            Box::new(
                Module::load_intrinsic("intrinsics::i32", &harness.context.session.codemap)
                    .expect("parsing failed"),
            )
            .freeze(),
        )
        .expect("failed to load intrinsics::i32");

    let zero = Felt::new(0i32 as u32 as u64);
    let neg_one = Felt::new(-1i32 as u32 as u64);
    let one = Felt::new(1i32 as u32 as u64);
    let max = Felt::new(i32::MAX as u32 as u64);
    let min = Felt::new(i32::MIN as u32 as u64);

    // NOTE: arguments are passed in reverse, i.e. [b, a] not [a, b]
    let is_gt = "intrinsics::i32::is_gt".parse().unwrap();
    // 0 > 1
    let mut stack = harness
        .invoke(is_gt, &[one, zero])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(zero));

    harness.emulator.stop();
    // 1 > 0
    let mut stack = harness
        .invoke(is_gt, &[zero, one])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(one));

    harness.emulator.stop();
    // 1 > 1
    let mut stack = harness
        .invoke(is_gt, &[one, one])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(zero));

    harness.emulator.stop();
    // 1 > -1
    let mut stack = harness
        .invoke(is_gt, &[neg_one, one])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(one));

    harness.emulator.stop();
    // i32::MAX > 1
    let mut stack = harness
        .invoke(is_gt, &[one, max])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(one));

    harness.emulator.stop();
    // 1 > i32::MIN
    let mut stack = harness
        .invoke(is_gt, &[min, one])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(one));
}

#[test]
fn i32_is_lt() {
    let mut harness = TestByEmulationHarness::default();

    harness
        .emulator
        .load_module(
            Box::new(
                Module::load_intrinsic("intrinsics::i32", &harness.context.session.codemap)
                    .expect("parsing failed"),
            )
            .freeze(),
        )
        .expect("failed to load intrinsics::i32");

    let zero = Felt::new(0i32 as u32 as u64);
    let neg_one = Felt::new(-1i32 as u32 as u64);
    let one = Felt::new(1i32 as u32 as u64);
    let max = Felt::new(i32::MAX as u32 as u64);
    let min = Felt::new(i32::MIN as u32 as u64);

    // NOTE: arguments are passed in reverse, i.e. [b, a] not [a, b]
    let is_lt = "intrinsics::i32::is_lt".parse().unwrap();
    // 0 < 1
    let mut stack = harness
        .invoke(is_lt, &[one, zero])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(one));

    harness.emulator.stop();
    // 1 < 0
    let mut stack = harness
        .invoke(is_lt, &[zero, one])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(zero));

    harness.emulator.stop();
    // 1 < 1
    let mut stack = harness
        .invoke(is_lt, &[one, one])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(zero));

    harness.emulator.stop();
    // 1 < -1
    let mut stack = harness
        .invoke(is_lt, &[neg_one, one])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(zero));

    harness.emulator.stop();
    // i32::MAX < 1
    let mut stack = harness
        .invoke(is_lt, &[one, max])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(zero));

    harness.emulator.stop();
    // 1 < i32::MIN
    let mut stack = harness
        .invoke(is_lt, &[min, one])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(zero));

    harness.emulator.stop();
    // -1 < i32::MIN
    let mut stack = harness
        .invoke(is_lt, &[min, neg_one])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(zero));

    harness.emulator.stop();
    // i32::MIN < -1
    let mut stack = harness
        .invoke(is_lt, &[neg_one, min])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(one));
}

#[test]
fn i32_is_signed() {
    let mut harness = TestByEmulationHarness::default();

    harness
        .emulator
        .load_module(
            Box::new(
                Module::load_intrinsic("intrinsics::i32", &harness.context.session.codemap)
                    .expect("parsing failed"),
            )
            .freeze(),
        )
        .expect("failed to load intrinsics::i32");

    let zero = Felt::new(0i32 as u32 as u64);
    let neg_one = Felt::new(-1i32 as u32 as u64);
    let one = Felt::new(1i32 as u32 as u64);
    let max = Felt::new(i32::MAX as u32 as u64);
    let min = Felt::new(i32::MIN as u32 as u64);

    let is_signed = "intrinsics::i32::is_signed".parse().unwrap();
    // 0
    let mut stack = harness
        .invoke(is_signed, &[zero])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(zero));

    harness.emulator.stop();
    // 1
    let mut stack = harness.invoke(is_signed, &[one]).expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(zero));

    harness.emulator.stop();
    // -1
    let mut stack = harness
        .invoke(is_signed, &[neg_one])
        .expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(one));

    harness.emulator.stop();
    // i32::MAX
    let mut stack = harness.invoke(is_signed, &[max]).expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(zero));

    harness.emulator.stop();
    // i32::MIN
    let mut stack = harness.invoke(is_signed, &[min]).expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop(), Some(one));
}

#[test]
fn i32_overflowing_add() {
    let mut harness = TestByEmulationHarness::default();

    harness
        .emulator
        .load_module(
            Box::new(
                Module::load_intrinsic("intrinsics::i32", &harness.context.session.codemap)
                    .expect("parsing failed"),
            )
            .freeze(),
        )
        .expect("failed to load intrinsics::i32");

    let zero = Felt::new(0i32 as u32 as u64);
    let neg_one = Felt::new(-1i32 as u32 as u64);
    let one = Felt::new(1i32 as u32 as u64);
    let max = Felt::new(i32::MAX as u32 as u64);
    let min = Felt::new(i32::MIN as u32 as u64);

    // NOTE: arguments are passed in reverse, i.e. [b, a] not [a, b]
    let add = "intrinsics::i32::overflowing_add".parse().unwrap();
    // 0 + 0
    let mut stack = harness
        .invoke(add, &[zero, zero])
        .expect("execution failed");
    assert_eq!(stack.len(), 2);
    assert_eq!(stack.pop(), Some(zero)); // overflowed
    assert_eq!(stack.pop(), Some(zero)); // result

    harness.emulator.stop();
    // 0 + 1
    let mut stack = harness.invoke(add, &[one, zero]).expect("execution failed");
    assert_eq!(stack.len(), 2);
    assert_eq!(stack.pop(), Some(zero));
    assert_eq!(stack.pop(), Some(one));

    harness.emulator.stop();
    // -1 + 1
    let mut stack = harness
        .invoke(add, &[one, neg_one])
        .expect("execution failed");
    assert_eq!(stack.len(), 2);
    assert_eq!(stack.pop(), Some(zero));
    assert_eq!(stack.pop(), Some(zero));

    harness.emulator.stop();
    // i32::MAX + 1
    let mut stack = harness.invoke(add, &[one, max]).expect("execution failed");
    assert_eq!(stack.len(), 2);
    assert_eq!(stack.pop(), Some(one));
    assert_eq!(stack.pop(), Some(min));

    harness.emulator.stop();
    // i32::MIN + i32::MAX
    let mut stack = harness.invoke(add, &[max, min]).expect("execution failed");
    assert_eq!(stack.len(), 2);
    assert_eq!(stack.pop(), Some(zero));
    assert_eq!(stack.pop(), Some(neg_one));

    harness.emulator.stop();
    // i32::MIN + -1
    let mut stack = harness
        .invoke(add, &[neg_one, min])
        .expect("execution failed");
    assert_eq!(stack.len(), 2);
    assert_eq!(stack.pop(), Some(one));
    assert_eq!(stack.pop(), Some(max));
}
