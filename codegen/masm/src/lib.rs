#![feature(array_windows)]
#![feature(iter_array_chunks)]
#![feature(is_sorted)]

mod codegen;
mod compiler;
mod convert;
mod emulator;
mod masm;
#[cfg(test)]
mod tests;

pub use self::{
    compiler::{
        default_function_rewrites, default_rewrites, MasmArtifact, MasmCompiler, MastArtifact,
    },
    convert::ConvertHirToMasm,
    emulator::{
        Breakpoint, BreakpointEvent, CallFrame, DebugInfo, DebugInfoWithStack, EmulationError,
        Emulator, EmulatorEvent, InstructionPointer, WatchMode, Watchpoint, WatchpointId,
    },
    masm::*,
};
