use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, VecDeque},
    rc::Rc,
    sync::Arc,
};

use miden_assembly::Library as CompiledLibrary;
use miden_core::{Program, StackInputs, Word};
use miden_processor::{
    AdviceInputs, ContextId, ExecutionError, Felt, MastForest, MemAdviceProvider, Process,
    ProcessState, RowIndex, StackOutputs, VmState, VmStateIterator,
};
use midenc_codegen_masm::{NativePtr, Package};
use midenc_hir::Type;
use midenc_session::{
    diagnostics::{IntoDiagnostic, Report},
    Session,
};

use super::{DebugExecutor, DebuggerHost, ExecutionTrace, TraceEvent};
use crate::{debug::CallStack, felt::PopFromStack, TestFelt};

/// The [Executor] is responsible for executing a program with the Miden VM.
///
/// It is used by either converting it into a [DebugExecutor], and using that to
/// manage execution step-by-step, such as is done by the debugger; or by running
/// the program to completion and obtaining an [ExecutionTrace], which can be used
/// to introspect the final program state.
pub struct Executor {
    stack: StackInputs,
    advice: AdviceInputs,
    libraries: Vec<Arc<MastForest>>,
}
impl Executor {
    /// Construct an executor with the given arguments on the operand stack
    pub fn new(args: Vec<Felt>) -> Self {
        Self {
            stack: StackInputs::new(args).expect("invalid stack inputs"),
            advice: AdviceInputs::default(),
            libraries: Default::default(),
        }
    }

    pub fn for_package(
        package: &Package,
        args: Vec<Felt>,
        session: &Session,
    ) -> Result<Self, Report> {
        use midenc_hir::formatter::DisplayHex;
        log::debug!(
            "creating executor for package '{}' (digest={})",
            package.name,
            DisplayHex::new(&package.digest.as_bytes())
        );

        let mut exec = Self::new(args);

        for link_library in package.manifest.link_libraries.iter() {
            log::debug!(
                "loading link library from package manifest: {} (kind = {}, from = {:#?})",
                link_library.name.as_ref(),
                link_library.kind,
                link_library.path.as_ref().map(|p| p.display())
            );
            let library = link_library.load(session)?;
            log::debug!("library loaded succesfully");
            exec.with_library(&library);
        }

        for rodata in package.rodata.iter() {
            log::debug!(
                "adding rodata segment for offset {} (size {}) to advice map: {}",
                rodata.start.as_ptr(),
                rodata.size_in_bytes(),
                DisplayHex::new(&rodata.digest.as_bytes())
            );
            exec.advice
                .extend_map([(rodata.digest, rodata.to_elements().map_err(Report::msg)?)]);
        }

        log::debug!("executor created");

        Ok(exec)
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

    /// Convert this [Executor] into a [DebugExecutor], which captures much more information
    /// about the program being executed, and must be stepped manually.
    pub fn into_debug(mut self, program: &Program, session: &Session) -> DebugExecutor {
        log::debug!("creating debug executor");

        let advice_provider = MemAdviceProvider::from(self.advice);
        let mut host = DebuggerHost::new(advice_provider);
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
        DebugExecutor {
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
    pub fn capture_trace(mut self, program: &Program, session: &Session) -> ExecutionTrace {
        let mut executor = self.into_debug(program, session);
        while let Some(step) = executor.next() {
            if step.is_err() {
                return executor.into_execution_trace();
            }
        }
        executor.into_execution_trace()
    }

    /// Execute the given program, producing a trace
    #[track_caller]
    pub fn execute(mut self, program: &Program, session: &Session) -> ExecutionTrace {
        let mut executor = self.into_debug(program, session);
        while let Some(step) = executor.next() {
            if let Err(err) = step {
                render_execution_error(err, &executor, session);
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

        executor.into_execution_trace()
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

#[track_caller]
fn render_execution_error(
    err: ExecutionError,
    execution_state: &DebugExecutor,
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
