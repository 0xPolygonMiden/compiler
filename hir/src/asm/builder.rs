use crate::{
    DataFlowGraph, Felt, FunctionIdent, Inst, InstBuilder, Instruction, Overflow, SourceSpan, Type,
    Value,
};

use super::*;

/// Used to construct an [InlineAssembly] instruction, while checking various safety invariants.
pub struct MasmBuilder<B> {
    builder: B,
    span: SourceSpan,
    asm: InlineAsm,
    ty: Type,
    current_block: MasmBlockId,
    stack: OperandStack<Type>,
}
impl<'f, B: InstBuilder<'f>> MasmBuilder<B> {
    /// Construct a new inline assembly builder in the function represented by `dfg`, to be inserted at `ip`.
    ///
    /// The `args` list represents the arguments which will be visible on the operand stack in this inline assembly block.
    ///
    /// The type given by `ty` represents the expected result type for this inline assembly block. If the inline assembly
    /// will not produce a result, use `Type::Unit`. It is expected that the value(s) remaining on the operand stack upon
    /// exit from the inline assembly block, are a match for `ty`. For example, if `Type::Unit` is given, no values should
    /// remain on the operand stack; if `Type::Felt` is given, then a single value should be on the operand stack; if
    /// `Type::Array[Type::Felt; 2]` is given, then two values should be on the operand stack, and so on.
    ///
    /// NOTE: Not all types are permitted as inline assembly results. The type must be "loadable", i.e. no larger than a word.
    ///
    /// Any attempt to modify the operand stack beyond what is made visible via arguments, or introduced within the
    /// inline assembly block, will cause an assertion to fail.
    pub fn new(mut builder: B, args: &[Value], ty: Type, span: SourceSpan) -> Self {
        assert!(
            ty.is_loadable(),
            "invalid inline assembly block type: type must be loadable, but got {}",
            &ty
        );
        // Construct the initial operand stack with the given arguments
        let mut stack = OperandStack::<Type>::default();
        {
            let dfg = builder.data_flow_graph();
            for arg in args.iter().rev().copied() {
                stack.push(dfg.value_type(arg).clone());
            }
        }

        // Construct an empty inline assembly block with the given arguments
        let mut asm = InlineAsm::new();
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
            ty,
            current_block,
            stack,
        }
    }

    #[inline]
    pub fn create_block(&mut self) -> MasmBlockId {
        self.asm.create_block()
    }

    #[inline(always)]
    pub fn switch_to_block(&mut self, block: MasmBlockId) {
        self.current_block = block;
    }

    pub fn ins<'a, 'b: 'a>(&'b mut self) -> MasmOpBuilder<'a> {
        MasmOpBuilder {
            asm: &mut self.asm,
            stack: &mut self.stack,
            ip: self.current_block,
        }
    }

    pub fn build(self) -> (Inst, &'f mut DataFlowGraph) {
        let ty = self.ty;
        match &ty {
            Type::Unit => assert!(self.stack.is_empty(), "invalid inline assembly: expected operand stack to be empty upon exit, found: {:?}", self.stack.display()),
            ty => {
                let len = ty.size_in_felts();
                assert_eq!(len, self.stack.len(), "invalid inline assembly: expected operand stack to have {} elements upon exit, found: {:?}", len, self.stack.display());
            }
        }

        let span = self.span;
        let data = Instruction::InlineAsm(self.asm);
        self.builder.build(data, ty, span)
    }
}

/// Used to construct a single MASM opcode
pub struct MasmOpBuilder<'a> {
    asm: &'a mut InlineAsm,
    stack: &'a mut OperandStack<Type>,
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
        self.stack
            .pushw([Type::Felt, Type::Felt, Type::Felt, Type::Felt]);
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
        self.asm.push(self.ip, MasmOp::MemLoad);
    }

    /// Loads the first element of the word at the given address to the top of the stack.
    pub fn load_imm(self, addr: u32) {
        self.stack.drop();
        self.asm.push(self.ip, MasmOp::MemLoadImm(addr));
    }

    /// Pops an element containing a memory address + element offset from the top of the stack,
    /// and loads the element of the word at that address + offset to the top of the stack.
    ///
    /// NOTE: This is an experimental instruction which is not implemented in Miden VM yet.
    pub fn load_offset(self) {
        self.stack.drop();
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
        self.stack.drop();
        self.asm
            .push(self.ip, MasmOp::MemLoadOffsetImm(addr, offset));
    }

    /// Pops an element containing a memory address from the top of the stack,
    /// and loads the word at that address to the top of the stack.
    pub fn loadw(self) {
        self.stack.drop();
        self.asm.push(self.ip, MasmOp::MemLoadw);
    }

    /// Loads the word at the given address to the top of the stack.
    pub fn loadw_imm(self, addr: u32) {
        self.stack.drop();
        self.asm.push(self.ip, MasmOp::MemLoadwImm(addr));
    }

    /// Pops two elements, the first containing a memory address from the top of the stack,
    /// the second the value to be stored as the first element of the word at that address.
    pub fn store(self) {
        self.stack.drop();
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
        self.stack.drop();
        self.asm.push(self.ip, MasmOp::MemStorew);
    }

    /// Pops a word from the stack and stores it as the word at the given address.
    pub fn storew_imm(self, addr: u32) {
        self.stack.drop();
        self.asm.push(self.ip, MasmOp::MemStorewImm(addr));
    }

    /// Pops a boolean value from the stack, and executes the first block if it is true,
    /// otherwise the second block.
    pub fn if_true(self, then_blk: MasmBlockId, else_blk: MasmBlockId) {
        self.stack.drop();
        self.asm.push(self.ip, MasmOp::If(then_blk, else_blk))
    }

    /// Pops a boolean value from the stack, and executes the given block if it is true,
    /// otherwise it is skipped. The given block will continue to execute for as long as
    /// the top value on the stack at the end of the block is true.
    pub fn while_true(self, body: MasmBlockId) {
        self.stack.drop();
        self.asm.push(self.ip, MasmOp::While(body));
    }

    /// Repeatedly executes `body`, `n` times.
    pub fn repeat(self, n: u8, body: MasmBlockId) {
        self.asm.push(self.ip, MasmOp::Repeat(n, body));
    }

    /// Executes the named procedure as a regular function.
    pub fn exec(self, id: FunctionIdent) {
        self.asm.push(self.ip, MasmOp::Exec(id));
    }

    /// Executes the named procedure as a syscall.
    pub fn syscall(self, id: FunctionIdent) {
        self.asm.push(self.ip, MasmOp::Syscall(id));
    }

    /// Pops two field elements from the stack, adds them, and places the result on the stack.
    pub fn add(self) {
        self.asm.push(self.ip, MasmOp::Add);
    }

    /// Pops a field element from the stack, adds the given value to it, and places the result on the stack.
    pub fn add_imm(self, imm: Felt) {
        self.asm.push(self.ip, MasmOp::AddImm(imm));
    }

    /// Pops two field elements from the stack, subtracts the second from the first, and places the result on the stack.
    pub fn sub(self) {
        self.asm.push(self.ip, MasmOp::Sub);
    }

    /// Pops a field element from the stack, subtracts the given value from it, and places the result on the stack.
    pub fn sub_imm(self, imm: Felt) {
        self.asm.push(self.ip, MasmOp::SubImm(imm));
    }

    /// Pops two field elements from the stack, multiplies them, and places the result on the stack.
    pub fn mul(self) {
        self.asm.push(self.ip, MasmOp::Mul);
    }

    /// Pops a field element from the stack, multiplies it by the given value, and places the result on the stack.
    pub fn mul_imm(self, imm: Felt) {
        self.asm.push(self.ip, MasmOp::MulImm(imm));
    }

    /// Pops two field elements from the stack, divides the first by the second, and places the result on the stack.
    pub fn div(self) {
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
        self.asm.push(self.ip, MasmOp::Not);
    }

    /// Pops two values off the stack, applies logical AND, and places the result back on the stack.
    ///
    /// Traps if either value is not 0 or 1.
    pub fn and(self) {
        self.asm.push(self.ip, MasmOp::And);
    }

    /// Pops a value off the stack, applies logical AND with the given immediate, and places the result back on the stack.
    ///
    /// Traps if the value is not 0 or 1.
    pub fn and_imm(self, imm: bool) {
        self.asm.push(self.ip, MasmOp::AndImm(imm));
    }

    /// Pops two values off the stack, applies logical OR, and places the result back on the stack.
    ///
    /// Traps if either value is not 0 or 1.
    pub fn or(self) {
        self.asm.push(self.ip, MasmOp::Or);
    }

    /// Pops a value off the stack, applies logical OR with the given immediate, and places the result back on the stack.
    ///
    /// Traps if the value is not 0 or 1.
    pub fn or_imm(self, imm: bool) {
        self.asm.push(self.ip, MasmOp::OrImm(imm));
    }

    /// Pops two values off the stack, applies logical XOR, and places the result back on the stack.
    ///
    /// Traps if either value is not 0 or 1.
    pub fn xor(self) {
        self.asm.push(self.ip, MasmOp::Xor);
    }

    /// Pops a value off the stack, applies logical XOR with the given immediate, and places the result back on the stack.
    ///
    /// Traps if the value is not 0 or 1.
    pub fn xor_imm(self, imm: bool) {
        self.asm.push(self.ip, MasmOp::XorImm(imm));
    }

    /// Pops two elements off the stack, and pushes 1 on the stack if they are equal, else 0.
    pub fn eq(self) {
        self.asm.push(self.ip, MasmOp::Eq);
    }

    /// Pops an element off the stack, and pushes 1 on the stack if that value and the given immediate are equal, else 0.
    pub fn eq_imm(self, imm: Felt) {
        self.asm.push(self.ip, MasmOp::EqImm(imm));
    }

    /// Pops two words off the stack, and pushes 1 on the stack if they are equal, else 0.
    pub fn eqw(self) {
        self.asm.push(self.ip, MasmOp::Eqw);
    }

    /// Pops two elements off the stack, and pushes 1 on the stack if they are not equal, else 0.
    pub fn neq(self) {
        self.asm.push(self.ip, MasmOp::Neq);
    }

    /// Pops an element off the stack, and pushes 1 on the stack if that value and the given immediate are not equal, else 0.
    pub fn neq_imm(self, imm: Felt) {
        self.asm.push(self.ip, MasmOp::NeqImm(imm));
    }

    /// Pops two elements off the stack, and pushes 1 on the stack if the first is greater than the second, else 0.
    pub fn gt(self) {
        self.asm.push(self.ip, MasmOp::Gt);
    }

    /// Pops an element off the stack, and pushes 1 on the stack if that value is greater than the given immediate, else 0.
    pub fn gt_imm(self, imm: Felt) {
        self.asm.push(self.ip, MasmOp::GtImm(imm));
    }

    /// Pops two elements off the stack, and pushes 1 on the stack if the first is greater than or equal to the second, else 0.
    pub fn gte(self) {
        self.asm.push(self.ip, MasmOp::Gte);
    }

    /// Pops an element off the stack, and pushes 1 on the stack if that value is greater than or equal to the given immediate, else 0.
    pub fn gte_imm(self, imm: Felt) {
        self.asm.push(self.ip, MasmOp::GteImm(imm));
    }

    /// Pops two elements off the stack, and pushes 1 on the stack if the first is less than the second, else 0.
    pub fn lt(self) {
        self.asm.push(self.ip, MasmOp::Lt);
    }

    /// Pops an element off the stack, and pushes 1 on the stack if that value is less than the given immediate, else 0.
    pub fn lt_imm(self, imm: Felt) {
        self.asm.push(self.ip, MasmOp::LtImm(imm));
    }

    /// Pops two elements off the stack, and pushes 1 on the stack if the first is less than or equal to the second, else 0.
    pub fn lte(self) {
        self.asm.push(self.ip, MasmOp::Lte);
    }

    /// Pops an element off the stack, and pushes 1 on the stack if that value is less than or equal to the given immediate, else 0.
    pub fn lte_imm(self, imm: Felt) {
        self.asm.push(self.ip, MasmOp::LteImm(imm));
    }

    /// Pops an element off the stack, and pushes 1 on the stack if that value is an odd number, else 0.
    pub fn is_odd(self) {
        self.asm.push(self.ip, MasmOp::IsOdd);
    }

    /// Pushes the current value of the cycle counter (clock) on the stack
    pub fn clk(self) {
        self.asm.push(self.ip, MasmOp::Clk);
    }

    /// Pushes 1 on the stack if the element on top of the stack is less than 2^32, else 0.
    pub fn test_u32(self) {
        self.asm.push(self.ip, MasmOp::U32Test);
    }

    /// Pushes 1 on the stack if every element of the word on top of the stack is less than 2^32, else 0.
    pub fn testw_u32(self) {
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
        self.asm.push(self.ip, MasmOp::U32Cast);
    }

    /// Pops an element, `a`, from the stack, and splits it into two elements, `b` and `c`, each of which are a valid u32 value.
    ///
    /// The value for `b` is given by `a mod 2^32`, and the value for `c` by `a / 2^32`. They are pushed on the stack in
    /// that order, i.e. `c` will be on top of the stack afterwards.
    pub fn split_u32(self) {
        self.asm.push(self.ip, MasmOp::U32Split);
    }

    /// Performs unsigned addition of the top two elements on the stack, `b` and `a` respectively, which
    /// are expected to be valid u32 values.
    ///
    /// The specific behavior of the addition depends on the given `overflow` flags:
    ///
    /// * `Overflow::Unchecked` - the addition is performed using the `add` op for field elements, which may
    /// produce a value that is outside of the u32 range, it is the callers responsibility to ensure that the
    /// resulting value is in range.
    /// * `Overflow::Checked` - the operation will trap if either operand, or the result, is not a valid u32
    /// * `Overflow::Wrapping` - computes the result as `(a + b) mod 2^32`, behavior is undefined if either operand
    /// is not a valid u32
    /// * `Overflow::Overflowing` - similar to above, the result is computed as `(a + b) mod 2^32`, however a boolean
    /// is also pushed on the stack after the result, which is 1 if the result of `a + b` overflowed, else 0.
    ///
    pub fn add_u32(self, overflow: Overflow) {
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
        self.asm.push(self.ip, MasmOp::U32OverflowingAdd3);
    }

    /// Pops three elements from the stack, `c`, `b`, and `a`, and computes `a + b + c` using the
    /// wrapping semantics of `add_u32`. The result will be on top of the stack afterwards, mod 2^32.
    pub fn add3_wrapping_u32(self) {
        self.asm.push(self.ip, MasmOp::U32WrappingAdd3);
    }

    /// Performs unsigned subtraction of the top two elements on the stack, `b` and `a` respectively, which
    /// are expected to be valid u32 values.
    ///
    /// The specific behavior of the subtraction depends on the given `overflow` flags:
    ///
    /// * `Overflow::Unchecked` - the subtraction is performed using the `sub` op for field elements, which may
    /// produce a value that is outside of the u32 range, it is the callers responsibility to ensure that the
    /// resulting value is in range.
    /// * `Overflow::Checked` - the operation will trap if either operand, or the result, is not a valid u32
    /// * `Overflow::Wrapping` - computes the result as `(a - b) mod 2^32`, behavior is undefined if either operand
    /// is not a valid u32
    /// * `Overflow::Overflowing` - similar to above, the result is computed as `(a - b) mod 2^32`, however a boolean
    /// is also pushed on the stack after the result, which is 1 if the result of `a - b` underflowed, else 0.
    ///
    pub fn sub_u32(self, overflow: Overflow) {
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
    /// The specific behavior of the subtraction depends on the given `overflow` flags:
    ///
    /// * `Overflow::Unchecked` - the multiplication is performed using the `mul` op for field elements, which may
    /// produce a value that is outside of the u32 range, it is the callers responsibility to ensure that the
    /// resulting value is in range.
    /// * `Overflow::Checked` - the operation will trap if either operand, or the result, is not a valid u32
    /// * `Overflow::Wrapping` - computes the result as `(a * b) mod 2^32`, behavior is undefined if either operand
    /// is not a valid u32
    /// * `Overflow::Overflowing` - similar to above, the result is computed as `(a * b) mod 2^32`, however a boolean
    /// is also pushed on the stack after the result, which is 1 if the result of `a * b` underflowed, else 0.
    ///
    pub fn mul_u32(self, overflow: Overflow) {
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
        self.asm.push(self.ip, MasmOp::U32OverflowingMadd);
    }

    /// Pops three elements from the stack, `b`, `a`, and `c`, and computes `a * b + c`, using wrapping
    /// semantics, i.e. the result is wrapped mod 2^32.
    pub fn madd_wrapping_u32(self) {
        self.asm.push(self.ip, MasmOp::U32WrappingMadd);
    }

    /// Performs unsigned division of the top two elements on the stack, `b` and `a` respectively, which
    /// are expected to be valid u32 values.
    ///
    /// This operation is checked, meaning that if either operand is >= 2^32, then it will trap.
    ///
    /// Traps if `b` is 0.
    pub fn div_u32(self) {
        self.asm.push(self.ip, MasmOp::U32CheckedDiv);
    }

    /// Same as above, but `b` is provided by the given immediate
    pub fn div_imm_u32(self, imm: u32) {
        self.asm.push(self.ip, MasmOp::U32CheckedDivImm(imm));
    }

    /// Performs unsigned division of the top two elements on the stack, `b` and `a` respectively, which
    /// are expected to be valid u32 values.
    ///
    /// This operation is unchecked, so if either operand is >= 2^32, the result is undefined.
    ///
    /// Traps if `b` is 0.
    pub fn div_unchecked_u32(self) {
        self.asm.push(self.ip, MasmOp::U32UncheckedDiv);
    }

    /// Same as above, but `b` is provided by the given immediate
    pub fn div_imm_unchecked_u32(self, imm: u32) {
        self.asm.push(self.ip, MasmOp::U32UncheckedDivImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and computes `a mod b`.
    ///
    /// This operation is checked, meaning that if either operand is >= 2^32, then it will trap.
    ///
    /// Traps if `b` is 0.
    pub fn mod_u32(self) {
        self.asm.push(self.ip, MasmOp::U32CheckedMod);
    }

    /// Same as above, but `b` is provided by the given immediate
    pub fn mod_imm_u32(self, imm: u32) {
        self.asm.push(self.ip, MasmOp::U32CheckedModImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and computes `a mod b`.
    ///
    /// This operation is unchecked, so if either operand is >= 2^32, the result is undefined.
    ///
    /// Traps if `b` is 0.
    pub fn mod_unchecked_u32(self) {
        self.asm.push(self.ip, MasmOp::U32UncheckedMod);
    }

    /// Same as above, but `b` is provided by the given immediate
    pub fn mod_imm_unchecked_u32(self, imm: u32) {
        self.asm.push(self.ip, MasmOp::U32UncheckedModImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and computes `a / b`, and `a mod b`,
    /// pushing the results of each on the stack in that order.
    ///
    /// This operation is checked, meaning that if either operand is >= 2^32, then it will trap.
    ///
    /// Traps if `b` is 0.
    pub fn divmod_u32(self) {
        self.asm.push(self.ip, MasmOp::U32CheckedDivMod);
    }

    /// Same as above, but `b` is provided by the given immediate
    pub fn divmod_imm_u32(self, imm: u32) {
        self.asm.push(self.ip, MasmOp::U32CheckedDivModImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and computes `a / b`, and `a mod b`,
    /// pushing the results of each on the stack in that order.
    ///
    /// This operation is unchecked, so if either operand is >= 2^32, the results are undefined.
    ///
    /// Traps if `b` is 0.
    pub fn divmod_unchecked_u32(self) {
        self.asm.push(self.ip, MasmOp::U32UncheckedDivMod);
    }

    /// Same as above, but `b` is provided by the given immediate
    pub fn divmod_imm_unchecked_u32(self, imm: u32) {
        self.asm.push(self.ip, MasmOp::U32UncheckedDivModImm(imm));
    }

    /// Pops two elements off the stack, and computes the bitwise AND of those values, placing the result on the stack.
    ///
    /// Traps if either element is not a valid u32 value.
    pub fn band_u32(self) {
        self.asm.push(self.ip, MasmOp::U32And);
    }

    /// Pops two elements off the stack, and computes the bitwise OR of those values, placing the result on the stack.
    ///
    /// Traps if either element is not a valid u32 value.
    pub fn bor_u32(self) {
        self.asm.push(self.ip, MasmOp::U32Or);
    }

    /// Pops two elements off the stack, and computes the bitwise XOR of those values, placing the result on the stack.
    ///
    /// Traps if either element is not a valid u32 value.
    pub fn bxor_u32(self) {
        self.asm.push(self.ip, MasmOp::U32Xor);
    }

    /// Pops an element off the stack, and computes the bitwise NOT of that value, placing the result on the stack.
    ///
    /// Traps if the element is not a valid u32 value.
    pub fn bnot_u32(self) {
        self.asm.push(self.ip, MasmOp::U32Not);
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and shifts `a` left by `b` bits. More precisely,
    /// the result is computed as `(a * 2^b) mod 2^32`.
    ///
    /// Traps if `a` is not a valid u32, or `b` > 31.
    pub fn shl_u32(self) {
        self.asm.push(self.ip, MasmOp::U32CheckedShl);
    }

    /// Same as `shl_u32`, but `b` is provided by immediate.
    pub fn shl_imm_u32(self, imm: u32) {
        self.asm.push(self.ip, MasmOp::U32CheckedShlImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and shifts `a` left by `b` bits. More precisely,
    /// the result is computed as `(a * 2^b) mod 2^32`.
    ///
    /// The result is undefined if `a` is not a valid u32, or `b` is > 31.
    pub fn shl_unchecked_u32(self) {
        self.asm.push(self.ip, MasmOp::U32UncheckedShl);
    }

    /// Same as `shl_unchecked_u32`, but `b` is provided by immediate.
    pub fn shl_unchecked_imm_u32(self, imm: u32) {
        self.asm.push(self.ip, MasmOp::U32UncheckedShlImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and shifts `a` right by `b` bits. More precisely,
    /// the result is computed as `a / 2^b`.
    ///
    /// Traps if `a` is not a valid u32, or `b` > 31.
    pub fn shr_u32(self) {
        self.asm.push(self.ip, MasmOp::U32CheckedShr);
    }

    /// Same as `shr_u32`, but `b` is provided by immediate.
    pub fn shr_imm_u32(self, imm: u32) {
        self.asm.push(self.ip, MasmOp::U32CheckedShrImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and shifts `a` right by `b` bits. More precisely,
    /// the result is computed as `a / 2^b`.
    ///
    /// The result is undefined if `a` is not a valid u32, or `b` is > 31.
    pub fn shr_unchecked_u32(self) {
        self.asm.push(self.ip, MasmOp::U32UncheckedShr);
    }

    /// Same as `shr_unchecked_u32`, but `b` is provided by immediate.
    pub fn shr_unchecked_imm_u32(self, imm: u32) {
        self.asm.push(self.ip, MasmOp::U32UncheckedShrImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and rotates the binary representation of `a`
    /// left by `b` bits.
    ///
    /// Traps if `a` is not a valid u32, or `b` > 31
    pub fn rotl_u32(self) {
        self.asm.push(self.ip, MasmOp::U32CheckedRotl);
    }

    /// Same as `rotl_u32`, but `b` is provided by immediate.
    pub fn rotl_imm_u32(self, imm: u32) {
        self.asm.push(self.ip, MasmOp::U32CheckedRotlImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and rotates the binary representation of `a`
    /// left by `b` bits.
    ///
    /// The result is undefined if `a` is not a valid u32, or `b` is > 31.
    pub fn rotl_unchecked_u32(self) {
        self.asm.push(self.ip, MasmOp::U32UncheckedRotl);
    }

    /// Same as `rotl_unchecked_u32`, but `b` is provided by immediate.
    pub fn rotl_unchecked_imm_u32(self, imm: u32) {
        self.asm.push(self.ip, MasmOp::U32UncheckedRotlImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and rotates the binary representation of `a`
    /// right by `b` bits.
    ///
    /// Traps if `a` is not a valid u32, or `b` > 31
    pub fn rotr_u32(self) {
        self.asm.push(self.ip, MasmOp::U32CheckedRotr);
    }

    /// Same as `rotr_u32`, but `b` is provided by immediate.
    pub fn rotr_imm_u32(self, imm: u32) {
        self.asm.push(self.ip, MasmOp::U32CheckedRotrImm(imm));
    }

    /// Pops two elements off the stack, `b` and `a` respectively, and rotates the binary representation of `a`
    /// right by `b` bits.
    ///
    /// The result is undefined if `a` is not a valid u32, or `b` is > 31.
    pub fn rotr_unchecked_u32(self) {
        self.asm.push(self.ip, MasmOp::U32UncheckedRotr);
    }

    /// Same as `rotr_unchecked_u32`, but `b` is provided by immediate.
    pub fn rotr_unchecked_imm_u32(self, imm: u32) {
        self.asm.push(self.ip, MasmOp::U32UncheckedRotrImm(imm));
    }

    /// Pops an element off the stack, and computes the number of set bits in its binary representation, i.e.
    /// its hamming weight, and places the result on the stack.
    ///
    /// Traps if the input value is not a valid u32.
    pub fn popcnt_u32(self) {
        self.asm.push(self.ip, MasmOp::U32CheckedPopcnt);
    }

    /// Pops an element off the stack, and computes the number of set bits in its binary representation, i.e.
    /// its hamming weight, and places the result on the stack.
    ///
    /// The result is undefined if the input value is not a valid u32.
    pub fn popcnt_unchecked_u32(self) {
        self.asm.push(self.ip, MasmOp::U32UncheckedPopcnt);
    }

    /// This is the same as `eq`, but also asserts that both operands are valid u32 values.
    pub fn eq_u32(self) {
        self.asm.push(self.ip, MasmOp::U32Eq);
    }

    /// This is the same as `eq_imm`, but also asserts that both operands are valid u32 values.
    pub fn eq_imm_u32(self, imm: u32) {
        self.asm.push(self.ip, MasmOp::U32EqImm(imm));
    }

    /// This is the same as `neq`, but also asserts that both operands are valid u32 values.
    pub fn neq_u32(self) {
        self.asm.push(self.ip, MasmOp::U32Neq);
    }

    /// This is the same as `neq_imm`, but also asserts that both operands are valid u32 values.
    pub fn neq_imm_u32(self, imm: u32) {
        self.asm.push(self.ip, MasmOp::U32NeqImm(imm));
    }

    /// This is the same as `lt`, but also asserts that both operands are valid u32 values.
    pub fn lt_u32(self) {
        self.asm.push(self.ip, MasmOp::U32CheckedLt);
    }

    /// This is the same as `lt`, but the result is undefined if either operand is not a valid u32 value.
    pub fn lt_unchecked_u32(self) {
        self.asm.push(self.ip, MasmOp::U32UncheckedLt);
    }

    /// This is the same as `lte`, but also asserts that both operands are valid u32 values.
    pub fn lte_u32(self) {
        self.asm.push(self.ip, MasmOp::U32CheckedLte);
    }

    /// This is the same as `lte`, but the result is undefined if either operand is not a valid u32 value.
    pub fn lte_unchecked_u32(self) {
        self.asm.push(self.ip, MasmOp::U32UncheckedLte);
    }

    /// This is the same as `gt`, but also asserts that both operands are valid u32 values.
    pub fn gt_u32(self) {
        self.asm.push(self.ip, MasmOp::U32CheckedGt);
    }

    /// This is the same as `gt`, but the result is undefined if either operand is not a valid u32 value.
    pub fn gt_unchecked_u32(self) {
        self.asm.push(self.ip, MasmOp::U32UncheckedGt);
    }

    /// This is the same as `gte`, but also asserts that both operands are valid u32 values.
    pub fn gte_u32(self) {
        self.asm.push(self.ip, MasmOp::U32CheckedGte);
    }

    /// This is the same as `gte`, but the result is undefined if either operand is not a valid u32 value.
    pub fn gte_unchecked_u32(self) {
        self.asm.push(self.ip, MasmOp::U32UncheckedGte);
    }

    /// This is the same as `min`, but also asserts that both operands are valid u32 values.
    pub fn min_u32(self) {
        self.asm.push(self.ip, MasmOp::U32CheckedMin);
    }

    /// This is the same as `min`, but the result is undefined if either operand is not a valid u32 value.
    pub fn min_unchecked_u32(self) {
        self.asm.push(self.ip, MasmOp::U32UncheckedMin);
    }

    /// This is the same as `max`, but also asserts that both operands are valid u32 values.
    pub fn max_u32(self) {
        self.asm.push(self.ip, MasmOp::U32CheckedMax);
    }

    /// This is the same as `max`, but the result is undefined if either operand is not a valid u32 value.
    pub fn max_unchecked_u32(self) {
        self.asm.push(self.ip, MasmOp::U32UncheckedMax);
    }
}
