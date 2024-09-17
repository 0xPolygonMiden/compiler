#![feature(array_windows)]
#![feature(iter_array_chunks)]
#![feature(debug_closure_helpers)]

extern crate alloc;

mod codegen;
mod compiler;
mod convert;
mod emulator;
mod masm;
mod packaging;
#[cfg(test)]
mod tests;

pub use self::{
    compiler::{
        default_function_rewrites, default_rewrites, CompilerResult, MasmArtifact, MasmCompiler,
        MastArtifact,
    },
    convert::ConvertHirToMasm,
    emulator::{
        Breakpoint, BreakpointEvent, CallFrame, DebugInfo, DebugInfoWithStack, EmulationError,
        Emulator, EmulatorEvent, InstructionPointer, WatchMode, Watchpoint, WatchpointId,
    },
    masm::*,
    packaging::*,
};
