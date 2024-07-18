/// The field modulus for Miden's prime field
pub const P: u64 = (2u128.pow(64) - 2u128.pow(32) + 1) as u64;

/// Assert that an argument specifying an integer size in bits follows the rules about what
/// integer sizes we support as a general rule.
macro_rules! assert_valid_integer_size {
    ($n:ident) => {
        assert!($n > 0, "invalid integer size: size in bits must be non-zero");
        assert!(
            $n.is_power_of_two(),
            "invalid integer size: size in bits must be a power of two, got {}",
            $n
        );
    };

    ($n:ident, $min:literal) => {
        assert_valid_integer_size!($n);
        assert!(
            $n >= $min,
            "invalid integer size: expected size in bits greater than or equal to {}, got {}",
            $n,
            $min
        );
    };

    ($n:ident, $min:ident) => {
        assert_valid_integer_size!($n);
        assert!(
            $n >= $min,
            "invalid integer size: expected size in bits greater than or equal to {}, got {}",
            $n,
            $min
        );
    };

    ($n:ident, $min:literal, $max:literal) => {
        assert_valid_integer_size!($n, $min);
        assert!(
            $n <= $max,
            "invalid integer size: expected size in bits less than or equal to {}, got {}",
            $n,
            $max
        );
    };

    ($n:ident, $min:ident, $max:literal) => {
        assert_valid_integer_size!($n, $min);
        assert!(
            $n <= $max,
            "invalid integer size: expected size in bits less than or equal to {}, got {}",
            $n,
            $max
        );
    };
}

/// Assert that an argument specifying a zero-based stack index does not access out of bounds
macro_rules! assert_valid_stack_index {
    ($idx:ident) => {
        assert!(
            $idx < 16,
            "invalid stack index: only the first 16 elements on the stack are directly \
             accessible, got {}",
            $idx
        );
    };

    ($idx:expr) => {
        assert!(
            ($idx) < 16,
            "invalid stack index: only the first 16 elements on the stack are directly \
             accessible, got {}",
            $idx
        );
    };
}

pub mod binary;
pub mod felt;
pub mod int128;
pub mod int32;
pub mod int64;
pub mod mem;
pub mod primop;
pub mod smallint;
pub mod unary;

use core::ops::{Deref, DerefMut};

use miden_assembly::ast::InvokeKind;
use midenc_hir::{self as hir, Immediate, Type};

use super::{Operand, OperandStack};
use crate::masm::{self as masm, Op};

/// This structure is used to emit the Miden Assembly ops corresponding to an IR instruction.
///
/// When dropped, it ensures that the operand stack is updated to reflect the results of the
/// instruction it was created on behalf of.
pub struct InstOpEmitter<'a> {
    dfg: &'a hir::DataFlowGraph,
    inst: hir::Inst,
    emitter: OpEmitter<'a>,
}
impl<'a> InstOpEmitter<'a> {
    #[inline(always)]
    pub fn new(
        function: &'a mut masm::Function,
        dfg: &'a hir::DataFlowGraph,
        inst: hir::Inst,
        block: masm::BlockId,
        stack: &'a mut OperandStack,
    ) -> Self {
        Self {
            dfg,
            inst,
            emitter: OpEmitter::new(function, block, stack),
        }
    }

    pub fn exec(&mut self, callee: hir::FunctionIdent) {
        let import = self.dfg.get_import(&callee).unwrap();
        self.emitter.exec(import);
    }

    pub fn syscall(&mut self, callee: hir::FunctionIdent) {
        let import = self.dfg.get_import(&callee).unwrap();
        self.emitter.syscall(import);
    }

    #[inline(always)]
    pub fn value_type(&self, value: hir::Value) -> &Type {
        self.dfg.value_type(value)
    }

    #[inline(always)]
    pub fn dfg<'c, 'b: 'c>(&'b self) -> &'c hir::DataFlowGraph {
        self.dfg
    }
}
impl<'a> Deref for InstOpEmitter<'a> {
    type Target = OpEmitter<'a>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.emitter
    }
}
impl<'a> DerefMut for InstOpEmitter<'a> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.emitter
    }
}
impl<'a> Drop for InstOpEmitter<'a> {
    fn drop(&mut self) {
        let results = self.dfg.inst_results(self.inst);
        for (i, result) in results.iter().copied().rev().enumerate() {
            self.emitter.stack.rename(i, result);
        }
    }
}

/// This structure is used to emit Miden Assembly ops into a given function and block.
///
/// The [OpEmitter] carries limited context of its own, and expects to receive arguments
/// to it's various builder functions to provide necessary context for specific constructs.
pub struct OpEmitter<'a> {
    stack: &'a mut OperandStack,
    function: &'a mut masm::Function,
    current_block: masm::BlockId,
}
impl<'a> OpEmitter<'a> {
    #[inline(always)]
    pub fn new(
        function: &'a mut masm::Function,
        block: masm::BlockId,
        stack: &'a mut OperandStack,
    ) -> Self {
        Self {
            stack,
            function,
            current_block: block,
        }
    }

    #[cfg(test)]
    #[inline(always)]
    pub fn stack_len(&self) -> usize {
        self.stack.len()
    }

    #[inline(always)]
    pub fn stack<'c, 'b: 'c>(&'b self) -> &'c OperandStack {
        self.stack
    }

    #[inline(always)]
    pub fn stack_mut<'c, 'b: 'c>(&'b mut self) -> &'c mut OperandStack {
        self.stack
    }

    #[inline]
    fn maybe_register_invoke(&mut self, op: &masm::Op) {
        match op {
            Op::Exec(id) => {
                self.function.register_absolute_invocation_target(InvokeKind::Exec, *id)
            }
            Op::Call(id) => {
                self.function.register_absolute_invocation_target(InvokeKind::Call, *id)
            }
            Op::Syscall(id) => {
                self.function.register_absolute_invocation_target(InvokeKind::SysCall, *id)
            }
            _ => (),
        }
    }

    /// Emit `op` to the current block
    #[inline(always)]
    pub fn emit(&mut self, op: masm::Op) {
        self.maybe_register_invoke(&op);
        self.current_block().push(op)
    }

    /// Emit `n` copies of `op` to the current block
    #[inline(always)]
    pub fn emit_n(&mut self, count: usize, op: masm::Op) {
        self.maybe_register_invoke(&op);
        self.current_block().push_n(count, op);
    }

    /// Emit `ops` to the current block
    #[inline(always)]
    pub fn emit_all(&mut self, ops: &[masm::Op]) {
        for op in ops {
            self.maybe_register_invoke(op);
        }
        self.current_block().extend_from_slice(ops);
    }

    /// Emit `n` copies of the sequence `ops` to the current block
    #[inline(always)]
    pub fn emit_repeat(&mut self, count: usize, ops: &[masm::Op]) {
        for op in ops {
            self.maybe_register_invoke(op);
        }
        self.current_block().push_repeat(ops, count);
    }

    /// Emit `n` copies of the sequence `ops` to the current block
    #[inline]
    pub fn emit_template<const N: usize, F>(&mut self, count: usize, template: F)
    where
        F: Fn(usize) -> [Op; N],
    {
        for op in template(0) {
            self.maybe_register_invoke(&op);
        }

        let block = self.current_block();
        for n in 0..count {
            let ops = template(n);
            block.extend_from_slice(&ops);
        }
    }

    /// Push an immediate value on the operand stack
    ///
    /// This has no effect on the state of the emulated operand stack
    #[inline]
    pub fn push_immediate(&mut self, imm: Immediate) {
        match imm {
            Immediate::I1(i) => self.emit(Op::PushU8(i as u8)),
            Immediate::I8(i) => self.emit(Op::PushU8(i as u8)),
            Immediate::U8(i) => self.emit(Op::PushU8(i)),
            Immediate::U16(i) => self.emit(Op::PushU32(i as u32)),
            Immediate::I16(i) => self.emit(Op::PushU32(i as u16 as u32)),
            Immediate::U32(i) => self.emit(Op::PushU32(i)),
            Immediate::I32(i) => self.emit(Op::PushU32(i as u32)),
            Immediate::U64(i) => self.push_u64(i),
            Immediate::I64(i) => self.push_i64(i),
            Immediate::U128(i) => self.push_u128(i),
            Immediate::I128(i) => self.push_i128(i),
            Immediate::Felt(i) => self.emit(Op::Push(i)),
            Immediate::F64(_) => unimplemented!("floating-point immediates are not supported"),
        }
    }

    /// Push a literal on the operand stack, and update the emulated stack accordingly
    pub fn literal<I: Into<Immediate>>(&mut self, imm: I) {
        let imm = imm.into();
        self.push_immediate(imm);
        self.stack.push(imm);
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<Operand> {
        self.stack.pop()
    }

    /// Push an operand on the stack
    #[inline(always)]
    pub fn push<O: Into<Operand>>(&mut self, operand: O) {
        self.stack.push(operand)
    }

    /// Duplicate an item on the stack to the top
    #[inline]
    #[track_caller]
    pub fn dup(&mut self, i: u8) {
        assert_valid_stack_index!(i);
        let index = i as usize;
        let i = self.stack.effective_index(index) as u8;
        self.stack.dup(index);
        // Emit low-level instructions corresponding to the operand we duplicated
        let last = self.stack.peek().expect("operand stack is empty");
        let n = last.size();
        let offset = (n - 1) as u8;
        for _ in 0..n {
            self.emit(Op::Dup(i + offset));
        }
    }

    /// Move an item on the stack to the top
    #[inline]
    #[track_caller]
    pub fn movup(&mut self, i: u8) {
        assert_valid_stack_index!(i);
        let index = i as usize;
        let i = self.stack.effective_index(index) as u8;
        self.stack.movup(index);
        // Emit low-level instructions corresponding to the operand we moved
        let moved = self.stack.peek().expect("operand stack is empty");
        let n = moved.size();
        let offset = (n - 1) as u8;
        for _ in 0..n {
            self.emit(Op::Movup(i + offset));
        }
    }

    /// Move an item from the top of the stack to the `n`th position
    #[inline]
    #[track_caller]
    pub fn movdn(&mut self, i: u8) {
        assert_valid_stack_index!(i);
        let index = i as usize;
        let i = self.stack.effective_index_inclusive(index) as u8;
        let top = self.stack.peek().expect("operand stack is empty");
        let top_size = top.size();
        self.stack.movdn(index);
        // Emit low-level instructions corresponding to the operand we moved
        for _ in 0..top_size {
            self.emit(Op::Movdn(i));
        }
    }

    /// Swap an item with the top of the stack
    #[inline]
    #[track_caller]
    pub fn swap(&mut self, i: u8) {
        assert!(i > 0, "swap requires a non-zero index");
        assert_valid_stack_index!(i);
        let index = i as usize;
        let src = self.stack[0].size() as u8;
        let dst = self.stack[index].size() as u8;
        let i = self.stack.effective_index(index) as u8;
        self.stack.swap(index);
        match (src, dst) {
            (1, 1) => {
                self.emit(Op::Swap(i));
            }
            (1, n) if i == 1 => {
                // We can simply move the top element below the `dst` operand
                self.emit(Op::Movdn(i + (n - 1)));
            }
            (n, 1) if i == n => {
                // We can simply move the `dst` element to the top
                self.emit(Op::Movup(i));
            }
            (n, m) if i == n => {
                // We can simply move `dst` down
                for _ in 0..n {
                    self.emit(Op::Movdn(i + (m - 1)));
                }
            }
            (n, m) => {
                assert!(i >= n);
                let offset = m - 1;
                for _ in 0..n {
                    self.emit(Op::Movdn(i + offset));
                }
                let i = (i as i8 + (m as i8 - n as i8)) as u8;
                match i - 1 {
                    1 => {
                        assert_eq!(m, 1);
                        self.emit(Op::Swap(1));
                    }
                    i => {
                        for _ in 0..m {
                            self.emit(Op::Movup(i));
                        }
                    }
                }
            }
        }
    }

    /// Drop the top operand on the stack
    #[inline]
    #[track_caller]
    pub fn drop(&mut self) {
        let elem = self.stack.pop().expect("operand stack is empty");
        match elem.size() {
            1 => {
                self.emit(Op::Drop);
            }
            4 => {
                self.emit(Op::Dropw);
            }
            n => {
                for _ in 0..n {
                    self.emit(Op::Drop);
                }
            }
        }
    }

    /// Drop the top `n` operands on the stack
    #[inline]
    #[track_caller]
    pub fn dropn(&mut self, n: usize) {
        assert!(self.stack.len() >= n);
        assert_ne!(n, 0);
        let raw_len: usize = self.stack.iter().rev().take(n).map(|o| o.size()).sum();
        self.stack.dropn(n);
        match raw_len {
            1 => {
                self.emit(Op::Drop);
            }
            4 => {
                self.emit(Op::Dropw);
            }
            n => {
                self.emit_n(n / 4, Op::Dropw);
                self.emit_n(n % 4, Op::Drop);
            }
        }
    }

    /// Remove all but the top `n` values on the operand stack
    pub fn truncate_stack(&mut self, n: usize) {
        let stack_size = self.stack.len();
        let num_to_drop = stack_size - n;

        if num_to_drop == 0 {
            return;
        }

        if stack_size == num_to_drop {
            let raw_size = self.stack.raw_len();
            self.stack.dropn(num_to_drop);
            self.emit_n(raw_size / 4, Op::Dropw);
            self.emit_n(raw_size % 4, Op::Dropw);
            return;
        }

        // This is the common case, and can be handled simply
        // by moving the value to the bottom of the stack and
        // dropping everything in-between
        if n == 1 {
            match stack_size {
                2 => {
                    self.swap(1);
                    self.drop();
                }
                n => {
                    self.movdn(n as u8 - 1);
                    self.dropn(n - 1);
                }
            }
            return;
        }

        // TODO: This is a very neive algorithm for clearing
        // the stack of all but the top `n` values, we should
        // come up with a smarter/more efficient method
        for offset in 0..num_to_drop {
            let index = stack_size - 1 - offset;
            self.drop_operand_at_position(index);
        }
    }

    /// Remove the `n`th value from the top of the operand stack
    pub fn drop_operand_at_position(&mut self, n: usize) {
        match n {
            0 => {
                self.drop();
            }
            1 => {
                self.swap(1);
                self.drop();
            }
            n => {
                self.movup(n as u8);
                self.drop();
            }
        }
    }

    /// Copy the `n`th operand on the stack, and make it the `m`th operand on the stack.
    ///
    /// If the operand is for a commutative, binary operator, indicated by
    /// `is_commutative_binary_operand`, and the desired position is just below the top of
    /// stack, this function may leave it on top of the stack instead, since the order of the
    /// operands is not strict. This can result in fewer stack manipulation instructions in some
    /// scenarios.
    pub fn copy_operand_to_position(
        &mut self,
        n: usize,
        m: usize,
        is_commutative_binary_operand: bool,
    ) {
        match (n, m) {
            (0, 0) => {
                self.dup(0);
            }
            (actual, 0) => {
                self.dup(actual as u8);
            }
            (actual, 1) => {
                // If the dependent is binary+commutative, we can
                // leave operands in either the 0th or 1st position,
                // as long as both operands are on top of the stack
                if !is_commutative_binary_operand {
                    self.dup(actual as u8);
                    self.swap(1);
                } else {
                    self.dup(actual as u8);
                }
            }
            (actual, expected) => {
                self.dup(actual as u8);
                self.movdn(expected as u8);
            }
        }
    }

    /// Make the `n`th operand on the stack, the `m`th operand on the stack.
    ///
    /// If the operand is for a commutative, binary operator, indicated by
    /// `is_commutative_binary_operand`, and the desired position is one of the first two items
    /// on the stack, this function may leave the operand in it's current position if it is
    /// already one of the first two items on the stack, since the order of the operands is not
    /// strict. This can result in fewer stack manipulation instructions in some scenarios.
    pub fn move_operand_to_position(
        &mut self,
        n: usize,
        m: usize,
        is_commutative_binary_operand: bool,
    ) {
        match (n, m) {
            (n, m) if n == m => (),
            (1, 0) | (0, 1) => {
                // If the dependent is binary+commutative, we can
                // leave operands in either the 0th or 1st position,
                // as long as both operands are on top of the stack
                if !is_commutative_binary_operand {
                    self.swap(1);
                }
            }
            (actual, 0) => {
                self.movup(actual as u8);
            }
            (actual, 1) => {
                self.movup(actual as u8);
                self.swap(1);
            }
            (actual, expected) => {
                self.movup(actual as u8);
                self.movdn(expected as u8);
            }
        }
    }

    /// Get mutable access to the current block we're emitting to
    #[inline(always)]
    pub fn current_block<'c, 'b: 'c>(&'b mut self) -> &'c mut masm::Block {
        self.function.body.block_mut(self.current_block)
    }

    #[allow(unused)]
    #[inline]
    pub fn switch_to_block(&mut self, block: masm::BlockId) -> masm::BlockId {
        let prev = self.current_block;
        self.current_block = block;
        prev
    }

    #[allow(unused)]
    #[inline(always)]
    pub fn position(&self) -> masm::BlockId {
        self.current_block
    }
}

#[cfg(test)]
mod tests {
    use midenc_hir::{AbiParam, Felt, FieldElement, Overflow, Signature};

    use super::*;
    use crate::{codegen::TypedValue, masm::Function};

    #[test]
    fn op_emitter_stack_manipulation_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);
        let three = Immediate::U8(3);
        let four = Immediate::U64(2u64.pow(32));
        let five = Immediate::U64(2u64.pow(32) | 2u64.pow(33) | u32::MAX as u64);

        emitter.literal(one);
        emitter.literal(two);
        emitter.literal(three);
        emitter.literal(four);
        emitter.literal(five);

        {
            let block = emitter.current_block();
            let ops = block.ops.as_slice();
            assert_eq!(ops.len(), 5);
            assert_eq!(ops[0], Op::PushU32(1));
            assert_eq!(ops[1], Op::PushU32(2));
            assert_eq!(ops[2], Op::PushU8(3));
            assert_eq!(ops[3], Op::Push2([Felt::new(1), Felt::ZERO]));
            assert_eq!(ops[4], Op::Push2([Felt::new(3), Felt::new(u32::MAX as u64)]));
        }

        assert_eq!(emitter.stack()[0], five);
        assert_eq!(emitter.stack()[1], four);
        assert_eq!(emitter.stack()[2], three);
        assert_eq!(emitter.stack()[3], two);
        assert_eq!(emitter.stack()[4], one);

        emitter.dup(0);
        assert_eq!(emitter.stack()[0], five);
        assert_eq!(emitter.stack()[1], five);
        assert_eq!(emitter.stack()[2], four);
        assert_eq!(emitter.stack()[3], three);
        assert_eq!(emitter.stack()[4], two);

        {
            let block = emitter.current_block();
            let ops = block.ops.as_slice();
            assert_eq!(ops.len(), 7);
            assert_eq!(ops[5], Op::Dup(1));
            assert_eq!(ops[6], Op::Dup(1));
        }

        assert_eq!(emitter.stack().effective_index(3), 6);
        emitter.dup(3);
        assert_eq!(emitter.stack()[0], three);
        assert_eq!(emitter.stack()[1], five);
        assert_eq!(emitter.stack()[2], five);
        assert_eq!(emitter.stack()[3], four);
        assert_eq!(emitter.stack()[4], three);
        assert_eq!(emitter.stack()[5], two);

        {
            let block = emitter.current_block();
            let ops = block.ops.as_slice();
            assert_eq!(ops.len(), 8);
            assert_eq!(ops[6], Op::Dup(1));
            assert_eq!(ops[7], Op::Dup(6));
        }

        assert_eq!(emitter.stack().effective_index(1), 1);
        emitter.swap(1);
        assert_eq!(emitter.stack().effective_index(1), 2);
        assert_eq!(emitter.stack()[0], five);
        assert_eq!(emitter.stack()[1], three);
        assert_eq!(emitter.stack()[2], five);
        assert_eq!(emitter.stack()[3], four);
        assert_eq!(emitter.stack()[4], three);
        assert_eq!(emitter.stack()[5], two);

        {
            let block = emitter.current_block();
            let ops = block.ops.as_slice();
            assert_eq!(ops.len(), 9);
            assert_eq!(ops[7], Op::Dup(6));
            assert_eq!(ops[8], Op::Movdn(2));
        }

        assert_eq!(emitter.stack().effective_index(3), 5);
        emitter.swap(3);
        assert_eq!(emitter.stack()[0], four);
        assert_eq!(emitter.stack()[1], three);
        assert_eq!(emitter.stack()[2], five);
        assert_eq!(emitter.stack()[3], five);
        assert_eq!(emitter.stack()[4], three);
        assert_eq!(emitter.stack()[5], two);

        {
            let block = emitter.current_block();
            let ops = block.ops.as_slice();
            assert_eq!(ops.len(), 13);
            assert_eq!(ops[8], Op::Movdn(2)); // [five_a, five_b, three, five_c, five_d, four_a, four_b]
            assert_eq!(ops[9], Op::Movdn(6)); // [five_b, three, five_c, five_d, four_a, four_b, five_a]
            assert_eq!(ops[10], Op::Movdn(6)); // [three, five_c, five_d, four_a, four_b, five_a, five_b]
            assert_eq!(ops[11], Op::Movup(4)); // [four_b, three, five_c, five_d, four_a, five_a, five_b]
            assert_eq!(ops[12], Op::Movup(4)); // [four_a, four_b, three, five_c, five_d, five_a,
                                               // five_b]
        }

        emitter.movdn(2);
        assert_eq!(emitter.stack()[0], three);
        assert_eq!(emitter.stack()[1], five);
        assert_eq!(emitter.stack()[2], four);
        assert_eq!(emitter.stack()[3], five);
        assert_eq!(emitter.stack()[4], three);
        assert_eq!(emitter.stack()[5], two);

        {
            let block = emitter.current_block();
            let ops = block.ops.as_slice();
            assert_eq!(ops.len(), 15);
            assert_eq!(ops[9], Op::Movdn(6)); // [five_b, three, five_c, five_d, four_a, four_b, five_a]
            assert_eq!(ops[10], Op::Movdn(6)); // [three, five_c, five_d, four_a, four_b, five_a, five_b]
            assert_eq!(ops[11], Op::Movup(4)); // [four_b, three, five_c, five_d, four_a, five_a, five_b]
            assert_eq!(ops[12], Op::Movup(4)); // [four_a, four_b, three, five_c, five_d, five_a, five_b]
            assert_eq!(ops[13], Op::Movdn(4)); // [four_b, three, five_c, five_d, four_a, five_a, five_b]
            assert_eq!(ops[14], Op::Movdn(4)); // [three, five_c, five_d, four_a, four_b, five_a,
                                               // five_b]
        }

        emitter.movup(2);
        assert_eq!(emitter.stack()[0], four);
        assert_eq!(emitter.stack()[1], three);
        assert_eq!(emitter.stack()[2], five);
        assert_eq!(emitter.stack()[3], five);
        assert_eq!(emitter.stack()[4], three);
        assert_eq!(emitter.stack()[5], two);

        {
            let block = emitter.current_block();
            let ops = block.ops.as_slice();
            assert_eq!(ops.len(), 17);
            assert_eq!(ops[13], Op::Movdn(4)); // [four_b, three, five_c, five_d, four_a, five_a, five_b]
            assert_eq!(ops[14], Op::Movdn(4)); // [three, five_c, five_d, four_a, four_b, five_a, five_b]
            assert_eq!(ops[15], Op::Movup(4)); // [four_b, three, five_c, five_d, four_a, five_a, five_b]
            assert_eq!(ops[16], Op::Movup(4)); // [four_a, four_b, three, five_c, five_d, five_a,
                                               // five_b]
        }

        emitter.drop();
        assert_eq!(emitter.stack()[0], three);
        assert_eq!(emitter.stack()[1], five);
        assert_eq!(emitter.stack()[2], five);
        assert_eq!(emitter.stack()[3], three);
        assert_eq!(emitter.stack()[4], two);
        assert_eq!(emitter.stack()[5], one);

        {
            let block = emitter.current_block();
            let ops = block.ops.as_slice();
            assert_eq!(ops.len(), 19);
            assert_eq!(ops[15], Op::Movup(4)); // [four_b, three, five_c, five_d, four_a, five_a, five_b]
            assert_eq!(ops[16], Op::Movup(4)); // [four_a, four_b, three, five_c, five_d, five_a, five_b]
            assert_eq!(ops[17], Op::Drop); // [four_b, three, five_c, five_d, five_a, five_b]
            assert_eq!(ops[18], Op::Drop); // [three, five_c, five_d, five_a, five_b]
        }

        emitter.copy_operand_to_position(5, 3, false);
        assert_eq!(emitter.stack()[0], three);
        assert_eq!(emitter.stack()[1], five);
        assert_eq!(emitter.stack()[2], five);
        assert_eq!(emitter.stack()[3], one);
        assert_eq!(emitter.stack()[4], three);
        assert_eq!(emitter.stack()[5], two);

        emitter.drop_operand_at_position(4);
        assert_eq!(emitter.stack()[0], three);
        assert_eq!(emitter.stack()[1], five);
        assert_eq!(emitter.stack()[2], five);
        assert_eq!(emitter.stack()[3], one);
        assert_eq!(emitter.stack()[4], two);

        emitter.move_operand_to_position(4, 2, false);
        assert_eq!(emitter.stack()[0], three);
        assert_eq!(emitter.stack()[1], five);
        assert_eq!(emitter.stack()[2], two);
        assert_eq!(emitter.stack()[3], five);
        assert_eq!(emitter.stack()[4], one);
    }

    #[test]
    fn op_emitter_copy_operand_to_position_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let v14 = hir::Value::from_u32(14);
        let v13 = hir::Value::from_u32(13);
        let v15 = hir::Value::from_u32(15);
        let v11 = hir::Value::from_u32(11);
        let v10 = hir::Value::from_u32(10);
        let v1 = hir::Value::from_u32(1);
        let v2 = hir::Value::from_u32(2);

        emitter.push(TypedValue {
            value: v2,
            ty: Type::U32,
        });
        emitter.push(TypedValue {
            value: v1,
            ty: Type::U32,
        });
        emitter.push(TypedValue {
            value: v10,
            ty: Type::U32,
        });
        emitter.push(TypedValue {
            value: v11,
            ty: Type::U32,
        });
        emitter.push(TypedValue {
            value: v15,
            ty: Type::U32,
        });
        emitter.push(TypedValue {
            value: v13,
            ty: Type::U32,
        });
        emitter.push(TypedValue {
            value: v14,
            ty: Type::U32,
        });

        assert_eq!(emitter.stack().find(&v10), Some(4));

        assert_eq!(emitter.stack()[4], v10);
        assert_eq!(emitter.stack()[2], v15);
        emitter.copy_operand_to_position(4, 2, false);
        assert_eq!(emitter.stack()[5], v10);
        assert_eq!(emitter.stack()[2], v10);

        {
            let block = emitter.current_block();
            let ops = block.ops.as_slice();
            assert_eq!(ops.len(), 2);
            assert_eq!(ops[0], Op::Dup(4));
            assert_eq!(ops[1], Op::Movdn(2));
        }
    }

    #[test]
    fn op_emitter_u32_add_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.add_imm(one, Overflow::Checked);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], one);

        emitter.add(Overflow::Checked);
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);

        emitter.add_imm(one, Overflow::Overflowing);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::I1);
        assert_eq!(emitter.stack()[1], Type::U32);

        emitter.drop();
        emitter.dup(0);
        emitter.add(Overflow::Overflowing);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::I1);
        assert_eq!(emitter.stack()[1], Type::U32);
    }

    #[test]
    fn op_emitter_u32_sub_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.sub_imm(one, Overflow::Checked);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], one);

        emitter.sub(Overflow::Checked);
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);

        emitter.sub_imm(one, Overflow::Overflowing);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::I1);
        assert_eq!(emitter.stack()[1], Type::U32);

        emitter.drop();
        emitter.dup(0);
        emitter.sub(Overflow::Overflowing);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::I1);
        assert_eq!(emitter.stack()[1], Type::U32);
    }

    #[test]
    fn op_emitter_u32_mul_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.mul_imm(one, Overflow::Checked);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], one);

        emitter.mul(Overflow::Checked);
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);

        emitter.mul_imm(one, Overflow::Overflowing);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::I1);
        assert_eq!(emitter.stack()[1], Type::U32);

        emitter.drop();
        emitter.dup(0);
        emitter.mul(Overflow::Overflowing);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::I1);
        assert_eq!(emitter.stack()[1], Type::U32);
    }

    #[test]
    fn op_emitter_u32_eq_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.eq_imm(two);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::I1);
        assert_eq!(emitter.stack()[1], one);

        emitter.assert(None);
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], one);

        emitter.dup(0);
        emitter.eq();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::I1);
    }

    #[test]
    fn op_emitter_u32_neq_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.neq_imm(two);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::I1);
        assert_eq!(emitter.stack()[1], one);

        emitter.assertz(None);
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], one);

        emitter.dup(0);
        emitter.neq();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::I1);
    }

    #[test]
    fn op_emitter_i1_and_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let t = Immediate::I1(true);
        let f = Immediate::I1(false);

        emitter.literal(t);
        emitter.literal(f);

        emitter.and_imm(t);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::I1);
        assert_eq!(emitter.stack()[1], t);

        emitter.and();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::I1);
    }

    #[test]
    fn op_emitter_i1_or_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let t = Immediate::I1(true);
        let f = Immediate::I1(false);

        emitter.literal(t);
        emitter.literal(f);

        emitter.or_imm(t);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::I1);
        assert_eq!(emitter.stack()[1], t);

        emitter.or();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::I1);
    }

    #[test]
    fn op_emitter_i1_xor_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let t = Immediate::I1(true);
        let f = Immediate::I1(false);

        emitter.literal(t);
        emitter.literal(f);

        emitter.xor_imm(t);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::I1);
        assert_eq!(emitter.stack()[1], t);

        emitter.xor();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::I1);
    }

    #[test]
    fn op_emitter_i1_not_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let t = Immediate::I1(true);

        emitter.literal(t);

        emitter.not();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::I1);
    }

    #[test]
    fn op_emitter_u32_gt_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.gt_imm(two);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::I1);
        assert_eq!(emitter.stack()[1], one);

        emitter.drop();
        emitter.dup(0);
        emitter.gt();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::I1);
    }

    #[test]
    fn op_emitter_u32_gte_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.gte_imm(two);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::I1);
        assert_eq!(emitter.stack()[1], one);

        emitter.drop();
        emitter.dup(0);
        emitter.gte();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::I1);
    }

    #[test]
    fn op_emitter_u32_lt_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.lt_imm(two);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::I1);
        assert_eq!(emitter.stack()[1], one);

        emitter.drop();
        emitter.dup(0);
        emitter.lt();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::I1);
    }

    #[test]
    fn op_emitter_u32_lte_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.lte_imm(two);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::I1);
        assert_eq!(emitter.stack()[1], one);

        emitter.drop();
        emitter.dup(0);
        emitter.lte();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::I1);
    }

    #[test]
    fn op_emitter_u32_checked_div_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.checked_div_imm(two);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], one);

        emitter.checked_div();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);
    }

    #[test]
    fn op_emitter_u32_unchecked_div_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.unchecked_div_imm(two);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], one);

        emitter.unchecked_div();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);
    }

    #[test]
    fn op_emitter_u32_checked_mod_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.checked_mod_imm(two);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], one);

        emitter.checked_mod();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);
    }

    #[test]
    fn op_emitter_u32_unchecked_mod_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.unchecked_mod_imm(two);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], one);

        emitter.unchecked_mod();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);
    }

    #[test]
    fn op_emitter_u32_checked_divmod_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.checked_divmod_imm(two);
        assert_eq!(emitter.stack_len(), 3);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], Type::U32);
        assert_eq!(emitter.stack()[2], one);

        emitter.checked_divmod();
        assert_eq!(emitter.stack_len(), 3);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], Type::U32);
        assert_eq!(emitter.stack()[2], one);
    }

    #[test]
    fn op_emitter_u32_unchecked_divmod_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.unchecked_divmod_imm(two);
        assert_eq!(emitter.stack_len(), 3);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], Type::U32);
        assert_eq!(emitter.stack()[2], one);

        emitter.unchecked_divmod();
        assert_eq!(emitter.stack_len(), 3);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], Type::U32);
        assert_eq!(emitter.stack()[2], one);
    }

    #[test]
    fn op_emitter_u32_exp_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.exp_imm(two);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], one);

        emitter.exp();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);
    }

    #[test]
    fn op_emitter_u32_band_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.band_imm(one);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], one);

        emitter.band();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);
    }

    #[test]
    fn op_emitter_u32_bor_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.bor_imm(one);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], one);

        emitter.bor();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);
    }

    #[test]
    fn op_emitter_u32_bxor_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.bxor_imm(one);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], one);

        emitter.bxor();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);
    }

    #[test]
    fn op_emitter_u32_shl_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.shl_imm(one);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], one);

        emitter.shl();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);
    }

    #[test]
    fn op_emitter_u32_shr_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.shr_imm(one);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], one);

        emitter.shr();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);
    }

    #[test]
    fn op_emitter_u32_rotl_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.rotl_imm(one);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], one);

        emitter.rotl();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);
    }

    #[test]
    fn op_emitter_u32_rotr_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.rotr_imm(one);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], one);

        emitter.rotr();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);
    }

    #[test]
    fn op_emitter_u32_min_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.min_imm(one);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], one);

        emitter.min();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);
    }

    #[test]
    fn op_emitter_u32_max_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);

        emitter.max_imm(one);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::U32);
        assert_eq!(emitter.stack()[1], one);

        emitter.max();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);
    }

    #[test]
    fn op_emitter_u32_trunc_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let max = Immediate::U32(u32::MAX);

        emitter.literal(max);

        emitter.trunc(&Type::U16);
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U16);
    }

    #[test]
    fn op_emitter_u32_zext_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let one = Immediate::U16(1);

        emitter.literal(one);

        emitter.zext(&Type::U32);
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);
    }

    #[test]
    fn op_emitter_u32_sext_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let num = Immediate::I16(-128);

        emitter.literal(num);

        emitter.sext(&Type::I32);
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::I32);
    }

    #[test]
    fn op_emitter_u32_cast_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let num = Immediate::U32(128);

        emitter.literal(num);

        emitter.cast(&Type::I32);
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::I32);
    }

    #[test]
    fn op_emitter_u32_inttoptr_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let addr = Immediate::U32(128);
        let ptr = Type::Ptr(Box::new(Type::Array(Box::new(Type::U64), 8)));

        emitter.literal(addr);

        emitter.inttoptr(&ptr);
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], ptr);
    }

    #[test]
    fn op_emitter_u32_is_odd_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let num = Immediate::U32(128);

        emitter.literal(num);

        emitter.is_odd();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::I1);
    }

    #[test]
    fn op_emitter_u32_popcnt_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let num = Immediate::U32(128);

        emitter.literal(num);

        emitter.popcnt();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);
    }

    #[test]
    fn op_emitter_u32_bnot_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let num = Immediate::U32(128);

        emitter.literal(num);

        emitter.bnot();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);
    }

    #[test]
    fn op_emitter_u32_pow2_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let ten = Immediate::U32(10);

        emitter.literal(ten);

        emitter.pow2();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);
    }

    #[test]
    fn op_emitter_u32_incr_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let ten = Immediate::U32(10);

        emitter.literal(ten);

        emitter.incr();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);
    }

    #[test]
    fn op_emitter_inv_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let ten = Immediate::Felt(Felt::new(10));

        emitter.literal(ten);

        emitter.inv();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::Felt);
    }

    #[test]
    fn op_emitter_neg_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let ten = Immediate::Felt(Felt::new(10));

        emitter.literal(ten);

        emitter.neg();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::Felt);
    }

    #[test]
    fn op_emitter_u32_assert_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let ten = Immediate::U32(10);

        emitter.literal(ten);
        assert_eq!(emitter.stack_len(), 1);

        emitter.assert(None);
        assert_eq!(emitter.stack_len(), 0);
    }

    #[test]
    fn op_emitter_u32_assertz_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let ten = Immediate::U32(10);

        emitter.literal(ten);
        assert_eq!(emitter.stack_len(), 1);

        emitter.assertz(None);
        assert_eq!(emitter.stack_len(), 0);
    }

    #[test]
    fn op_emitter_u32_assert_eq_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let ten = Immediate::U32(10);

        emitter.literal(ten);
        emitter.literal(ten);
        emitter.literal(ten);
        assert_eq!(emitter.stack_len(), 3);

        emitter.assert_eq_imm(ten);
        assert_eq!(emitter.stack_len(), 2);

        emitter.assert_eq();
        assert_eq!(emitter.stack_len(), 0);
    }

    #[test]
    fn op_emitter_u32_select_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let t = Immediate::I1(true);
        let one = Immediate::U32(1);
        let two = Immediate::U32(2);

        emitter.literal(one);
        emitter.literal(two);
        emitter.literal(t);
        assert_eq!(emitter.stack_len(), 3);

        emitter.select();
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);
    }

    #[test]
    fn op_emitter_u32_exec_test() {
        use midenc_hir::ExternalFunction;

        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let return_ty = Type::Array(Box::new(Type::U32), 1);
        let callee = ExternalFunction {
            id: "test::add".parse().unwrap(),
            signature: Signature::new(
                [AbiParam::new(Type::U32), AbiParam::new(Type::I1)],
                [AbiParam::new(return_ty.clone())],
            ),
        };

        let t = Immediate::I1(true);
        let one = Immediate::U32(1);

        emitter.literal(t);
        emitter.literal(one);
        assert_eq!(emitter.stack_len(), 2);

        emitter.exec(&callee);
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], return_ty);
    }

    #[test]
    fn op_emitter_u32_load_test() {
        let mut function = setup();
        let entry = function.body.id();
        let mut stack = OperandStack::default();
        let mut emitter = OpEmitter::new(&mut function, entry, &mut stack);

        let addr = Type::Ptr(Box::new(Type::U8));

        emitter.push(addr);
        assert_eq!(emitter.stack_len(), 1);

        emitter.load(Type::U32);
        assert_eq!(emitter.stack_len(), 1);
        assert_eq!(emitter.stack()[0], Type::U32);

        emitter.load_imm(128, Type::I32);
        assert_eq!(emitter.stack_len(), 2);
        assert_eq!(emitter.stack()[0], Type::I32);
        assert_eq!(emitter.stack()[1], Type::U32);
    }

    #[inline]
    fn setup() -> Function {
        Function::new(
            "test::test".parse().unwrap(),
            Signature::new([AbiParam::new(Type::U32)], [AbiParam::new(Type::U32)]),
        )
    }
}
