use std::sync::Arc;

use miden_hir::{
    pass::{AnalysisManager, ConversionPass},
    testing::{self, TestContext},
    AbiParam, Felt, FieldElement, FunctionIdent, Immediate, InstBuilder, OperandStack,
    ProgramBuilder, Signature, SourceSpan, Stack, StarkField, Type,
};
use proptest::prelude::*;

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
#[allow(unused)]
impl TestByEmulationHarness {
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

    pub fn codegen(
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
            Ok(_) | Err(EmulationError::BreakpointHit(_)) => (),
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

    pub fn execute_module(
        &mut self,
        module: Arc<Module>,
        args: &[Felt],
    ) -> Result<OperandStack<Felt>, EmulationError> {
        let entrypoint = module.entrypoint().expect("cannot execute a library");
        self.emulator.load_module(module).expect("failed to load module");
        self.emulator.invoke(entrypoint, args)
    }

    pub fn execute_program(
        &mut self,
        program: Arc<Program>,
        args: &[Felt],
    ) -> Result<OperandStack<Felt>, EmulationError> {
        self.emulator.load_program(program).expect("failed to load program");
        {
            let stack = self.emulator.stack_mut();
            for arg in args.iter().copied().rev() {
                stack.push(arg);
            }
        }
        self.emulator.start()
    }

    #[allow(unused)]
    pub fn execute_program_with_entry(
        &mut self,
        program: Arc<Program>,
        entrypoint: FunctionIdent,
        args: &[Felt],
    ) -> Result<OperandStack<Felt>, EmulationError> {
        self.emulator.load_program(program).expect("failed to load program");
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

#[test]
fn issue56() {
    let harness = TestByEmulationHarness::default();

    let mut builder = ProgramBuilder::new(&harness.context.session.diagnostics);
    let mut mb = builder.module("test");
    testing::issue56(mb.as_mut(), &harness.context);
    mb.build().unwrap();

    let program = builder.with_entrypoint("test::entrypoint".parse().unwrap()).link().unwrap();

    let mut compiler = MasmCompiler::new(&harness.context.session);
    compiler.compile(program).expect("compilation failed");
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
    mb.build().expect("unexpected error constructing test module");

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
    let mut stack = harness.execute_program(program.freeze(), &[n]).expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop().map(|e| e.as_int()), Some(55));
}

/// Test the code generator on a very simple program with a conditional as a sanity check
#[test]
fn codegen_fundamental_if() {
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
        fb.ins().cond_br(is_odd, is_odd_blk, &[], is_even_blk, &[], SourceSpan::UNKNOWN);
        fb.switch_to_block(is_odd_blk);
        let c = fb.ins().add_checked(a, b, SourceSpan::UNKNOWN);
        fb.ins().ret(Some(c), SourceSpan::UNKNOWN);
        fb.switch_to_block(is_even_blk);
        let d = fb.ins().mul_checked(a, b, SourceSpan::UNKNOWN);
        fb.ins().ret(Some(d), SourceSpan::UNKNOWN);
        fb.build().expect("unexpected error building function")
    };

    mb.build().expect("unexpected error constructing test module");

    // Link the program
    let program = builder.with_entrypoint(id).link().expect("failed to link program");

    let mut compiler = MasmCompiler::new(&harness.context.session);
    let program = compiler.compile(program).expect("compilation failed");

    let a = Felt::new(3);
    let b = Felt::new(4);

    let mut stack = harness.execute_program(program.freeze(), &[a, b]).expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop().map(|e| e.as_int()), Some(12));
}

/// Test the code generator on a very simple program with a loop as a sanity check
#[test]
fn codegen_fundamental_loops() {
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
        fb.ins()
            .cond_br(is_zero, loop_exit_blk, &[a1], loop_body_blk, &[], SourceSpan::UNKNOWN);

        fb.switch_to_block(loop_body_blk);
        let a2 = fb.ins().incr_checked(a1, SourceSpan::UNKNOWN);
        let n2 = fb.ins().sub_imm_checked(n1, Immediate::U32(1), SourceSpan::UNKNOWN);
        fb.ins().br(loop_header_blk, &[a2, n2], SourceSpan::UNKNOWN);

        fb.switch_to_block(loop_exit_blk);
        fb.ins().ret(Some(result0), SourceSpan::UNKNOWN);

        fb.build().expect("unexpected error building function")
    };

    mb.build().expect("unexpected error constructing test module");

    // Link the program
    let program = builder.with_entrypoint(id).link().expect("failed to link program");

    let mut compiler = MasmCompiler::new(&harness.context.session);
    let program = compiler.compile(program).expect("compilation failed");

    let a = Felt::new(3);
    let n = Felt::new(4);

    let mut stack = harness.execute_program(program.freeze(), &[a, n]).expect("execution failed");
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.pop().map(|e| e.as_int()), Some(7));
}

/// Test the code generator on a simple program containing [testing::sum_matrix].
#[test]
fn codegen_sum_matrix() {
    let mut harness = TestByEmulationHarness::default();

    // Build a simple program
    let mut builder = ProgramBuilder::new(&harness.context.session.diagnostics);

    // Build test module with fib function
    let mut mb = builder.module("test");
    testing::sum_matrix(mb.as_mut(), &harness.context);
    mb.build().expect("unexpected error constructing test module");

    // Link the program
    let program = builder
        .with_entrypoint("test::sum_matrix".parse().unwrap())
        .link()
        .expect("failed to link program");

    // Compile
    let mut compiler = MasmCompiler::new(&harness.context.session);
    let program = compiler.compile(program).expect("compilation failed");

    println!("{}", program.get("test").unwrap());

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
#[should_panic(expected = "assertion failed: expected false, got true")]
fn i32_checked_neg() {
    let mut harness = TestByEmulationHarness::default();

    harness
        .emulator
        .load_module(
            Box::new(
                intrinsics::load("intrinsics::i32", &harness.context.session.codemap)
                    .expect("undefined intrinsic module"),
            )
            .freeze(),
        )
        .expect("failed to load intrinsics::i32");

    let min = Felt::new(i32::MIN as u32 as u64);

    let neg = "intrinsics::i32::checked_neg".parse().unwrap();
    // i32::MIN
    harness.invoke(neg, &[min]).expect("execution failed");
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 1000, failure_persistence: None, ..Default::default() })]

    #[test]
    fn i32_icmp(a: i32, b: i32) {
        use core::cmp::Ordering;

        let mut harness = TestByEmulationHarness::default();

        harness
            .emulator
            .load_module(
                Box::new(
                    intrinsics::load("intrinsics::i32", &harness.context.session.codemap)
                        .expect("undefined intrinsics module"),
                )
                .freeze(),
            )
            .expect("failed to load intrinsics::i32");
        // NOTE: arguments are passed in reverse, i.e. [b, a] not [a, b]
        let icmp = "intrinsics::i32::icmp".parse().unwrap();
        let is_gt = "intrinsics::i32::is_gt".parse().unwrap();
        let is_lt = "intrinsics::i32::is_lt".parse().unwrap();

        let a_felt = Felt::new(a as u32 as u64);
        let b_felt = Felt::new(b as u32 as u64);
        let mut stack = harness.invoke(icmp, &[b_felt, a_felt]).expect("execution failed");
        harness.emulator.stop();
        prop_assert_eq!(stack.len(), 1);

        const NEG_ONE_U64: u64 = -1i32 as u32 as u64;
        let result = match stack.pop().unwrap().as_int() {
            NEG_ONE_U64 => Ordering::Less,
            0 => Ordering::Equal,
            1 => Ordering::Greater,
            other => panic!("invalid comparison result, expected -1, 0, or 1 but got {other}"),
        };

        prop_assert_eq!(result, a.cmp(&b));

        let mut stack = harness.invoke(is_gt, &[b_felt, a_felt]).expect("execution failed");
        harness.emulator.stop();
        prop_assert_eq!(stack.len(), 1);

        let result = match stack.pop().unwrap().as_int() {
            0 => false,
            1 => true,
            other => panic!("invalid boolean result, expected 0 or 1, got {other}"),
        };

        prop_assert_eq!(result, a.cmp(&b).is_gt());

        let mut stack = harness.invoke(is_lt, &[b_felt, a_felt]).expect("execution failed");
        harness.emulator.stop();
        prop_assert_eq!(stack.len(), 1);

        let result = match stack.pop().unwrap().as_int() {
            0 => false,
            1 => true,
            other => panic!("invalid boolean result, expected 0 or 1, got {other}"),
        };

        prop_assert_eq!(result, a.cmp(&b).is_lt());
    }

    #[test]
    fn i32_is_signed(a: i32) {
        let mut harness = TestByEmulationHarness::default();

        harness
            .emulator
            .load_module(
                Box::new(
                    intrinsics::load("intrinsics::i32", &harness.context.session.codemap)
                        .expect("undefined intrinsic module"),
                )
                .freeze(),
            )
            .expect("failed to load intrinsics::i32");

        let a_felt = Felt::new(a as u32 as u64);
        let is_signed = "intrinsics::i32::is_signed".parse().unwrap();
        let mut stack = harness.invoke(is_signed, &[a_felt]).expect("execution failed");
        prop_assert_eq!(stack.len(), 1);
        let result = stack.pop().unwrap().as_int();

        prop_assert_eq!(result, a.is_negative() as u64);
    }

    #[test]
    fn i32_overflowing_add(a: i32, b: i32) {
        let mut harness = TestByEmulationHarness::default();

        harness
            .emulator
            .load_module(
                Box::new(
                    intrinsics::load("intrinsics::i32", &harness.context.session.codemap)
                        .expect("undefined intrinsic module"),
                )
                .freeze(),
            )
            .expect("failed to load intrinsics::i32");

        let a_felt = Felt::new(a as u32 as u64);
        let b_felt = Felt::new(b as u32 as u64);
        // NOTE: arguments are passed in reverse, i.e. [b, a] not [a, b]
        let add = "intrinsics::i32::overflowing_add".parse().unwrap();
        let mut stack = harness
            .invoke(add, &[b_felt, a_felt])
            .expect("execution failed");
        prop_assert_eq!(stack.len(), 2);

        let overflowed = stack.pop().unwrap() == Felt::ONE;
        let raw_result = stack.pop().unwrap().as_int();
        let result = u32::try_from(raw_result).map_err(|_| raw_result).map(|res| res as i32);
        let (expected_result, expected_overflow) = a.overflowing_add(b);

        prop_assert_eq!((result, overflowed), (Ok(expected_result), expected_overflow));
    }

    #[test]
    fn i32_overflowing_sub(a: i32, b: i32) {
        let mut harness = TestByEmulationHarness::default();

        harness
            .emulator
            .load_module(
                Box::new(
                    intrinsics::load("intrinsics::i32", &harness.context.session.codemap)
                        .expect("undefined intrinsic module"),
                )
                .freeze(),
            )
            .expect("failed to load intrinsics::i32");

        let a_felt = Felt::new(a as u32 as u64);
        let b_felt = Felt::new(b as u32 as u64);
        // NOTE: arguments are passed in reverse, i.e. [b, a] not [a, b]
        let sub = "intrinsics::i32::overflowing_sub".parse().unwrap();
        let mut stack = harness
            .invoke(sub, &[b_felt, a_felt])
            .expect("execution failed");
        prop_assert_eq!(stack.len(), 2);

        let overflowed = stack.pop().unwrap() == Felt::ONE;
        let raw_result = stack.pop().unwrap().as_int();
        let result = u32::try_from(raw_result).map_err(|_| raw_result).map(|res| res as i32);
        let (expected_result, expected_overflow) = a.overflowing_sub(b);

        prop_assert_eq!((result, overflowed), (Ok(expected_result), expected_overflow));
    }

    #[test]
    fn i32_overflowing_mul(a: i32, b: i32) {
        let mut harness = TestByEmulationHarness::default();

        harness
            .emulator
            .load_module(
                Box::new(
                    intrinsics::load("intrinsics::i32", &harness.context.session.codemap)
                        .expect("undefined intrinsic module"),
                )
                .freeze(),
            )
            .expect("failed to load intrinsics::i32");

        let a_felt = Felt::new(a as u32 as u64);
        let b_felt = Felt::new(b as u32 as u64);
        // NOTE: arguments are passed in reverse, i.e. [b, a] not [a, b]
        let mul = "intrinsics::i32::overflowing_mul".parse().unwrap();
        let mut stack = harness
            .invoke(mul, &[b_felt, a_felt])
            .expect("execution failed");
        prop_assert_eq!(stack.len(), 2);

        let overflowed = stack.pop().unwrap() == Felt::ONE;
        let raw_result = stack.pop().unwrap().as_int();
        let result = u32::try_from(raw_result).map_err(|_| raw_result).map(|res| res as i32);
        let (expected_result, expected_overflow) = a.overflowing_mul(b);

        prop_assert_eq!((result, overflowed), (Ok(expected_result), expected_overflow));
    }

    #[test]
    fn i32_unchecked_neg(a: i32) {
        prop_assume!(a != i32::MIN, "unchecked_neg is meaningless for i32::MIN");

        let mut harness = TestByEmulationHarness::default();

        harness
            .emulator
            .load_module(
                Box::new(
                    intrinsics::load("intrinsics::i32", &harness.context.session.codemap)
                        .expect("undefined intrinsic module"),
                )
                .freeze(),
            )
            .expect("failed to load intrinsics::i32");

        let a_felt = Felt::new(a as u32 as u64);
        let neg = "intrinsics::i32::unchecked_neg".parse().unwrap();
        let mut stack = harness.invoke(neg, &[a_felt]).expect("execution failed");

        prop_assert_eq!(stack.len(), 1);
        let raw_result = stack.pop().unwrap().as_int();
        let result = u32::try_from(raw_result).map_err(|_| raw_result).map(|res| res as i32);
        prop_assert_eq!(result, Ok(-a));
    }

    #[test]
    fn i32_checked_div(a: i32, b: core::num::NonZeroI32) {
        let mut harness = TestByEmulationHarness::default();

        harness
            .emulator
            .load_module(
                Box::new(
                    intrinsics::load("intrinsics::i32", &harness.context.session.codemap)
                        .expect("undefined intrinsic module"),
                )
                .freeze(),
            )
            .expect("failed to load intrinsics::i32");

        let a_felt = Felt::new(a as u32 as u64);
        let b_felt = Felt::new(b.get() as u32 as u64);
        let div = "intrinsics::i32::checked_div".parse().unwrap();
        let mut stack = harness.invoke(div, &[b_felt, a_felt]).expect("execution failed");

        prop_assert_eq!(stack.len(), 1);
        let raw_result = stack.pop().unwrap().as_int();
        let result = u32::try_from(raw_result).map_err(|_| raw_result).map(|res| res as i32);
        prop_assert_eq!(result, Ok(a / b.get()));
    }

    #[test]
    fn i32_pow2(a in 0u32..30u32) {
        let mut harness = TestByEmulationHarness::default();

        harness
            .emulator
            .load_module(
                Box::new(
                    intrinsics::load("intrinsics::i32", &harness.context.session.codemap)
                        .expect("undefined intrinsic module"),
                )
                .freeze(),
            )
            .expect("failed to load intrinsics::i32");

        let a_felt = Felt::new(a as u64);
        let pow2 = "intrinsics::i32::pow2".parse().unwrap();
        let mut stack = harness.invoke(pow2, &[a_felt]).expect("execution failed");

        prop_assert_eq!(stack.len(), 1);
        let raw_result = stack.pop().unwrap().as_int();
        let result = u32::try_from(raw_result).map_err(|_| raw_result).map(|res| res as i32);
        prop_assert_eq!(result, Ok(2i32.pow(a)));
    }

    #[test]
    fn i32_ipow(a: i32, b in 0u32..30u32) {
        let mut harness = TestByEmulationHarness::default();

        harness
            .emulator
            .load_module(
                Box::new(
                    intrinsics::load("intrinsics::i32", &harness.context.session.codemap)
                        .expect("undefined intrinsic module"),
                )
                .freeze(),
            )
            .expect("failed to load intrinsics::i32");

        let a_felt = Felt::new(a as u32 as u64);
        let b_felt = Felt::new(b as u64);
        let ipow = "intrinsics::i32::ipow".parse().unwrap();
        let mut stack = harness.invoke(ipow, &[b_felt, a_felt]).expect("execution failed");

        prop_assert_eq!(stack.len(), 1);
        let raw_result = stack.pop().unwrap().as_int();
        let result = u32::try_from(raw_result).map_err(|_| raw_result).map(|res| res as i32);
        prop_assert_eq!(result, Ok(a.wrapping_pow(b)));
    }

    #[test]
    fn i32_checked_shr(a: i32, b in 0i32..32i32) {
        let mut harness = TestByEmulationHarness::default();

        harness
            .emulator
            .load_module(
                Box::new(
                    intrinsics::load("intrinsics::i32", &harness.context.session.codemap)
                        .expect("undefined intrinsic module"),
                )
                .freeze(),
            )
            .expect("failed to load intrinsics::i32");

        let a_felt = Felt::new(a as u32 as u64);
        let b_felt = Felt::new(b as u32 as u64);
        let shr = "intrinsics::i32::checked_shr".parse().unwrap();
        let mut stack = harness.invoke(shr, &[b_felt, a_felt]).expect("execution failed");

        prop_assert_eq!(stack.len(), 1);
        let raw_result = stack.pop().unwrap().as_int();
        let result = u32::try_from(raw_result).map_err(|_| raw_result).map(|res| res as i32);
        prop_assert_eq!(result, Ok(a >> b));
    }
}
