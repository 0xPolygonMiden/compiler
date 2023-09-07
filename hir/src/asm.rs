use std::fmt;

use cranelift_entity::{entity_impl, PrimaryMap};
use winter_math::FieldElement;

use super::{write::DisplayIndent, *};

/// Represents Miden Assembly (MASM) directly in the IR
///
/// Each block of inline assembly executes in its own pseudo-isolated environment,
/// i.e. other than arguments provided to the inline assembly, and values introduced
/// within the inline assembly, it is not permitted to access anything else on the
/// operand stack
#[derive(Debug, Clone)]
pub struct InlineAsm {
    pub op: Opcode,
    /// Arguments on which the inline assembly can operate
    ///
    /// The operand stack will be set up such that the given arguments
    /// will appear in LIFO order, i.e. the first argument will be on top
    /// of the stack, and so on.
    ///
    /// The inline assembly will be validated so that all other values on
    /// the operand stack below the given arguments will remain on the stack
    /// when the inline assembly finishes executing.
    pub args: ValueList,
    /// The main code block
    pub body: MasmBlockId,
    /// The set of all code blocks contained in this inline assembly
    ///
    /// This is necessary to support control flow operations within asm blocks
    pub blocks: PrimaryMap<MasmBlockId, MasmBlock>,
}
impl InlineAsm {
    /// Constructs a new, empty inline assembly block
    pub fn new() -> Self {
        let mut blocks = PrimaryMap::<MasmBlockId, MasmBlock>::new();
        let id = blocks.next_key();
        let body = blocks.push(MasmBlock { id, ops: vec![] });
        Self {
            op: Opcode::InlineAsm,
            args: ValueList::default(),
            body,
            blocks,
        }
    }

    /// Create a new code block for use with this inline assembly
    pub fn create_block(&mut self) -> MasmBlockId {
        let id = self.blocks.next_key();
        self.blocks.push(MasmBlock { id, ops: vec![] });
        id
    }

    /// Appends `op` to the end of `block`
    pub fn push(&mut self, block: MasmBlockId, op: MasmOp) {
        self.blocks[block].push(op);
    }

    pub fn display<'a, 'b: 'a>(
        &'b self,
        dfg: &'b DataFlowGraph,
        indent: usize,
    ) -> DisplayInlineAsm<'a> {
        DisplayInlineAsm {
            asm: self,
            dfg,
            indent,
        }
    }
}

pub struct DisplayInlineAsm<'a> {
    asm: &'a InlineAsm,
    dfg: &'a DataFlowGraph,
    indent: usize,
}
impl<'a> fmt::Display for DisplayInlineAsm<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use super::write::DisplayValues;

        {
            let args = self.asm.args.as_slice(&self.dfg.value_lists);
            writeln!(f, "({}) {{", DisplayValues(args))?;
        }

        let indent = self.indent;
        let block = self.asm.body;
        writeln!(
            f,
            "{}",
            DisplayBlock {
                asm: self.asm,
                block,
                indent: indent + 1,
            }
        )?;

        writeln!(f, "{}}}", DisplayIndent(indent))
    }
}

/// A handle that refers to a MASM code block
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MasmBlockId(u32);
entity_impl!(MasmBlockId, "blk");

/// Represents a single code block in Miden Assembly
#[derive(Debug, Clone)]
pub struct MasmBlock {
    pub id: MasmBlockId,
    pub ops: Vec<MasmOp>,
}
impl MasmBlock {
    /// Returns true if there are no instructions in this block
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }

    /// Returns the instructions contained in this block as a slice
    #[inline(always)]
    pub fn ops(&self) -> &[MasmOp] {
        self.ops.as_slice()
    }

    /// Appends `op` to this code block
    #[inline(always)]
    pub fn push(&mut self, op: MasmOp) {
        self.ops.push(op);
    }

    /// Appends instructions from `other` to the end of this block
    #[inline]
    pub fn append(&mut self, other: &mut Vec<MasmOp>) {
        self.ops.append(other);
    }
}

struct DisplayBlock<'a> {
    asm: &'a InlineAsm,
    block: MasmBlockId,
    indent: usize,
}
impl<'a> fmt::Display for DisplayBlock<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let block = &self.asm.blocks[self.block];
        let indent = self.indent;
        for op in block.ops.iter() {
            writeln!(
                f,
                "{}",
                DisplayOp {
                    asm: self.asm,
                    op,
                    indent
                }
            )?;
        }
        Ok(())
    }
}

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

/// This enum represents the Miden Assembly (MASM) instruction set.
///
/// Not all MASM instructions are necessarily represented here, only those we
/// actually use, or intend to use, when compiling from Miden IR.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MasmOp {
    /// Pushes a null word on the stack, i.e. four 0 values
    Padw,
    /// Pushes the given field element constant on top of the stack
    Push(Felt),
    /// Pushes the given word constant on top of the stack
    Pushw([Felt; 4]),
    /// Pushes the given 8-bit constant on top of the stack
    PushU8(u8),
    /// Pushes the given 16-bit constant on top of the stack
    PushU16(u16),
    /// Pushes the given 32-bit constant on top of the stack
    PushU32(u32),
    /// Removes the item on the top of the stack
    Drop,
    /// Removes the top 4 items on the stack
    Dropw,
    /// Copies the `n`th item on the stack to the top of stack
    ///
    /// * `Dup(0)` duplicates the item on top of the stack
    Dup(u8),
    /// Copies the `n`th word on the stack, to the top of the stack
    ///
    /// The only values of `n` which are valid, are 0, 1, 2, 3; or
    /// in other words, the 4 words which make up the top 16 elements
    /// of the stack.
    Dupw(u8),
    /// Swaps the 1st and `n`th items on the stack
    ///
    /// * `Swap(1)` swaps the top two elements of the stack
    Swap(u8),
    /// Swaps the 1st and `n`th words on the stack
    ///
    /// The only values of `n` which are valid, are 1, 2, 3; or
    /// in other words, the 3 words which make up the last 12 elements
    /// of the stack.
    Swapw(u8),
    /// Moves the `n`th stack item to top of stack
    ///
    /// * `Movup(1)` is equivalent to `Swap(1)`
    Movup(u8),
    /// Moves the `n`th stack word to the top of the stack
    ///
    /// The only values of `n` which are valid are 2 and 3. Use `Swapw(1)`
    /// if you want to move the second word to the top.
    Movupw(u8),
    /// Moves the top of stack to the `n`th index of the stack
    ///
    /// * `Movdn(1)` is equivalent to `Swap(1)`
    Movdn(u8),
    /// Moves the top word of the stack, into position as the `n`th word on the stack.
    ///
    /// The only values of `n` which are valid are 2 and 3. Use `Swapw(1)`
    /// if you want to make the top word the second word.
    Movdnw(u8),
    /// Pops `c, b, a` off the stack, and swaps `b` and `a` if `c` is 1, or leaves
    /// them as-is when 0.
    ///
    /// Traps if `c` is > 1.
    Cswap,
    /// Pops `c, B, A` off the stack, where `B` and `A` are words, and swaps `B` and `A`
    /// if `c` is 1, or leaves them as-is when 0.
    ///
    /// Traps if `c` is > 1.
    Cswapw,
    /// Pops `c, b, a` off the stack, and pushes back `b` if `c` is 1, and `a` if 0.
    ///
    /// Traps if `c` is > 1.
    Cdrop,
    /// Pops `c, B, A` off the stack, where `B` and `A` are words, and pushes back `B`
    /// if `c` is 1, and `A` if 0.
    ///
    /// Traps if `c` is > 1.
    Cdropw,
    /// Pops a value off the stack and asserts that it is equal to 1
    Assert,
    /// Pops a value off the stack and asserts that it is equal to 0
    Assertz,
    /// Pops two values off the stack and asserts that they are equal
    AssertEq,
    /// Pops two words off the stack and asserts that they are equal
    AssertEqw,
    /// Places the memory address of the given local index on top of the stack
    LocAddr(LocalId),
    /// Pops `a`, representing a memory address, from the top of the stack, then loads the
    /// first element of the word starting at that address, placing it on top of the stack.
    ///
    /// Traps if `a` >= 2^32
    MemLoad,
    /// Same as above, but the address is given as an immediate
    MemLoadImm(u32),
    /// Pops `a`, representing a memory address + offset pair, from the top of the stack, then loads the
    /// element at the given offset from the base of the word starting at that address, placing it on top
    /// of the stack.
    ///
    /// Traps if `a` >= 2^32
    ///
    /// NOTE: This instruction doesn't actually exist in Miden Assembly yet, it is a proposed extension of
    /// `MemLoad` which allows addressing all field elements of a word individually. It is here for testing.
    MemLoadOffset,
    /// Same as above, but the address and offset are given as a immediates
    MemLoadOffsetImm(u32, u8),
    /// Pops `a`, representing a memory address, from the top of the stack, then overwrites
    /// the top word of the stack with the word starting at that address.
    ///
    /// Traps if `a` >= 2^32
    MemLoadw,
    /// Same as above, but the address is given as an immediate
    MemLoadwImm(u32),
    /// Pops `a, v` from the stack, where `a` represents a memory address, and `v` the value
    /// to be stored, and stores `v` as the element as the first element of the word starting
    /// at that address. The remaining elements of the word are not modified.
    ///
    /// Traps if `a` >= 2^32
    MemStore,
    /// Same as above, but the address is given as an immediate
    MemStoreImm(u32),
    /// Pops `a, v` from the stack, where `a` represents a memory address + offset pair, and `v` the value
    /// to be stored, and stores `v` as the element at the given offset from the base of the word starting
    /// at that address. The remaining elements of the word are not modified.
    ///
    /// Traps if `a` >= 2^32
    ///
    /// NOTE: This instruction doesn't actually exist in Miden Assembly yet, it is a proposed extension of
    /// `MemStore` which allows addressing all field elements of a word individually. It is here for testing.
    MemStoreOffset,
    /// Same as above, but the address and offset are given as a immediates
    MemStoreOffsetImm(u32, u8),
    /// Pops `a, V` from the stack, where `a` represents a memory address, and `V` is a word to be stored
    /// at that location, and overwrites the word located at `a`.
    ///
    /// Traps if `a` >= 2^32
    MemStorew,
    /// Same as above, but the address is given as an immediate
    MemStorewImm(u32),
    /// Pops the top of the stack, and evaluates the ops in
    /// the block of code corresponding to the branch taken.
    ///
    /// If the value is `1`, corresponding to `true`, the first block
    /// is evaluated. Otherwise, the value must be `0`, corresponding to
    /// `false`, and the second block is evaluated.
    If(MasmBlockId, MasmBlockId),
    /// Pops the top of the stack, and evaluates the given block of
    /// code if the value is `1`, corresponding to `true`.
    ///
    /// Otherwise, the value must be `0`, corresponding to `false`,
    /// and the block is skipped.
    While(MasmBlockId),
    /// Repeatedly evaluates the given block, `n` times.
    Repeat(u8, MasmBlockId),
    /// Pops `N` args off the stack, executes the procedure, results will be placed on the stack
    Exec(FunctionIdent),
    /// Pops `N` args off the stack, executes the procedure in the root context, results will be placed on the stack
    Syscall(FunctionIdent),
    /// Pops `b, a` off the stack, and places the result of `(a + b) mod p` on the stack
    Add,
    /// Same as above, but the immediate is used for `b`
    AddImm(Felt),
    /// Pops `b, a` off the stack, and places the result of `(a - b) mod p` on the stack
    Sub,
    /// Same as above, but the immediate is used for `b`
    SubImm(Felt),
    /// Pops `b, a` off the stack, and places the result of `(a * b) mod p` on the stack
    Mul,
    /// Same as above, but the immediate is used for `b`
    MulImm(Felt),
    /// Pops `b, a` off the stack, and places the result of `(a * b^-1) mod p` on the stack
    ///
    /// NOTE: `b` must not be 0
    Div,
    /// Same as above, but the immediate is used for `b`
    DivImm(Felt),
    /// Pops `a` off the stack, and places the result of `-a mod p` on the stack
    Neg,
    /// Pops `a` off the stack, and places the result of `a^-1 mod p` on the stack
    ///
    /// NOTE: `a` must not be equal to 0
    Inv,
    /// Pops `a` off the stack, and places the result of incrementing it by 1 back on the stack
    Incr,
    /// Pops `a` off the stack, and places the result of `2^a` on the stack
    ///
    /// NOTE: `a` must not be > 63
    Pow2,
    /// Pops `a` and `b` off the stack, and places the result of `a^b` on the stack
    ///
    /// NOTE: `b` must not be > 63
    Exp,
    /// Pops `a` off the stack, and places the result of `a^<imm>` on the stack
    ///
    /// NOTE: `imm` must not be > 63
    ExpImm(u8),
    /// Pops `a` off the stack, and places the result of `1 - a` on the stack
    ///
    /// NOTE: `a` must be boolean
    Not,
    /// Pops `b, a` off the stack, and places the result of `a * b` on the stack
    ///
    /// NOTE: `a` must be boolean
    And,
    /// Same as above, but `a` is taken from the stack, and `b` is the immediate.
    ///
    /// NOTE: `a` must be boolean
    AndImm(bool),
    /// Pops `b, a` off the stack, and places the result of `a + b - a * b` on the stack
    ///
    /// NOTE: `a` must be boolean
    Or,
    /// Same as above, but `a` is taken from the stack, and `b` is the immediate.
    ///
    /// NOTE: `a` must be boolean
    OrImm(bool),
    /// Pops `b, a` off the stack, and places the result of `a + b - 2 * a * b` on the stack
    ///
    /// NOTE: `a` and `b` must be boolean
    Xor,
    /// Same as above, but `a` is taken from the stack, and `b` is the immediate.
    ///
    /// NOTE: `a` must be boolean
    XorImm(bool),
    /// Pops `b, a` off the stack, and places the result of `a == b` on the stack
    Eq,
    /// Same as above, but `b` is provided by the immediate
    EqImm(Felt),
    /// Pops `b, a` off the stack, and places the result of `a != b` on the stack
    Neq,
    /// Same as above, but `b` is provided by the immediate
    NeqImm(Felt),
    /// Pops `b, a` off the stack, and places the result of `a > b` on the stack
    Gt,
    /// Same as above, but `b` is provided by the immediate
    GtImm(Felt),
    /// Pops `b, a` off the stack, and places the result of `a >= b` on the stack
    Gte,
    /// Same as above, but `b` is provided by the immediate
    GteImm(Felt),
    /// Pops `b, a` off the stack, and places the result of `a < b` on the stack
    Lt,
    /// Same as above, but `b` is provided by the immediate
    LtImm(Felt),
    /// Pops `b, a` off the stack, and places the result of `a <= b` on the stack
    Lte,
    /// Same as above, but `b` is provided by the immediate
    LteImm(Felt),
    /// Pops `a` off the stack, and places the 1 on the stack if `a` is odd, else 0
    IsOdd,
    /// Pops `B, A` off the stack, and places the result of `A == B` on the stack,
    /// where the uppercase variables here represent words, rather than field elements.
    ///
    /// The comparison works by comparing pairs of elements from each word
    Eqw,
    /// Pushes the current value of the cycle counter (clock) on the stack
    Clk,
    /// Peeks `a` from the top of the stack, and places the 1 on the stack if `a < 2^32`, else 0
    U32Test,
    /// Peeks `A` from the top of the stack, and places the 1 on the stack if `forall a : A, a < 2^32`, else 0
    U32Testw,
    /// Peeks `a` from the top of the stack, and traps if `a >= 2^32`
    U32Assert,
    /// Peeks `b, a` from the top of the stack, and traps if either `a` or `b` is >= 2^32
    U32Assert2,
    /// Peeks `A` from the top of the stack, and traps unless `forall a : A, a < 2^32`, else 0
    U32Assertw,
    /// Pops `a` from the top of the stack, and places the result of `a mod 2^32` on the stack
    ///
    /// This is used to cast a field element to the u32 range
    U32Cast,
    /// Pops `a` from the top of the stack, and splits it into upper and lower 32-bit values,
    /// placing them back on the stack. The lower part is calculated as `a mod 2^32`,
    /// and the higher part as `a / 2^32`. The higher part will be on top of the stack after.
    U32Split,
    /// Pops `b, a` from the stack, and places the result of `a + b` on the stack,
    /// trapping if the result, or either operand, are >= 2^32
    U32CheckedAdd,
    /// Same as above, but with `b` provided by the immediate
    U32CheckedAddImm(u32),
    /// Pops `b, a` from the stack, and places the result of `(a + b) mod 2^32` on the stack,
    /// followed by 1 if `(a + b) >= 2^32`, else 0. Thus the first item on the stack will be
    /// a boolean indicating whether the arithmetic overflowed, and the second will be the
    /// result of the addition.
    ///
    /// The behavior is undefined if either `b` or `a` are >= 2^32
    U32OverflowingAdd,
    /// Same as above, but with `b` provided by the immediate
    U32OverflowingAddImm(u32),
    /// Pops `b, a` from the stack, and places the result of `(a + b) mod 2^32` on the stack.
    ///
    /// The behavior is undefined if either `b` or `a` are >= 2^32
    U32WrappingAdd,
    /// Same as above, but with `b` provided by the immediate
    U32WrappingAddImm(u32),
    /// Pops `c, b, a` from the stack, adds them together, and splits the result into higher
    /// and lower parts. The lower part is calculated as `(a + b + c) mod 2^32`,
    /// the higher part as `(a + b + c) / 2^32`.
    ///
    /// The behavior is undefined if any of `c`, `b` or `a` are >= 2^32
    U32OverflowingAdd3,
    /// Pops `c, b, a` from the stack, adds them together, and splits the result into higher
    /// and lower parts. The lower part is calculated as `(a + b + c) mod 2^32`,
    /// the higher part as `(a + b + c) / 2^32`.
    ///
    /// The behavior is undefined if any of `c`, `b` or `a` are >= 2^32
    U32WrappingAdd3,
    /// Pops `b, a` from the stack, and places the result of `a - b` on the stack,
    /// trapping if the result, or either operand, are >= 2^32; OR if `a < b`.
    U32CheckedSub,
    /// Same as above, but with `b` provided by the immediate
    U32CheckedSubImm(u32),
    /// Pops `b, a` from the stack, and places the result of `(a - b) mod 2^32` on the stack,
    /// followed by 1 if `a < b`, else 0. Thus the first item on the stack will be
    /// a boolean indicating whether the arithmetic underflowed, and the second will be the
    /// result of the subtraction.
    ///
    /// The behavior is undefined if either `b` or `a` are >= 2^32
    U32OverflowingSub,
    /// Same as above, but with `b` provided by the immediate
    U32OverflowingSubImm(u32),
    /// Pops `b, a` from the stack, and places the result of `(a - b) mod 2^32` on the stack.
    ///
    /// The behavior is undefined if either `b` or `a` are >= 2^32
    U32WrappingSub,
    /// Same as above, but with `b` provided by the immediate
    U32WrappingSubImm(u32),
    /// Pops `b, a` from the stack, and places the result of `a * b` on the stack,
    /// trapping if the result, or either operand, are >= 2^32.
    U32CheckedMul,
    /// Same as above, but with `b` provided by the immediate
    U32CheckedMulImm(u32),
    /// Pops `b, a` from the stack, and places the result of `(a * b) mod 2^32` on the stack,
    /// followed by `(a * b) / 2^32`. Thus the first item on the stack will be the number
    /// of times the multiplication overflowed, followed by the result.
    ///
    /// The behavior is undefined if either `b` or `a` are >= 2^32
    U32OverflowingMul,
    /// Same as above, but with `b` provided by the immediate
    U32OverflowingMulImm(u32),
    /// Pops `b, a` from the stack, and places the result of `(a * b) mod 2^32` on the stack.
    ///
    /// The behavior is undefined if either `b` or `a` are >= 2^32
    U32WrappingMul,
    /// Same as above, but with `b` provided by the immediate
    U32WrappingMulImm(u32),
    /// Pops `c, b, a` off the stack, and calculates `d = c * b + a`, then splits the result
    /// into higher and lower parts, the lower given by `d mod 2^32`, the higher by `d / 2^32`,
    /// and pushes them back on the stack, with the higher part on top of the stack at the end.
    ///
    /// Behavior is undefined if any of `a`, `b`, or `c` are >= 2^32
    U32OverflowingMadd,
    /// Pops `c, b, a` off the stack, and pushes `(c * a + b) mod 2^32` on the stack.
    ///
    /// Behavior is undefined if any of `a`, `b`, or `c` are >= 2^32
    U32WrappingMadd,
    /// Pops `b, a` off the stack, and pushes `a / b` on the stack.
    ///
    /// Traps if `b` is 0, or if `a` or `b` >= 2^32
    U32CheckedDiv,
    /// Same as above, except `b` is provided by the immediate
    U32CheckedDivImm(u32),
    /// Pops `b, a` off the stack, and pushes `a / b` on the stack.
    ///
    /// Traps if `b` is 0.
    ///
    /// Behavior is undefined if `a` or `b` >= 2^32
    U32UncheckedDiv,
    /// Same as above, except `b` is provided by the immediate
    U32UncheckedDivImm(u32),
    /// Pops `b, a` off the stack, and pushes `a mod b` on the stack.
    ///
    /// Traps if `b` is 0, or if `a` or `b` >= 2^32
    U32CheckedMod,
    /// Same as above, except `b` is provided by the immediate
    U32CheckedModImm(u32),
    /// Pops `b, a` off the stack, and pushes `a mod b` on the stack.
    ///
    /// Traps if `b` is 0.
    ///
    /// Behavior is undefined if `a` or `b` >= 2^32
    U32UncheckedMod,
    /// Same as above, except `b` is provided by the immediate
    U32UncheckedModImm(u32),
    /// Pops `b, a` off the stack, and first pushes `a / b` on the stack, followed by `a mod b`.
    ///
    /// Traps if `b` is 0, or if `a` or `b` >= 2^32
    U32CheckedDivMod,
    /// Same as above, except `b` is provided by the immediate
    U32CheckedDivModImm(u32),
    /// Pops `b, a` off the stack, and first pushes `a / b` on the stack, followed by `a mod b`.
    ///
    /// Traps if `b` is 0.
    ///
    /// Behavior is undefined if `a` or `b` >= 2^32
    U32UncheckedDivMod,
    /// Same as above, except `b` is provided by the immediate
    U32UncheckedDivModImm(u32),
    /// Pops `b, a` off the stack, and places the bitwise AND of `a` and `b` on the stack.
    ///
    /// Traps if either `a` or `b` >= 2^32
    U32And,
    /// Pops `b, a` off the stack, and places the bitwise OR of `a` and `b` on the stack.
    ///
    /// Traps if either `a` or `b` >= 2^32
    U32Or,
    /// Pops `b, a` off the stack, and places the bitwise XOR of `a` and `b` on the stack.
    ///
    /// Traps if either `a` or `b` >= 2^32
    U32Xor,
    /// Pops `a` off the stack, and places the bitwise NOT of `a` on the stack.
    ///
    /// Traps if `a >= 2^32`
    U32Not,
    /// Pops `b, a` off the stack, and places the result of `(a * 2^b) mod 2^32` on the stack.
    ///
    /// Traps if `a >= 2^32` or `b > 31`
    U32CheckedShl,
    /// Same as above, except `b` is provided by the immediate
    U32CheckedShlImm(u32),
    /// Pops `b, a` off the stack, and places the result of `(a * 2^b) mod 2^32` on the stack.
    ///
    /// Behavior is undefined if `a >= 2^32` or `b > 31`
    U32UncheckedShl,
    /// Same as above, except `b` is provided by the immediate
    U32UncheckedShlImm(u32),
    /// Pops `b, a` off the stack, and places the result of `a / 2^b` on the stack.
    ///
    /// Traps if `a >= 2^32` or `b > 31`
    U32CheckedShr,
    /// Same as above, except `b` is provided by the immediate
    U32CheckedShrImm(u32),
    /// Pops `b, a` off the stack, and places the result of `a / 2^b` on the stack.
    ///
    /// Behavior is undefined if `a >= 2^32` or `b > 31`
    U32UncheckedShr,
    /// Same as above, except `b` is provided by the immediate
    U32UncheckedShrImm(u32),
    /// Pops `b, a` off the stack, and places the result of rotating the 32-bit
    /// representation of `a` to the left by `b` bits.
    ///
    /// Traps if `a` >= 2^32, or `b` > 31
    U32CheckedRotl,
    /// Same as above, except `b` is provided by the immediate
    U32CheckedRotlImm(u32),
    /// Pops `b, a` off the stack, and places the result of rotating the 32-bit
    /// representation of `a` to the left by `b` bits.
    ///
    /// Behavior is undefined if `a` >= 2^32, or `b` > 31
    U32UncheckedRotl,
    /// Same as above, except `b` is provided by the immediate
    U32UncheckedRotlImm(u32),
    /// Pops `b, a` off the stack, and places the result of rotating the 32-bit
    /// representation of `a` to the right by `b` bits.
    ///
    /// Traps if `a` >= 2^32, or `b` > 31
    U32CheckedRotr,
    /// Same as above, except `b` is provided by the immediate
    U32CheckedRotrImm(u32),
    /// Pops `b, a` off the stack, and places the result of rotating the 32-bit
    /// representation of `a` to the right by `b` bits.
    ///
    /// Behavior is undefined if `a` >= 2^32, or `b` > 31
    U32UncheckedRotr,
    /// Same as above, except `b` is provided by the immediate
    U32UncheckedRotrImm(u32),
    /// Pops `a` off the stack, and places the number of set bits in `a` (it's hamming weight).
    ///
    /// Traps if `a` >= 2^32
    U32CheckedPopcnt,
    /// Pops `a` off the stack, and places the number of set bits in `a` (it's hamming weight).
    ///
    /// Behavior is undefined if `a` >= 2^32
    U32UncheckedPopcnt,
    /// Pops `b, a` from the stack, and places 1 on the stack if `a == b`, else 0
    ///
    /// Traps if either `a` or `b` are >= 2^32
    U32Eq,
    /// Same as above, except `b` is provided by the immediate
    U32EqImm(u32),
    /// Pops `b, a` from the stack, and places 1 on the stack if `a != b`, else 0
    ///
    /// Traps if either `a` or `b` are >= 2^32
    U32Neq,
    /// Same as above, except `b` is provided by the immediate
    U32NeqImm(u32),
    /// Pops `b, a` from the stack, and places 1 on the stack if `a < b`, else 0
    ///
    /// Traps if either `a` or `b` are >= 2^32
    U32CheckedLt,
    /// Pops `b, a` from the stack, and places 1 on the stack if `a < b`, else 0
    ///
    /// Traps if either `a` or `b` are >= 2^32
    U32UncheckedLt,
    /// Pops `b, a` from the stack, and places 1 on the stack if `a <= b`, else 0
    ///
    /// Traps if either `a` or `b` are >= 2^32
    U32CheckedLte,
    /// Pops `b, a` from the stack, and places 1 on the stack if `a <= b`, else 0
    ///
    /// Traps if either `a` or `b` are >= 2^32
    U32UncheckedLte,
    /// Pops `b, a` from the stack, and places 1 on the stack if `a > b`, else 0
    ///
    /// Traps if either `a` or `b` are >= 2^32
    U32CheckedGt,
    /// Pops `b, a` from the stack, and places 1 on the stack if `a > b`, else 0
    ///
    /// The behavior is undefined if either `a` or `b` are >= 2^32
    U32UncheckedGt,
    /// Pops `b, a` from the stack, and places 1 on the stack if `a >= b`, else 0
    ///
    /// Traps if either `a` or `b` are >= 2^32
    U32CheckedGte,
    /// Pops `b, a` from the stack, and places 1 on the stack if `a >= b`, else 0
    ///
    /// The behavior is undefined if either `a` or `b` are >= 2^32
    U32UncheckedGte,
    /// Pops `b, a` from the stack, and places `a` back on the stack if `a < b`, else `b`
    ///
    /// Traps if either `a` or `b` are >= 2^32
    U32CheckedMin,
    /// Pops `b, a` from the stack, and places `a` back on the stack if `a < b`, else `b`
    ///
    /// The behavior is undefined if either `a` or `b` are >= 2^32
    U32UncheckedMin,
    /// Pops `b, a` from the stack, and places `a` back on the stack if `a > b`, else `b`
    ///
    /// Traps if either `a` or `b` are >= 2^32
    U32CheckedMax,
    /// Pops `b, a` from the stack, and places `a` back on the stack if `a > b`, else `b`
    ///
    /// The behavior is undefined if either `a` or `b` are >= 2^32
    U32UncheckedMax,
}

struct DisplayOp<'a> {
    asm: &'a InlineAsm,
    op: &'a MasmOp,
    indent: usize,
}
impl<'a> fmt::Display for DisplayOp<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", DisplayIndent(self.indent))?;
        match self.op {
            MasmOp::Padw => f.write_str("padw"),
            MasmOp::Push(imm) => write!(f, "push.{}", imm),
            MasmOp::Pushw(word) => write!(
                f,
                "push.{}.{}.{}.{}",
                &word[0], &word[1], &word[2], &word[3]
            ),
            MasmOp::PushU8(imm) => write!(f, "push.{:#0x}", imm),
            MasmOp::PushU16(imm) => write!(f, "push.{:#0x}", imm),
            MasmOp::PushU32(imm) => write!(f, "push.{:#0x}", imm),
            MasmOp::Drop => f.write_str("drop"),
            MasmOp::Dropw => f.write_str("dropw"),
            MasmOp::Dup(idx) => write!(f, "dup.{idx}"),
            MasmOp::Dupw(idx) => write!(f, "dupw.{idx}"),
            MasmOp::Swap(idx) => write!(f, "swap.{idx}"),
            MasmOp::Swapw(idx) => write!(f, "swapw.{idx}"),
            MasmOp::Movup(idx) => write!(f, "movup.{idx}"),
            MasmOp::Movupw(idx) => write!(f, "movupw.{idx}"),
            MasmOp::Movdn(idx) => write!(f, "movdn.{idx}"),
            MasmOp::Movdnw(idx) => write!(f, "movdnw.{idx}"),
            MasmOp::Cswap => f.write_str("cswap"),
            MasmOp::Cswapw => f.write_str("cswapw"),
            MasmOp::Cdrop => f.write_str("cdrop"),
            MasmOp::Cdropw => f.write_str("cdropw"),
            MasmOp::Assert => f.write_str("assert"),
            MasmOp::Assertz => f.write_str("assertz"),
            MasmOp::AssertEq => f.write_str("assert_eq"),
            MasmOp::AssertEqw => f.write_str("assert_eqw"),
            MasmOp::LocAddr(id) => write!(f, "locaddr.{}", id.as_usize()),
            MasmOp::MemLoad | MasmOp::MemLoadOffset => write!(f, "mem_load"),
            MasmOp::MemLoadImm(addr) => write!(f, "mem_load.{:#0x}", addr),
            MasmOp::MemLoadOffsetImm(addr, offset) => write!(f, "mem_load.{:#0x}.{}", addr, offset),
            MasmOp::MemLoadw => write!(f, "mem_loadw"),
            MasmOp::MemLoadwImm(addr) => write!(f, "mem_loadw.{:#0x}", addr),
            MasmOp::MemStore | MasmOp::MemStoreOffset => write!(f, "mem_store"),
            MasmOp::MemStoreImm(addr) => write!(f, "mem_store.{:#0x}", addr),
            MasmOp::MemStoreOffsetImm(addr, offset) => {
                write!(f, "mem_store.{:#0x}.{}", addr, offset)
            }
            MasmOp::MemStorew => write!(f, "mem_storew"),
            MasmOp::MemStorewImm(addr) => write!(f, "mem_storew.{:#0x}", addr),
            MasmOp::If(then_blk, else_blk) => {
                f.write_str("if.true\n")?;
                {
                    let then_block = &self.asm.blocks[*then_blk];
                    let indent = self.indent + 1;
                    for op in then_block.ops.iter() {
                        writeln!(
                            f,
                            "{}",
                            DisplayOp {
                                asm: self.asm,
                                op,
                                indent
                            }
                        )?;
                    }
                }
                writeln!(f, "{}else", DisplayIndent(self.indent))?;
                {
                    let else_block = &self.asm.blocks[*else_blk];
                    let indent = self.indent + 1;
                    for op in else_block.ops.iter() {
                        writeln!(
                            f,
                            "{}",
                            DisplayOp {
                                asm: self.asm,
                                op,
                                indent
                            }
                        )?;
                    }
                }
                write!(f, "{}end", DisplayIndent(self.indent))
            }
            MasmOp::While(blk) => {
                f.write_str("while.true\n")?;
                {
                    let body = &self.asm.blocks[*blk];
                    let indent = self.indent + 1;
                    for op in body.ops.iter() {
                        writeln!(
                            f,
                            "{}",
                            DisplayOp {
                                asm: self.asm,
                                op,
                                indent
                            }
                        )?;
                    }
                }
                write!(f, "{}end", DisplayIndent(self.indent))
            }
            MasmOp::Repeat(n, blk) => {
                writeln!(f, "repeat.{}", n)?;
                {
                    let body = &self.asm.blocks[*blk];
                    let indent = self.indent + 1;
                    for op in body.ops.iter() {
                        writeln!(
                            f,
                            "{}",
                            DisplayOp {
                                asm: self.asm,
                                op,
                                indent
                            }
                        )?;
                    }
                }
                write!(f, "{}end", DisplayIndent(self.indent))
            }
            MasmOp::Exec(id) => write!(f, "exec.{}", id),
            MasmOp::Syscall(id) => write!(f, "syscall.{}", id),
            MasmOp::Add => f.write_str("add"),
            MasmOp::AddImm(imm) => write!(f, "add.{}", imm),
            MasmOp::Sub => f.write_str("sub"),
            MasmOp::SubImm(imm) => write!(f, "sub.{}", imm),
            MasmOp::Mul => f.write_str("mul"),
            MasmOp::MulImm(imm) => write!(f, "mul.{}", imm),
            MasmOp::Div => f.write_str("div"),
            MasmOp::DivImm(imm) => write!(f, "div.{}", imm),
            MasmOp::Neg => f.write_str("neg"),
            MasmOp::Inv => f.write_str("inv"),
            MasmOp::Incr => f.write_str("incr"),
            MasmOp::Pow2 => f.write_str("pow2"),
            MasmOp::Exp => f.write_str("exp.u64"),
            MasmOp::ExpImm(imm) => write!(f, "exp.{}", imm),
            MasmOp::Not => f.write_str("not"),
            MasmOp::And => f.write_str("and"),
            MasmOp::AndImm(imm) => write!(f, "and.{}", imm),
            MasmOp::Or => f.write_str("or"),
            MasmOp::OrImm(imm) => write!(f, "or.{}", imm),
            MasmOp::Xor => f.write_str("xor"),
            MasmOp::XorImm(imm) => write!(f, "xor.{}", imm),
            MasmOp::Eq => f.write_str("eq"),
            MasmOp::EqImm(imm) => write!(f, "eq.{}", imm),
            MasmOp::Neq => f.write_str("neq"),
            MasmOp::NeqImm(imm) => write!(f, "neq.{}", imm),
            MasmOp::Gt => f.write_str("gt"),
            MasmOp::GtImm(imm) => write!(f, "gt.{}", imm),
            MasmOp::Gte => f.write_str("gte"),
            MasmOp::GteImm(imm) => write!(f, "gte.{}", imm),
            MasmOp::Lt => f.write_str("lt"),
            MasmOp::LtImm(imm) => write!(f, "lt.{}", imm),
            MasmOp::Lte => f.write_str("lte"),
            MasmOp::LteImm(imm) => write!(f, "lte.{}", imm),
            MasmOp::IsOdd => f.write_str("is_odd"),
            MasmOp::Eqw => f.write_str("eqw"),
            MasmOp::Clk => f.write_str("clk"),
            MasmOp::U32Test => f.write_str("u32.test"),
            MasmOp::U32Testw => f.write_str("u32.testw"),
            MasmOp::U32Assert => f.write_str("u32.assert"),
            MasmOp::U32Assert2 => f.write_str("u32.assert2"),
            MasmOp::U32Assertw => f.write_str("u32.assertw"),
            MasmOp::U32Cast => f.write_str("u23.cast"),
            MasmOp::U32Split => f.write_str("u32.split"),
            MasmOp::U32CheckedAdd => f.write_str("u32.add.checked"),
            MasmOp::U32CheckedAddImm(imm) => write!(f, "u32.add.checked.{:#0x}", imm),
            MasmOp::U32OverflowingAdd => f.write_str("u32.add.overflowing"),
            MasmOp::U32OverflowingAddImm(imm) => write!(f, "u32.add.overflowing.{:#0x}", imm),
            MasmOp::U32WrappingAdd => f.write_str("u32.add.wrapping"),
            MasmOp::U32WrappingAddImm(imm) => write!(f, "u32.add.wrapping.{:#0x}", imm),
            MasmOp::U32OverflowingAdd3 => f.write_str("u32.add3.overflowing"),
            MasmOp::U32WrappingAdd3 => f.write_str("u32.add3.wrapping"),
            MasmOp::U32CheckedSub => f.write_str("u32.sub.checked"),
            MasmOp::U32CheckedSubImm(imm) => write!(f, "u32.sub.checked.{:#0x}", imm),
            MasmOp::U32OverflowingSub => f.write_str("u32.sub.overflowing"),
            MasmOp::U32OverflowingSubImm(imm) => write!(f, "u32.sub.overflowing.{:#0x}", imm),
            MasmOp::U32WrappingSub => f.write_str("u32.sub.wrapping"),
            MasmOp::U32WrappingSubImm(imm) => write!(f, "u32.sub.wrapping.{:#0x}", imm),
            MasmOp::U32CheckedMul => f.write_str("u32.mul.checked"),
            MasmOp::U32CheckedMulImm(imm) => write!(f, "u32.mul.checked.{:#0x}", imm),
            MasmOp::U32OverflowingMul => f.write_str("u32.mul.overflowing"),
            MasmOp::U32OverflowingMulImm(imm) => write!(f, "u32.mul.overflowing.{:#0x}", imm),
            MasmOp::U32WrappingMul => f.write_str("u32.mul.wrapping"),
            MasmOp::U32WrappingMulImm(imm) => write!(f, "u32.mul.wrapping.{:#0x}", imm),
            MasmOp::U32OverflowingMadd => f.write_str("u32.madd.overflowing"),
            MasmOp::U32WrappingMadd => f.write_str("u32.madd.wrapping"),
            MasmOp::U32CheckedDiv => f.write_str("u32.div.checked"),
            MasmOp::U32CheckedDivImm(imm) => write!(f, "u32.div.checked.{:#0x}", imm),
            MasmOp::U32UncheckedDiv => f.write_str("u32.div.unchecked"),
            MasmOp::U32UncheckedDivImm(imm) => write!(f, "u32.div.unchecked.{:#0x}", imm),
            MasmOp::U32CheckedMod => f.write_str("u32.mod.checked"),
            MasmOp::U32CheckedModImm(imm) => write!(f, "u32.mod.unchecked.{:#0x}", imm),
            MasmOp::U32UncheckedMod => f.write_str("u32.mod.unchecked"),
            MasmOp::U32UncheckedModImm(imm) => write!(f, "u32.mod.unchecked.{:#0x}", imm),
            MasmOp::U32CheckedDivMod => f.write_str("u32.divmod.checked"),
            MasmOp::U32CheckedDivModImm(imm) => write!(f, "u32.divmod.checked.{:#0x}", imm),
            MasmOp::U32UncheckedDivMod => f.write_str("u32.divmod.unchecked"),
            MasmOp::U32UncheckedDivModImm(imm) => write!(f, "u32.divmod.unchecked.{:#0x}", imm),
            MasmOp::U32And => f.write_str("u32.and"),
            MasmOp::U32Or => f.write_str("u32.or"),
            MasmOp::U32Xor => f.write_str("u32.xor"),
            MasmOp::U32Not => f.write_str("u32.not"),
            MasmOp::U32CheckedShl => f.write_str("u32.shl.checked"),
            MasmOp::U32CheckedShlImm(imm) => write!(f, "u32.shl.checked.{}", imm),
            MasmOp::U32UncheckedShl => f.write_str("u32.shl.unchecked"),
            MasmOp::U32UncheckedShlImm(imm) => write!(f, "u32.shl.unchecked.{}", imm),
            MasmOp::U32CheckedShr => f.write_str("u32.shr.checked"),
            MasmOp::U32CheckedShrImm(imm) => write!(f, "u32.shr.checked.{}", imm),
            MasmOp::U32UncheckedShr => f.write_str("u32.shr.unchecked"),
            MasmOp::U32UncheckedShrImm(imm) => write!(f, "u32.shr.unchecked.{}", imm),
            MasmOp::U32CheckedRotl => f.write_str("u32.rotl.checked"),
            MasmOp::U32CheckedRotlImm(imm) => write!(f, "u32.rotl.checked.{}", imm),
            MasmOp::U32UncheckedRotl => f.write_str("u32.rotl.unchecked"),
            MasmOp::U32UncheckedRotlImm(imm) => write!(f, "u32.rotl.unchecked.{}", imm),
            MasmOp::U32CheckedRotr => f.write_str("u32.rotr.checked"),
            MasmOp::U32CheckedRotrImm(imm) => write!(f, "u32.rotr.checked.{}", imm),
            MasmOp::U32UncheckedRotr => f.write_str("u32.rotr.unchecked"),
            MasmOp::U32UncheckedRotrImm(imm) => write!(f, "u32.rotr.unchecked.{}", imm),
            MasmOp::U32CheckedPopcnt => f.write_str("u32.popcnt.checked"),
            MasmOp::U32UncheckedPopcnt => f.write_str("u32.popcnt.unchecked"),
            MasmOp::U32Eq => f.write_str("u32.eq"),
            MasmOp::U32EqImm(imm) => write!(f, "u32.eq.{:#0x}", imm),
            MasmOp::U32Neq => f.write_str("u32.neq"),
            MasmOp::U32NeqImm(imm) => write!(f, "u32.neq.{:#0x}", imm),
            MasmOp::U32CheckedLt => f.write_str("u32.lt.checked"),
            MasmOp::U32UncheckedLt => f.write_str("u32.lt.unchecked"),
            MasmOp::U32CheckedLte => f.write_str("u32.lte.checked"),
            MasmOp::U32UncheckedLte => f.write_str("u32.lte.unchecked"),
            MasmOp::U32CheckedGt => f.write_str("u32.gt.checked"),
            MasmOp::U32UncheckedGt => f.write_str("u32.gt.unchecked"),
            MasmOp::U32CheckedGte => f.write_str("u32.gte.checked"),
            MasmOp::U32UncheckedGte => f.write_str("u32.gte.unchecked"),
            MasmOp::U32CheckedMin => f.write_str("u32.min.checked"),
            MasmOp::U32UncheckedMin => f.write_str("u32.min.unchecked"),
            MasmOp::U32CheckedMax => f.write_str("u32.max.checked"),
            MasmOp::U32UncheckedMax => f.write_str("u32.max.unchecked"),
        }
    }
}

pub trait StackElement: Clone + fmt::Debug {
    /// A value of this type which represents the "zero" value for the type
    const DEFAULT: Self;
}
impl StackElement for Felt {
    const DEFAULT: Self = Felt::ZERO;
}
impl StackElement for Type {
    const DEFAULT: Self = Type::Felt;
}

pub trait Stack: std::ops::IndexMut<usize, Output = <Self as Stack>::Element> {
    type Element: StackElement;

    fn storage(&self) -> &Vec<Self::Element>;
    fn storage_mut(&mut self) -> &mut Vec<Self::Element>;

    /// Display this stack using its debugging representation
    fn display(&self) -> DebugStack<Self> {
        DebugStack(self)
    }

    /// Returns true if the operand stack is empty
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.storage().is_empty()
    }

    /// Returns the number of elements on the stack
    #[inline]
    fn len(&self) -> usize {
        self.storage().len()
    }

    /// Returns the value on top of the stack, without consuming it
    #[inline]
    fn peek(&self) -> Self::Element {
        self.storage()
            .last()
            .cloned()
            .expect("operand stack is empty")
    }

    /// Returns the word on top of the stack, without consuming it
    #[inline]
    fn peekw(&self) -> [Self::Element; 4] {
        let stack = self.storage();
        let end = stack.len().checked_sub(1).expect("operand stack is empty");
        [
            stack[end].clone(),
            stack[end - 1].clone(),
            stack[end - 2].clone(),
            stack[end - 3].clone(),
        ]
    }

    /// Pushes a word of zeroes on top of the stack
    fn padw(&mut self) {
        self.storage_mut().extend([
            Self::Element::DEFAULT,
            Self::Element::DEFAULT,
            Self::Element::DEFAULT,
            Self::Element::DEFAULT,
        ]);
    }

    /// Pushes `value` on top of the stac
    fn push(&mut self, value: Self::Element) {
        self.storage_mut().push(value);
    }

    /// Pushes `word` on top of the stack
    fn pushw(&mut self, word: [Self::Element; 4]) {
        let stack = self.storage_mut();
        for value in word.into_iter().rev() {
            stack.push(value);
        }
    }

    /// Pops the value on top of the stack
    fn pop(&mut self) -> Option<Self::Element> {
        self.storage_mut().pop()
    }

    /// Pops the first word on top of the stack
    fn popw(&mut self) -> Option<[Self::Element; 4]> {
        let stack = self.storage_mut();
        let a = stack.pop()?;
        let b = stack.pop()?;
        let c = stack.pop()?;
        let d = stack.pop()?;
        Some([a, b, c, d])
    }

    /// Drops the top item on the stack
    fn drop(&mut self) {
        self.dropn(1);
    }

    /// Drops the top word on the stack
    fn dropw(&mut self) {
        self.dropn(4);
    }

    #[inline]
    fn dropn(&mut self, n: usize) {
        let stack = self.storage_mut();
        let len = stack.len();
        assert!(
            n <= len,
            "unable to drop {} elements, operand stack only has {}",
            n,
            len
        );
        stack.truncate(len - n);
    }

    /// Duplicates the value in the `n`th position on the stack
    ///
    /// If `n` is 0, duplicates the top of the stack.
    fn dup(&mut self, n: usize) {
        let value = self[n].clone();
        self.storage_mut().push(value);
    }

    /// Duplicates the `n`th word on the stack, to the top of the stack.
    ///
    /// Valid values for `n` are 0, 1, 2, or 3.
    ///
    /// If `n` is 0, duplicates the top word of the stack.
    fn dupw(&mut self, n: usize) {
        assert!(n < 4, "invalid word index: must be in the range 0..=3");
        let len = self.storage().len();
        let index = n * 4;
        assert!(
            index < len,
            "invalid operand stack index ({}), only {} elements are available",
            index,
            len
        );
        match index {
            0 => {
                let word = self.peekw();
                self.pushw(word);
            }
            n => {
                let end = len - n - 1;
                let word = {
                    let stack = self.storage();
                    [
                        stack[end].clone(),
                        stack[end - 1].clone(),
                        stack[end - 2].clone(),
                        stack[end - 3].clone(),
                    ]
                };
                self.pushw(word);
            }
        }
    }

    /// Swaps the `n`th value from the top of the stack, with the top of the stack
    ///
    /// If `n` is 1, it swaps the first two elements on the stack.
    ///
    /// NOTE: This function will panic if `n` is 0, or out of bounds.
    fn swap(&mut self, n: usize) {
        assert_ne!(n, 0, "invalid swap, index must be in the range 1..=15");
        let stack = self.storage_mut();
        let len = stack.len();
        assert!(
            n < len,
            "invalid operand stack index ({}), only {} elements are available",
            n,
            len
        );
        let a = len - 1;
        let b = a - n;
        stack.swap(a, b);
    }

    /// Swaps the `n`th word from the top of the stack, with the word on top of the stack
    ///
    /// If `n` is 1, it swaps the first two words on the stack.
    ///
    /// Valid values for `n` are: 1, 2, 3.
    fn swapw(&mut self, n: usize) {
        assert_ne!(n, 0, "invalid swap, index must be in the range 1..=3");
        let stack = self.storage_mut();
        let len = stack.len();
        let index = n * 4;
        assert!(
            index < len,
            "invalid operand stack index ({}), only {} elements are available",
            index,
            len
        );
        for offset in 0..4 {
            // The index of the element in the top word
            let a = len - 1 - offset;
            // The index of the element in the `n`th word
            let b = len - index - offset;
            stack.swap(a, b);
        }
    }

    /// Moves the `n`th value to the top of the stack
    ///
    /// If `n` is 1, this is equivalent to `swap(1)`.
    ///
    /// NOTE: This function will panic if `n` is 0, or out of bounds.
    fn movup(&mut self, n: usize) {
        assert_ne!(n, 0, "invalid move, index must be in the range 1..=15");
        let stack = self.storage_mut();
        let len = stack.len();
        assert!(
            n < len,
            "invalid operand stack index ({}), only {} elements are available",
            n,
            len
        );
        // Pick the midpoint by counting backwards from the end
        let end = len - 1;
        let mid = end - n;
        // Split the stack, and rotate the half that
        // contains our desired value to place it on top.
        let (_, r) = stack.split_at_mut(mid);
        r.rotate_left(1);
    }

    /// Moves the `n`th word to the top of the stack
    ///
    /// If `n` is 1, this is equivalent to `swapw(1)`.
    ///
    /// Valid values for `n` are: 1, 2, 3
    fn movupw(&mut self, n: usize) {
        assert_ne!(n, 0, "invalid move, index must be in the range 1..=3");
        let stack = self.storage_mut();
        let len = stack.len();
        let index = n * 4;
        let last_index = index - 4;
        assert!(
            last_index < len,
            "invalid operand stack index ({}), only {} elements are available",
            last_index,
            len
        );
        // Pick the midpoint by counting backwards from the end
        let end = len - 1;
        let mid = end - last_index;
        // Split the stack, and rotate the half that
        // contains our desired word to place it on top.
        let (_, r) = stack.split_at_mut(mid);
        r.rotate_left(4);
    }

    /// Makes the value on top of the stack, the `n`th value on the stack
    ///
    /// If `n` is 1, this is equivalent to `swap(1)`.
    ///
    /// NOTE: This function will panic if `n` is 0, or out of bounds.
    fn movdn(&mut self, n: usize) {
        assert_ne!(n, 0, "invalid move, index must be in the range 1..=15");
        let stack = self.storage_mut();
        let len = stack.len();
        assert!(
            n < len,
            "invalid operand stack index ({}), only {} elements are available",
            n,
            len
        );
        // Split the stack so that the desired position is in the top half
        let end = len - 1;
        let mid = end - n;
        let (_, r) = stack.split_at_mut(mid);
        // Move all elements above the `n`th position up by one, moving the top element to the `n`th position
        r.rotate_right(1);
    }

    /// Makes the word on top of the stack, the `n`th word on the stack
    ///
    /// If `n` is 1, this is equivalent to `swapw(1)`.
    ///
    /// Valid values for `n` are: 1, 2, 3
    fn movdnw(&mut self, n: usize) {
        assert_ne!(n, 0, "invalid move, index must be in the range 1..=3");
        let stack = self.storage_mut();
        let len = stack.len();
        let index = n * 4;
        let last_index = index - 4;
        assert!(
            last_index < len,
            "invalid operand stack index ({}), only {} elements are available",
            last_index,
            len
        );
        // Split the stack so that the desired position is in the top half
        let end = len - 1;
        let mid = end - last_index;
        let (_, r) = stack.split_at_mut(mid);
        // Move all elements above the `n`th word up by one word, moving the top word to the `n`th position
        r.rotate_right(4);
    }
}

/// This structure emulates the Miden VM operand stack
pub struct OperandStack<T> {
    stack: Vec<T>,
}
impl<T: Clone> Clone for OperandStack<T> {
    fn clone(&self) -> Self {
        Self {
            stack: self.stack.clone(),
        }
    }
}
impl<T> Default for OperandStack<T> {
    fn default() -> Self {
        Self { stack: vec![] }
    }
}
impl<T: StackElement> Stack for OperandStack<T> {
    type Element = T;

    #[inline(always)]
    fn storage(&self) -> &Vec<Self::Element> {
        &self.stack
    }
    #[inline(always)]
    fn storage_mut(&mut self) -> &mut Vec<Self::Element> {
        &mut self.stack
    }

    fn padw(&mut self) {
        self.stack.extend([<T as StackElement>::DEFAULT; 4]);
    }
}
impl OperandStack<Felt> {
    /// Pushes `value` on top of the stack, with an optional set of aliases
    pub fn push_u8(&mut self, value: u8) {
        self.stack.push(Felt::new(value as u64));
    }

    /// Pushes `value` on top of the stack, with an optional set of aliases
    pub fn push_u16(&mut self, value: u16) {
        self.stack.push(Felt::new(value as u64));
    }

    /// Pushes `value` on top of the stack, with an optional set of aliases
    pub fn push_u32(&mut self, value: u32) {
        self.stack.push(Felt::new(value as u64));
    }
}
impl<T: StackElement> std::ops::Index<usize> for OperandStack<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let len = self.stack.len();
        assert!(
            index < 16,
            "invalid operand stack index ({}), only the top 16 elements are directly accessible",
            index
        );
        assert!(
            index < len,
            "invalid operand stack index ({}), only {} elements are available",
            index,
            len
        );
        &self.stack[len - index - 1]
    }
}
impl<T: StackElement> std::ops::IndexMut<usize> for OperandStack<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let len = self.stack.len();
        assert!(
            index < 16,
            "invalid operand stack index ({}), only the top 16 elements are directly accessible",
            index
        );
        assert!(
            index < len,
            "invalid operand stack index ({}), only {} elements are available",
            index,
            len
        );
        &mut self.stack[len - index - 1]
    }
}

pub struct DebugStack<'a, T: ?Sized + Stack>(&'a T);
impl<'a, T: ?Sized + Stack> fmt::Debug for DebugStack<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[derive(Debug)]
        #[allow(unused)]
        struct StackEntry<'a, E: fmt::Debug> {
            index: usize,
            value: &'a E,
        }

        f.debug_list()
            .entries(
                self.0
                    .storage()
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(index, value)| StackEntry { index, value }),
            )
            .finish()
    }
}
