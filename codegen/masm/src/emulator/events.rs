use midenc_hir::FunctionIdent;

use super::{Addr, InstructionPointer};
use crate::BlockId;

/// A control-flow event that occurred as a side-effect of
/// advancing the instruction pointer.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ControlEffect {
    /// No control effects occurred
    None,
    /// We jumped to a nested block
    Enter,
    /// We jumped to a parent block
    Exit,
    /// We jumped back to the start of a while loop
    Loopback,
    /// We started the `n`th iteration of a repeat block
    Repeat(u16),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BreakpointEvent {
    /// Breakpoint was hit because we always break on each step
    Step,
    /// Breakpoint was hit because we are stepping out of the current function
    StepOut,
    /// Breakpoint for a specific clock cycle was reached
    ReachedCycle(usize),
    /// Breakpoint for a specific instruction pointer value was reached
    Reached(InstructionPointer),
    /// Breakpoint for a loop was hit
    Loop(BlockId),
    /// Breakpoint for the given function was hit
    Called(FunctionIdent),
    /// The given watchpoint was hit as a breakpoint
    Watch(super::Watchpoint),
}

#[derive(Debug, Copy, Clone)]
pub enum EmulatorEvent {
    /// The start of a new cycle has begun
    CycleStart(usize),
    /// The specified function was called and the emulator is at the first instruction in its body
    EnterFunction(FunctionIdent),
    /// The emulator has returned from the specified function, and the emulator is at the first
    /// instruction following it in the caller, or if there are no more instructions in the caller,
    /// waiting to return from the caller function on the next resumption.
    ExitFunction(FunctionIdent),
    /// The emulator has entered a loop, whose body is the specified block.
    ///
    /// The emulator is at the first instruction in that block.
    EnterLoop(BlockId),
    /// The emulator has exited a loop, whose body is the specified block, and is at the first
    /// instruction following it in the enclosing block. If there are no more instructions after
    /// the loop, the emulator will return from the enclosing function on the next resumption.
    ExitLoop(BlockId),
    /// Control has transferred to `block`
    ///
    /// This event is only used when the control flow instruction was not a loop instruction
    Jump(BlockId),
    /// The emulator just performed a store to `addr` of `size` bytes
    MemoryWrite { addr: Addr, size: u32 },
    /// The emulator has reached a breakpoint
    Breakpoint(BreakpointEvent),
    /// The emulator has suspended, and can be resumed at will
    Suspended,
    /// The emulator has reached the end of the program and has stopped executing
    Stopped,
}
