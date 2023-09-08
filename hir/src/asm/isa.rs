use cranelift_entity::entity_impl;

use crate::{Felt, FunctionIdent, LocalId};

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
