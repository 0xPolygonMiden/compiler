use std::collections::{BTreeMap, VecDeque};

use miden_assembly::library::CompiledLibrary;
use miden_core::{Program, StackInputs};
use miden_processor::{AdviceInputs, DefaultHost, ExecutionError, MastForest, Process};
use midenc_hir::Felt;
use midenc_session::Session;

use crate::felt_conversion::{PopFromStack, TestFelt};

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
    pub fn execute(mut self, program: &Program, session: &Session) -> MidenExecutionTrace {
        use std::collections::BTreeSet;

        use miden_processor::{MemAdviceProvider, ProcessState, VmStateIterator};

        let advice_provider = MemAdviceProvider::from(self.advice);
        let mut host = DefaultHost::new(advice_provider);
        for lib in core::mem::take(&mut self.libraries) {
            host.load_mast_forest(lib);
        }
        //dbg!(&self.stack);
        let mut process = Process::new_debug(program.kernel().clone(), self.stack, host);
        let root_context = process.ctx();
        let result = process.execute(program);
        let mut iter = VmStateIterator::new(process, result.clone());
        let mut contexts = BTreeSet::default();
        let mut last_state: Option<miden_processor::VmState> = None;
        for (i, state) in iter.by_ref().enumerate() {
            match state {
                Ok(state) => {
                    contexts.insert(state.ctx);
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
                    render_execution_error(err, i, last_state.as_ref(), session);
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
    pub fn read_memory_element(&self, addr: u32, index: u8) -> Option<Felt> {
        assert!(index < 4, "invalid element index");
        self.read_memory_word(addr).map(|word| word[index as usize])
    }

    /// Read a value of the given type, given an address in Rust's address space
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

fn render_execution_error(
    err: ExecutionError,
    step: usize,
    last_state: Option<&miden_processor::VmState>,
    session: &Session,
) -> ! {
    use midenc_hir::diagnostics::{
        miette::miette, reporting::PrintDiagnostic, LabeledSpan, SourceManagerExt,
    };

    if let Some(last_state) = last_state {
        let mut source_code = None;
        let mut labels = vec![];
        let last_op;
        let last_context_name;
        match dbg!(last_state.asmop.as_ref()) {
            Some(op) => {
                last_op = op.op().to_string();
                last_context_name = op.context_name();
                let asmop = op.as_ref();
                if let Some(loc) = dbg!(asmop.location()) {
                    let path = std::path::Path::new(loc.path.as_ref());
                    source_code = if path.exists() {
                        session.source_manager.load_file(path).ok()
                    } else {
                        session.source_manager.get_by_path(loc.path.as_ref())
                    };
                    labels.push(LabeledSpan::new_with_span(
                        None,
                        loc.start.to_usize()..loc.end.to_usize(),
                    ));
                }
            }
            None => {
                last_op =
                    last_state.op.map(|op| op.to_string()).unwrap_or_else(|| "N/A".to_string());
                last_context_name = "N/A";
            }
        };
        let stack = last_state.stack.iter().map(|elem| elem.as_int());
        let stack = midenc_hir::DisplayValues::new(stack);
        let help = format!(
            "
last known context:       {last_context_name}
last known op:            {last_op}
last known frame pointer: {fmp} (frame pointer starts at 2^30)
last known operand stack: [{stack}]",
            fmp = last_state.fmp.as_int()
        );
        let report = miette!(
            help = help,
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
