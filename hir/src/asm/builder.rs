use crate::{
    CallConv, DataFlowGraph, Felt, FunctionIdent, Inst, InstBuilder, Instruction, Overflow,
    SourceSpan, Type, TypeRepr, Value,
};

use super::*;

/// Used to construct an [InlineAsm] instruction, while checking the input/output types,
/// and enforcing various safety invariants.
pub struct MasmBuilder<B> {
    /// The [InstBuilderBase] which we are building from
    builder: B,
    /// The span of the resulting inline assembly block
    span: SourceSpan,
    /// The inline assembly block we're building
    asm: InlineAsm,
    /// The current code block in the inline assembly that the builder is inserting into
    current_block: MasmBlockId,
    /// The emulated operand stack, primarily used to track the number of stack elements
    /// upon entry and exit from the inline assembly block.
    ///
    /// The only `Type` which is represented on this stack is `Type::Felt`, since we're only
    /// interested in the number of stack elements at any given point. In the future, we may
    /// choose to do additional type checking.
    ///
    /// Upon exit from the inline assembly block, the state of the stack must have enough elements
    /// to store a value of the expected result type, represented by `ty`. Whether those elements
    /// actually store a valid value of that type is beyond the scope of this builder, for now.
    stack: OperandStack<Type>,
}
impl<'f, B: InstBuilder<'f>> MasmBuilder<B> {
    /// Construct a new inline assembly builder in the function represented by `dfg`, to be inserted at `ip`.
    ///
    /// The `args` list represents the arguments which will be visible on the operand stack in this inline assembly block.
    ///
    /// The `results` set represents the types that are expected to be found on the operand stack when the inline
    /// assembly block finishes executing. Use an empty set to represent no results.
    ///
    /// Any attempt to modify the operand stack beyond what is made visible via arguments, or introduced within the
    /// inline assembly block, will cause an assertion to fail.
    pub fn new(mut builder: B, args: &[Value], results: Vec<Type>, span: SourceSpan) -> Self {
        // Construct the initial operand stack with the given arguments
        let mut stack = OperandStack::<Type>::default();
        {
            let dfg = builder.data_flow_graph();
            for arg in args.iter().rev().copied() {
                stack.push(dfg.value_type(arg).clone());
            }
        }

        // Construct an empty inline assembly block with the given arguments
        let mut asm = InlineAsm::new(results);
        {
            let dfg = builder.data_flow_graph_mut();
            let mut vlist = ValueList::default();
            vlist.extend(args.iter().copied(), &mut dfg.value_lists);
            asm.args = vlist;
        }

        let current_block = asm.body;
        Self {
            builder,
            span,
            asm,
            current_block,
            stack,
        }
    }

    /// Create a new, empty MASM code block, for use with control flow instructions
    #[inline]
    pub fn create_block(&mut self) -> MasmBlockId {
        self.asm.create_block()
    }

    /// Change the insertion point of the builder to the end of `block`
    #[inline(always)]
    pub fn switch_to_block(&mut self, block: MasmBlockId) {
        self.current_block = block;
    }

    /// Get a builder for a single MASM instruction
    pub fn ins<'a, 'b: 'a>(&'b mut self) -> MasmOpBuilder<'a> {
        MasmOpBuilder {
            dfg: self.builder.data_flow_graph_mut(),
            asm: &mut self.asm,
            stack: &mut self.stack,
            ip: self.current_block,
        }
    }

    /// Finalize this inline assembly block, inserting it into the `Function` from which this builder was derived.
    ///
    /// Returns the [Inst] which corresponds to the inline assembly instruction, and the inner [DataFlowGraph] reference
    /// held by the underlying [InstBuilderBase].
    pub fn build(self) -> Inst {
        if self.asm.results.is_empty() {
            assert!(self.stack.is_empty(), "invalid inline assembly: expected operand stack to be empty upon exit, found: {:?}", self.stack.display());
        } else {
            let mut len = 0;
            for ty in self.asm.results.iter() {
                let repr = ty.repr().expect("invalid result type");
                len += repr.size();
            }
            assert_eq!(
                len,
                self.stack.len(),
                "invalid inline assembly: needed {} elements on the operand stack, found: {:?}",
                len,
                self.stack.display()
            );
        }

        let span = self.span;
        let data = Instruction::InlineAsm(self.asm);
        self.builder.build(data, Type::Unit, span).0
    }
}

/// Used to construct a single MASM opcode
pub struct MasmOpBuilder<'a> {
    dfg: &'a mut DataFlowGraph,
    /// The inline assembly block being created
    asm: &'a mut InlineAsm,
    /// The state of the operand stack at this point in the program
    stack: &'a mut OperandStack<Type>,
    /// The block to which this builder should append the instruction it builds
    ip: MasmBlockId,
}
impl<'a> MasmOpBuilder<'a> {
    /// Pads the stack with four zero elements
    pub fn padw(self) {
        self.stack.padw();
        self.asm.push(self.ip, MasmOp::Padw);
    }

    /// Pushes an element on the stack
    pub fn push(self, imm: Felt) {
        self.stack.push(Type::Felt);
        self.asm.push(self.ip, MasmOp::Push(imm));
    }

    /// Pushes a word on the stack
    pub fn pushw(self, word: [Felt; 4]) {
        self.stack.padw();
        self.asm.push(self.ip, MasmOp::Pushw(word));
    }

    /// Pushes an element representing an unsigned 8-bit integer on the stack
    pub fn push_u8(self, imm: u8) {
        self.stack.push(Type::U8);
        self.asm.push(self.ip, MasmOp::PushU8(imm));
    }

    /// Pushes an element representing an unsigned 16-bit integer on the stack
    pub fn push_u16(self, imm: u16) {
        self.stack.push(Type::U16);
        self.asm.push(self.ip, MasmOp::PushU16(imm));
    }

    /// Pushes an element representing an unsigned 32-bit integer on the stack
    pub fn push_u32(self, imm: u32) {
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::PushU32(imm));
    }

    /// Drops the element on the top of the stack
    pub fn drop(self) {
        self.stack.drop();
        self.asm.push(self.ip, MasmOp::Drop);
    }

    /// Drops the word (first four elements) on the top of the stack
    pub fn dropw(self) {
        self.stack.dropw();
        self.asm.push(self.ip, MasmOp::Dropw);
    }

    /// Duplicates the `n`th element from the top of the stack, to the top of the stack
    ///
    /// A `n` of zero, duplicates the element on top of the stack
    ///
    /// The valid range for `n` is 0..=15
    pub fn dup(self, n: usize) {
        self.stack.dup(n);
        self.asm.push(self.ip, MasmOp::Dup(n as u8));
    }

    /// Duplicates the `n`th word from the top of the stack, to the top of the stack
    ///
    /// A `n` of zero, duplicates the word on top of the stack
    ///
    /// The valid range for `n` is 0..=3
    pub fn dupw(self, n: usize) {
        self.stack.dupw(n);
        self.asm.push(self.ip, MasmOp::Dupw(n as u8));
    }

    /// Swaps the `n`th element and the element on top of the stack
    ///
    /// The valid range for `n` is 1..=15
    pub fn swap(self, n: usize) {
        self.stack.swap(n);
        self.asm.push(self.ip, MasmOp::Swap(n as u8));
    }

    /// Swaps the `n`th word and the word on top of the stack
    ///
    /// The valid range for `n` is 1..=3
    pub fn swapw(self, n: usize) {
        self.stack.swapw(n);
        self.asm.push(self.ip, MasmOp::Swapw(n as u8));
    }

    /// Moves the `n`th element to the top of the stack
    ///
    /// The valid range for `n` is 2..=15
    pub fn movup(self, idx: usize) {
        self.stack.movup(idx);
        self.asm.push(self.ip, MasmOp::Movup(idx as u8));
    }

    /// Moves the `n`th word to the top of the stack
    ///
    /// The valid range for `n` is 2..=3
    pub fn movupw(self, idx: usize) {
        self.stack.movupw(idx);
        self.asm.push(self.ip, MasmOp::Movupw(idx as u8));
    }

    /// Moves the element on top of the stack, making it the `n`th element
    ///
    /// The valid range for `n` is 2..=15
    pub fn movdn(self, idx: usize) {
        self.stack.movdn(idx);
        self.asm.push(self.ip, MasmOp::Movdn(idx as u8));
    }

    /// Moves the word on top of the stack, making it the `n`th word
    ///
    /// The valid range for `n` is 2..=3
    pub fn movdnw(self, idx: usize) {
        self.stack.movdnw(idx);
        self.asm.push(self.ip, MasmOp::Movdnw(idx as u8));
    }

    /// Pops a boolean element off the stack, and swaps the top two elements
    /// on the stack if that boolean is true.
    ///
    /// Traps if the conditional is not 0 or 1.
    pub fn cswap(self) {
        self.stack.drop();
        self.asm.push(self.ip, MasmOp::Cswap);
    }

    /// Pops a boolean element off the stack, and swaps the top two words
    /// on the stack if that boolean is true.
    ///
    /// Traps if the conditional is not 0 or 1.
    pub fn cswapw(self) {
        self.stack.drop();
        self.asm.push(self.ip, MasmOp::Cswapw);
    }

    /// Pops a boolean element off the stack, and drops the top element on the
    /// stack if the boolean is true, otherwise it drops the next element down.
    ///
    /// Traps if the conditional is not 0 or 1.
    pub fn cdrop(self) {
        self.stack.dropn(2);
        self.asm.push(self.ip, MasmOp::Cdrop);
    }

    /// Pops a boolean element off the stack, and drops the top word on the
    /// stack if the boolean is true, otherwise it drops the next word down.
    ///
    /// Traps if the conditional is not 0 or 1.
    pub fn cdropw(self) {
        self.stack.dropn(5);
        self.asm.push(self.ip, MasmOp::Cdropw);
    }

    /// Pops the top element on the stack, and traps if that element is != 1.
    pub fn assert(self) {
        self.stack.drop();
        self.asm.push(self.ip, MasmOp::Assert);
    }

    /// Pops the top element on the stack, and traps if that element is != 0.
    pub fn assertz(self) {
        self.stack.drop();
        self.asm.push(self.ip, MasmOp::Assertz);
    }

    /// Pops the top two elements on the stack, and traps if they are not equal.
    pub fn assert_eq(self) {
        self.stack.dropn(2);
        self.asm.push(self.ip, MasmOp::AssertEq);
    }

    /// Pops the top two words on the stack, and traps if they are not equal.
    pub fn assert_eqw(self) {
        self.stack.dropn(8);
        self.asm.push(self.ip, MasmOp::AssertEq);
    }

    /// Pops an element containing a memory address from the top of the stack,
    /// and loads the first element of the word at that address to the top of the stack.
    pub fn load(self) {
        self.stack.drop();
        self.stack.push(Type::Felt);
        self.asm.push(self.ip, MasmOp::MemLoad);
    }

    /// Loads the first element of the word at the given address to the top of the stack.
    pub fn load_imm(self, addr: u32) {
        self.stack.push(Type::Felt);
        self.asm.push(self.ip, MasmOp::MemLoadImm(addr));
    }

    /// Pops an element containing a memory address + element offset from the top of the stack,
    /// and loads the element of the word at that address + offset to the top of the stack.
    ///
    /// NOTE: This is an experimental instruction which is not implemented in Miden VM yet.
    pub fn load_offset(self) {
        self.stack.drop();
        self.stack.push(Type::Felt);
        self.asm.push(self.ip, MasmOp::MemLoadOffset);
    }

    /// Loads the element of the word at the given address and element offset to the top of the stack.
    ///
    /// NOTE: This is an experimental instruction which is not implemented in Miden VM yet.
    pub fn load_offset_imm(self, addr: u32, offset: u8) {
        assert!(
            offset < 4,
            "invalid element offset, must be in the range 0..=3, got {}",
            offset
        );
        self.stack.push(Type::Felt);
        self.asm
            .push(self.ip, MasmOp::MemLoadOffsetImm(addr, offset));
    }

    /// Pops an element containing a memory address from the top of the stack,
    /// and loads the word at that address to the top of the stack.
    pub fn loadw(self) {
        self.stack.drop();
        self.stack.padw();
        self.asm.push(self.ip, MasmOp::MemLoadw);
    }

    /// Loads the word at the given address to the top of the stack.
    pub fn loadw_imm(self, addr: u32) {
        self.stack.padw();
        self.asm.push(self.ip, MasmOp::MemLoadwImm(addr));
    }

    /// Pops two elements, the first containing a memory address from the top of the stack,
    /// the second the value to be stored as the first element of the word at that address.
    pub fn store(self) {
        self.stack.dropn(2);
        self.asm.push(self.ip, MasmOp::MemStore);
    }

    /// Pops an element from the top of the stack, and stores it as the first element of
    /// the word at the given address.
    pub fn store_imm(self, addr: u32) {
        self.stack.drop();
        self.asm.push(self.ip, MasmOp::MemStoreImm(addr));
    }

    /// Pops two elements, the first containing a memory address + element offset from the
    /// top of the stack, the second the value to be stored to the word at that address,
    /// using the offset to determine which element will be written to.
    pub fn store_offset(self) {
        self.stack.drop();
        self.asm.push(self.ip, MasmOp::MemStoreOffset);
    }

    /// Pops an element from the top of the stack, and stores it at the given offset of
    /// the word at the given address.
    pub fn store_offset_imm(self, addr: u32, offset: u8) {
        assert!(
            offset < 4,
            "invalid element offset, must be in the range 0..=3, got {}",
            offset
        );
        self.stack.drop();
        self.asm
            .push(self.ip, MasmOp::MemStoreOffsetImm(addr, offset));
    }

    /// Pops an element containing a memory address from the top of the stack,
    /// and then pops a word from the stack and stores it as the word at that address.
    pub fn storew(self) {
        self.stack.dropn(5);
        self.asm.push(self.ip, MasmOp::MemStorew);
    }

    /// Pops a word from the stack and stores it as the word at the given address.
    pub fn storew_imm(self, addr: u32) {
        self.stack.dropw();
        self.asm.push(self.ip, MasmOp::MemStorewImm(addr));
    }

    /// Begins construction of a `if.true` statement.
    ///
    /// An `if.true` pops a boolean value off the stack, and uses it to choose between
    /// one of two branches. The "then" branch is taken if the conditional is true,
    /// the "else" branch otherwise.
    ///
    /// NOTE: This function will panic if the top of the operand stack is not of boolean type
    /// when called.
    ///
    /// You must ensure that both branches of the `if.true` statement leave the operand stack
    /// in the same abstract state, so that when control resumes after the `if.true`, the remaining
    /// program is well-formed. This will be validated automatically for you, but if validation
    /// fails, the builder will panic.
    pub fn if_true(self) -> IfTrueBuilder<'a> {
        let cond = self.stack.pop().expect("operand stack is empty");
        assert_eq!(
            cond,
            Type::I1,
            "expected while.true condition to be a boolean value"
        );
        let out_stack = self.stack.clone();
        IfTrueBuilder {
            dfg: self.dfg,
            asm: self.asm,
            in_stack: self.stack,
            out_stack,
            ip: self.ip,
            then_blk: None,
            else_blk: None,
        }
    }

    /// Begins construction of a `while.true` loop.
    ///
    /// A `while.true` pops a boolean value off the stack to use as the condition for
    /// entering the loop, and will then execute the loop body for as long as the value
    /// on top of the stack is a boolean and true. If the condition is not a boolean,
    /// execution traps.
    ///
    /// NOTE: This function will panic if the top of the operand stack is not of boolean type
    /// when called.
    ///
    /// Before finalizing construction of the loop body, you must ensure two things:
    ///
    /// 1. There is a value of boolean type on top of the operand stack
    /// 2. The abstract state of the operand stack, assuming the boolean just mentioned
    /// has been popped, must be consistent with the state of the operand stack when the
    /// loop was entered, as well as if the loop was skipped due to the conditional being
    /// false. The abstract state referred to here is the number, and type, of the elements
    /// on the operand stack.
    ///
    /// Both of these are validated by [LoopBuilder], and a panic is raised if validation fails.
    pub fn while_true(self) -> LoopBuilder<'a> {
        let cond = self.stack.pop().expect("operand stack is empty");
        assert_eq!(
            cond,
            Type::I1,
            "expected while.true condition to be a boolean value"
        );
        let out_stack = self.stack.clone();
        let body = self.asm.create_block();
        LoopBuilder {
            dfg: self.dfg,
            asm: self.asm,
            in_stack: self.stack,
            out_stack,
            ip: self.ip,
            body,
            style: LoopType::While,
        }
    }

    /// Begins construction of a `repeat` loop, with an iteration count of `n`.
    ///
    /// A `repeat` instruction requires no operands on the stack, and will execute the loop body `n` times.
    ///
    /// NOTE: The iteration count must be non-zero, or this function will panic.
    pub fn repeat(self, n: u8) -> LoopBuilder<'a> {
        assert!(
            n > 0,
            "invalid iteration count for `repeat.n`, must be non-zero"
        );
        let out_stack = self.stack.clone();
        let body = self.asm.create_block();
        LoopBuilder {
            dfg: self.dfg,
            asm: self.asm,
            in_stack: self.stack,
            out_stack,
            ip: self.ip,
            body,
            style: LoopType::Repeat(n),
        }
    }

    /// Executes the named procedure as a regular function.
    pub fn exec(mut self, id: FunctionIdent) {
        self.execute_call(&id, false);
        self.asm.push(self.ip, MasmOp::Exec(id));
    }

    /// Executes the named procedure as a syscall.
    pub fn syscall(mut self, id: FunctionIdent) {
        self.execute_call(&id, true);
        self.asm.push(self.ip, MasmOp::Syscall(id));
    }

    /// Validate that a call to `id` is possible given the current state of the operand stack,
    /// and if so, update the state of the operand stack to reflect the call.
    fn execute_call(&mut self, id: &FunctionIdent, is_syscall: bool) {
        let import = self
            .dfg
            .get_import(&id)
            .expect("unknown function, are you missing an import?");
        if is_syscall {
            assert_eq!(
                import.signature.cc,
                CallConv::Kernel,
                "cannot call a non-kernel function with the `syscall` instruction"
            );
        } else {
            assert_ne!(
                import.signature.cc,
                CallConv::Kernel,
                "`syscall` cannot be used to call non-kernel functions"
            );
        }
        match import.signature.cc {
            // For now, we're treating all calling conventions the same as SystemV
            CallConv::Fast | CallConv::SystemV | CallConv::Kernel => {
                // Visit the argument list in reverse (so that the top of the stack on entry
                // is the first argument), and allocate elements based on the argument types.
                let mut elements_needed = 0;
                for param in import.signature.params().iter().rev() {
                    let repr = param.repr().expect("invalid parameter type");
                    elements_needed += repr.size();
                }

                // Verify that we have `elements_needed` values on the operand stack
                let elements_available = self.stack.len();
                assert!(elements_needed <= elements_available, "the operand stack does not contain enough values to call {} ({} exepected vs {} available)", id, elements_needed, elements_available);
                self.stack.dropn(elements_needed);

                // Update the operand stack to reflect the results
                for result in import.signature.results().iter().rev() {
                    let repr = result.repr().expect("invalid result type");
                    match repr {
                        TypeRepr::Zst(_) => continue,
                        TypeRepr::Default(ty) => self.stack.push(ty),
                        TypeRepr::Sparse(_, n) => {
                            for _ in 0..n.get() {
                                self.stack.push(Type::Felt);
                            }
                        }
                        TypeRepr::Packed(ty) => {
                            for _ in 0..ty.size_in_felts() {
                                self.stack.push(Type::Felt);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Pops two field elements from the stack, adds them, and places the result on the stack.
    pub fn add(self) {
        self.stack.drop();
        self.asm.push(self.ip, MasmOp::Add);
    }

    /// Pops a field element from the stack, adds the given value to it, and places the result on the stack.
    pub fn add_imm(self, imm: Felt) {
        self.asm.push(self.ip, MasmOp::AddImm(imm));
    }

    /// Pops two field elements from the stack, subtracts the second from the first, and places the result on the stack.
    pub fn sub(self) {
        self.stack.drop();
        self.asm.push(self.ip, MasmOp::Sub);
    }

    /// Pops a field element from the stack, subtracts the given value from it, and places the result on the stack.
    pub fn sub_imm(self, imm: Felt) {
        self.asm.push(self.ip, MasmOp::SubImm(imm));
    }

    /// Pops two field elements from the stack, multiplies them, and places the result on the stack.
    pub fn mul(self) {
        self.stack.drop();
        self.asm.push(self.ip, MasmOp::Mul);
    }

    /// Pops a field element from the stack, multiplies it by the given value, and places the result on the stack.
    pub fn mul_imm(self, imm: Felt) {
        self.asm.push(self.ip, MasmOp::MulImm(imm));
    }

    /// Pops two field elements from the stack, divides the first by the second, and places the result on the stack.
    pub fn div(self) {
        self.stack.drop();
        self.asm.push(self.ip, MasmOp::Div);
    }

    /// Pops a field element from the stack, divides it by the given value, and places the result on the stack.
    pub fn div_imm(self, imm: Felt) {
        self.asm.push(self.ip, MasmOp::DivImm(imm));
    }

    /// Negates the field element on top of the stack
    pub fn neg(self) {
        self.asm.push(self.ip, MasmOp::Neg);
    }

    /// Replaces the field element on top of the stack with it's multiplicative inverse, i.e. `a^-1 mod p`
    pub fn inv(self) {
        self.asm.push(self.ip, MasmOp::Inv);
    }

    /// Increments the field element on top of the stack
    pub fn incr(self) {
        self.asm.push(self.ip, MasmOp::Incr);
    }

    /// Pops an element, `a`, from the top of the stack, and places the result of `2^a` on the stack.
    ///
    /// Traps if `a` is not in the range 0..=63
    pub fn pow2(self) {
        self.asm.push(self.ip, MasmOp::Pow2);
    }

    /// Pops two elements from the stack, `b` and `a` respectively, and places the result of `a^b` on the stack.
    ///
    /// Traps if `b` is not in the range 0..=63
    pub fn exp(self) {
        self.stack.drop();
        self.asm.push(self.ip, MasmOp::Exp);
    }

    /// Pops an element from the stack, `a`, and places the result of `a^b` on the stack, where `b` is
    /// the given immediate value.
    ///
    /// Traps if `b` is not in the range 0..=63
    pub fn exp_imm(self, exponent: u8) {
        self.asm.push(self.ip, MasmOp::ExpImm(exponent));
    }

    /// Pops a value off the stack, and applies logical NOT, and places the result back on the stack.
    ///
    /// Traps if the value is not 0 or 1.
    pub fn not(self) {
        assert_eq!(
            self.stack.peek(),
            Some(Type::I1),
            "expected a boolean operand on the stack"
        );
        self.asm.push(self.ip, MasmOp::Not);
    }

    /// Pops two values off the stack, applies logical AND, and places the result back on the stack.
    ///
    /// Traps if either value is not 0 or 1.
    pub fn and(self) {
        let rhs = self.stack.pop().expect("operand stack is empty");
        let lhs = self.stack.peek().expect("operand stack is empty");
        assert_eq!(lhs, rhs, "expected both operands to be the same type");
        assert_eq!(lhs, Type::I1, "expected boolean operands");
        self.asm.push(self.ip, MasmOp::And);
    }

    /// Pops a value off the stack, applies logical AND with the given immediate, and places the result back on the stack.
    ///
    /// Traps if the value is not 0 or 1.
    pub fn and_imm(self, imm: bool) {
        assert_eq!(
            self.stack.peek(),
            Some(Type::I1),
            "expected a boolean operand on the stack"
        );
        self.asm.push(self.ip, MasmOp::AndImm(imm));
    }

    /// Pops two values off the stack, applies logical OR, and places the result back on the stack.
    ///
    /// Traps if either value is not 0 or 1.
    pub fn or(self) {
        let rhs = self.stack.pop().expect("operand stack is empty");
        let lhs = self.stack.peek().expect("operand stack is empty");
        assert_eq!(lhs, rhs, "expected both operands to be the same type");
        assert_eq!(lhs, Type::I1, "expected boolean operands");
        self.asm.push(self.ip, MasmOp::Or);
    }

    /// Pops a value off the stack, applies logical OR with the given immediate, and places the result back on the stack.
    ///
    /// Traps if the value is not 0 or 1.
    pub fn or_imm(self, imm: bool) {
        assert_eq!(
            self.stack.peek(),
            Some(Type::I1),
            "expected a boolean operand on the stack"
        );
        self.asm.push(self.ip, MasmOp::OrImm(imm));
    }

    /// Pops two values off the stack, applies logical XOR, and places the result back on the stack.
    ///
    /// Traps if either value is not 0 or 1.
    pub fn xor(self) {
        let rhs = self.stack.pop().expect("operand stack is empty");
        let lhs = self.stack.peek().expect("operand stack is empty");
        assert_eq!(lhs, rhs, "expected both operands to be the same type");
        assert_eq!(lhs, Type::I1, "expected boolean operands");
        self.asm.push(self.ip, MasmOp::Xor);
    }

    /// Pops a value off the stack, applies logical XOR with the given immediate, and places the result back on the stack.
    ///
    /// Traps if the value is not 0 or 1.
    pub fn xor_imm(self, imm: bool) {
        assert_eq!(
            self.stack.peek(),
            Some(Type::I1),
            "expected a boolean operand on the stack"
        );
        self.asm.push(self.ip, MasmOp::XorImm(imm));
    }

    /// Pops two elements off the stack, and pushes 1 on the stack if they are equal, else 0.
    pub fn eq(self) {
        self.stack.dropn(2);
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::Eq);
    }

    /// Pops an element off the stack, and pushes 1 on the stack if that value and the given immediate are equal, else 0.
    pub fn eq_imm(self, imm: Felt) {
        self.stack.drop();
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::EqImm(imm));
    }

    /// Pops two words off the stack, and pushes 1 on the stack if they are equal, else 0.
    pub fn eqw(self) {
        self.stack.dropw();
        self.stack.dropw();
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::Eqw);
    }

    /// Pops two elements off the stack, and pushes 1 on the stack if they are not equal, else 0.
    pub fn neq(self) {
        self.stack.dropn(2);
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::Neq);
    }

    /// Pops an element off the stack, and pushes 1 on the stack if that value and the given immediate are not equal, else 0.
    pub fn neq_imm(self, imm: Felt) {
        self.stack.drop();
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::NeqImm(imm));
    }

    /// Pops two elements off the stack, and pushes 1 on the stack if the first is greater than the second, else 0.
    pub fn gt(self) {
        self.stack.dropn(2);
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::Gt);
    }

    /// Pops an element off the stack, and pushes 1 on the stack if that value is greater than the given immediate, else 0.
    pub fn gt_imm(self, imm: Felt) {
        self.stack.drop();
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::GtImm(imm));
    }

    /// Pops two elements off the stack, and pushes 1 on the stack if the first is greater than or equal to the second, else 0.
    pub fn gte(self) {
        self.stack.dropn(2);
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::Gte);
    }

    /// Pops an element off the stack, and pushes 1 on the stack if that value is greater than or equal to the given immediate, else 0.
    pub fn gte_imm(self, imm: Felt) {
        self.stack.drop();
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::GteImm(imm));
    }

    /// Pops two elements off the stack, and pushes 1 on the stack if the first is less than the second, else 0.
    pub fn lt(self) {
        self.stack.dropn(2);
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::Lt);
    }

    /// Pops an element off the stack, and pushes 1 on the stack if that value is less than the given immediate, else 0.
    pub fn lt_imm(self, imm: Felt) {
        self.stack.drop();
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::LtImm(imm));
    }

    /// Pops two elements off the stack, and pushes 1 on the stack if the first is less than or equal to the second, else 0.
    pub fn lte(self) {
        self.stack.dropn(2);
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::Lte);
    }

    /// Pops an element off the stack, and pushes 1 on the stack if that value is less than or equal to the given immediate, else 0.
    pub fn lte_imm(self, imm: Felt) {
        self.stack.drop();
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::LteImm(imm));
    }

    /// Pops an element off the stack, and pushes 1 on the stack if that value is an odd number, else 0.
    pub fn is_odd(self) {
        self.stack.drop();
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::IsOdd);
    }

    /// Pushes the current value of the cycle counter (clock) on the stack
    pub fn clk(self) {
        self.stack.push(Type::Felt);
        self.asm.push(self.ip, MasmOp::Clk);
    }

    /// Pushes 1 on the stack if the element on top of the stack is less than 2^32, else 0.
    pub fn test_u32(self) {
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::U32Test);
    }

    /// Pushes 1 on the stack if every element of the word on top of the stack is less than 2^32, else 0.
    pub fn testw_u32(self) {
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::U32Testw);
    }

    /// Traps if the element on top of the stack is greater than or equal to 2^32
    pub fn assert_u32(self) {
        self.asm.push(self.ip, MasmOp::U32Assert);
    }

    /// Traps if either of the first two elements on top of the stack are greater than or equal to 2^32
    pub fn assert2_u32(self) {
        self.asm.push(self.ip, MasmOp::U32Assert2);
    }

    /// Traps if any element of the first word on the stack are greater than or equal to 2^32
    pub fn assertw_u32(self) {
        self.asm.push(self.ip, MasmOp::U32Assertw);
    }

    /// Casts the element on top of the stack, `a`, to a valid u32 value, by computing `a mod 2^32`
    pub fn cast_u32(self) {
        self.stack.drop();
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32Cast);
    }

    /// Pops an element, `a`, from the stack, and splits it into two elements, `b` and `c`, each of which are a valid u32 value.
    ///
    /// The value for `b` is given by `a mod 2^32`, and the value for `c` by `a / 2^32`. They are pushed on the stack in
    /// that order, i.e. `c` will be on top of the stack afterwards.
    pub fn split_u32(self) {
        self.stack.drop();
        self.stack.push(Type::U32);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32Split);
    }

    /// Performs unsigned addition of the top two elements on the stack, `b` and `a` respectively, which
    /// are expected to be valid u32 values.
    ///
    /// See the [Overflow] enum for how `overflow` modifies the semantics of this instruction.
    pub fn add_u32(self, overflow: Overflow) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        let op = match overflow {
            Overflow::Unchecked => MasmOp::Add,
            Overflow::Checked => MasmOp::U32CheckedAdd,
            Overflow::Overflowing => MasmOp::U32OverflowingAdd,
            Overflow::Wrapping => MasmOp::U32WrappingAdd,
        };
        self.asm.push(self.ip, op);
    }

    /// Same as above, but `a` is provided by the given immediate.
    pub fn add_imm_u32(self, imm: u32, overflow: Overflow) {
        self.stack.drop();
        self.stack.push(Type::U32);
        let op = match overflow {
            Overflow::Unchecked => MasmOp::AddImm(Felt::new(imm as u64)),
            Overflow::Checked => MasmOp::U32CheckedAddImm(imm),
            Overflow::Overflowing => MasmOp::U32OverflowingAddImm(imm),
            Overflow::Wrapping => MasmOp::U32WrappingAddImm(imm),
        };
        self.asm.push(self.ip, op);
    }

    /// Pops three elements from the stack, `c`, `b`, and `a`, and computes `a + b + c` using the
    /// overflowing semantics of `add_u32`. The first two elements on the stack after this instruction
    /// will be a boolean indicating whether addition overflowed, and the result itself, mod 2^32.
    pub fn add3_overflowing_u32(self) {
        self.stack.dropn(3);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32OverflowingAdd3);
    }

    /// Pops three elements from the stack, `c`, `b`, and `a`, and computes `a + b + c` using the
    /// wrapping semantics of `add_u32`. The result will be on top of the stack afterwards, mod 2^32.
    pub fn add3_wrapping_u32(self) {
        self.stack.dropn(3);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32WrappingAdd3);
    }

    /// Performs unsigned subtraction of the top two elements on the stack, `b` and `a` respectively, which
    /// are expected to be valid u32 values.
    ///
    /// See the [Overflow] enum for how `overflow` modifies the semantics of this instruction.
    pub fn sub_u32(self, overflow: Overflow) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        let op = match overflow {
            Overflow::Unchecked => MasmOp::Sub,
            Overflow::Checked => MasmOp::U32CheckedSub,
            Overflow::Overflowing => MasmOp::U32OverflowingSub,
            Overflow::Wrapping => MasmOp::U32WrappingSub,
        };
        self.asm.push(self.ip, op);
    }

    /// Same as above, but `a` is provided by the given immediate.
    pub fn sub_imm_u32(self, imm: u32, overflow: Overflow) {
        self.stack.drop();
        self.stack.push(Type::U32);
        let op = match overflow {
            Overflow::Unchecked => MasmOp::SubImm(Felt::new(imm as u64)),
            Overflow::Checked => MasmOp::U32CheckedSubImm(imm),
            Overflow::Overflowing => MasmOp::U32OverflowingSubImm(imm),
            Overflow::Wrapping => MasmOp::U32WrappingSubImm(imm),
        };
        self.asm.push(self.ip, op);
    }

    /// Performs unsigned multiplication of the top two elements on the stack, `b` and `a` respectively, which
    /// are expected to be valid u32 values.
    ///
    /// See the [Overflow] enum for how `overflow` modifies the semantics of this instruction.
    pub fn mul_u32(self, overflow: Overflow) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        let op = match overflow {
            Overflow::Unchecked => MasmOp::Mul,
            Overflow::Checked => MasmOp::U32CheckedMul,
            Overflow::Overflowing => MasmOp::U32OverflowingMul,
            Overflow::Wrapping => MasmOp::U32WrappingMul,
        };
        self.asm.push(self.ip, op);
    }

    /// Same as above, but `a` is provided by the given immediate.
    pub fn mul_imm_u32(self, imm: u32, overflow: Overflow) {
        self.stack.drop();
        self.stack.push(Type::U32);
        let op = match overflow {
            Overflow::Unchecked => MasmOp::MulImm(Felt::new(imm as u64)),
            Overflow::Checked => MasmOp::U32CheckedMulImm(imm),
            Overflow::Overflowing => MasmOp::U32OverflowingMulImm(imm),
            Overflow::Wrapping => MasmOp::U32WrappingMulImm(imm),
        };
        self.asm.push(self.ip, op);
    }

    /// Pops three elements from the stack, `b`, `a`, and `c`, and computes `a * b + c`, using overflowing
    /// semantics, i.e. the result is wrapped mod 2^32, and a flag is pushed on the stack if the result
    /// overflowed the u32 range.
    pub fn madd_overflowing_u32(self) {
        self.stack.dropn(3);
        self.stack.push(Type::U32);
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::U32OverflowingMadd);
    }

    /// Pops three elements from the stack, `b`, `a`, and `c`, and computes `a * b + c`, using wrapping
    /// semantics, i.e. the result is wrapped mod 2^32.
    pub fn madd_wrapping_u32(self) {
        self.stack.dropn(3);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32WrappingMadd);
    }

    /// Performs unsigned division of the top two elements on the stack, `b` and `a` respectively, which
    /// are expected to be valid u32 values.
    ///
    /// This operation is checked, meaning that if either operand is >= 2^32, then it will trap.
    ///
    /// Traps if `b` is 0.
    pub fn div_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32CheckedDiv);
    }

    /// Same as above, but `b` is provided by the given immediate
    pub fn div_imm_u32(self, imm: u32) {
        self.stack.drop();
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32CheckedDivImm(imm));
    }

    /// Performs unsigned division of the top two elements on the stack, `b` and `a` respectively, which
    /// are expected to be valid u32 values.
    ///
    /// This operation is unchecked, so if either operand is >= 2^32, the result is undefined.
    ///
    /// Traps if `b` is 0.
    pub fn div_unchecked_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32UncheckedDiv);
    }

    /// Same as above, but `b` is provided by the given immediate
    pub fn div_imm_unchecked_u32(self, imm: u32) {
        self.stack.drop();
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32UncheckedDivImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and computes `a mod b`.
    ///
    /// This operation is checked, meaning that if either operand is >= 2^32, then it will trap.
    ///
    /// Traps if `b` is 0.
    pub fn mod_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32CheckedMod);
    }

    /// Same as above, but `b` is provided by the given immediate
    pub fn mod_imm_u32(self, imm: u32) {
        self.stack.drop();
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32CheckedModImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and computes `a mod b`.
    ///
    /// This operation is unchecked, so if either operand is >= 2^32, the result is undefined.
    ///
    /// Traps if `b` is 0.
    pub fn mod_unchecked_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32UncheckedMod);
    }

    /// Same as above, but `b` is provided by the given immediate
    pub fn mod_imm_unchecked_u32(self, imm: u32) {
        self.stack.drop();
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32UncheckedModImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and computes `a / b`, and `a mod b`,
    /// pushing the results of each on the stack in that order.
    ///
    /// This operation is checked, meaning that if either operand is >= 2^32, then it will trap.
    ///
    /// Traps if `b` is 0.
    pub fn divmod_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32CheckedDivMod);
    }

    /// Same as above, but `b` is provided by the given immediate
    pub fn divmod_imm_u32(self, imm: u32) {
        self.stack.drop();
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32CheckedDivModImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and computes `a / b`, and `a mod b`,
    /// pushing the results of each on the stack in that order.
    ///
    /// This operation is unchecked, so if either operand is >= 2^32, the results are undefined.
    ///
    /// Traps if `b` is 0.
    pub fn divmod_unchecked_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32UncheckedDivMod);
    }

    /// Same as above, but `b` is provided by the given immediate
    pub fn divmod_imm_unchecked_u32(self, imm: u32) {
        self.stack.drop();
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32UncheckedDivModImm(imm));
    }

    /// Pops two elements off the stack, and computes the bitwise AND of those values, placing the result on the stack.
    ///
    /// Traps if either element is not a valid u32 value.
    pub fn band_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32And);
    }

    /// Pops two elements off the stack, and computes the bitwise OR of those values, placing the result on the stack.
    ///
    /// Traps if either element is not a valid u32 value.
    pub fn bor_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32Or);
    }

    /// Pops two elements off the stack, and computes the bitwise XOR of those values, placing the result on the stack.
    ///
    /// Traps if either element is not a valid u32 value.
    pub fn bxor_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32Xor);
    }

    /// Pops an element off the stack, and computes the bitwise NOT of that value, placing the result on the stack.
    ///
    /// Traps if the element is not a valid u32 value.
    pub fn bnot_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32Not);
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and shifts `a` left by `b` bits. More precisely,
    /// the result is computed as `(a * 2^b) mod 2^32`.
    ///
    /// Traps if `a` is not a valid u32, or `b` > 31.
    pub fn shl_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32CheckedShl);
    }

    /// Same as `shl_u32`, but `b` is provided by immediate.
    pub fn shl_imm_u32(self, imm: u32) {
        self.stack.drop();
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32CheckedShlImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and shifts `a` left by `b` bits. More precisely,
    /// the result is computed as `(a * 2^b) mod 2^32`.
    ///
    /// The result is undefined if `a` is not a valid u32, or `b` is > 31.
    pub fn shl_unchecked_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32UncheckedShl);
    }

    /// Same as `shl_unchecked_u32`, but `b` is provided by immediate.
    pub fn shl_unchecked_imm_u32(self, imm: u32) {
        self.stack.drop();
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32UncheckedShlImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and shifts `a` right by `b` bits. More precisely,
    /// the result is computed as `a / 2^b`.
    ///
    /// Traps if `a` is not a valid u32, or `b` > 31.
    pub fn shr_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32CheckedShr);
    }

    /// Same as `shr_u32`, but `b` is provided by immediate.
    pub fn shr_imm_u32(self, imm: u32) {
        self.stack.drop();
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32CheckedShrImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and shifts `a` right by `b` bits. More precisely,
    /// the result is computed as `a / 2^b`.
    ///
    /// The result is undefined if `a` is not a valid u32, or `b` is > 31.
    pub fn shr_unchecked_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32UncheckedShr);
    }

    /// Same as `shr_unchecked_u32`, but `b` is provided by immediate.
    pub fn shr_unchecked_imm_u32(self, imm: u32) {
        self.stack.drop();
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32UncheckedShrImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and rotates the binary representation of `a`
    /// left by `b` bits.
    ///
    /// Traps if `a` is not a valid u32, or `b` > 31
    pub fn rotl_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32CheckedRotl);
    }

    /// Same as `rotl_u32`, but `b` is provided by immediate.
    pub fn rotl_imm_u32(self, imm: u32) {
        self.stack.drop();
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32CheckedRotlImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and rotates the binary representation of `a`
    /// left by `b` bits.
    ///
    /// The result is undefined if `a` is not a valid u32, or `b` is > 31.
    pub fn rotl_unchecked_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32UncheckedRotl);
    }

    /// Same as `rotl_unchecked_u32`, but `b` is provided by immediate.
    pub fn rotl_unchecked_imm_u32(self, imm: u32) {
        self.stack.drop();
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32UncheckedRotlImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and rotates the binary representation of `a`
    /// right by `b` bits.
    ///
    /// Traps if `a` is not a valid u32, or `b` > 31
    pub fn rotr_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32CheckedRotr);
    }

    /// Same as `rotr_u32`, but `b` is provided by immediate.
    pub fn rotr_imm_u32(self, imm: u32) {
        self.stack.drop();
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32CheckedRotrImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and rotates the binary representation of `a`
    /// right by `b` bits.
    ///
    /// The result is undefined if `a` is not a valid u32, or `b` is > 31.
    pub fn rotr_unchecked_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32UncheckedRotr);
    }

    /// Same as `rotr_unchecked_u32`, but `b` is provided by immediate.
    pub fn rotr_unchecked_imm_u32(self, imm: u32) {
        self.stack.drop();
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32UncheckedRotrImm(imm));
    }

    /// Pops an element off the stack, and computes the number of set bits in its binary representation, i.e.
    /// its hamming weight, and places the result on the stack.
    ///
    /// Traps if the input value is not a valid u32.
    pub fn popcnt_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32CheckedPopcnt);
    }

    /// Pops an element off the stack, and computes the number of set bits in its binary representation, i.e.
    /// its hamming weight, and places the result on the stack.
    ///
    /// The result is undefined if the input value is not a valid u32.
    pub fn popcnt_unchecked_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32UncheckedPopcnt);
    }

    /// This is the same as `eq`, but also asserts that both operands are valid u32 values.
    pub fn eq_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::U32Eq);
    }

    /// This is the same as `eq_imm`, but also asserts that both operands are valid u32 values.
    pub fn eq_imm_u32(self, imm: u32) {
        self.stack.drop();
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::U32EqImm(imm));
    }

    /// This is the same as `neq`, but also asserts that both operands are valid u32 values.
    pub fn neq_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::U32Neq);
    }

    /// This is the same as `neq_imm`, but also asserts that both operands are valid u32 values.
    pub fn neq_imm_u32(self, imm: u32) {
        self.stack.drop();
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::U32NeqImm(imm));
    }

    /// This is the same as `lt`, but also asserts that both operands are valid u32 values.
    pub fn lt_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::U32CheckedLt);
    }

    /// This is the same as `lt`, but the result is undefined if either operand is not a valid u32 value.
    pub fn lt_unchecked_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::U32UncheckedLt);
    }

    /// This is the same as `lte`, but also asserts that both operands are valid u32 values.
    pub fn lte_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::U32CheckedLte);
    }

    /// This is the same as `lte`, but the result is undefined if either operand is not a valid u32 value.
    pub fn lte_unchecked_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::U32UncheckedLte);
    }

    /// This is the same as `gt`, but also asserts that both operands are valid u32 values.
    pub fn gt_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::U32CheckedGt);
    }

    /// This is the same as `gt`, but the result is undefined if either operand is not a valid u32 value.
    pub fn gt_unchecked_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::U32UncheckedGt);
    }

    /// This is the same as `gte`, but also asserts that both operands are valid u32 values.
    pub fn gte_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::U32CheckedGte);
    }

    /// This is the same as `gte`, but the result is undefined if either operand is not a valid u32 value.
    pub fn gte_unchecked_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::I1);
        self.asm.push(self.ip, MasmOp::U32UncheckedGte);
    }

    /// This is the same as `min`, but also asserts that both operands are valid u32 values.
    pub fn min_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32CheckedMin);
    }

    /// This is the same as `min`, but the result is undefined if either operand is not a valid u32 value.
    pub fn min_unchecked_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32UncheckedMin);
    }

    /// This is the same as `max`, but also asserts that both operands are valid u32 values.
    pub fn max_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32CheckedMax);
    }

    /// This is the same as `max`, but the result is undefined if either operand is not a valid u32 value.
    pub fn max_unchecked_u32(self) {
        self.stack.dropn(2);
        self.stack.push(Type::U32);
        self.asm.push(self.ip, MasmOp::U32UncheckedMax);
    }
}

#[doc(hidden)]
enum IfBranch {
    Then,
    Else,
}

/// This builder is used to construct an `if.true` instruction, while maintaining
/// the invariant that the operand stack has a uniform state upon exit from either
/// branch of the `if.true`, i.e. the number of elements, and their types, must
/// match.
///
/// We do this by snapshotting the state of the operand stack on entry, using it
/// when visiting each branch as the initial stack state, and then validating that
/// when both branches have been constructed, that the stack state on exit is the
/// same. The first branch to be completed defines the expected state of the stack
/// for the remaining branch.
///
/// # Example
///
/// The general usage here looks like this, where `masm_builder` is an instance of
/// [MasmBuilder]:
///
/// ```rust,ignore
/// // If the current top of the stack is > 0, decrement the next stack element, which
/// is a counter, and then call a function, otherwise, pop the counter, push 0, and proceed.
/// masm_builder.ins().gt_imm(Felt::ZERO);
/// let if_builder = masm_builder.ins().if_true();
///
/// // Build the then branch
/// let then_b = if_builder.build_then();
/// then_b.ins().sub_imm(Felt::new(1 as u64));
/// then_b.ins().exec("do_some_stuff_and_return_a_boolean".parse().unwrap());
/// then_b.end();
///
/// // Build the else branch
/// let else_b = if_builder.build_else();
/// else_b.ins().pop();
/// else_b.ins().push(Felt::ZERO);
/// else_b.end();
///
/// // Finalize
/// if_builder.build();
/// ```
pub struct IfTrueBuilder<'a> {
    dfg: &'a mut DataFlowGraph,
    asm: &'a mut InlineAsm,
    /// This reference is to the operand stack in the parent [MasmOpBuilder],
    /// which represents the operand stack on entry to the `if.true`. Upon
    /// finalizatio of the `if.true`, we use update this operand stack to
    /// reflect the state upon exit from the `if.true`.
    ///
    /// In effect, when the call to `if_true` returns, the operand stack in the
    /// parent builder will look as if the `if.true` instruction has finished executing.
    in_stack: &'a mut OperandStack<Type>,
    /// This is set when the first branch is finished being constructed, and
    /// will be used as the expected state of the operand stack when we finish
    /// constructing the second branch and validate the `if.true`.
    out_stack: OperandStack<Type>,
    /// This is the block to which the `if.true` will be appended
    ip: MasmBlockId,
    /// The block id for the then branch, unset until it has been finalized
    then_blk: Option<MasmBlockId>,
    /// The block id for the else branch, unset until it has been finalized
    else_blk: Option<MasmBlockId>,
}
impl<'f> IfTrueBuilder<'f> {
    /// Start constructing the then block for this `if.true` instruction
    ///
    /// NOTE: This function will panic if the then block has already been built
    pub fn build_then<'a: 'f, 'b: 'f + 'a>(&'b mut self) -> IfTrueBlockBuilder<'a> {
        assert!(
            self.then_blk.is_none(),
            "cannot build the 'then' branch twice"
        );
        let then_blk = self.asm.create_block();
        let stack = self.in_stack.clone();
        IfTrueBlockBuilder {
            builder: self,
            stack,
            block: then_blk,
            branch: IfBranch::Then,
        }
    }

    /// Start constructing the else block for this `if.true` instruction
    ///
    /// NOTE: This function will panic if the else block has already been built
    pub fn build_else<'a: 'f, 'b: 'f + 'a>(&'b mut self) -> IfTrueBlockBuilder<'a> {
        assert!(
            self.else_blk.is_none(),
            "cannot build the 'else' branch twice"
        );
        let else_blk = self.asm.create_block();
        let stack = self.in_stack.clone();
        IfTrueBlockBuilder {
            builder: self,
            stack,
            block: else_blk,
            branch: IfBranch::Else,
        }
    }

    /// Finalize this `if.true` instruction, inserting it into the block this
    /// builder was constructed from.
    pub fn build(mut self) {
        let then_blk = self.then_blk.expect("missing 'then' block");
        let else_blk = self.else_blk.expect("missing 'else' block");
        self.asm.push(self.ip, MasmOp::If(then_blk, else_blk));
        // Update the operand stack to represent the state after execution of the `if.true`
        let in_stack = self.in_stack.stack_mut();
        in_stack.clear();
        in_stack.append(self.out_stack.stack_mut());
    }
}

/// Used to construct a single branch of an `if.true` instruction
///
/// See [IfTrueBuilder] for usage.
pub struct IfTrueBlockBuilder<'a> {
    builder: &'a mut IfTrueBuilder<'a>,
    // The state of the operand stack in this block
    stack: OperandStack<Type>,
    // The block we're building
    block: MasmBlockId,
    branch: IfBranch,
}
impl<'f> IfTrueBlockBuilder<'f> {
    /// Construct a MASM instruction in this block
    pub fn ins<'a, 'b: 'a>(&'b mut self) -> MasmOpBuilder<'a> {
        MasmOpBuilder {
            dfg: self.builder.dfg,
            asm: self.builder.asm,
            stack: &mut self.stack,
            ip: self.block,
        }
    }

    /// Finalize this block, and release the builder
    pub fn end(self) {}
}
impl<'a> Drop for IfTrueBlockBuilder<'a> {
    fn drop(&mut self) {
        match self.branch {
            IfBranch::Then => {
                self.builder.then_blk = Some(self.block);
            }
            IfBranch::Else => {
                self.builder.else_blk = Some(self.block);
            }
        }

        // If the if.true instruction is complete, validate that the operand stack in
        // both branches is identical
        //
        // Otherwise, save the state of the stack here to be compared to the other
        // branch when it is constructed
        let is_complete = self.builder.then_blk.is_some() && self.builder.else_blk.is_some();
        if is_complete {
            assert_eq!(self.stack.stack(), self.builder.out_stack.stack(), "expected the operand stack to be in the same abstract state upon exit from either branch of this if.true instruction");
        } else {
            core::mem::swap(&mut self.builder.out_stack, &mut self.stack);
        }
    }
}

#[doc(hidden)]
enum LoopType {
    While,
    Repeat(u8),
}

/// This builder is used to construct both `while.true` and `repeat.n` loops, enforcing
/// their individual invariants with regard to the operand stack.
///
/// In particular, this builder ensures that the body of a `while.true` loop is valid,
/// i.e. that when returning to the top of the loop to evaluate the conditional, that
/// there is a boolean value on top of the stack for that purpose. Similarly, it validates
/// that after the conditional has been evaluated, that the abstract state of the operand
/// stack is the same across iterations, and regardless of whether the loop is taken. The
/// abstract state in question is the number, and type, of the operands on the stack.
///
/// # Example
///
/// The general usage here looks like this, where `masm_builder` is an instance of
/// [MasmBuilder]:
///
/// ```rust,ignore
/// // For our example here, we're generating inline assembly that performs
/// // the equivalent of `for (i = 0; i < len; i++) sum += array[i / 4][i % 4]`,
/// // where `array` is a pointer to words, and we're attempting to sum `len`
/// // field elements, across how ever many words that spans.
/// //
/// // We assume the operand stack is as follows (top to bottom):
/// //
/// //    [len, sum, array]
/// //
/// // First, build out the loop header
/// masm_builder.ins().push(Felt::ZERO); // [i, len, sum, array]
/// masm_builder.ins().dup(0);  // [i, i, len, sum, array]
/// masm_builder.ins().dup(2);  // [len, i, i, len, sum, array]
/// masm_builder.ins().lt();    // [i < len, i, len, sum, array]
///
/// // Now, build the loop body
/// //
/// // The state of the stack on entry is: [i, len, sum, array]
/// let mut lb = masm_builder.ins().while_true();
///
/// // Calculate `i / 4`
/// lb.ins().dup(0);     // [i, i, len, sum, array]
/// lb.ins().div_imm(4); // [word_offset, i, len, sum, array]
///
/// // Calculate the address for `array[i / 4]`
/// lb.ins().dup(4);     // [array, word_offset, ..]
/// lb.ins().add_u32(Overflow::Checked); // [array + word_offset, i, ..]
///
/// // Calculate the `i % 4`
/// lb.ins().dup(1);     // [i, array + word_offset, ..]
/// lb.ins().mod_imm_u32(4); // [element_offset, array + word_offset, ..]
///
/// // Precalculate what elements of the word to drop, so that
/// // we are only left with the specific element we wanted
/// lb.ins().dup(0);     // [element_offset, element_offset, ..]
/// lb.ins().lt_imm(Felt::new(3)); // [element_offset < 3, element_offset, ..]
/// lb.ins().dup(1);     // [element_offset, element_offset < 3, ..]
/// lb.ins().lt_imm(Felt::new(2)); // [element_offset < 2, element_offset < 3, ..]
/// lb.ins().dup(2);     // [element_offset, element_offset < 2, ..]
/// lb.ins().lt_imm(Felt::new(1)); // [element_offset < 1, element_offset < 2, ..]
///
/// // Load the word
/// lb.ins().dup(4);      // [array + word_offset, element_offset < 1]
/// lb.ins().loadw(); // [word[0], word[1], word[2], word[3], element_offset < 1]
///
/// // Select the element, `E`, that we want by conditionally dropping
/// // elements on the operand stack with a carefully chosen sequence
/// // of conditionals: E < N forall N in 0..=3
/// lb.ins().movup(4);   // [element_offset < 1, word[0], ..]
/// lb.ins().cdrop();    // [word[0 or 1], word[2], word[3], element_offset < 2]
/// lb.ins().movup(3);   // [element_offset < 2, word[0 or 1], ..]
/// lb.ins().cdrop();    // [word[0 or 1 or 2], word[3], element_offset < 3]
/// lb.ins().movup(2);   // [element_offset < 3, ..]
/// lb.ins().cdrop();    // [array[i], i, len, sum, array]
/// lb.ins().movup(3);   // [sum, array[i], i, len, array]
/// lb.ins().add();      // [sum + array[i], i, len, array]
/// lb.ins().movdn(2);   // [i, len, sum + array[i], array]
///
/// // We've reached the end of the loop, but we need a copy of the
/// // loop header here in order to use the expression `i < len` as
/// // the condition for the loop
/// lb.ins().dup(0);     // [i, i, len, ..]
/// lb.ins().dup(2);     // [len, i, i, len, ..]
/// lb.ins().lt();       // [i < len, i, len, sum, array]
///
/// // Finalize, it is at this point that validation will occur
/// lb.build();
/// ```
pub struct LoopBuilder<'a> {
    dfg: &'a mut DataFlowGraph,
    asm: &'a mut InlineAsm,
    /// This reference is to the operand stack in the parent [MasmOpBuilder],
    /// which represents the operand stack on entry to the loop. Upon finalization
    /// of the loop, we use update this operand stack to reflect the state upon
    /// exit from the loop.
    ///
    /// In effect, when the call to `while_true` or `repeat` returns, the operand
    /// stack in the parent builder will look as if the loop instruction has finished
    /// executing.
    in_stack: &'a mut OperandStack<Type>,
    /// This is the operand stack state within the loop.
    ///
    /// Upon finalization of the loop instruction, this state is used to validate
    /// the effect of the loop body on the operand stack. For `repeat`, which is
    /// unconditionally entered, no special validation is performed. However, for
    /// `while.true`, we must validate two things:
    ///
    /// 1. That the top of the stack holds a boolean value
    /// 2. That after popping the boolean, the output state of the operand stack
    /// matches the input state in number and type of elements. This is required,
    /// as otherwise program behavior is undefined based on whether the loop is
    /// entered or not.
    out_stack: OperandStack<Type>,
    /// The block to which the loop instruction will be appended
    ip: MasmBlockId,
    /// The top-level block for the loop
    body: MasmBlockId,
    /// The type of loop we're building
    style: LoopType,
}
impl<'f> LoopBuilder<'f> {
    /// Get a builder for a single MASM instruction
    pub fn ins<'a, 'b: 'a>(&'b mut self) -> MasmOpBuilder<'a> {
        MasmOpBuilder {
            dfg: self.dfg,
            asm: self.asm,
            stack: &mut self.out_stack,
            ip: self.body,
        }
    }

    /// Finalize construction of this loop, performing any final validation.
    pub fn build(mut self) {
        match self.style {
            LoopType::While => {
                // First, validate that the top of the stack holds a boolean
                let cond = self.out_stack.pop().expect("operand stack is empty");
                assert_eq!(cond, Type::I1, "expected there to be a boolean on top of the stack at the end of the while.true body");
                // Next, validate that the contents of the operand stack match
                // the input stack, in order to ensure that the operand stack
                // is consistent whether the loop is taken or not
                assert_eq!(self.in_stack.stack(), self.out_stack.stack(), "expected the operand stack to be in the same abstract state whether the while.true loop is taken or skipped");
                self.asm.push(self.ip, MasmOp::While(self.body));
            }
            LoopType::Repeat(n) => {
                // No special validation is needed, we're done
                self.asm.push(self.ip, MasmOp::Repeat(n, self.body));
            }
        }

        // Update the operand stack to represent the state after execution of this loop
        let in_stack = self.in_stack.stack_mut();
        in_stack.clear();
        in_stack.append(self.out_stack.stack_mut());
    }
}
