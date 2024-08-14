use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, VecDeque},
    rc::Rc,
};

use miden_assembly::Library as CompiledLibrary;
use miden_core::{Program, StackInputs, Word};
use miden_processor::{
    AdviceInputs, ContextId, ExecutionError, Felt, MastForest, MemAdviceProvider, Process,
    ProcessState, RowIndex, StackOutputs, VmState, VmStateIterator,
};
use midenc_codegen_masm::NativePtr;
use midenc_hir::{TraceEvent, Type};
use midenc_session::Session;

use crate::{debug::CallStack, felt::PopFromStack, TestFelt, TestHost};

pub struct ExecutionState {
    pub iter: VmStateIterator,
    pub result: Result<StackOutputs, ExecutionError>,
    pub contexts: BTreeSet<miden_processor::ContextId>,
    pub root_context: miden_processor::ContextId,
    pub current_context: miden_processor::ContextId,
    pub callstack: CallStack,
    pub recent: VecDeque<miden_core::Operation>,
    pub last: Option<VmState>,
    pub cycle: usize,
    pub stopped: bool,
}
impl ExecutionState {
    pub fn step(&mut self) -> Result<(), ExecutionError> {
        if self.stopped {
            return Ok(());
        }
        match self.iter.next() {
            Some(Ok(state)) => {
                self.cycle += 1;
                if self.current_context != state.ctx {
                    self.contexts.insert(state.ctx);
                    self.current_context = state.ctx;
                }

                if let Some(op) = state.op {
                    if self.recent.len() == 5 {
                        self.recent.pop_front();
                    }
                    self.recent.push_back(op);
                }

                self.callstack.next(&state);

                self.last = Some(state);

                Ok(())
            }
            Some(Err(err)) => Err(err),
            None => {
                self.stopped = true;
                Ok(())
            }
        }
    }

    pub fn into_execution_trace(self) -> MidenExecutionTrace {
        let last_cycle = self.cycle;
        let (_, _, _, chiplets, _) = self.iter.into_parts();
        let outputs = self
            .result
            .map(|res| res.stack().iter().copied().map(TestFelt).collect::<VecDeque<_>>())
            .unwrap_or_default();
        MidenExecutionTrace {
            root_context: self.root_context,
            last_cycle: RowIndex::from(last_cycle),
            chiplets: Chiplets::new(move |context, clk| chiplets.get_mem_state_at(context, clk)),
            outputs,
        }
    }
}
impl core::iter::FusedIterator for ExecutionState {}
impl Iterator for ExecutionState {
    type Item = Result<VmState, ExecutionError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.stopped {
            return None;
        }
        match self.step() {
            Ok(_) => self.last.clone().map(Ok),
            Err(err) => Some(Err(err)),
        }
    }
}

// Dirty, gross, horrible hack until miden_processor::chiplets::Chiplets is exported
#[allow(clippy::type_complexity)]
pub struct Chiplets(Box<dyn Fn(ContextId, RowIndex) -> Vec<(u64, Word)>>);
impl Chiplets {
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(ContextId, RowIndex) -> Vec<(u64, Word)> + 'static,
    {
        Self(Box::new(callback))
    }

    pub fn get_mem_state_at(&self, context: ContextId, clk: RowIndex) -> Vec<(u64, Word)> {
        (self.0)(context, clk)
    }
}

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

    pub fn into_execution_state(mut self, program: &Program, session: &Session) -> ExecutionState {
        let advice_provider = MemAdviceProvider::from(self.advice);
        let mut host = TestHost::new(advice_provider);
        for lib in core::mem::take(&mut self.libraries) {
            host.load_mast_forest(lib);
        }

        let trace_events: Rc<RefCell<BTreeMap<RowIndex, TraceEvent>>> = Rc::new(Default::default());
        let frame_start_events = Rc::clone(&trace_events);
        host.register_trace_handler(TraceEvent::FrameStart, move |clk, event| {
            frame_start_events.borrow_mut().insert(clk, event);
        });
        let frame_end_events = Rc::clone(&trace_events);
        host.register_trace_handler(TraceEvent::FrameEnd, move |clk, event| {
            frame_end_events.borrow_mut().insert(clk, event);
        });
        let assertion_events = Rc::clone(&trace_events);
        host.register_assert_failed_tracer(move |clk, event| {
            assertion_events.borrow_mut().insert(clk, event);
        });

        let mut process = Process::new_debug(program.kernel().clone(), self.stack, host);
        let root_context = process.ctx();
        let result = process.execute(program);
        let mut iter = VmStateIterator::new(process, result.clone());
        let mut callstack = CallStack::new(trace_events);
        ExecutionState {
            iter,
            result,
            contexts: Default::default(),
            root_context,
            current_context: root_context,
            callstack,
            recent: VecDeque::with_capacity(5),
            last: None,
            cycle: 0,
            stopped: false,
        }
    }

    /// Execute the given program until termination, producing a trace
    pub fn capture_trace(mut self, program: &Program, session: &Session) -> MidenExecutionTrace {
        let mut execution_state = self.into_execution_state(program, session);
        while let Some(step) = execution_state.next() {
            if step.is_err() {
                return execution_state.into_execution_trace();
            }
        }
        execution_state.into_execution_trace()
    }

    /// Execute the given program, producing a trace
    #[track_caller]
    pub fn execute(mut self, program: &Program, session: &Session) -> MidenExecutionTrace {
        let mut execution_state = self.into_execution_state(program, session);
        while let Some(step) = execution_state.next() {
            if let Err(err) = step {
                render_execution_error(err, &execution_state, session);
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
        }

        execution_state.into_execution_trace()
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

#[derive(Debug, thiserror::Error)]
pub enum MemoryReadError {
    #[error("attempted to read beyond end of linear memory")]
    OutOfBounds,
    #[error("unaligned reads are not supported yet")]
    UnalignedRead,
}

pub struct MidenExecutionTrace {
    root_context: ContextId,
    last_cycle: RowIndex,
    chiplets: Chiplets,
    outputs: VecDeque<TestFelt>,
}
impl MidenExecutionTrace {
    pub fn parse_result<T>(&self) -> Option<T>
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
    pub fn read_memory_word(&self, addr: u32) -> Option<Word> {
        self.read_memory_word_in_context(addr, self.root_context, self.last_cycle)
    }

    pub fn read_memory_word_in_context(
        &self,
        addr: u32,
        ctx: ContextId,
        clk: RowIndex,
    ) -> Option<Word> {
        use miden_core::FieldElement;

        let words = self.chiplets.get_mem_state_at(ctx, clk);
        let addr = addr as u64;
        match words.binary_search_by_key(&addr, |item| item.0) {
            Ok(index) => Some(words[index].1),
            Err(_) => Some([Felt::ZERO; 4]),
        }
    }

    /// Read the word at the given Miden memory address and element offset
    #[track_caller]
    pub fn read_memory_element(&self, addr: u32, index: u8) -> Option<Felt> {
        self.read_memory_element_in_context(addr, index, self.root_context, self.last_cycle)
    }

    #[track_caller]
    pub fn read_memory_element_in_context(
        &self,
        addr: u32,
        index: u8,
        ctx: ContextId,
        clk: RowIndex,
    ) -> Option<Felt> {
        assert!(index < 4, "invalid element index");
        self.read_memory_word_in_context(addr, ctx, clk)
            .map(|word| word[index as usize])
    }

    pub fn read_bytes_for_type(
        &self,
        addr: NativePtr,
        ty: &Type,
        ctx: ContextId,
        clk: RowIndex,
    ) -> Result<Vec<u8>, MemoryReadError> {
        const U32_MASK: u64 = u32::MAX as u64;
        let size = ty.size_in_bytes();
        let mut buf = Vec::with_capacity(size);

        let size_in_words = ty.size_in_words();
        let mut elems = Vec::with_capacity(size_in_words);

        if addr.is_word_aligned() {
            for i in 0..size_in_words {
                let addr = addr.waddr.checked_add(i as u32).ok_or(MemoryReadError::OutOfBounds)?;
                elems.extend(self.read_memory_word_in_context(addr, ctx, clk).unwrap_or_default());
            }
        } else if addr.is_element_aligned() {
            let leading =
                self.read_memory_word_in_context(addr.waddr, ctx, clk).unwrap_or_default();
            for item in leading.into_iter().skip(addr.index as usize) {
                elems.push(item);
            }
            for i in 1..size_in_words {
                let addr = addr.waddr.checked_add(i as u32).ok_or(MemoryReadError::OutOfBounds)?;
                elems.extend(self.read_memory_word_in_context(addr, ctx, clk).unwrap_or_default());
            }
            let trailing_addr = addr
                .waddr
                .checked_add(size_in_words as u32)
                .ok_or(MemoryReadError::OutOfBounds)?;
            let trailing =
                self.read_memory_word_in_context(trailing_addr, ctx, clk).unwrap_or_default();
            for item in trailing.into_iter().take(4 - addr.index as usize) {
                elems.push(item);
            }
        } else {
            return Err(MemoryReadError::UnalignedRead);
        }

        let mut needed = size - buf.len();
        for elem in elems {
            let bytes = ((elem.as_int() & U32_MASK) as u32).to_be_bytes();
            let take = core::cmp::min(needed, 4);
            buf.extend(&bytes[0..take]);
            needed -= take;
        }

        Ok(buf)
    }

    /// Read a value of the given type, given an address in Rust's address space
    #[track_caller]
    pub fn read_from_rust_memory<T>(&self, addr: u32) -> Option<T>
    where
        T: core::any::Any + PopFromStack,
    {
        self.read_from_rust_memory_in_context(addr, self.root_context, self.last_cycle)
    }

    #[track_caller]
    pub fn read_from_rust_memory_in_context<T>(
        &self,
        addr: u32,
        ctx: ContextId,
        clk: RowIndex,
    ) -> Option<T>
    where
        T: core::any::Any + PopFromStack,
    {
        use core::any::TypeId;

        let ptr = NativePtr::from_ptr(addr);
        if TypeId::of::<T>() == TypeId::of::<Felt>() {
            assert_eq!(ptr.offset, 0, "cannot read values of type Felt from unaligned addresses");
            let elem = self.read_memory_element_in_context(ptr.waddr, ptr.index, ctx, clk)?;
            let mut stack = VecDeque::from([TestFelt(elem)]);
            return Some(T::try_pop(&mut stack).unwrap_or_else(|| {
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
                let elem = self.read_memory_element_in_context(ptr.waddr, ptr.index, ctx, clk)?;
                let elem = if ptr.offset > 0 {
                    let mask = 2u64.pow(32 - (ptr.offset as u32 * 8)) - 1;
                    let elem = elem.as_int() & mask;
                    Felt::new(elem << (ptr.offset as u64 * 8))
                } else {
                    elem
                };
                let mut stack = VecDeque::from([TestFelt(elem)]);
                Some(T::try_pop(&mut stack).unwrap_or_else(|| {
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
                let elem = self.read_memory_element_in_context(ptr.waddr, ptr.index, ctx, clk)?;
                let mut stack = VecDeque::from([TestFelt(elem)]);
                Some(T::try_pop(&mut stack).unwrap_or_else(|| {
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
                let word = self.read_memory_word_in_context(ptr.waddr, ctx, clk)?;
                let mut stack = VecDeque::from_iter(word.into_iter().map(TestFelt));
                Some(T::try_pop(&mut stack).unwrap_or_else(|| {
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
                            .read_memory_word_in_context(ptr.waddr + i as u32, ctx, clk)
                            .expect("invalid memory access");
                        buf.extend(word.into_iter().map(TestFelt));
                    }
                }
                Some(T::try_pop(&mut buf).unwrap_or_else(|| {
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

#[track_caller]
fn render_execution_error(
    err: ExecutionError,
    execution_state: &ExecutionState,
    session: &Session,
) -> ! {
    use midenc_hir::diagnostics::{miette::miette, reporting::PrintDiagnostic, LabeledSpan};

    let stacktrace = execution_state.callstack.stacktrace(&execution_state.recent, session);

    eprintln!("{stacktrace}");

    if let Some(last_state) = execution_state.last.as_ref() {
        let stack = last_state.stack.iter().map(|elem| elem.as_int());
        let stack = midenc_hir::DisplayValues::new(stack);
        let fmp = last_state.fmp.as_int();
        eprintln!(
            "\nLast Known State (at most recent instruction which succeeded):
 | Frame Pointer: {fmp} (starts at 2^30)
 | Operand Stack: [{stack}]
 "
        );

        let mut labels = vec![];
        if let Some(span) = stacktrace
            .current_frame()
            .and_then(|frame| frame.location.as_ref())
            .map(|loc| loc.span)
        {
            labels.push(LabeledSpan::new_with_span(
                None,
                span.start().to_usize()..span.end().to_usize(),
            ));
        }
        let report = miette!(
            labels = labels,
            "program execution failed at step {step} (cycle {cycle}): {err}",
            step = execution_state.cycle,
            cycle = last_state.clk,
        );
        let report = match stacktrace
            .current_frame()
            .and_then(|frame| frame.location.as_ref())
            .map(|loc| loc.source_file.clone())
        {
            Some(source) => report.with_source_code(source),
            None => report,
        };

        panic!("{}", PrintDiagnostic::new(report));
    } else {
        panic!("program execution failed at step {step}: {err}", step = execution_state.cycle);
    }
}
