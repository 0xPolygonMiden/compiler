use std::{cell::RefCell, fmt, rc::Rc, sync::Arc};

use midenc_hir::Felt;
use smallvec::{smallvec, SmallVec};

use super::{Addr, ControlEffect, EmulationError, Emulator, InstructionPointer};
use crate::{BlockId, Function, Op};

/// The type signature for native Rust functions callable from MASM IR
pub type NativeFn = dyn FnMut(&mut Emulator, &[Felt]) -> Result<(), EmulationError>;

/// We allow functions in the emulator to be defined in either MASM IR, or native Rust.
///
/// Functions implemented in Rust are given a mutable reference to the emulator, so they
/// have virtually unlimited power, but are correspondingly very unsafe. With great
/// power comes great responsibility, etc.
#[derive(Clone)]
pub enum Stub {
    /// This function has a definition in Miden Assembly
    Asm(Arc<Function>),
    /// This function has a native Rust implementation
    Native(Rc<RefCell<Box<NativeFn>>>),
}

/// This enum represents a frame on the control stack
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ControlFrame {
    /// A control frame used to revisit a single instruction
    /// used to control entry into a loop, e.g. `while.true`
    Loopback(InstructionPointer),
    /// Control is in a normal block
    Block(InstructionPointer),
    /// Control was transferred into a while.true loop
    While(InstructionPointer),
    /// Control was transferred to a repeat loop
    Repeat(RepeatState),
}
impl Default for ControlFrame {
    fn default() -> Self {
        Self::Block(InstructionPointer::new(BlockId::from_u32(0)))
    }
}
impl ControlFrame {
    pub const fn ip(&self) -> InstructionPointer {
        match self {
            Self::Loopback(ip)
            | Self::Block(ip)
            | Self::While(ip)
            | Self::Repeat(RepeatState { ip, .. }) => *ip,
        }
    }

    /// Move the instruction pointer forward one instruction and return a copy to the caller
    fn move_next(&mut self) -> InstructionPointer {
        match self {
            Self::Block(ref mut ip)
            | Self::While(ref mut ip)
            | Self::Repeat(RepeatState { ref mut ip, .. }) => {
                ip.index += 1;
                *ip
            }
            Self::Loopback(_) => panic!("cannot move a loopback control frame"),
        }
    }

    /// Move the instruction pointer backward one instruction and return a copy to the caller
    #[allow(unused)]
    fn move_prev(&mut self) -> InstructionPointer {
        match self {
            Self::Block(ref mut ip)
            | Self::While(ref mut ip)
            | Self::Repeat(RepeatState { ref mut ip, .. }) => {
                let index = ip.index.saturating_sub(1);
                ip.index = index;
                *ip
            }
            Self::Loopback(_) => panic!("cannot move a loopback control frame"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RepeatState {
    /// The instruction pointer in the repeat block
    pub ip: InstructionPointer,
    /// Corresponds to `n` in `repeat.n`, i.e. the number of iterations to perform
    pub n: u16,
    /// The number of iterations completed so far
    pub iterations: u16,
}

#[derive(Debug)]
pub struct ControlStack {
    /// The control frame for the current instruction being executed
    current: ControlFrame,
    /// The next instruction to be executed
    pending: Option<Instruction>,
    /// The control frame corresponding to the next instruction
    pending_frame: Option<ControlFrame>,
    /// Pending frames from which to fetch the next instruction
    frames: SmallVec<[ControlFrame; 2]>,
}
impl ControlStack {
    pub fn new(ip: InstructionPointer) -> Self {
        let current = ControlFrame::Block(ip);
        Self {
            current,
            pending: None,
            pending_frame: Some(current),
            frames: smallvec![],
        }
    }

    /// The instruction pointer corresponding to the currently executing instruction
    #[inline(always)]
    pub const fn ip(&self) -> InstructionPointer {
        self.current.ip()
    }

    /// Push a new control frame for a repeat loop on the stack, and move the instruction
    /// pointer so that the pending instruction is the first instruction of the body
    #[inline]
    pub fn enter_repeat(&mut self, block: BlockId, n: u16) {
        let ip = InstructionPointer::new(block);
        let pending_frame = self.pending_frame.replace(ControlFrame::Repeat(RepeatState {
            ip,
            n,
            iterations: 0,
        }));
        self.pending = None;
        self.current = ControlFrame::Repeat(RepeatState {
            ip,
            n,
            iterations: 0,
        });
        if let Some(pending_frame) = pending_frame {
            self.frames.push(pending_frame);
        }
    }

    /// Push a new control frame for a while loop on the stack, and move the instruction
    /// pointer so that the pending instruction is the first instruction of the body
    #[inline]
    pub fn enter_while_loop(&mut self, block: BlockId) {
        let ip = InstructionPointer::new(block);
        let pending_frame = self.pending_frame.replace(ControlFrame::While(ip));
        // Make sure we preserve the pending frame for when we loopback to the
        // while the final time, and skip over it
        if let Some(pending_frame) = pending_frame {
            self.frames.push(pending_frame);
        }
        // We need to revisit the `while.true` at least once, so we stage a special
        // control frame that expires as soon as that instruction is visited.
        self.frames.push(ControlFrame::Loopback(self.current.ip()));
        self.pending = None;
        self.current = ControlFrame::While(ip);
    }

    /// Push a new control frame for a normal block on the stack, and move the instruction
    /// pointer so that the pending instruction is the first instruction of the body
    #[inline]
    pub fn enter_block(&mut self, block: BlockId) {
        let ip = InstructionPointer::new(block);
        let pending_frame = self.pending_frame.replace(ControlFrame::Block(ip));
        self.pending = None;
        self.current = ControlFrame::Block(ip);
        if let Some(pending_frame) = pending_frame {
            self.frames.push(pending_frame);
        }
    }

    /// Get the next instruction to execute without moving the instruction pointer
    pub fn peek(&self) -> Option<Instruction> {
        match self.pending {
            None => self.pending_frame.map(|frame| Instruction {
                continuing_from: None,
                ip: frame.ip(),
                effect: ControlEffect::Enter,
            }),
            pending @ Some(_) => pending,
        }
    }

    pub fn next(&mut self, function: &Function) -> Option<Instruction> {
        if self.pending.is_none() {
            let pending_frame = self.pending_frame?;
            let ip = pending_frame.ip();
            let effect = if ip.index == 0 {
                ControlEffect::Enter
            } else {
                ControlEffect::None
            };
            self.pending = Some(Instruction {
                continuing_from: None,
                ip,
                effect,
            });
        }

        let pending = self.pending.take()?;
        let mut pending_frame = self.pending_frame.unwrap();
        let current_frame = pending_frame;

        if is_last_instruction(current_frame, function) {
            let pending_frame_and_effect = self.find_continuation_frame(current_frame, function);
            match pending_frame_and_effect {
                Some((pending_frame, effect)) => {
                    self.pending = Some(Instruction {
                        continuing_from: Some(current_frame),
                        ip: pending_frame.ip(),
                        effect,
                    });
                    self.pending_frame = Some(pending_frame);
                    self.current = current_frame;
                }
                None => {
                    self.pending = None;
                    self.pending_frame = None;
                    self.current = current_frame;
                }
            }
        } else {
            pending_frame.move_next();
            self.pending = Some(Instruction {
                continuing_from: None,
                ip: pending_frame.ip(),
                effect: ControlEffect::None,
            });
            self.pending_frame = Some(pending_frame);
            self.current = current_frame;
        }

        Some(pending)
    }

    fn find_continuation_frame(
        &mut self,
        current: ControlFrame,
        function: &Function,
    ) -> Option<(ControlFrame, ControlEffect)> {
        match current {
            ControlFrame::Loopback(_) => {
                // This frame is usually preceded by a top-level block frame, but if
                // the body of a function starts with a loop header, then there may not
                // be any parent frames, in which case we're returning from the function
                let continuation = self.frames.pop()?;
                return Some((continuation, ControlEffect::Exit));
            }
            ControlFrame::While(_) => {
                // There will always be a frame available when a while frame is on the stack
                let continuation = self.frames.pop().unwrap();
                return Some((continuation, ControlEffect::Loopback));
            }
            ControlFrame::Repeat(repeat) => {
                let next_iteration = repeat.iterations + 1;
                if next_iteration < repeat.n {
                    let ip = InstructionPointer::new(repeat.ip.block);
                    let pending_frame = ControlFrame::Repeat(RepeatState {
                        ip,
                        iterations: next_iteration,
                        n: repeat.n,
                    });
                    let effect = ControlEffect::Repeat(next_iteration);
                    return Some((pending_frame, effect));
                }
            }
            _ => (),
        }

        let mut current = current;
        loop {
            let transfer_to = self.frames.pop();
            match current {
                ControlFrame::While(_) => {
                    break Some((transfer_to.unwrap(), ControlEffect::Loopback));
                }
                ControlFrame::Repeat(mut repeat) => {
                    let next_iteration = repeat.iterations + 1;
                    if next_iteration < repeat.n {
                        if let Some(transfer_to) = transfer_to {
                            self.frames.push(transfer_to);
                        }
                        let ip = InstructionPointer::new(repeat.ip.block);
                        repeat.iterations = next_iteration;
                        repeat.ip = ip;

                        let pending_frame = ControlFrame::Repeat(repeat);
                        let effect = ControlEffect::Repeat(next_iteration);
                        break Some((pending_frame, effect));
                    }
                    current = transfer_to?;
                }
                ControlFrame::Loopback(_) | ControlFrame::Block(_) => {
                    let pending_frame = transfer_to?;
                    if is_valid_instruction(pending_frame.ip(), function) {
                        break Some((pending_frame, ControlEffect::Exit));
                    }
                    current = pending_frame;
                }
            }
        }
    }
}

#[inline(always)]
fn is_last_instruction(frame: ControlFrame, function: &Function) -> bool {
    match frame {
        ControlFrame::Loopback(_) => true,
        frame => {
            let ip = frame.ip();
            ip.index >= function.block(ip.block).ops.len().saturating_sub(1)
        }
    }
}

#[inline(always)]
fn is_valid_instruction(ip: InstructionPointer, function: &Function) -> bool {
    ip.index < function.block(ip.block).ops.len()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Instruction {
    /// The new instruction pointer value
    pub ip: InstructionPointer,
    /// If set, this instruction is in a control frame which was suspended
    /// and is now being resumed/continued. The given frame was the state
    /// of that frame when the instruction pointer was advanced.
    pub continuing_from: Option<ControlFrame>,
    /// The control flow effect that occurred when advancing the instruction pointer
    pub effect: ControlEffect,
}
impl Instruction {
    pub fn with_op(self, function: &Function) -> Option<InstructionWithOp> {
        self.op(function).map(|op| InstructionWithOp {
            ip: self.ip,
            continuing_from: self.continuing_from,
            effect: self.effect,
            op,
        })
    }

    #[inline(always)]
    pub fn op(&self, function: &Function) -> Option<Op> {
        function.body.get(self.ip).map(|op| op.item)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionWithOp {
    /// The new instruction pointer value
    pub ip: InstructionPointer,
    /// If set, this instruction is in a control frame which was suspended
    /// and is now being resumed/continued. The given frame was the state
    /// of that frame when the instruction pointer was advanced.
    pub continuing_from: Option<ControlFrame>,
    /// The control flow effect that occurred when advancing the instruction pointer
    pub effect: ControlEffect,
    /// The op the instruction pointer points to
    pub op: Op,
}

/// This struct represents an activation record for a function on the call stack
///
/// When a program begins executing, an activation record is created for the entry point,
/// and any calls made from the entry get their own activation record, recursively down the
/// call graph.
///
/// The activation record contains state about the execution of that function of interest
/// to the emulator, in particular, the instruction pointer, the frame pointer for locals,
/// and the function-local control stack
pub struct Activation {
    function: Arc<Function>,
    fp: Addr,
    control_stack: ControlStack,
}
impl fmt::Debug for Activation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Activation")
            .field("function", &self.function.name)
            .field("fp", &self.fp)
            .field("control_stack", &self.control_stack)
            .finish()
    }
}
impl Activation {
    /// Create a new activation record for `function`, using `fp` as the frame pointer for this
    /// activation
    pub fn new(function: Arc<Function>, fp: Addr) -> Self {
        let block = function.body.id();
        let control_stack = ControlStack::new(InstructionPointer::new(block));
        Self {
            function,
            fp,
            control_stack,
        }
    }

    #[cfg(test)]
    pub fn at(function: Arc<Function>, fp: Addr, control_stack: ControlStack) -> Self {
        Self {
            function,
            fp,
            control_stack,
        }
    }

    #[inline(always)]
    pub fn function(&self) -> &Function {
        &self.function
    }

    #[inline(always)]
    pub fn fp(&self) -> Addr {
        self.fp
    }

    #[inline(always)]
    pub fn ip(&self) -> InstructionPointer {
        self.control_stack.ip()
    }

    /// Advance to the next instruction, returning the current [InstructionWithOp]
    ///
    /// If all code in the function has been executed, None will be returned.
    #[inline]
    pub fn next(&mut self) -> Option<InstructionWithOp> {
        self.move_next().and_then(|ix| ix.with_op(&self.function))
    }

    /// Advance to the next instruction, returning the current [Instruction]
    ///
    /// If all code in the function has been executed, None will be returned.
    pub fn move_next(&mut self) -> Option<Instruction> {
        self.control_stack.next(&self.function)
    }

    /// Peek at the [Instruction] which will be returned when [move_next] is called next
    #[inline(always)]
    pub fn peek(&self) -> Option<Instruction> {
        self.control_stack.peek()
    }

    /// Peek at the [InstructionWithOp] corresponding to the next instruction to be returned from
    /// [move_next]
    pub fn peek_with_op(&self) -> Option<InstructionWithOp> {
        self.control_stack.peek().and_then(|ix| ix.with_op(&self.function))
    }

    /// Peek at the [Op] coresponding to the next instruction to be returned from [move_next]
    #[allow(unused)]
    pub fn peek_op(&self) -> Option<Op> {
        self.control_stack.peek().and_then(|ix| ix.op(&self.function))
    }

    /// Set the instruction pointer to the first instruction of `block`
    ///
    /// This helper ensures the internal state of the activation record is maintained
    pub(super) fn enter_block(&mut self, block: BlockId) {
        self.control_stack.enter_block(block);
    }

    /// Set the instruction pointer to the first instruction of `block`, which is the body of
    /// a while loop.
    ///
    /// This ensures that when we attempt to leave the while loop, the "next" instruction will
    /// be the `while.true` itself, so that its condition gets re-evaluated. This will result
    /// in an infinite loop unless the value of the condition is zero, or the operand stack is
    /// exhausted, causing an assertion.
    pub(super) fn enter_while_loop(&mut self, block: BlockId) {
        self.control_stack.enter_while_loop(block);
    }

    /// Set the instruction pointer to the first instruction of `block`, which is the body of
    /// a repeat loop with `n` iterations.
    ///
    /// We use an auxiliary structure to track repeat loops, so this works a bit differently
    /// than `enter_while_loop`, but has the same effect. The state we track is used to determine
    /// when we've executed `count` iterations of the loop and should exit to the next instruction
    /// following the `repeat.N`.
    pub(super) fn repeat_block(&mut self, block: BlockId, count: u16) {
        self.control_stack.enter_repeat(block, count);
    }
}

#[cfg(test)]
mod tests {
    use midenc_hir::{assert_matches, Signature, SourceSpan};

    use super::*;

    #[test]
    fn activation_record_start_of_block() {
        let mut activation = Activation::new(test_function(), 0);
        let body_blk = activation.function.body.id();
        assert_eq!(
            activation.peek(),
            Some(Instruction {
                continuing_from: None,
                ip: InstructionPointer {
                    block: body_blk,
                    index: 0
                },
                effect: ControlEffect::Enter,
            })
        );
        assert_eq!(activation.peek_op(), Some(Op::PushU8(2)));

        // Advance the instruction pointer
        assert_matches!(
            activation.next(),
            Some(InstructionWithOp {
                op: Op::PushU8(2),
                ..
            })
        );
        assert_eq!(
            activation.peek_with_op(),
            Some(InstructionWithOp {
                op: Op::PushU8(1),
                continuing_from: None,
                ip: InstructionPointer {
                    block: body_blk,
                    index: 1
                },
                effect: ControlEffect::None
            })
        );
    }

    #[test]
    fn activation_record_if_true_entry() {
        let function = test_function();
        let body_blk = function.body.id();
        let control_stack = ControlStack::new(InstructionPointer {
            block: body_blk,
            index: 5,
        });
        let mut activation = Activation::at(test_function(), 0, control_stack);

        assert_eq!(
            activation.peek(),
            Some(Instruction {
                continuing_from: None,
                ip: InstructionPointer {
                    block: body_blk,
                    index: 5
                },
                effect: ControlEffect::Enter
            })
        );
        let Some(Op::If(then_blk, _)) = activation.peek_op() else {
            panic!("expected if.true, got {:?}", activation.peek_with_op())
        };

        // Enter the truthy branch of the if.true
        activation.enter_block(then_blk);

        assert_eq!(
            activation.peek_with_op(),
            Some(InstructionWithOp {
                op: Op::PushU8(1),
                continuing_from: None,
                ip: InstructionPointer {
                    block: then_blk,
                    index: 0
                },
                effect: ControlEffect::Enter,
            })
        );

        // Advance the instruction pointer
        assert_matches!(
            activation.next(),
            Some(InstructionWithOp {
                op: Op::PushU8(1),
                effect: ControlEffect::Enter,
                ..
            })
        );
        assert_eq!(
            activation.peek_with_op(),
            Some(InstructionWithOp {
                op: Op::While(BlockId::from_u32(3)),
                continuing_from: None,
                ip: InstructionPointer {
                    block: then_blk,
                    index: 1
                },
                effect: ControlEffect::None
            })
        );
    }

    #[test]
    fn activation_record_nested_control_flow_exit() {
        let function = test_function();
        let body_blk = function.body.id();
        let control_stack = ControlStack::new(InstructionPointer {
            block: body_blk,
            index: 5,
        });
        let mut activation = Activation::at(test_function(), 0, control_stack);

        let next = activation.next().unwrap();
        assert_eq!(ControlEffect::None, next.effect);
        let Op::If(then_blk, _) = next.op else {
            panic!("expected if.true, got {next:?}")
        };

        // Enter the truthy branch of the if.true
        activation.enter_block(then_blk);

        // Step over the first instruction, to the `while.true`
        assert_matches!(
            activation.next(),
            Some(InstructionWithOp {
                op: Op::PushU8(1),
                effect: ControlEffect::Enter,
                ..
            })
        );

        let Some(Op::While(loop_body)) = activation.peek_op() else {
            panic!("expected while.true, got {:?}", activation.peek_op())
        };

        // Enter loop body
        activation.next().unwrap();
        activation.enter_while_loop(loop_body);

        assert_eq!(
            activation.peek_with_op(),
            Some(InstructionWithOp {
                op: Op::Dup(1),
                continuing_from: None,
                ip: InstructionPointer {
                    block: loop_body,
                    index: 0
                },
                effect: ControlEffect::Enter
            })
        );

        // Advance the instruction pointer to the end of the loop body
        let next = activation.next().unwrap();
        assert_eq!(next.op, Op::Dup(1));
        let next = activation.next().unwrap();
        assert_eq!(next.op, Op::Dup(1));
        let next = activation.next().unwrap();
        assert_eq!(next.op, Op::Incr);

        // Ensure things are normal at the last instruction
        assert_eq!(
            activation.peek_with_op(),
            Some(InstructionWithOp {
                op: Op::U32Lt,
                continuing_from: None,
                ip: InstructionPointer {
                    block: loop_body,
                    index: 3
                },
                effect: ControlEffect::None
            })
        );

        // Advance the instruction pointer, obtaining the last instruction of the loop body
        assert_eq!(activation.next().map(|ix| ix.op), Some(Op::U32Lt));

        // Exit while.true
        assert_eq!(
            activation.peek_with_op(),
            Some(InstructionWithOp {
                op: Op::While(loop_body),
                continuing_from: Some(ControlFrame::While(InstructionPointer {
                    block: loop_body,
                    index: 3
                })),
                ip: InstructionPointer {
                    block: then_blk,
                    index: 1
                },
                effect: ControlEffect::Loopback,
            })
        );
        assert_eq!(
            activation.next(),
            Some(InstructionWithOp {
                op: Op::While(loop_body),
                continuing_from: Some(ControlFrame::While(InstructionPointer {
                    block: loop_body,
                    index: 3
                })),
                ip: InstructionPointer {
                    block: then_blk,
                    index: 1
                },
                effect: ControlEffect::Loopback,
            })
        );

        // Exit if.true
        let callee = "test::foo".parse().unwrap();
        assert_eq!(
            activation.peek_with_op(),
            Some(InstructionWithOp {
                op: Op::Exec(callee),
                continuing_from: Some(ControlFrame::Loopback(InstructionPointer {
                    block: then_blk,
                    index: 1
                })),
                ip: InstructionPointer {
                    block: body_blk,
                    index: 6,
                },
                effect: ControlEffect::Exit,
            })
        );
        assert_eq!(
            activation.next(),
            Some(InstructionWithOp {
                op: Op::Exec(callee),
                continuing_from: Some(ControlFrame::Loopback(InstructionPointer {
                    block: then_blk,
                    index: 1
                })),
                ip: InstructionPointer {
                    block: body_blk,
                    index: 6,
                },
                effect: ControlEffect::Exit,
            })
        );

        // Return from the function
        assert_matches!(activation.next(), None);
    }

    fn test_function() -> Arc<Function> {
        let span = SourceSpan::default();
        let mut function =
            Function::new("test::main".parse().unwrap(), Signature::new(vec![], vec![]));
        let then_blk = function.create_block();
        let else_blk = function.create_block();
        let while_blk = function.create_block();
        {
            let body = function.block_mut(function.body.id());
            body.push(Op::PushU8(2), span);
            body.push(Op::PushU8(1), span);
            body.push(Op::Dup(1), span);
            body.push(Op::Dup(1), span);
            body.push(Op::U32Lt, span);
            body.push(Op::If(then_blk, else_blk), span);
            body.push(Op::Exec("test::foo".parse().unwrap()), span);
        }
        {
            let then_body = function.block_mut(then_blk);
            then_body.push(Op::PushU8(1), span);
            then_body.push(Op::While(while_blk), span);
        }
        {
            let else_body = function.block_mut(else_blk);
            else_body.push(Op::U32Max, span);
        }
        {
            let while_body = function.block_mut(while_blk);
            while_body.push(Op::Dup(1), span);
            while_body.push(Op::Dup(1), span);
            while_body.push(Op::Incr, span);
            while_body.push(Op::U32Lt, span);
        }

        Arc::new(function)
    }
}
