use std::fmt;

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

/// This implementation displays the opcode name for the given [MasmOp]
impl fmt::Display for MasmOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Padw => f.write_str("padw"),
            Self::Push(_)
            | Self::Pushw(_)
            | Self::PushU8(_)
            | Self::PushU16(_)
            | Self::PushU32(_) => f.write_str("push"),
            Self::Drop => f.write_str("drop"),
            Self::Dropw => f.write_str("dropw"),
            Self::Dup(_) => f.write_str("dup"),
            Self::Dupw(_) => f.write_str("dupw"),
            Self::Swap(_) => f.write_str("swap"),
            Self::Swapw(_) => f.write_str("swapw"),
            Self::Movup(_) => f.write_str("movup"),
            Self::Movupw(_) => f.write_str("movupw"),
            Self::Movdn(_) => f.write_str("movdn"),
            Self::Movdnw(_) => f.write_str("movdnw"),
            Self::Cswap => f.write_str("cswap"),
            Self::Cswapw => f.write_str("cswapw"),
            Self::Cdrop => f.write_str("cdrop"),
            Self::Cdropw => f.write_str("cdropw"),
            Self::Assert => f.write_str("assert"),
            Self::Assertz => f.write_str("assertz"),
            Self::AssertEq => f.write_str("assert_eq"),
            Self::AssertEqw => f.write_str("assert_eqw"),
            Self::LocAddr(_) => f.write_str("locaddr"),
            Self::MemLoad
            | Self::MemLoadOffset
            | Self::MemLoadImm(_)
            | Self::MemLoadOffsetImm(_, _) => f.write_str("mem_load"),
            Self::MemLoadw | Self::MemLoadwImm(_) => f.write_str("mem_loadw"),
            Self::MemStore
            | Self::MemStoreOffset
            | Self::MemStoreImm(_)
            | Self::MemStoreOffsetImm(_, _) => f.write_str("mem_store"),
            Self::MemStorew | Self::MemStorewImm(_) => f.write_str("mem_storew"),
            Self::If(_, _) => f.write_str("if.true"),
            Self::While(_) => f.write_str("while.true"),
            Self::Repeat(_, _) => f.write_str("repeat"),
            Self::Exec(_) => f.write_str("exec"),
            Self::Syscall(_) => f.write_str("syscall"),
            Self::Add | Self::AddImm(_) => f.write_str("add"),
            Self::Sub | Self::SubImm(_) => f.write_str("sub"),
            Self::Mul | Self::MulImm(_) => f.write_str("mul"),
            Self::Div | Self::DivImm(_) => f.write_str("div"),
            Self::Neg => f.write_str("neg"),
            Self::Inv => f.write_str("inv"),
            Self::Incr => f.write_str("incr"),
            Self::Pow2 => f.write_str("pow2"),
            Self::Exp | Self::ExpImm(_) => f.write_str("exp.u64"),
            Self::Not => f.write_str("not"),
            Self::And | Self::AndImm(_) => f.write_str("and"),
            Self::Or | Self::OrImm(_) => f.write_str("or"),
            Self::Xor | Self::XorImm(_) => f.write_str("xor"),
            Self::Eq | Self::EqImm(_) => f.write_str("eq"),
            Self::Neq | Self::NeqImm(_) => f.write_str("neq"),
            Self::Gt | Self::GtImm(_) => f.write_str("gt"),
            Self::Gte | Self::GteImm(_) => f.write_str("gte"),
            Self::Lt | Self::LtImm(_) => f.write_str("lt"),
            Self::Lte | Self::LteImm(_) => f.write_str("lte"),
            Self::IsOdd => f.write_str("is_odd"),
            Self::Eqw => f.write_str("eqw"),
            Self::Clk => f.write_str("clk"),
            Self::U32Test => f.write_str("u32.test"),
            Self::U32Testw => f.write_str("u32.testw"),
            Self::U32Assert => f.write_str("u32.assert"),
            Self::U32Assert2 => f.write_str("u32.assert2"),
            Self::U32Assertw => f.write_str("u32.assertw"),
            Self::U32Cast => f.write_str("u23.cast"),
            Self::U32Split => f.write_str("u32.split"),
            Self::U32CheckedAdd | Self::U32CheckedAddImm(_) => f.write_str("u32.add.checked"),
            Self::U32OverflowingAdd | Self::U32OverflowingAddImm(_) => {
                f.write_str("u32.add.overflowing")
            }
            Self::U32WrappingAdd | Self::U32WrappingAddImm(_) => f.write_str("u32.add.wrapping"),
            Self::U32OverflowingAdd3 => f.write_str("u32.add3.overflowing"),
            Self::U32WrappingAdd3 => f.write_str("u32.add3.wrapping"),
            Self::U32CheckedSub | Self::U32CheckedSubImm(_) => f.write_str("u32.sub.checked"),
            Self::U32OverflowingSub | Self::U32OverflowingSubImm(_) => {
                f.write_str("u32.sub.overflowing")
            }
            Self::U32WrappingSub | Self::U32WrappingSubImm(_) => f.write_str("u32.sub.wrapping"),
            Self::U32CheckedMul | Self::U32CheckedMulImm(_) => f.write_str("u32.mul.checked"),
            Self::U32OverflowingMul | Self::U32OverflowingMulImm(_) => {
                f.write_str("u32.mul.overflowing")
            }
            Self::U32WrappingMul | Self::U32WrappingMulImm(_) => f.write_str("u32.mul.wrapping"),
            Self::U32OverflowingMadd => f.write_str("u32.madd.overflowing"),
            Self::U32WrappingMadd => f.write_str("u32.madd.wrapping"),
            Self::U32CheckedDiv | Self::U32CheckedDivImm(_) => f.write_str("u32.div.checked"),
            Self::U32UncheckedDiv | Self::U32UncheckedDivImm(_) => f.write_str("u32.div.unchecked"),
            Self::U32CheckedMod | Self::U32CheckedModImm(_) => f.write_str("u32.mod.checked"),
            Self::U32UncheckedMod | Self::U32UncheckedModImm(_) => f.write_str("u32.mod.unchecked"),
            Self::U32CheckedDivMod | Self::U32CheckedDivModImm(_) => {
                f.write_str("u32.divmod.checked")
            }
            Self::U32UncheckedDivMod | Self::U32UncheckedDivModImm(_) => {
                f.write_str("u32.divmod.unchecked")
            }
            Self::U32And => f.write_str("u32.and"),
            Self::U32Or => f.write_str("u32.or"),
            Self::U32Xor => f.write_str("u32.xor"),
            Self::U32Not => f.write_str("u32.not"),
            Self::U32CheckedShl | Self::U32CheckedShlImm(_) => f.write_str("u32.shl.checked"),
            Self::U32UncheckedShl | Self::U32UncheckedShlImm(_) => f.write_str("u32.shl.unchecked"),
            Self::U32CheckedShr | Self::U32CheckedShrImm(_) => f.write_str("u32.shr.checked"),
            Self::U32UncheckedShr | Self::U32UncheckedShrImm(_) => f.write_str("u32.shr.unchecked"),
            Self::U32CheckedRotl | Self::U32CheckedRotlImm(_) => f.write_str("u32.rotl.checked"),
            Self::U32UncheckedRotl | Self::U32UncheckedRotlImm(_) => {
                f.write_str("u32.rotl.unchecked")
            }
            Self::U32CheckedRotr | Self::U32CheckedRotrImm(_) => f.write_str("u32.rotr.checked"),
            Self::U32UncheckedRotr | Self::U32UncheckedRotrImm(_) => {
                f.write_str("u32.rotr.unchecked")
            }
            Self::U32CheckedPopcnt => f.write_str("u32.popcnt.checked"),
            Self::U32UncheckedPopcnt => f.write_str("u32.popcnt.unchecked"),
            Self::U32Eq | Self::U32EqImm(_) => f.write_str("u32.eq"),
            Self::U32Neq | Self::U32NeqImm(_) => f.write_str("u32.neq"),
            Self::U32CheckedLt => f.write_str("u32.lt.checked"),
            Self::U32UncheckedLt => f.write_str("u32.lt.unchecked"),
            Self::U32CheckedLte => f.write_str("u32.lte.checked"),
            Self::U32UncheckedLte => f.write_str("u32.lte.unchecked"),
            Self::U32CheckedGt => f.write_str("u32.gt.checked"),
            Self::U32UncheckedGt => f.write_str("u32.gt.unchecked"),
            Self::U32CheckedGte => f.write_str("u32.gte.checked"),
            Self::U32UncheckedGte => f.write_str("u32.gte.unchecked"),
            Self::U32CheckedMin => f.write_str("u32.min.checked"),
            Self::U32UncheckedMin => f.write_str("u32.min.unchecked"),
            Self::U32CheckedMax => f.write_str("u32.max.checked"),
            Self::U32UncheckedMax => f.write_str("u32.max.unchecked"),
        }
    }
}
