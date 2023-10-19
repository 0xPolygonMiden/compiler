use std::{cell::RefCell, cmp, fmt, rc::Rc};

use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;

use miden_hir::{Felt, FieldElement, FunctionIdent, Ident, OperandStack, Stack, StarkField};

use crate::{BlockId, Function, Module, Op, Program};

/// The type signature for native Rust functions callable from MASM IR
pub type NativeFn = dyn FnMut(&mut Emulator, &[Felt]) -> Result<(), EmulationError>;

/// The size/type of pointers in the emulator
type Addr = u32;

/// This type represents the various sorts of errors which can occur when
/// running the emulator on a MASM program. Some errors may result in panics,
/// but those which we can handle are represented here.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum EmulationError {
    /// The given module is already loaded
    #[error("unable to load module: '{0}' is already loaded")]
    AlreadyLoaded(Ident),
    /// The given function is already loaded
    #[error("unable to load function: '{0}' is already loaded")]
    DuplicateFunction(FunctionIdent),
    /// The given function cannot be found
    #[error("unable to invoke function: '{0}' is not defined")]
    UndefinedFunction(FunctionIdent),
    /// The emulator ran out of available memory
    #[error("system limit: out of memory")]
    OutOfMemory,
    /// The emulator was terminated due to a program failing to terminate in its budgeted time
    #[error("execution terminated prematurely: maximum cycle count reached")]
    CycleBudgetExceeded,
    /// A breakpoint was reached, so execution was suspended and can be resumed
    #[error("execution suspended by breakpoint")]
    BreakpointHit,
}

/// We allow functions in the emulator to be defined in either MASM IR, or native Rust.
///
/// Functions implemented in Rust are given a mutable reference to the emulator, so they
/// have virtually unlimited power, but are correspondingly very unsafe. With great
/// power comes great responsibility, etc.
#[derive(Clone)]
enum Stub {
    /// This function has a definition in Miden Assembly
    Asm(Rc<Function>),
    /// This function has a native Rust implementation
    Native(Rc<RefCell<Box<NativeFn>>>),
}

#[derive(Copy, Clone)]
pub enum Breakpoint {
    /// Break after one cycle
    Step,
    /// Break after `n` cycles
    StepN(usize),
    /// Break after one cycle, clearing the breakpoint in the process
    StepOnce,
    /// Break when leaving a block
    StepOut,
    /// Break after the next instruction is executed.
    ///
    /// For calls and control flow instructions, "executed" is defined as
    /// having executed all instructions nested within that instruction, i.e.
    /// stepping over a `while.true` will execute until the next instruction
    /// after the loop is reached.
    StepOver,
    /// Step until control reaches the given instruction pointer value
    StepUntil(InstructionPointer),
    /// Break at loop instructions
    ///
    /// The break will start on the looping instruction itself, and when
    /// execution resumes, will break either at the next nested loop, or
    /// if a complete iteration is reached, one of two places depending on
    /// the type of looping instruction we're in:
    ///
    /// * `while.true` will break at the `while.true` on each iteration
    /// * `repeat.n` will break at the top of the loop body on each iteration
    Loop,
    /// Break when the given function is called
    Call(FunctionIdent),
    /// Break when a write touches the region specified
    MemoryWrite { addr: usize, size: usize },
}

/// [Emulator] provides us with a means to execute our MASM IR directly
/// without having to emit "real" MASM and run it via the Miden VM.
/// In other words, it's a convenient way to run tests to verify the
/// expected behavior of a program without all of the baggage of the
/// Miden VM.
///
/// [Emulator] is necessarily a more limited execution environment:
///
/// * It only handles instructions which are defined in the [Op] enum
/// * Anything related to proving, calling contracts, etc. is not supported
/// * The default environment is empty, i.e. there are no Miden VM standard
/// library functions available. Users must emit Miden IR for all functions
/// they wish to call, or alternatively, provide native stubs.
pub struct Emulator {
    functions: FxHashMap<FunctionIdent, Stub>,
    locals: FxHashMap<FunctionIdent, Addr>,
    modules_loaded: FxHashSet<Ident>,
    modules_pending: FxHashSet<Ident>,
    memory: Vec<[Felt; 4]>,
    stack: OperandStack<Felt>,
    callstack: Vec<Activation>,
    hp: u32,
    lp: u32,
    bp: Option<Breakpoint>,
    clk: usize,
    clk_limit: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct InstructionPointer {
    /// The block in which the instruction pointer is located
    pub block: BlockId,
    /// The index of the instruction pointed to
    pub index: usize,
}
impl InstructionPointer {
    pub const fn new(block: BlockId) -> Self {
        Self { block, index: 0 }
    }
}

/// Represents the current state of the program being executed for use in debugging/troubleshooting
pub struct DebugInfo<'a> {
    /// The current function being executed
    pub function: FunctionIdent,
    /// The address at which locals for the current function begin
    pub fp: Addr,
    /// The current instruction pointer value
    pub ip: InstructionPointer,
    /// The instruction under the instruction pointer
    pub ix: Option<Op>,
    /// Indicates whether any control flow actions occur during this cycle
    pub action: Jump,
    /// The current state of the operand stack
    pub stack: &'a OperandStack<Felt>,
}
impl<'a> fmt::Debug for DebugInfo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DebugInfo")
            .field("function", &self.function)
            .field("fp", &self.fp)
            .field("ip", &self.ip)
            .field("ix", &self.ix)
            .field("action", &self.action)
            .field("stack", &self.stack.debug())
            .finish()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Jump {
    /// No jumps made during this cycle
    None,
    /// We returned from the current function during this cycle
    Return,
    /// We jumped to another block during this cycle
    Branch,
    /// This cycle will start the `n`th iteration of a repeat block
    Repeat(u8),
}

struct Activation {
    function: Rc<Function>,
    ip: InstructionPointer,
    fp: Addr,
    repeat_stack: SmallVec<[Option<(u8, u8)>; 2]>,
    ip_stack: SmallVec<[InstructionPointer; 2]>,
}
impl Activation {
    pub fn new(function: Rc<Function>, fp: Addr) -> Self {
        let block = function.body;
        Self {
            function,
            ip: InstructionPointer::new(block),
            fp,
            repeat_stack: Default::default(),
            ip_stack: Default::default(),
        }
    }

    fn pending_ip(&self) -> (InstructionPointer, Jump) {
        // Get code for this activation record
        let code = self.function.blocks[self.ip.block].ops.as_slice();
        // If we've reached the end of the current code block, attempt
        // to resume execution of the parent code block, if there is one
        if self.ip.index == code.len() {
            if let Some(Some((count, n))) = self.repeat_stack.last().copied() {
                if count <= n {
                    return (InstructionPointer::new(self.ip.block), Jump::Repeat(count));
                }
            }
            for ip in self.ip_stack.iter().rev().copied() {
                match self.function.blocks[ip.block].ops.get(ip.index).copied() {
                    Some(_) => {
                        return (ip, Jump::Branch);
                    }
                    None => continue,
                }
            }

            (self.ip, Jump::Return)
        } else {
            (self.ip, Jump::None)
        }
    }

    // Peek at the next instruction to be executed, as well as what the state
    // of the instruction pointer will be at this step.
    fn peek_instruction(&self) -> (Option<Op>, Jump, InstructionPointer) {
        let (ip, jump) = self.pending_ip();
        let ix = self.function.blocks[ip.block].ops.get(ip.index).copied();
        (ix, jump, ip)
    }

    // Get the instruction under the instruction pointer, and move the instruction pointer forward
    //
    // Also returns a value indicating whether or not, and what kind of jump was performed if we
    // reached the end of a block
    fn next_instruction(&mut self) -> (Option<Op>, Jump) {
        // Get code for this activation record
        let code = self.function.blocks[self.ip.block].ops.as_slice();
        // If we've reached the end of the current code block, attempt
        // to resume execution of the parent code block, if there is one
        if self.ip.index == code.len() {
            if let Some(Some((count, n))) = self.repeat_stack.pop() {
                if count <= n {
                    self.repeat_stack.push(Some((count + 1, n)));
                    self.ip.index = 1;
                    return (Some(code[0]), Jump::Repeat(count));
                }
            }

            // Find the next instruction to execute
            while let Some(ip) = self.ip_stack.pop() {
                self.ip = ip;
                match self.current_op() {
                    ix @ Some(_) => {
                        self.ip.index += 1;
                        return (ix, Jump::Branch);
                    }
                    None => continue,
                }
            }

            // If we reach here, there was no more code to execute in this function
            (None, Jump::Return)
        } else {
            let ix = code.get(self.ip.index).copied();
            self.ip.index += 1;
            (ix, Jump::None)
        }
    }

    #[inline(always)]
    fn current_op(&self) -> Option<Op> {
        self.function.blocks[self.ip.block]
            .ops
            .get(self.ip.index)
            .copied()
    }

    fn enter_block(&mut self, block: BlockId) {
        self.ip_stack.push(self.ip);
        self.repeat_stack.push(None);
        self.ip = InstructionPointer::new(block);
    }

    fn enter_while_loop(&mut self, block: BlockId) {
        // We must revisit the while.true instruction on each iteration,
        // so move the instruction pointer back one
        let ip = InstructionPointer {
            block: self.ip.block,
            index: self.ip.index - 1,
        };
        self.ip_stack.push(ip);
        self.repeat_stack.push(None);
        self.ip = InstructionPointer::new(block);
    }

    fn repeat_block(&mut self, block: BlockId, count: u8) {
        self.ip_stack.push(self.ip);
        self.repeat_stack.push(Some((1, count)));
        self.ip = InstructionPointer::new(block);
    }
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new(
            Self::DEFAULT_HEAP_SIZE,
            Self::DEFAULT_HEAP_START,
            Self::DEFAULT_LOCALS_START,
        )
    }
}
impl Emulator {
    const PAGE_SIZE: u32 = 64 * 1024;
    const DEFAULT_HEAP_SIZE: u32 = (4 * Self::PAGE_SIZE) / 16;
    const DEFAULT_HEAP_START: u32 = (2 * Self::PAGE_SIZE) / 16;
    const DEFAULT_LOCALS_START: u32 = (3 * Self::PAGE_SIZE) / 16;
    const EMPTY_WORD: [Felt; 4] = [Felt::ZERO; 4];

    /// Construct a new, empty emulator with:
    ///
    /// * A linear memory heap of `memory_size` words
    /// * The start of the usable heap set to `hp` (an address in words)
    /// * The start of the reserved heap used for locals set to `lp` (an address in words)
    ///
    pub fn new(memory_size: u32, hp: u32, lp: u32) -> Self {
        let memory = vec![Self::EMPTY_WORD; memory_size as usize];
        Self {
            functions: Default::default(),
            locals: Default::default(),
            modules_loaded: Default::default(),
            modules_pending: Default::default(),
            memory,
            stack: Default::default(),
            callstack: vec![],
            hp,
            lp,
            bp: None,
            clk: 0,
            clk_limit: usize::MAX,
        }
    }

    /// Place a cap on the number of cycles the emulator will execute before failing with an error
    pub fn set_max_cycles(&mut self, max: usize) {
        self.clk_limit = max;
    }

    /// Sets the next breakpoint for the emulator
    pub fn set_breakpoint(&mut self, bp: Breakpoint) {
        self.bp = Some(bp);
    }

    /// Clears any active breakpoint
    pub fn clear_breakpoint(&mut self) {
        self.bp = None;
    }

    /// Get's debug information about the current emulator state
    pub fn info(&self) -> Option<DebugInfo<'_>> {
        let current = self.callstack.last()?;
        let (ix, action, ip) = current.peek_instruction();
        Some(DebugInfo {
            function: current.function.name,
            fp: current.fp,
            ip,
            ix,
            action,
            stack: &self.stack,
        })
    }

    pub fn current_ip(&self) -> Option<InstructionPointer> {
        self.callstack.last().map(|cur| cur.pending_ip().0)
    }

    fn pending_ip(&self) -> Option<(InstructionPointer, Jump)> {
        self.callstack.last().map(|cur| cur.pending_ip())
    }

    /// Load `program` into this emulator
    pub fn load_program(&mut self, program: Program) -> Result<(), EmulationError> {
        for module in program.modules.into_iter() {
            self.load_module(module)?;
        }

        // TODO: Load data segments

        Ok(())
    }

    /// Load `module` into this emulator
    pub fn load_module(&mut self, mut module: Module) -> Result<(), EmulationError> {
        if !self.modules_loaded.insert(module.name) {
            return Err(EmulationError::AlreadyLoaded(module.name));
        }

        // Register module dependencies
        for import in module.imports.iter() {
            let name = Ident::with_empty_span(import.name);
            if self.modules_loaded.contains(&name) {
                continue;
            }
            self.modules_pending.insert(name);
        }
        self.modules_pending.remove(&module.name);

        // Load functions from this module
        while let Some(function) = module.functions.pop_front() {
            let function = Rc::from(function);
            self.load_function(function)?;
        }

        Ok(())
    }

    /// Load `function` into this emulator
    fn load_function(&mut self, function: Rc<Function>) -> Result<(), EmulationError> {
        let id = function.name;
        if self.functions.contains_key(&id) {
            return Err(EmulationError::DuplicateFunction(id));
        }
        let fp = self.lp;
        self.lp += function.locals().len() as u32;
        self.functions.insert(id, Stub::Asm(function));
        self.locals.insert(id, fp);

        Ok(())
    }

    /// Load `function` into this emulator, with the given identifier
    ///
    /// Because we don't know the set of [FuncId] that have already been allocated,
    /// we leave the the choice up to the caller. We assert that functions do
    /// not get defined twice to catch conflicts, just in case.
    pub fn load_nif(
        &mut self,
        id: FunctionIdent,
        function: Box<NativeFn>,
    ) -> Result<(), EmulationError> {
        if self.functions.contains_key(&id) {
            return Err(EmulationError::DuplicateFunction(id));
        }
        self.functions
            .insert(id, Stub::Native(Rc::new(RefCell::new(function))));

        Ok(())
    }

    /// Allocate space for `value` on the emulator heap, and copy it's contents there.
    ///
    /// NOTE: The smallest unit of addressable memory is 4 bytes (32 bits). If you provide
    /// a value that is smaller than this, or is not a multiple of 4, the data will be padded
    /// with zeroes to ensure that it is.
    pub fn write_bytes_to_memory(&mut self, value: &[u8]) -> u32 {
        let addr = self.hp;
        if value.is_empty() {
            return addr;
        }

        let mut elem_idx = 0;
        for chunk in value.chunks(4) {
            let elem = match chunk.len() {
                4 => u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]),
                3 => u32::from_le_bytes([chunk[0], chunk[1], chunk[2], 0]),
                2 => u32::from_le_bytes([chunk[0], chunk[1], 0, 0]),
                1 => u32::from_le_bytes([chunk[0], 0, 0, 0]),
                0 => 0,
                _ => unreachable!(),
            };
            if elem_idx == 4 {
                elem_idx = 0;
                assert!(
                    self.hp + 1 < self.lp,
                    "heap has overflowed into reserved region"
                );
                self.hp += 1;
            }
            self.memory[self.hp as usize][elem_idx] = Felt::new(elem as u64);
            elem_idx += 1;
        }

        addr
    }

    /// Allocate enough words to hold `size` bytes of memory
    ///
    /// Returns the pointer as a byte-addressable address
    pub fn malloc(&mut self, size: usize) -> u32 {
        let addr = self.hp;

        if size == 0 {
            return addr;
        }

        let size = size as u32;
        let extra = size % 16;
        let words = (size / 16) + (extra > 0) as u32;
        assert!(
            self.hp + words < self.lp,
            "heap has overflowed into reserved region"
        );
        self.hp += words;

        addr * 16
    }

    /// Write `value` to the word at `addr`, and element `index`
    pub fn store(&mut self, addr: usize, value: Felt) {
        use crate::NativePtr;

        let ptr = NativePtr::from_ptr(addr.try_into().expect("invalid address"));
        let addr = ptr.waddr as usize;
        assert_eq!(ptr.offset, 0, "invalid store: unaligned address {addr:#?}");
        assert!(addr < self.memory.len(), "invalid address");

        self.memory[addr][ptr.index as usize] = value;
    }

    /// Run the emulator by invoking `callee` with `args` placed on the
    /// operand stack in FIFO order.
    ///
    /// If a fatal error occurs during emulation, `Err` is returned,
    /// e.g. if `callee` has not been loaded.
    ///
    /// When `callee` returns, it's result will be returned wrapped in `Ok`.
    /// For functions with no return value, this will be `Ok(None)`, or all
    /// others it will be `Ok(Some(value))`.
    pub fn invoke(
        &mut self,
        callee: FunctionIdent,
        args: &[Felt],
    ) -> Result<OperandStack<Felt>, EmulationError> {
        let fun = self
            .functions
            .get(&callee)
            .cloned()
            .ok_or(EmulationError::UndefinedFunction(callee))?;
        match fun {
            Stub::Asm(ref function) => self.invoke_function(function.clone(), args),
            Stub::Native(function) => {
                let mut function = function.borrow_mut();
                function(self, args)?;
                Ok(self.stack.clone())
            }
        }
    }

    /// Invoke a function defined in MASM IR, placing the given arguments on the
    /// operand stack in FIFO order.
    #[inline]
    fn invoke_function(
        &mut self,
        function: Rc<Function>,
        args: &[Felt],
    ) -> Result<OperandStack<Felt>, EmulationError> {
        // Place the arguments on the operand stack
        assert_eq!(args.len(), function.arity());
        for arg in args.iter().copied().rev() {
            self.stack.push(arg);
        }

        // Schedule `function`
        let name = function.name;
        let fp = self.locals[&name];
        let state = Activation::new(function, fp);
        self.callstack.push(state);

        match self.bp {
            // Break on the first instruction, if applicable
            Some(Breakpoint::Step) => Err(EmulationError::BreakpointHit),
            // Break on the first instruction, if applicable
            Some(Breakpoint::Call(ref callee)) if callee == &name => {
                Err(EmulationError::BreakpointHit)
            }
            _ => self.resume(),
        }
    }

    /// Resume execution when the emulator suspended due to a breakpoint
    pub fn resume(&mut self) -> Result<OperandStack<Felt>, EmulationError> {
        self.run(true)?;

        Ok(self.stack.clone())
    }
}

/// Pops the top element off the stack
macro_rules! pop {
    ($emu:ident) => {
        $emu.stack.pop().expect("operand stack is empty")
    };

    ($emu:ident, $msg:literal) => {
        $emu.stack.pop().expect($msg)
    };

    ($emu:ident, $msg:literal, $($arg:expr),+) => {
        match $emu.stack.pop() {
            Some(value) => value,
            None => panic!($msg, $($arg),*),
        }
    }
}

/// Pops the top word off the stack
macro_rules! popw {
    ($emu:ident) => {
        $emu.stack.popw().expect("operand stack does not contain a full word")
    };

    ($emu:ident, $msg:literal) => {
        $emu.stack.popw().expect($msg)
    };

    ($emu:ident, $msg:literal, $($arg:expr),+) => {{
        match $emu.stack.popw() {
            Some(value) => value,
            None => panic!($msg, $($arg),*),
        }
    }}
}

/// Pops the top two elements off the stack, returning them in order of appearance
macro_rules! pop2 {
    ($emu:ident) => {{
        let b = pop!($emu);
        let a = pop!($emu);
        (b, a)
    }};
}

/// Pops a u32 value from the top of the stack, and asserts if it is out of range
macro_rules! pop_u32 {
    ($emu:ident) => {{
        let value = pop!($emu).as_int();
        assert!(value < 2u64.pow(32), "assertion failed: {value} is not a valid u32, value is out of range");
        value as u32
    }};

    ($emu:ident, $format:literal $(, $args:expr)*) => {{
        let value = pop!($emu).as_int();
        assert!(value < 2u64.pow(32), $format, value, $($args),*);
        value as u32
    }}
}

/// Pops a pointer value from the top of the stack, and asserts if it is not a valid boolean
macro_rules! pop_addr {
    ($emu:ident) => {{
        let addr = pop_u32!($emu, "expected valid 32-bit address, got {}") as usize;
        assert!(addr < $emu.memory.len(), "out of bounds memory access");
        addr
    }};
}

/// Pops a boolean value from the top of the stack, and asserts if it is not a valid boolean
macro_rules! pop_bool {
    ($emu:ident) => {{
        let value = pop!($emu).as_int();
        assert!(
            value < 2,
            "assertion failed: {value} is not a valid boolean, value must be either 1 or 0"
        );
        value == 1
    }};
}

/// Applies a binary operator that produces a result of the same input type:
///
/// 1. The top two elements of the stack
/// 2. The top element of the stack and an immediate.
macro_rules! binop {
    ($emu:ident, $op:ident) => {{
        use core::ops::*;
        let b = pop!($emu);
        let a = pop!($emu);
        $emu.stack.push(a.$op(b));
    }};

    ($emu:ident, $op:ident, $imm:expr) => {{
        use core::ops::*;
        let a = pop!($emu);
        $emu.stack.push(a.$op($imm));
    }};
}

/// Applies a binary operator to two u32 values, either:
///
/// 1. The top two elements of the stack
/// 2. The top element of the stack and an immediate.
macro_rules! binop32 {
    ($emu:ident, $op:ident) => {{
        #[allow(unused)]
        use core::ops::*;
        let b = pop_u32!($emu);
        let a = pop_u32!($emu);
        $emu.stack.push_u32(a.$op(b));
    }};

    ($emu:ident, $op:ident, $imm:expr) => {{
        #[allow(unused)]
        use core::ops::*;
        let a = pop_u32!($emu);
        $emu.stack.push_u32(a.$op($imm));
    }};
}

/// Applies a checked binary operator to two u32 values, either:
///
/// 1. The top two elements of the stack
/// 2. The top element of the stack and an immediate.
macro_rules! binop_checked_u32 {
    ($emu:ident, $op:ident) => {{
        paste::paste! {
            binop_checked_u32_impl!($emu, [<checked_ $op>]);
        }
    }};

    ($emu:ident, $op:ident, $imm:expr) => {{
        paste::paste! {
            binop_checked_u32_impl!($emu, [<checked_ $op>], $imm);
        }
    }};
}

#[doc(hidden)]
macro_rules! binop_checked_u32_impl {
    ($emu:ident, $op:ident) => {{
        #[allow(unused)]
        use core::ops::*;
        let b = pop_u32!($emu);
        let a = pop_u32!($emu);
        let result = a
            .$op(b)
            .expect("checked operation failed: result has overflowed the u32 range");
        $emu.stack.push(Felt::new(result as u64));
    }};

    ($emu:ident, $op:ident, $imm:expr) => {{
        #[allow(unused)]
        use core::ops::*;
        let a = pop_u32!($emu);
        let result = a
            .$op($imm)
            .expect("checked operation failed: result has overflowed the u32 range");
        $emu.stack.push(Felt::new(result as u64));
    }};
}

/// Applies an overflowing binary operator to two u32 values, either:
///
/// 1. The top two elements of the stack
/// 2. The top element of the stack and an immediate.
macro_rules! binop_overflowing_u32 {
    ($emu:ident, $op:ident) => {{
        paste::paste! {
            binop_overflowing_u32_impl!($emu, [<overflowing_ $op>]);
        }
    }};

    ($emu:ident, $op:ident, $imm:expr) => {{
        paste::paste! {
            binop_overflowing_u32_impl!($emu, [<overflowing_ $op>], $imm);
        }
    }};
}

#[doc(hidden)]
macro_rules! binop_overflowing_u32_impl {
    ($emu:ident, $op:ident) => {{
        #[allow(unused)]
        use core::ops::*;
        let b = pop_u32!($emu);
        let a = pop_u32!($emu);
        let (result, overflowed) = a.$op(b);
        $emu.stack.push_u32(result);
        $emu.stack.push_u8(overflowed as u8);
    }};

    ($emu:ident, $op:ident, $imm:expr) => {{
        #[allow(unused)]
        use core::ops::*;
        let a = pop_u32!($emu);
        let (result, overflowed) = a.$op($imm);
        $emu.stack.push_u32(result);
        $emu.stack.push_u8(overflowed as u8);
    }};
}

/// Applies a wrapping binary operator to two u32 values, either:
///
/// 1. The top two elements of the stack
/// 2. The top element of the stack and an immediate.
macro_rules! binop_wrapping_u32 {
    ($emu:ident, $op:ident) => {{
        paste::paste! {
            binop_wrapping_u32_impl!($emu, [<wrapping_ $op>]);
        }
    }};

    ($emu:ident, $op:ident, $imm:expr) => {{
        paste::paste! {
            binop_wrapping_u32_impl!($emu, [<wrapping_ $op>], $imm);
        }
    }};
}

#[doc(hidden)]
macro_rules! binop_wrapping_u32_impl {
    ($emu:ident, $op:ident) => {{
        #[allow(unused)]
        use core::ops::*;
        let b = pop_u32!($emu);
        let a = pop_u32!($emu);
        $emu.stack.push_u32(a.$op(b));
    }};

    ($emu:ident, $op:ident, $imm:expr) => {{
        #[allow(unused)]
        use core::ops::*;
        let a = pop_u32!($emu);
        $emu.stack.push_u32(a.$op($imm));
    }};
}

/// Applies a binary comparison operator, to either:
///
/// 1. The top two elements of the stack
/// 2. The top element of the stack and an immediate.
macro_rules! comparison {
    ($emu:ident, $op:ident) => {{
        let b = pop!($emu).as_int();
        let a = pop!($emu).as_int();
        let result: bool = a.$op(&b);
        $emu.stack.push_u8(result as u8);
    }};

    ($emu:ident, $op:ident, $imm:expr) => {{
        let a = pop!($emu).as_int();
        let result: bool = a.$op(&$imm);
        $emu.stack.push_u8(result as u8);
    }};
}

/// Applies a binary comparison operator to two u32 values, either:
///
/// 1. The top two elements of the stack
/// 2. The top element of the stack and an immediate.
macro_rules! comparison32 {
    ($emu:ident, $op:ident) => {{
        let b = pop_u32!($emu);
        let a = pop_u32!($emu);
        let result: bool = a.$op(&b);
        $emu.stack.push_u8(result as u8);
    }};

    ($emu:ident, $op:ident, $imm:expr) => {{
        let a = pop_u32!($emu);
        let result: bool = a.$op(&$imm);
        $emu.stack.push_u8(result as u8);
    }};
}

impl Emulator {
    /// Run the emulator until all calls are completed, the cycle budget is exhausted,
    /// or a breakpoint is hit.
    ///
    /// It is expected that the caller has set up the operand stack with the correct
    /// number of arguments. If not, undefined behavior (from the perspective of the
    /// MASM program) will result.
    #[inline(never)]
    fn run(&mut self, mut resuming: bool) -> Result<(), EmulationError> {
        // If a breakpoint is set for a certain number of cycles, set
        // the value of the cycle counter we should up to
        let step_until_cycle = if let Some(Breakpoint::StepN(count)) = self.bp {
            self.clk + (count - 1)
        } else {
            usize::MAX
        };

        // This is the core interpreter loop for MASM IR, it runs until one of the
        // following occurs:
        //
        // * We run out of code to execute, i.e. the function is returning normally
        // * Execution was explicitly aborted from within the function
        // * Execution traps due to a MASM invariant being violated, indicating the
        // code is malformed.
        // * Execution traps due to a runtime system error, e.g. out of memory
        // * Execution traps due to exceeding the predefined execution budget
        // * Execution breaks due to a breakpoint
        'outer: loop {
            // Terminate execution early if we reach a predetermined number of cycles
            self.clk += 1;
            if self.clk > self.clk_limit {
                return Err(EmulationError::CycleBudgetExceeded);
            }

            // The "resuming" flag is reset after one step
            let resuming = core::mem::take(&mut resuming);
            let mut action = match self.bp {
                // When resuming, certain breakpoints break at the instruction that
                // we'll be resuming with, which would just cause execution to break
                // immediately again. In order to solve this, we disable such breakpoints
                // for one step, then re-enable them afterwards.
                Some(Breakpoint::Loop | Breakpoint::MemoryWrite { .. }) if resuming => {
                    let bp = self.bp.take();
                    let action = self.step()?;
                    self.bp = bp;
                    action
                }
                None | Some(_) => self.step()?,
            };

            'handle_action: loop {
                match action {
                    // There is no more code to execute, so halt the emulator
                    Action::Halt => break 'outer,
                    // Execution was suspended after dispatching an instruction normally
                    Action::Suspend => match self.bp {
                        // Execution should break immediately
                        Some(Breakpoint::Step) | Some(Breakpoint::StepOver) => {
                            return Err(EmulationError::BreakpointHit)
                        }
                        Some(Breakpoint::StepOnce) => {
                            self.bp.take();
                            return Err(EmulationError::BreakpointHit);
                        }
                        Some(Breakpoint::StepN(_)) if self.clk >= step_until_cycle => {
                            return Err(EmulationError::BreakpointHit)
                        }
                        Some(Breakpoint::StepUntil(ip)) => {
                            if let Some((pending_ip, _)) = self.pending_ip() {
                                if pending_ip == ip {
                                    self.bp = None;
                                    return Err(EmulationError::BreakpointHit);
                                }
                            }
                            continue 'outer;
                        }
                        // Execution should resume with the next instruction
                        _ => continue 'outer,
                    },
                    // There was no code remaining in the current function, effectively
                    // returning from it. Since no instructions were dispatched, we don't
                    // count the cycle, and resume immediately at the continuation point
                    // in the caller
                    Action::Return => {
                        loop {
                            match self.pending_ip() {
                                // Step forward as the next instruction is also a return
                                Some((_, Jump::Return)) => {
                                    action = self.step()?;
                                    continue 'handle_action;
                                }
                                // This step will resume control in a previous caller,
                                // so handle this as a suspension
                                Some(_) => {
                                    action = Action::Suspend;
                                    continue 'handle_action;
                                }
                                // There is no code remaining in the caller either, so
                                // pop them off the call stack and try again until we
                                // either reach the bottom of the callstack, or a caller
                                // with more instructions
                                None => {
                                    if self.callstack.pop().is_none() {
                                        break 'outer;
                                    }
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn step(&mut self) -> Result<Action, EmulationError> {
        const U32_P: u64 = 2u64.pow(32);

        // If there are no more activation records, we're done
        if self.callstack.is_empty() {
            return Ok(Action::Halt);
        }
        let mut state = self.callstack.pop().unwrap();

        let (ix, jump) = state.next_instruction();
        if let Some(ix) = ix {
            if jump != Jump::None {
                match self.bp {
                    Some(Breakpoint::StepOut) => {
                        if jump == Jump::Branch {
                            state.ip.index -= 1;
                            return Err(EmulationError::BreakpointHit);
                        }
                    }
                    Some(Breakpoint::Loop) => {
                        if let Jump::Repeat(_) = jump {
                            state.ip.index -= 1;
                            return Err(EmulationError::BreakpointHit);
                        }
                    }
                    _ => (),
                }
            }
            match ix {
                Op::Padw => {
                    self.stack.padw();
                }
                Op::Push(v) => {
                    self.stack.push(v);
                }
                Op::Push2([a, b]) => {
                    self.stack.push(a);
                    self.stack.push(b);
                }
                Op::Pushw(word) => {
                    self.stack.pushw(word);
                }
                Op::PushU8(i) => {
                    self.stack.push_u8(i);
                }
                Op::PushU16(i) => {
                    self.stack.push_u16(i);
                }
                Op::PushU32(i) => {
                    self.stack.push_u32(i);
                }
                Op::Drop => {
                    self.stack.drop();
                }
                Op::Dropw => {
                    self.stack.dropw();
                }
                Op::Dup(pos) => {
                    self.stack.dup(pos as usize);
                }
                Op::Dupw(pos) => {
                    self.stack.dupw(pos as usize);
                }
                Op::Swap(pos) => {
                    self.stack.swap(pos as usize);
                }
                Op::Swapw(pos) => {
                    self.stack.swapw(pos as usize);
                }
                Op::Movup(pos) => {
                    self.stack.movup(pos as usize);
                }
                Op::Movupw(pos) => {
                    self.stack.movupw(pos as usize);
                }
                Op::Movdn(pos) => {
                    self.stack.movdn(pos as usize);
                }
                Op::Movdnw(pos) => {
                    self.stack.movdnw(pos as usize);
                }
                Op::Cswap => {
                    let cond = pop_bool!(self);
                    if cond {
                        self.stack.swap(1);
                    }
                }
                Op::Cswapw => {
                    let cond = pop_bool!(self);
                    if cond {
                        self.stack.swapw(1);
                    }
                }
                Op::Cdrop => {
                    let cond = pop_bool!(self);
                    let (b, a) = pop2!(self);
                    if cond {
                        self.stack.push(b);
                    } else {
                        self.stack.push(a);
                    }
                }
                Op::Cdropw => {
                    let cond = pop_bool!(self);
                    let b = popw!(self);
                    let a = popw!(self);
                    if cond {
                        self.stack.pushw(b);
                    } else {
                        self.stack.pushw(a);
                    }
                }
                Op::Assert => {
                    let cond = pop_bool!(self);
                    assert!(cond, "assertion failed: expected true, got false");
                }
                Op::Assertz => {
                    let cond = pop_bool!(self);
                    assert!(!cond, "assertion failed: expected false, got true");
                }
                Op::AssertEq => {
                    let (b, a) = pop2!(self);
                    assert_eq!(a, b, "equality assertion failed");
                }
                Op::AssertEqw => {
                    let b = popw!(self);
                    let a = popw!(self);
                    assert_eq!(a, b, "equality assertion failed");
                }
                Op::LocAddr(id) => {
                    let addr = state.fp + id.as_usize() as u32;
                    debug_assert!(addr < self.memory.len() as u32);
                    self.stack.push_u32(addr * 16);
                }
                Op::LocStore(id) => {
                    let addr = (state.fp + id.as_usize() as u32) as usize;
                    debug_assert!(addr < self.memory.len());
                    let value = pop!(self);
                    if let Some(Breakpoint::MemoryWrite {
                        addr: min_addr,
                        size,
                    }) = self.bp
                    {
                        let max_addr = min_addr + size;
                        if addr >= min_addr && addr < max_addr {
                            // Push operands back on the stack
                            self.stack.push(value);
                            // Suspend execution state
                            state.ip.index -= 1;
                            self.callstack.push(state);
                            return Err(EmulationError::BreakpointHit);
                        }
                    }
                    self.memory[addr][0] = value;
                }
                Op::LocStorew(id) => {
                    let addr = (state.fp + id.as_usize() as u32) as usize;
                    if let Some(Breakpoint::MemoryWrite {
                        addr: min_addr,
                        size,
                    }) = self.bp
                    {
                        let max_addr = min_addr + size;
                        if addr >= min_addr && addr < max_addr {
                            // Suspend execution state
                            state.ip.index -= 1;
                            self.callstack.push(state);
                            return Err(EmulationError::BreakpointHit);
                        }
                    }
                    assert!(addr < self.memory.len() - 4, "out of bounds memory access");
                    let word = self
                        .stack
                        .peekw()
                        .expect("operand stack does not contain a full word");
                    self.memory[addr] = word;
                }
                Op::MemLoad => {
                    let addr = pop_addr!(self);
                    self.stack.push(self.memory[addr][0]);
                }
                Op::MemLoadOffset => {
                    let offset = pop_u32!(self) as usize;
                    assert!(offset < 4, "expected valid element offset, got {offset}");
                    let addr = pop_addr!(self);
                    self.stack.push(self.memory[addr][offset]);
                }
                Op::MemLoadImm(addr) => {
                    let addr = addr as usize;
                    assert!(addr < self.memory.len(), "out of bounds memory access");
                    self.stack.push(self.memory[addr][0]);
                }
                Op::MemLoadOffsetImm(addr, offset) => {
                    let addr = addr as usize;
                    let offset = offset as usize;
                    assert!(addr < self.memory.len(), "out of bounds memory access");
                    self.stack.push(self.memory[addr][offset]);
                }
                Op::MemLoadw => {
                    let addr = pop_addr!(self);
                    self.stack.dropw();
                    self.stack.pushw(self.memory[addr]);
                }
                Op::MemLoadwImm(addr) => {
                    let addr = addr as usize;
                    assert!(addr < self.memory.len() - 4, "out of bounds memory access");
                    self.stack.dropw();
                    self.stack.pushw(self.memory[addr]);
                }
                Op::MemStore => {
                    let addr = pop_addr!(self);
                    let value = pop!(self);
                    if let Some(Breakpoint::MemoryWrite {
                        addr: min_addr,
                        size,
                    }) = self.bp
                    {
                        let max_addr = min_addr + size;
                        if addr >= min_addr && addr < max_addr {
                            // Push operands back on the stack
                            self.stack.push(value);
                            self.stack.push(Felt::new(addr as u64));
                            // Suspend execution state
                            state.ip.index -= 1;
                            self.callstack.push(state);
                            return Err(EmulationError::BreakpointHit);
                        }
                    }
                    self.memory[addr][0] = value;
                }
                Op::MemStoreOffset => {
                    let offset = pop_u32!(self);
                    assert!(offset < 4, "expected valid element offset, got {offset}");
                    let addr = pop_addr!(self);
                    let value = pop!(self);
                    let offset = offset as usize;
                    if let Some(Breakpoint::MemoryWrite {
                        addr: min_addr,
                        size,
                    }) = self.bp
                    {
                        let max_addr = min_addr + size;
                        if addr >= min_addr && addr < max_addr {
                            // Push operands back on the stack
                            self.stack.push(Felt::new(offset as u64));
                            self.stack.push(value);
                            self.stack.push(Felt::new(addr as u64));
                            // Suspend execution state
                            state.ip.index -= 1;
                            self.callstack.push(state);
                            return Err(EmulationError::BreakpointHit);
                        }
                    }
                    self.memory[addr][offset] = value;
                }
                Op::MemStoreImm(addr) => {
                    let addr = addr as usize;
                    if let Some(Breakpoint::MemoryWrite {
                        addr: min_addr,
                        size,
                    }) = self.bp
                    {
                        let max_addr = min_addr + size;
                        if addr >= min_addr && addr < max_addr {
                            // Suspend execution state
                            state.ip.index -= 1;
                            self.callstack.push(state);
                            return Err(EmulationError::BreakpointHit);
                        }
                    }
                    assert!(addr < self.memory.len(), "out of bounds memory access");
                    let value = self.stack.pop().expect("operand stack is empty");
                    self.memory[addr][0] = value;
                }
                Op::MemStoreOffsetImm(addr, offset) => {
                    let addr = addr as usize;
                    let offset = offset as usize;
                    if let Some(Breakpoint::MemoryWrite {
                        addr: min_addr,
                        size,
                    }) = self.bp
                    {
                        let max_addr = min_addr + size;
                        if addr >= min_addr && addr < max_addr {
                            // Suspend execution state
                            state.ip.index -= 1;
                            self.callstack.push(state);
                            return Err(EmulationError::BreakpointHit);
                        }
                    }
                    assert!(addr < self.memory.len(), "out of bounds memory access");
                    let value = self.stack.pop().expect("operand stack is empty");
                    self.memory[addr][offset] = value;
                }
                Op::MemStorew => {
                    let addr = pop_addr!(self);
                    let word = self
                        .stack
                        .peekw()
                        .expect("operand stack does not contain a full word");
                    let addr = addr as usize;
                    if let Some(Breakpoint::MemoryWrite {
                        addr: min_addr,
                        size,
                    }) = self.bp
                    {
                        let max_addr = min_addr + size;
                        if addr >= min_addr && addr < max_addr {
                            // Push operands back on the stack
                            self.stack.push(Felt::new(addr as u64));
                            // Suspend execution state
                            state.ip.index -= 1;
                            self.callstack.push(state);
                            return Err(EmulationError::BreakpointHit);
                        }
                    }
                    self.memory[addr] = word;
                }
                Op::MemStorewImm(addr) => {
                    let addr = addr as usize;
                    if let Some(Breakpoint::MemoryWrite {
                        addr: min_addr,
                        size,
                    }) = self.bp
                    {
                        let max_addr = min_addr + size;
                        if addr >= min_addr && addr < max_addr {
                            // Suspend execution state
                            state.ip.index -= 1;
                            self.callstack.push(state);
                            return Err(EmulationError::BreakpointHit);
                        }
                    }
                    assert!(addr < self.memory.len() - 4, "out of bounds memory access");
                    let word = self
                        .stack
                        .peekw()
                        .expect("operand stack does not contain a full word");
                    self.memory[addr] = word;
                }
                Op::If(then_blk, else_blk) => {
                    if let Some(Breakpoint::StepOver) = self.bp {
                        self.bp = Some(Breakpoint::StepUntil(state.pending_ip().0));
                    }
                    let cond = pop_bool!(self);
                    if cond {
                        state.enter_block(then_blk);
                    } else {
                        state.enter_block(else_blk);
                    }
                }
                Op::While(body_blk) => {
                    match self.bp {
                        Some(Breakpoint::Loop) => {
                            state.ip.index -= 1;
                            self.callstack.push(state);
                            return Err(EmulationError::BreakpointHit);
                        }
                        Some(Breakpoint::StepOver) => {
                            self.bp = Some(Breakpoint::StepUntil(state.pending_ip().0));
                        }
                        _ => (),
                    }
                    let cond = pop_bool!(self);
                    if cond {
                        state.enter_while_loop(body_blk);
                    }
                }
                Op::Repeat(n, body_blk) => {
                    match self.bp {
                        Some(Breakpoint::Loop) => {
                            state.ip.index -= 1;
                            self.callstack.push(state);
                            return Err(EmulationError::BreakpointHit);
                        }
                        Some(Breakpoint::StepOver) => {
                            self.bp = Some(Breakpoint::StepUntil(state.pending_ip().0));
                        }
                        _ => (),
                    }
                    state.repeat_block(body_blk, n);
                }
                Op::Exec(callee) => {
                    let callee = callee;
                    let fun = self
                        .functions
                        .get(&callee)
                        .cloned()
                        .ok_or(EmulationError::UndefinedFunction(callee))?;
                    match fun {
                        Stub::Asm(ref function) => {
                            let fp = self.locals[&function.name];
                            let callee_state = Activation::new(function.clone(), fp);
                            match self.bp {
                                Some(Breakpoint::Call(bp)) => {
                                    // Suspend caller
                                    self.callstack.push(state);
                                    // Schedule callee next
                                    self.callstack.push(callee_state);
                                    if callee == bp {
                                        return Err(EmulationError::BreakpointHit);
                                    }
                                    return Ok(Action::Suspend);
                                }
                                Some(Breakpoint::StepOver) => {
                                    self.bp = Some(Breakpoint::StepUntil(state.pending_ip().0));
                                }
                                _ => (),
                            }
                            // Suspend caller
                            self.callstack.push(state);
                            // Schedule callee next
                            self.callstack.push(callee_state);
                            return Ok(Action::Suspend);
                        }
                        Stub::Native(_function) => unimplemented!(),
                    }
                }
                Op::Syscall(_callee) => unimplemented!(),
                Op::Add => binop!(self, add),
                Op::AddImm(imm) => binop!(self, add, imm),
                Op::Sub => binop!(self, sub),
                Op::SubImm(imm) => binop!(self, sub, imm),
                Op::Mul => binop!(self, mul),
                Op::MulImm(imm) => binop!(self, mul, imm),
                Op::Div => binop!(self, div),
                Op::DivImm(imm) => binop!(self, div, imm),
                Op::Neg => {
                    let a = self.stack.pop().expect("operand stack is empty");
                    self.stack.push(-a);
                }
                Op::Inv => {
                    let a = self.stack.pop().expect("operand stack is empty");
                    self.stack.push(a.inv());
                }
                Op::Incr => binop!(self, add, Felt::ONE),
                Op::Pow2 => {
                    let a = pop!(self).as_int();
                    assert!(
                        a < 64,
                        "invalid power of two: expected {a} to be a value less than 64"
                    );
                    let two = Felt::new(2);
                    self.stack.push(two.exp(a));
                }
                Op::Exp => {
                    let (b, a) = pop2!(self);
                    let b = b.as_int();
                    assert!(
                        b < 64,
                        "invalid power of two: expected {b} to be a value less than 64"
                    );
                    self.stack.push(a.exp(b));
                }
                Op::ExpImm(pow) => {
                    let pow = pow as u64;
                    let a = pop!(self);
                    assert!(
                        pow < 64,
                        "invalid power of two: expected {pow} to be a value less than 64"
                    );
                    self.stack.push(a.exp(pow));
                }
                Op::Not => {
                    let a = pop_bool!(self);
                    self.stack.push_u8(!a as u8);
                }
                Op::And => {
                    let b = pop_bool!(self);
                    let a = pop_bool!(self);
                    self.stack.push_u8((b & a) as u8);
                }
                Op::AndImm(b) => {
                    let a = pop_bool!(self);
                    self.stack.push_u8((a & b) as u8);
                }
                Op::Or => {
                    let b = pop_bool!(self);
                    let a = pop_bool!(self);
                    self.stack.push_u8((b | a) as u8);
                }
                Op::OrImm(b) => {
                    let a = pop_bool!(self);
                    self.stack.push_u8((a | b) as u8);
                }
                Op::Xor => {
                    let b = pop_bool!(self);
                    let a = pop_bool!(self);
                    self.stack.push_u8((b ^ a) as u8);
                }
                Op::XorImm(b) => {
                    let a = pop_bool!(self);
                    self.stack.push_u8((a ^ b) as u8);
                }
                Op::Eq => comparison!(self, eq),
                Op::EqImm(imm) => comparison!(self, eq, imm.as_int()),
                Op::Neq => comparison!(self, ne),
                Op::NeqImm(imm) => comparison!(self, ne, imm.as_int()),
                Op::Gt => comparison!(self, gt),
                Op::GtImm(imm) => comparison!(self, gt, imm.as_int()),
                Op::Gte => comparison!(self, ge),
                Op::GteImm(imm) => comparison!(self, ge, imm.as_int()),
                Op::Lt => comparison!(self, lt),
                Op::LtImm(imm) => comparison!(self, lt, imm.as_int()),
                Op::Lte => comparison!(self, le),
                Op::LteImm(imm) => comparison!(self, le, imm.as_int()),
                Op::IsOdd => {
                    let a = pop!(self).as_int();
                    self.stack.push_u8((a % 2 == 0) as u8);
                }
                Op::Eqw => {
                    let b = popw!(self);
                    let a = popw!(self);
                    self.stack.push_u8((a == b) as u8);
                }
                Op::Clk => {
                    self.stack.push(Felt::new(self.clk as u64));
                }
                Op::U32Test => {
                    let top = self.stack.peek().expect("operand stack is empty").as_int();
                    self.stack.push_u8((top < U32_P) as u8);
                }
                Op::U32Testw => {
                    let word = self.stack.peekw().expect("operand stack is empty");
                    let is_true = word.iter().all(|elem| elem.as_int() < U32_P);
                    self.stack.push_u8(is_true as u8);
                }
                Op::U32Assert => {
                    let top = self.stack.peek().expect("operand stack is empty").as_int();
                    assert!(top < U32_P, "assertion failed: {top} is larger than 2^32");
                }
                Op::U32Assert2 => {
                    let a = self.stack.peek().expect("operand stack is empty").as_int();
                    let b = self.stack.peek().expect("operand stack is empty").as_int();
                    assert!(a < U32_P, "assertion failed: {a} is larger than 2^32");
                    assert!(b < U32_P, "assertion failed: {b} is larger than 2^32");
                }
                Op::U32Assertw => {
                    let word = self.stack.peekw().expect("operand stack is empty");
                    for elem in word.into_iter() {
                        assert!(
                            elem.as_int() < U32_P,
                            "assertion failed: {elem} is larger than 2^32"
                        );
                    }
                }
                Op::U32Cast => {
                    let a = pop!(self).as_int();
                    self.stack.push(Felt::new(a % U32_P));
                }
                Op::U32Split => {
                    let a = pop!(self).as_int();
                    let hi = a / U32_P;
                    let lo = a % U32_P;
                    self.stack.push(Felt::new(lo));
                    self.stack.push(Felt::new(hi));
                }
                Op::U32CheckedAdd => binop_checked_u32!(self, add),
                Op::U32CheckedAddImm(imm) => binop_checked_u32!(self, add, imm),
                Op::U32OverflowingAdd => binop_overflowing_u32!(self, add),
                Op::U32OverflowingAddImm(imm) => binop_overflowing_u32!(self, add, imm),
                Op::U32WrappingAdd => binop_wrapping_u32!(self, add),
                Op::U32WrappingAddImm(imm) => binop_wrapping_u32!(self, add, imm),
                Op::U32OverflowingAdd3 => todo!(),
                Op::U32WrappingAdd3 => todo!(),
                Op::U32CheckedSub => binop_checked_u32!(self, sub),
                Op::U32CheckedSubImm(imm) => binop_checked_u32!(self, sub, imm),
                Op::U32OverflowingSub => binop_overflowing_u32!(self, sub),
                Op::U32OverflowingSubImm(imm) => binop_overflowing_u32!(self, sub, imm),
                Op::U32WrappingSub => binop_wrapping_u32!(self, sub),
                Op::U32WrappingSubImm(imm) => binop_wrapping_u32!(self, sub, imm),
                Op::U32CheckedMul => binop_checked_u32!(self, mul),
                Op::U32CheckedMulImm(imm) => binop_checked_u32!(self, mul, imm),
                Op::U32OverflowingMul => binop_overflowing_u32!(self, mul),
                Op::U32OverflowingMulImm(imm) => binop_overflowing_u32!(self, mul, imm),
                Op::U32WrappingMul => binop_wrapping_u32!(self, mul),
                Op::U32WrappingMulImm(imm) => binop_wrapping_u32!(self, mul, imm),
                Op::U32OverflowingMadd => {
                    let b = pop_u32!(self) as u64;
                    let a = pop_u32!(self) as u64;
                    let c = pop_u32!(self) as u64;
                    let result = a * b + c;
                    let d = result % 2u64.pow(32);
                    let e = result / 2u64.pow(32);
                    self.stack.push(Felt::new(d));
                    self.stack.push(Felt::new(e));
                }
                Op::U32WrappingMadd => {
                    let b = pop_u32!(self) as u64;
                    let a = pop_u32!(self) as u64;
                    let c = pop_u32!(self) as u64;
                    let d = (a * b + c) % 2u64.pow(32);
                    self.stack.push(Felt::new(d));
                }
                Op::U32CheckedDiv => binop_checked_u32!(self, div),
                Op::U32CheckedDivImm(imm) => binop_checked_u32!(self, div, imm),
                Op::U32UncheckedDiv => binop!(self, div),
                Op::U32UncheckedDivImm(imm) => binop!(self, div, Felt::new(imm as u64)),
                Op::U32CheckedMod => binop_checked_u32!(self, rem),
                Op::U32CheckedModImm(imm) => binop_checked_u32!(self, rem, imm),
                Op::U32UncheckedMod => {
                    let b = pop!(self).as_int();
                    let a = pop!(self).as_int();
                    self.stack.push(Felt::new(a % b));
                }
                Op::U32UncheckedModImm(imm) => {
                    let a = pop!(self).as_int();
                    self.stack.push(Felt::new(a % imm as u64));
                }
                Op::U32CheckedDivMod => {
                    let b = pop_u32!(self);
                    let a = pop_u32!(self);
                    self.stack.push_u32(a / b);
                    self.stack.push_u32(a % b);
                }
                Op::U32CheckedDivModImm(imm) => {
                    let a = pop_u32!(self);
                    self.stack.push_u32(a / imm);
                    self.stack.push_u32(a % imm);
                }
                Op::U32UncheckedDivMod => {
                    let b = pop!(self).as_int();
                    let a = pop!(self).as_int();
                    self.stack.push(Felt::new(a / b));
                    self.stack.push(Felt::new(a % b));
                }
                Op::U32UncheckedDivModImm(b) => {
                    let b = b as u64;
                    let a = pop!(self).as_int();
                    self.stack.push(Felt::new(a / b));
                    self.stack.push(Felt::new(a % b));
                }
                Op::U32And => binop32!(self, bitand),
                Op::U32Or => binop32!(self, bitor),
                Op::U32Xor => binop32!(self, bitxor),
                Op::U32Not => {
                    let a = pop_u32!(self);
                    self.stack.push_u32(!a);
                }
                Op::U32CheckedShl => binop_checked_u32!(self, shl),
                Op::U32CheckedShlImm(imm) => binop_checked_u32!(self, shl, imm),
                Op::U32UncheckedShl => binop_wrapping_u32!(self, shl),
                Op::U32UncheckedShlImm(imm) => binop_wrapping_u32!(self, shl, imm),
                Op::U32CheckedShr => binop_checked_u32!(self, shr),
                Op::U32CheckedShrImm(imm) => binop_checked_u32!(self, shr, imm),
                Op::U32UncheckedShr => binop_wrapping_u32!(self, shr),
                Op::U32UncheckedShrImm(imm) => binop_wrapping_u32!(self, shr, imm),
                Op::U32CheckedRotl => binop32!(self, rotate_left),
                Op::U32CheckedRotlImm(imm) => binop32!(self, rotate_left, imm),
                Op::U32UncheckedRotl => {
                    let b = pop_u32!(self);
                    let a = pop!(self).as_int();
                    self.stack.push(Felt::new(a.rotate_left(b)));
                }
                Op::U32UncheckedRotlImm(imm) => {
                    let a = pop!(self).as_int();
                    self.stack.push(Felt::new(a.rotate_left(imm)));
                }
                Op::U32CheckedRotr => binop32!(self, rotate_right),
                Op::U32CheckedRotrImm(imm) => binop32!(self, rotate_right, imm),
                Op::U32UncheckedRotr => {
                    let b = pop_u32!(self);
                    let a = pop!(self).as_int();
                    self.stack.push(Felt::new(a.rotate_right(b)));
                }
                Op::U32UncheckedRotrImm(imm) => {
                    let a = pop!(self).as_int();
                    self.stack.push(Felt::new(a.rotate_right(imm)));
                }
                Op::U32CheckedPopcnt => {
                    let a = pop_u32!(self);
                    self.stack.push_u32(a.count_ones());
                }
                Op::U32UncheckedPopcnt => {
                    let a = pop!(self).as_int();
                    self.stack.push_u32(a.count_ones());
                }
                Op::U32Eq => comparison32!(self, eq),
                Op::U32EqImm(imm) => comparison32!(self, eq, imm),
                Op::U32Neq => comparison32!(self, ne),
                Op::U32NeqImm(imm) => comparison32!(self, ne, imm),
                Op::U32CheckedGt => comparison32!(self, gt),
                Op::U32UncheckedGt => comparison!(self, gt),
                Op::U32CheckedGte => comparison32!(self, ge),
                Op::U32UncheckedGte => comparison!(self, ge),
                Op::U32CheckedLt => comparison32!(self, lt),
                Op::U32UncheckedLt => comparison!(self, lt),
                Op::U32CheckedLte => comparison32!(self, le),
                Op::U32UncheckedLte => comparison!(self, le),
                Op::U32CheckedMin => {
                    let b = pop_u32!(self);
                    let a = pop_u32!(self);
                    self.stack.push_u32(cmp::min(a, b));
                }
                Op::U32UncheckedMin => {
                    let b = pop!(self).as_int();
                    let a = pop!(self).as_int();
                    self.stack.push(Felt::new(cmp::min(a, b)));
                }
                Op::U32CheckedMax => {
                    let b = pop_u32!(self);
                    let a = pop_u32!(self);
                    self.stack.push_u32(cmp::max(a, b));
                }
                Op::U32UncheckedMax => {
                    let b = pop!(self).as_int();
                    let a = pop!(self).as_int();
                    self.stack.push(Felt::new(cmp::max(a, b)));
                }
            }
        }

        match jump {
            Jump::Return => {
                if self.callstack.is_empty() {
                    Ok(Action::Halt)
                } else {
                    Ok(Action::Return)
                }
            }
            _ => {
                // Suspend the current activation record
                self.callstack.push(state);

                Ok(Action::Suspend)
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Action {
    /// All code has been executed, so stop the emulator
    Halt,
    /// The step function executed an instruction in the current
    /// function and suspended the execution state until the next resumption.
    Suspend,
    /// The step function returned from a callee function without
    /// executing any new instructions, so the emulator loop
    /// should resume the caller immediately
    Return,
}
