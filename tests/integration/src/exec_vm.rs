use std::{
    borrow::Cow,
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, VecDeque},
    rc::Rc,
    sync::Arc,
};

use miden_assembly::library::CompiledLibrary;
use miden_core::{crypto::hash::RpoDigest, Program, StackInputs};
use miden_processor::{
    AdviceExtractor, AdviceInjector, AdviceInputs, AdviceProvider, DefaultHost, ExecutionError,
    Host, HostResponse, MastForest, MastForestStore, Operation, Process, ProcessState, RowIndex,
};
use midenc_hir::{Felt, TraceEvent};
use midenc_session::Session;

use crate::{
    compiler_test::demangle,
    felt_conversion::{PopFromStack, TestFelt},
};

type TraceHandler = dyn FnMut(RowIndex, TraceEvent);

#[derive(Default)]
struct TestHost {
    adv_provider: miden_processor::MemAdviceProvider,
    store: miden_processor::MemMastForestStore,
    tracing_callbacks: BTreeMap<u32, Vec<Box<TraceHandler>>>,
    on_assert_failed: Option<Box<TraceHandler>>,
}
impl TestHost {
    pub fn new(adv_provider: miden_processor::MemAdviceProvider) -> Self {
        Self {
            adv_provider,
            store: Default::default(),
            tracing_callbacks: Default::default(),
            on_assert_failed: None,
        }
    }

    pub fn register_trace_handler<F>(&mut self, event: TraceEvent, callback: F)
    where
        F: FnMut(RowIndex, TraceEvent) + 'static,
    {
        let key = match event {
            TraceEvent::AssertionFailed(None) => u32::MAX,
            ev => ev.into(),
        };
        self.tracing_callbacks.entry(key).or_default().push(Box::new(callback));
    }

    pub fn register_assert_failed_tracer<F>(&mut self, callback: F)
    where
        F: FnMut(RowIndex, TraceEvent) + 'static,
    {
        self.on_assert_failed = Some(Box::new(callback));
    }

    pub fn load_mast_forest(&mut self, forest: MastForest) {
        self.store.insert(forest);
    }
}
impl Host for TestHost {
    fn get_advice<P: ProcessState>(
        &mut self,
        process: &P,
        extractor: AdviceExtractor,
    ) -> Result<HostResponse, ExecutionError> {
        self.adv_provider.get_advice(process, &extractor)
    }

    fn set_advice<P: ProcessState>(
        &mut self,
        process: &P,
        injector: AdviceInjector,
    ) -> Result<HostResponse, ExecutionError> {
        self.adv_provider.set_advice(process, &injector)
    }

    fn get_mast_forest(&self, node_digest: &RpoDigest) -> Option<Arc<MastForest>> {
        self.store.get(node_digest)
    }

    fn on_trace<S: ProcessState>(
        &mut self,
        process: &S,
        trace_id: u32,
    ) -> Result<HostResponse, ExecutionError> {
        let event = TraceEvent::from(trace_id);
        let clk = process.clk();
        if let Some(handlers) = self.tracing_callbacks.get_mut(&trace_id) {
            for handler in handlers.iter_mut() {
                handler(clk, event);
            }
        }
        Ok(HostResponse::None)
    }

    fn on_assert_failed<S: ProcessState>(&mut self, process: &S, err_code: u32) -> ExecutionError {
        let clk = process.clk();
        if let Some(handler) = self.on_assert_failed.as_mut() {
            handler(clk, TraceEvent::AssertionFailed(core::num::NonZeroU32::new(err_code)));
        }
        let err_msg = match err_code {
            midenc_hir::ASSERT_FAILED_ALIGNMENT => Some(
                "failed alignment: use of memory address violates minimum alignment requirements \
                 for that use"
                    .to_string(),
            ),
            _ => None,
        };
        ExecutionError::FailedAssertion {
            clk,
            err_code,
            err_msg,
        }
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

    /// Execute the given program, producing a trace
    #[track_caller]
    pub fn execute(mut self, program: &Program, session: &Session) -> MidenExecutionTrace {
        use miden_processor::{MemAdviceProvider, VmStateIterator};

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
        let mut contexts = BTreeSet::default();
        let mut callstack = CallStack::new(trace_events);
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
                    callstack.next(&state);
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

#[derive(Debug, Clone)]
struct SpanContext {
    frame_index: usize,
    location: Option<miden_core::debuginfo::Location>,
}

struct CallStack {
    trace_events: Rc<RefCell<BTreeMap<RowIndex, TraceEvent>>>,
    contexts: BTreeSet<Rc<str>>,
    frames: Vec<CallFrame>,
    block_stack: Vec<Option<SpanContext>>,
}
impl CallStack {
    pub fn new(trace_events: Rc<RefCell<BTreeMap<RowIndex, TraceEvent>>>) -> Self {
        Self {
            trace_events,
            contexts: BTreeSet::default(),
            frames: vec![],
            block_stack: vec![],
        }
    }

    pub fn next(&mut self, state: &miden_processor::VmState) {
        use miden_processor::Operation;
        if let Some(op) = state.op {
            // Do not do anything if this cycle is a continuation of the last instruction
            //let skip = state.asmop.as_ref().map(|op| op.cycle_idx() > 1).unwrap_or(false);
            //if skip {
            //return;
            //}

            // Get the current procedure name context, if available
            let procedure =
                state.asmop.as_ref().map(|op| self.cache_procedure_name(op.context_name()));
            /*
                       if procedure.is_none() {
                           dbg!(self.frames.last().map(|frame| frame.procedure.as_deref()));
                           dbg!(self.block_stack.last().map(|ctx| ctx.as_ref()));
                       }
            */
            // Handle trace events for this cycle
            let event = self.trace_events.borrow().get(&state.clk).copied();
            log::trace!("handling {op} at cycle {}: {:?}", state.clk, &event);
            let is_frame_end = self.handle_trace_event(event, procedure.as_ref());

            // These ops we do not record in call frame details
            let ignore = matches!(
                op,
                Operation::Join
                    | Operation::Split
                    | Operation::Span
                    | Operation::Respan
                    | Operation::End
            );

            // Manage block stack
            match op {
                Operation::Span => {
                    if let Some(asmop) = state.asmop.as_ref() {
                        dbg!(asmop);
                        self.block_stack.push(Some(SpanContext {
                            frame_index: self.frames.len().saturating_sub(1),
                            location: asmop.as_ref().location().cloned(),
                        }));
                    } else {
                        self.block_stack.push(None);
                    }
                }
                Operation::End => {
                    self.block_stack.pop();
                }
                Operation::Join | Operation::Split => {
                    self.block_stack.push(None);
                }
                _ => (),
            }

            if ignore || is_frame_end {
                return;
            }

            // Attempt to supply procedure context from the current span context, if needed +
            // available
            let (procedure, asmop) = match procedure {
                proc @ Some(_) => {
                    (proc, state.asmop.as_ref().map(|info| info.as_ref()).map(Cow::Borrowed))
                }
                None => match self.block_stack.last() {
                    Some(Some(span_ctx)) => {
                        let proc =
                            self.frames.get(span_ctx.frame_index).and_then(|f| f.procedure.clone());
                        let info = state
                            .asmop
                            .as_ref()
                            .map(|info| info.as_ref())
                            .map(Cow::Borrowed)
                            .or_else(|| {
                                let context_name =
                                    proc.as_deref().unwrap_or("<unknown>").to_string();
                                let raw_asmop = miden_core::AssemblyOp::new(
                                    span_ctx.location.clone(),
                                    context_name,
                                    1,
                                    op.to_string(),
                                    false,
                                );
                                Some(Cow::Owned(raw_asmop))
                            });
                        (proc, info)
                    }
                    _ => (None, state.asmop.as_ref().map(|info| info.as_ref()).map(Cow::Borrowed)),
                },
            };

            // Use the current frame's procedure context, if no other more precise context is
            // available
            let procedure =
                procedure.or_else(|| self.frames.last().and_then(|f| f.procedure.clone()));

            // Do we have a frame? If not, create one
            if self.frames.is_empty() {
                self.frames.push(CallFrame::new(procedure.clone()));
            }

            let current_frame = self.frames.last_mut().unwrap();

            // Does the current frame have a procedure context/location? Use the one from this op if
            // so
            let procedure_context_updated =
                current_frame.procedure.is_none() && procedure.is_some();
            if procedure_context_updated {
                current_frame.procedure.clone_from(&procedure);
            }

            // If this is the frame pointer prologue/epilogue drop the last op, which should be a
            // push
            if matches!(op, Operation::FmpUpdate) {
                current_frame.context.pop_back();
            }

            // Push op into call frame if this is any op other than `nop` or frame setup
            if !matches!(op, Operation::Noop | Operation::FmpUpdate) {
                let cycle_idx = state.asmop.as_ref().map(|info| info.cycle_idx()).unwrap_or(1);
                current_frame.push(op, cycle_idx, asmop.as_deref());
            }

            // Check if we should also update the caller frame's exec detail
            let num_frames = self.frames.len();
            if procedure_context_updated && num_frames > 1 {
                let caller_frame = &mut self.frames[num_frames - 2];
                if let Some(OpDetail::Exec { ref mut callee }) = caller_frame.context.back_mut() {
                    if callee.is_none() {
                        *callee = procedure;
                    }
                }
            }
        }
    }

    // Get or cache procedure name/context as `Rc<str>`
    fn cache_procedure_name(&mut self, context_name: &str) -> Rc<str> {
        match self.contexts.get(context_name) {
            Some(name) => Rc::clone(name),
            None => {
                let name = Rc::from(context_name.to_string().into_boxed_str());
                self.contexts.insert(Rc::clone(&name));
                name
            }
        }
    }

    fn handle_trace_event(
        &mut self,
        event: Option<TraceEvent>,
        procedure: Option<&Rc<str>>,
    ) -> bool {
        // Do we need to handle any frame events?
        if let Some(event) = event {
            match event {
                TraceEvent::FrameStart => {
                    // Record the fact that we exec'd a new procedure in the op context
                    if let Some(current_frame) = self.frames.last_mut() {
                        current_frame.push_exec(procedure.cloned());
                    }
                    // Push a new frame
                    self.frames.push(CallFrame::new(procedure.cloned()));
                }
                TraceEvent::Unknown(code) => log::debug!("unknown trace event: {code}"),
                TraceEvent::FrameEnd => {
                    self.frames.pop();
                    return true;
                }
                _ => (),
            }
        }
        false
    }
}

struct CallFrame {
    procedure: Option<Rc<str>>,
    context: VecDeque<OpDetail>,
    display_name: std::cell::OnceCell<Rc<str>>,
}
impl CallFrame {
    pub fn new(procedure: Option<Rc<str>>) -> Self {
        Self {
            procedure,
            context: Default::default(),
            display_name: Default::default(),
        }
    }

    pub fn procedure(&self, strip_prefix: &str) -> Option<Rc<str>> {
        self.procedure.as_ref()?;
        let name = self.display_name.get_or_init(|| {
            let name = self.procedure.as_deref().unwrap();
            let name = match name.split_once("::") {
                Some((module, rest)) if module == strip_prefix => demangle(rest),
                _ => demangle(name),
            };
            Rc::from(name.into_boxed_str())
        });
        Some(Rc::clone(name))
    }

    pub fn push_exec(&mut self, callee: Option<Rc<str>>) {
        if self.context.len() == 5 {
            self.context.pop_front();
        }

        self.context.push_back(OpDetail::Exec { callee });
    }

    pub fn push(&mut self, opcode: Operation, cycle_idx: u8, op: Option<&miden_core::AssemblyOp>) {
        if cycle_idx > 1 {
            // Should we ignore this op?
            let skip = self.context.back().map(|detail| matches!(detail, OpDetail::Full { op, .. } | OpDetail::Basic { op } if op == &opcode)).unwrap_or(false);
            if skip {
                return;
            }
        }

        if self.context.len() == 5 {
            self.context.pop_front();
        }

        match op {
            Some(op) => {
                let location = op.location().cloned();
                self.context.push_back(OpDetail::Full {
                    op: opcode,
                    location,
                });
            }
            None => {
                // If this instruction does not have a location, inherit the location
                // of the previous op in the frame, if one is present
                if let Some(loc) = self.context.back().map(|op| op.location().cloned()) {
                    self.context.push_back(OpDetail::Full {
                        op: opcode,
                        location: loc,
                    });
                } else {
                    self.context.push_back(OpDetail::Basic { op: opcode });
                }
            }
        }
    }

    pub fn last_location(&self) -> Option<&miden_core::debuginfo::Location> {
        match dbg!(self.context.back()) {
            Some(OpDetail::Full { location, .. }) => {
                let loc = location.as_ref();
                if loc.is_none() {
                    dbg!(&self.context);
                }
                loc
            }
            Some(OpDetail::Basic { .. }) => None,
            Some(OpDetail::Exec { .. }) => {
                let op = self.context.iter().rev().nth(1)?;
                op.location()
            }
            None => None,
        }
    }
}

#[derive(Debug, Clone)]
enum OpDetail {
    Full {
        op: Operation,
        location: Option<miden_core::debuginfo::Location>,
    },
    Exec {
        callee: Option<Rc<str>>,
    },
    Basic {
        op: Operation,
    },
}
impl OpDetail {
    pub fn callee(&self, strip_prefix: &str) -> Option<Box<str>> {
        match self {
            Self::Exec { callee: None } => Some(Box::from("<unknown>")),
            Self::Exec {
                callee: Some(ref callee),
            } => {
                let name = match callee.split_once("::") {
                    Some((module, rest)) if module == strip_prefix => demangle(rest),
                    _ => demangle(callee),
                };
                Some(name.into_boxed_str())
            }
            _ => None,
        }
    }

    pub fn opcode(&self) -> Operation {
        match self {
            Self::Full { op, .. } | Self::Basic { op } => *op,
            Self::Exec { .. } => panic!("no opcode associated with execs"),
        }
    }

    pub fn location(&self) -> Option<&miden_core::debuginfo::Location> {
        match self {
            Self::Full { ref location, .. } => location.as_ref(),
            Self::Basic { .. } | Self::Exec { .. } => None,
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
        let name = frame.procedure(session_name);
        let name = name.as_deref().unwrap_or("<unknown>");
        if is_top {
            write!(&mut stacktrace, " `-> {name}").unwrap();
        } else {
            write!(&mut stacktrace, " |-> {name}").unwrap();
        }
        if let Some(loc) = frame.last_location() {
            let path = std::path::Path::new(loc.path.as_ref());
            let loc_source_code = if path.exists() {
                session.source_manager.load_file(path).ok()
            } else {
                session.source_manager.get_by_path(loc.path.as_ref())
            };
            if is_top {
                source_code.clone_from(&loc_source_code);
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
                //if let Some(filename) = path.file_name().map(std::path::Path::new) {
                if let Some(filename) = Some(path) {
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
            let context_size = frame.context.len();
            writeln!(&mut stacktrace, ":\n\nLast {context_size} Instructions (of current frame):")
                .unwrap();
            for (i, op) in frame.context.iter().enumerate() {
                let is_last = i + 1 == context_size;
                if let Some(callee) = op.callee(session_name) {
                    write!(&mut stacktrace, " |   exec.{callee}").unwrap();
                } else {
                    write!(&mut stacktrace, " |   {}", &op.opcode()).unwrap();
                }
                if is_last {
                    writeln!(&mut stacktrace, "\n `-> <error occured here>").unwrap();
                } else {
                    stacktrace.push('\n');
                }
            }

            let context_size = recent_ops.len();
            writeln!(&mut stacktrace, "\n\nLast {context_size} Instructions (any frame):").unwrap();
            for (i, op) in recent_ops.iter().enumerate() {
                let is_last = i + 1 == context_size;
                if is_last {
                    writeln!(&mut stacktrace, " |   {}", &op).unwrap();
                    writeln!(&mut stacktrace, " `-> <error occured here>").unwrap();
                } else {
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
