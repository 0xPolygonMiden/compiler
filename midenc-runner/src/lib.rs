#![feature(iter_array_chunks)]

mod debug;
mod felt;
mod host;
mod inputs;
mod run;
mod runner;

use std::rc::Rc;

pub use midenc_hir::TraceEvent;
use midenc_session::{diagnostics::Report, Session};

pub use self::{
    debug::*,
    felt::{PopFromStack, PushToStack, TestFelt},
    host::TestHost,
    inputs::ProgramInputs,
    run::{MidenExecutionTrace, MidenExecutor},
    runner::Runner,
};

pub type ExecutionResult<T> = Result<T, Report>;

pub type TraceHandler = dyn FnMut(miden_processor::RowIndex, TraceEvent);

pub fn run(
    _inputs: Option<ProgramInputs>,
    _args: Vec<String>,
    _session: Rc<Session>,
) -> ExecutionResult<()> {
    todo!()
}

pub fn trace(
    _options: Option<ProgramInputs>,
    _args: Vec<String>,
    _session: Rc<Session>,
) -> ExecutionResult<MidenExecutionTrace> {
    todo!()
}
