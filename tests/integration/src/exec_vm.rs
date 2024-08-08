use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    rc::Rc,
};

use miden_assembly::library::Library as CompiledLibrary;
use miden_core::{Program, StackInputs};
use miden_processor::{AdviceInputs, DefaultHost, ExecutionError, MastForest, Process};
use midenc_hir::Felt;
use midenc_session::Session;

use crate::{
    compiler_test::demangle,
    felt_conversion::{PopFromStack, TestFelt},
};

/// A test executor for Miden VM tests
pub struct MidenExecutor {
    stack: StackInputs,
    advice: AdviceInputs,
    libraries: Vec<MastForest>,
}
impl MidenExecutor {
    /// Construct an executor with the given arguments on the operand stack
    pub fn new(args: Vec<Felt>) -> Self {
        Self {
            stack: StackInputs::new(args).expect("invalid stack inputs"),
            advice: AdviceInputs::default(),
            libraries: Default::default(),
        }
    }

    /// Set the contents of memory for the shadow stack frame of the entrypoint
    pub fn with_advice_inputs(&mut self, advice: AdviceInputs) -> &mut Self {
        self.advice.extend(advice);
        self
    }

    /// Add a [CompiledLibrary] to the execution context
    pub fn with_library(&mut self, lib: &CompiledLibrary) -> &mut Self {
        self.libraries.push(lib.mast_forest().clone());
        self
    }

    /// Execute the given program, producing a trace
    #[track_caller]
    pub fn execute(mut self, program: &Program, session: &Session) -> MidenExecutionTrace {
        use miden_processor::{MemAdviceProvider, ProcessState, VmStateIterator};

        let advice_provider = MemAdviceProvider::from(self.advice);
        let mut host = DefaultHost::new(advice_provider);
        for lib in core::mem::take(&mut self.libraries) {
            host.load_mast_forest(lib);
        }
        let mut process = Process::new_debug(program.kernel().clone(), self.stack, host);
        let root_context = process.ctx();
        let result = process.execute(program);
        let mut iter = VmStateIterator::new(process, result.clone());
        let mut contexts = BTreeSet::default();
        let mut callstack = CallStack::default();
        let mut recent_ops = VecDeque::with_capacity(5);
        let mut last_state: Option<miden_processor::VmState> = None;
        for (i, state) in iter.by_ref().enumerate() {
            match state {
                Ok(state) => {
                    if let Some(op) = state.op {
                        if recent_ops.len() == 5 {
                            recent_ops.pop_front();
                        }
                        recent_ops.push_back(op);
                    }
                    contexts.insert(state.ctx);
                    if let Some(op) = state.asmop.as_ref() {
                        callstack.next_op(state.clk.into(), op);
                    } else {
                        match state.op {
                            Some(
                                miden_core::Operation::Join
                                | miden_core::Operation::Split
                                | miden_core::Operation::Span
                                | miden_core::Operation::Respan
                                | miden_core::Operation::End
                                | miden_core::Operation::Noop,
                            ) => (),
                            Some(op) => {
                                callstack.next_opcode(state.clk.into(), op);
                            }
                            None => (),
                        }
                    }
                    /*
                    if let Some(op) = state.op {
                        match op {
                            miden_core::Operation::MLoad => {
                                let load_addr = last_state
                                    .as_ref()
                                    .map(|state| state.stack[0].as_int())
                                    .unwrap();
                                let loaded = match state
                                    .memory
                                    .binary_search_by_key(&load_addr, |&(addr, _)| addr)
                                {
                                    Ok(index) => state.memory[index].1[0].as_int(),
                                    Err(_) => 0,
                                };
                                //dbg!(load_addr, loaded, format!("{loaded:08x}"));
                            }
                            miden_core::Operation::MLoadW => {
                                let load_addr = last_state
                                    .as_ref()
                                    .map(|state| state.stack[0].as_int())
                                    .unwrap();
                                let loaded = match state
                                    .memory
                                    .binary_search_by_key(&load_addr, |&(addr, _)| addr)
                                {
                                    Ok(index) => {
                                        let word = state.memory[index].1;
                                        [
                                            word[0].as_int(),
                                            word[1].as_int(),
                                            word[2].as_int(),
                                            word[3].as_int(),
                                        ]
                                    }
                                    Err(_) => [0; 4],
                                };
                                let loaded_bytes = {
                                    let word = loaded;
                                    let a = (word[0] as u32).to_be_bytes();
                                    let b = (word[1] as u32).to_be_bytes();
                                    let c = (word[2] as u32).to_be_bytes();
                                    let d = (word[3] as u32).to_be_bytes();
                                    let bytes = [
                                        a[0], a[1], a[2], a[3], b[0], b[1], b[2], b[3], c[0], c[1],
                                        c[2], c[3], d[0], d[1], d[2], d[3],
                                    ];
                                    u128::from_be_bytes(bytes)
                                };
                                //dbg!(load_addr, loaded, format!("{loaded_bytes:032x}"));
                            }
                            miden_core::Operation::MStore => {
                                let store_addr = last_state
                                    .as_ref()
                                    .map(|state| state.stack[0].as_int())
                                    .unwrap();
                                let stored = match state
                                    .memory
                                    .binary_search_by_key(&store_addr, |&(addr, _)| addr)
                                {
                                    Ok(index) => state.memory[index].1[0].as_int(),
                                    Err(_) => 0,
                                };
                                //dbg!(store_addr, stored, format!("{stored:08x}"));
                            }
                            miden_core::Operation::MStoreW => {
                                let store_addr = last_state
                                    .as_ref()
                                    .map(|state| state.stack[0].as_int())
                                    .unwrap();
                                let stored = {
                                    let memory = state
                                        .memory
                                        .iter()
                                        .find_map(|(addr, word)| {
                                            if addr == &store_addr {
                                                Some(word)
                                            } else {
                                                None
                                            }
                                        })
                                        .unwrap();
                                    let a = memory[0].as_int();
                                    let b = memory[1].as_int();
                                    let c = memory[2].as_int();
                                    let d = memory[3].as_int();
                                    [a, b, c, d]
                                };
                                let stored_bytes = {
                                    let word = stored;
                                    let a = (word[0] as u32).to_be_bytes();
                                    let b = (word[1] as u32).to_be_bytes();
                                    let c = (word[2] as u32).to_be_bytes();
                                    let d = (word[3] as u32).to_be_bytes();
                                    let bytes = [
                                        a[0], a[1], a[2], a[3], b[0], b[1], b[2], b[3], c[0], c[1],
                                        c[2], c[3], d[0], d[1], d[2], d[3],
                                    ];
                                    u128::from_be_bytes(bytes)
                                };
                                //dbg!(store_addr, stored, format!("{stored_bytes:032x}"));
                            }
                            _ => (),
                        }
                    }
                    */
                    last_state = Some(state);
                }
                Err(err) => {
                    render_execution_error(
                        err,
                        i,
                        &callstack,
                        &recent_ops,
                        last_state.as_ref(),
                        session,
                    );
                }
            }
        }
        let (_, _, _, chiplets, _) = iter.into_parts();
        let mut memories = std::collections::BTreeMap::default();
        for context in contexts {
            let mem = chiplets.get_mem_state_at(
                context,
                last_state
                    .as_ref()
                    .map(|state| state.clk)
                    .unwrap_or(miden_processor::RowIndex::from(0)),
            );
            memories.insert(context, mem);
        }
        let outputs = result.unwrap().stack().iter().copied().map(TestFelt).collect();
        MidenExecutionTrace {
            root_context,
            outputs,
            memories,
        }
    }

    /// Execute a program, parsing the operand stack outputs as a value of type `T`
    pub fn execute_into<T>(self, program: &Program, session: &Session) -> T
    where
        T: PopFromStack + PartialEq,
    {
        let out = self.execute(program, session);
        out.parse_result().expect("invalid result")
    }
}

pub struct MidenExecutionTrace {
    root_context: miden_processor::ContextId,
    outputs: VecDeque<TestFelt>,
    memories: BTreeMap<miden_processor::ContextId, Vec<(u64, [Felt; 4])>>,
}
impl MidenExecutionTrace {
    pub fn parse_result<T>(&self) -> Result<T, ()>
    where
        T: PopFromStack,
    {
        let mut stack = self.outputs.clone();
        T::try_pop(&mut stack)
    }

    #[inline]
    pub fn into_outputs(self) -> VecDeque<TestFelt> {
        self.outputs
    }

    /// Read the word at the given Miden memory address
    pub fn read_memory_word(&self, addr: u32) -> Option<[Felt; 4]> {
        use miden_core::FieldElement;

        let words = self.memories.get(&self.root_context)?;
        let addr = addr as u64;
        match words.binary_search_by_key(&addr, |item| item.0) {
            Ok(index) => Some(words[index].1),
            Err(_) => Some([Felt::ZERO; 4]),
        }
    }

    /// Read the word at the given Miden memory address and element offset
    #[track_caller]
    pub fn read_memory_element(&self, addr: u32, index: u8) -> Option<Felt> {
        assert!(index < 4, "invalid element index");
        self.read_memory_word(addr).map(|word| word[index as usize])
    }

    /// Read a value of the given type, given an address in Rust's address space
    #[track_caller]
    pub fn read_from_rust_memory<T>(&self, addr: u32) -> Option<T>
    where
        T: core::any::Any + PopFromStack,
    {
        use core::any::TypeId;

        use midenc_codegen_masm::NativePtr;

        let ptr = NativePtr::from_ptr(addr);
        if TypeId::of::<T>() == TypeId::of::<Felt>() {
            assert_eq!(ptr.offset, 0, "cannot read values of type Felt from unaligned addresses");
            let elem = self.read_memory_element(ptr.waddr, ptr.index)?;
            let mut stack = VecDeque::from([TestFelt(elem)]);
            return Some(T::try_pop(&mut stack).unwrap_or_else(|_| {
                panic!(
                    "could not decode a value of type {} from {}",
                    core::any::type_name::<T>(),
                    addr
                )
            }));
        }
        match core::mem::size_of::<T>() {
            n if n < 4 => {
                if (4 - ptr.offset as usize) < n {
                    todo!("unaligned, split read")
                }
                let elem = self.read_memory_element(ptr.waddr, ptr.index)?;
                let elem = if ptr.offset > 0 {
                    let mask = 2u64.pow(32 - (ptr.offset as u32 * 8)) - 1;
                    let elem = elem.as_int() & mask;
                    Felt::new(elem << (ptr.offset as u64 * 8))
                } else {
                    elem
                };
                let mut stack = VecDeque::from([TestFelt(elem)]);
                Some(T::try_pop(&mut stack).unwrap_or_else(|_| {
                    panic!(
                        "could not decode a value of type {} from {}",
                        core::any::type_name::<T>(),
                        addr
                    )
                }))
            }
            4 if ptr.offset > 0 => {
                todo!("unaligned, split read")
            }
            4 => {
                let elem = self.read_memory_element(ptr.waddr, ptr.index)?;
                let mut stack = VecDeque::from([TestFelt(elem)]);
                Some(T::try_pop(&mut stack).unwrap_or_else(|_| {
                    panic!(
                        "could not decode a value of type {} from {}",
                        core::any::type_name::<T>(),
                        addr
                    )
                }))
            }
            n if n <= 16 && ptr.offset > 0 => {
                todo!("unaligned, split read")
            }
            n if n <= 16 => {
                let word = self.read_memory_word(ptr.waddr)?;
                let mut stack = VecDeque::from_iter(word.into_iter().map(TestFelt));
                Some(T::try_pop(&mut stack).unwrap_or_else(|_| {
                    panic!(
                        "could not decode a value of type {} from {}",
                        core::any::type_name::<T>(),
                        addr
                    )
                }))
            }
            n => {
                let mut buf = VecDeque::default();
                let chunks_needed = n / 4;
                if ptr.offset > 0 {
                    todo!()
                } else if ptr.index > 0 {
                    todo!()
                } else {
                    for i in 0..chunks_needed {
                        let word = self
                            .read_memory_word(ptr.waddr + i as u32)
                            .expect("invalid memory access");
                        buf.extend(word.into_iter().map(TestFelt));
                    }
                }
                Some(T::try_pop(&mut buf).unwrap_or_else(|_| {
                    panic!(
                        "could not decode a value of type {} from {}",
                        core::any::type_name::<T>(),
                        addr
                    )
                }))
            }
        }
    }
}

/// Execute the program using the VM with the given arguments
/// Prints the trace (VM state) after each step to stdout
/// Arguments are expected to be in the order they are passed to the entrypoint function
#[allow(unused)]
pub fn execute_vm_tracing(
    program: &Program,
    args: &[Felt],
) -> Result<Vec<TestFelt>, ExecutionError> {
    // Reverse the arguments to counteract the StackInputs::new() reversing them into a stack
    let args_reversed = args.iter().copied().rev().collect();
    let stack_inputs = StackInputs::new(args_reversed).expect("invalid stack inputs");
    let vm_state_iterator =
        miden_processor::execute_iter(program, stack_inputs, DefaultHost::default());
    let mut last_stack = Vec::new();
    for vm_state in vm_state_iterator {
        let vm_state = vm_state?;
        eprintln!("{}", vm_state);
        last_stack.clone_from(&vm_state.stack);
    }
    Ok(last_stack.into_iter().map(TestFelt).collect())
}

#[derive(Default)]
struct CallStack {
    contexts: BTreeSet<Rc<str>>,
    frames: Vec<CallFrame>,
}
impl CallStack {
    pub fn next_opcode(&mut self, cycle: usize, opcode: miden_core::Operation) {
        if let Some(frame) = self.frames.last_mut() {
            frame.push_opcode(cycle, opcode);
        }
    }

    pub fn next_op(&mut self, cycle: usize, state: &miden_processor::AsmOpInfo) {
        // Only handle the first cycle of each op
        if state.cycle_idx() > 1 {
            return;
        }
        let procedure = match self.contexts.get(state.context_name()) {
            Some(name) => Rc::clone(name),
            None => {
                let name = Rc::from(state.context_name().to_string().into_boxed_str());
                self.contexts.insert(Rc::clone(&name));
                name
            }
        };

        // Is the name of the new frame a parent of `current_frame`?
        // If so, then we're returning to the parent, otherwise, we're calling a new proc
        let num_frames = self.frames.len();
        let return_to = if num_frames == 0 {
            None
        } else {
            match self.frames.iter().position(|f| f.procedure == procedure) {
                Some(index) if index + 1 == num_frames => None,
                return_to => return_to,
            }
        };
        let is_return = return_to.is_some();
        if let Some(return_to) = return_to {
            self.frames.truncate(return_to + 1);
        }

        let op = match self.contexts.get(state.op()) {
            Some(cached) => Rc::clone(cached),
            None => {
                let op = Rc::from(state.op().to_string().into_boxed_str());
                self.contexts.insert(Rc::clone(&op));
                op
            }
        };
        match self.frames.last_mut() {
            Some(current_frame) if current_frame.procedure == procedure => {
                let asmop = state.as_ref();
                current_frame.push(cycle, op, asmop);
            }
            prev_frame => {
                assert!(
                    !is_return,
                    "we should only be returning to a procedure with the same name as the current \
                     frame"
                );
                // Inherit the caller context, so that we always have as close to maximum context as
                // possible
                let context = prev_frame.map(|frame| frame.context.clone()).unwrap_or_default();
                let asmop = state.as_ref();
                let mut frame = CallFrame { procedure, context };
                frame.push(cycle, op, asmop);
                self.frames.push(frame);
            }
        }
    }
}

struct CallFrame {
    procedure: Rc<str>,
    context: VecDeque<OpDetail>,
}
impl CallFrame {
    pub fn push(&mut self, cycle: usize, opcode: Rc<str>, op: &miden_core::AssemblyOp) {
        if self.context.len() == 5 {
            self.context.pop_front();
        }

        self.context.push_back(OpDetail::Full {
            opcode,
            location: op.location().cloned(),
            cycle,
        });
    }

    pub fn push_opcode(&mut self, cycle: usize, op: miden_core::Operation) {
        if self.context.len() == 5 {
            self.context.pop_front();
        }

        self.context.push_back(OpDetail::Basic { op, cycle });
    }
}

#[derive(Clone)]
enum OpDetail {
    Full {
        #[allow(dead_code)]
        opcode: Rc<str>,
        location: Option<miden_core::debuginfo::Location>,
        #[allow(dead_code)]
        cycle: usize,
    },
    Basic {
        #[allow(dead_code)]
        op: miden_core::Operation,
        #[allow(dead_code)]
        cycle: usize,
    },
}
impl OpDetail {
    #[allow(dead_code)]
    pub fn opcode(&self) -> Rc<str> {
        match self {
            Self::Full { ref opcode, .. } => Rc::clone(opcode),
            Self::Basic { op, .. } => op.to_string().into_boxed_str().into(),
        }
    }

    pub fn location(&self) -> Option<&miden_core::debuginfo::Location> {
        match self {
            Self::Full { ref location, .. } => location.as_ref(),
            Self::Basic { .. } => None,
        }
    }
}

fn render_execution_error(
    err: ExecutionError,
    step: usize,
    callstack: &CallStack,
    recent_ops: &VecDeque<miden_core::Operation>,
    last_state: Option<&miden_processor::VmState>,
    session: &Session,
) -> ! {
    use std::fmt::Write;

    use midenc_hir::diagnostics::{
        miette::miette, reporting::PrintDiagnostic, LabeledSpan, SourceManagerExt,
    };

    let session_name = session.name();
    let num_frames = callstack.frames.len();
    let mut source_code = None;
    let mut labels = vec![];
    let mut stacktrace = String::new();
    writeln!(&mut stacktrace, "\nStack Trace:").unwrap();
    for (i, frame) in callstack.frames.iter().enumerate() {
        let is_top = i + 1 == num_frames;
        let name = match frame.procedure.split_once("::") {
            Some((module, rest)) if module == session_name.as_str() => demangle(rest),
            _ => demangle(frame.procedure.as_ref()),
        };
        if is_top {
            write!(&mut stacktrace, " `-> {name}").unwrap();
        } else {
            write!(&mut stacktrace, " |-> {name}").unwrap();
        }
        if let Some(loc) = frame.context.back().and_then(|op| op.location()) {
            let path = std::path::Path::new(loc.path.as_ref());
            let loc_source_code = if path.exists() {
                session.source_manager.load_file(path).ok()
            } else {
                session.source_manager.get_by_path(loc.path.as_ref())
            };
            if is_top {
                source_code = loc_source_code.clone();
                labels.push(LabeledSpan::new_with_span(
                    None,
                    loc.start.to_usize()..loc.end.to_usize(),
                ));
            }
            if let Some(source_file) = loc_source_code.as_ref() {
                let span = midenc_hir::SourceSpan::new(source_file.id(), loc.start..loc.end);
                let file_line_col = source_file.location(span);
                let path = file_line_col.path();
                let path = std::path::Path::new(path.as_ref());
                if let Some(filename) = path.file_name().map(std::path::Path::new) {
                    write!(
                        &mut stacktrace,
                        " in {}:{}:{}",
                        filename.display(),
                        file_line_col.line,
                        file_line_col.column
                    )
                    .unwrap();
                } else {
                    write!(
                        &mut stacktrace,
                        " in {}:{}:{}",
                        path.display(),
                        file_line_col.line,
                        file_line_col.column
                    )
                    .unwrap();
                }
            } else {
                write!(&mut stacktrace, " in <unavailable>").unwrap();
            }
        }
        if is_top {
            // Print op context
            //let context_size = frame.context.len();
            let context_size = recent_ops.len();
            writeln!(&mut stacktrace, ":\n\nLast {context_size} Instructions:").unwrap();
            //for (i, op) in frame.context.iter().enumerate() {
            for (i, op) in recent_ops.iter().enumerate() {
                let is_last = i + 1 == context_size;
                if is_last {
                    //writeln!(&mut stacktrace, " `-> {}", &op.opcode()).unwrap();
                    writeln!(&mut stacktrace, " |   {}", &op).unwrap();
                    writeln!(&mut stacktrace, " `-> <error occured here>").unwrap();
                } else {
                    //writeln!(&mut stacktrace, " |   {}", &op.opcode()).unwrap();
                    writeln!(&mut stacktrace, " |   {}", &op).unwrap();
                }
            }
        } else {
            stacktrace.push('\n');
        }
    }

    eprintln!("{stacktrace}");

    if let Some(last_state) = last_state {
        let stack = last_state.stack.iter().map(|elem| elem.as_int());
        let stack = midenc_hir::DisplayValues::new(stack);
        let fmp = last_state.fmp.as_int();
        eprintln!(
            "\nLast Known State (at most recent instruction which succeeded):
 | Frame Pointer: {fmp} (starts at 2^30)
 | Operand Stack: [{stack}]
 "
        );
        let report = miette!(
            labels = labels,
            "program execution failed at step {step} (cycle {cycle}): {err}",
            step = step,
            cycle = last_state.clk,
        );
        let report = match source_code {
            Some(source) => report.with_source_code(source),
            None => report,
        };

        panic!("{}", PrintDiagnostic::new(report));
    } else {
        panic!("program execution failed at step {step}: {err}");
    }
}
