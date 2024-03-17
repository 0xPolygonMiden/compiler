use std::fmt;

use cranelift_entity::entity_impl;
use rustc_hash::FxHashMap;
use smallvec::{smallvec, SmallVec};

use crate::{Felt, FunctionIdent, Ident, LocalId};

/// A handle that refers to a MASM code block
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MasmBlockId(u32);
entity_impl!(MasmBlockId, "blk");

/// Represents a single code block in Miden Assembly
#[derive(Debug, Clone, PartialEq)]
pub struct MasmBlock {
    pub id: MasmBlockId,
    pub ops: SmallVec<[MasmOp; 4]>,
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

    /// Append `n` copies of `op` to the current block
    #[inline]
    pub fn push_n(&mut self, count: usize, op: MasmOp) {
        for _ in 0..count {
            self.ops.push(op);
        }
    }

    /// Append `n` copies of the sequence `ops` to this block
    #[inline]
    pub fn push_repeat(&mut self, ops: &[MasmOp], count: usize) {
        for _ in 0..count {
            self.ops.extend_from_slice(ops);
        }
    }

    /// Append `n` copies of the sequence `ops` to this block
    #[inline]
    pub fn push_template<const N: usize, F>(&mut self, count: usize, template: F)
    where
        F: Fn(usize) -> [MasmOp; N],
    {
        for n in 0..count {
            self.ops.extend_from_slice(&template(n));
        }
    }

    /// Appends instructions from `slice` to the end of this block
    #[inline]
    pub fn extend_from_slice(&mut self, slice: &[MasmOp]) {
        self.ops.extend_from_slice(slice);
    }

    /// Appends instructions from `slice` to the end of this block
    #[inline]
    pub fn extend(&mut self, ops: impl IntoIterator<Item = MasmOp>) {
        self.ops.extend(ops);
    }

    /// Appends instructions from `other` to the end of this block
    #[inline]
    pub fn append<B>(&mut self, other: &mut SmallVec<B>)
    where
        B: smallvec::Array<Item = MasmOp>,
    {
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
    /// Pushes a pair of field elements on top of the stack
    Push2([Felt; 2]),
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
    /// Pops a value off the stack and asserts that it is equal to 1, raising the given error code
    AssertWithError(u32),
    /// Pops a value off the stack and asserts that it is equal to 0
    Assertz,
    /// Pops a value off the stack and asserts that it is equal to 0, raising the given error code
    AssertzWithError(u32),
    /// Pops two values off the stack and asserts that they are equal
    AssertEq,
    /// Pops two values off the stack and asserts that they are equal, raising the given error code
    AssertEqWithError(u32),
    /// Pops two words off the stack and asserts that they are equal
    AssertEqw,
    /// Pops two words off the stack and asserts that they are equal, raising the given error code
    AssertEqwWithError(u32),
    /// Places the memory address of the given local index on top of the stack
    LocAddr(LocalId),
    /// Writes a value to the first element of the word at the address corresponding to the given
    /// local index
    LocStore(LocalId),
    /// Writes a word to the address corresponding to the given local index
    LocStorew(LocalId),
    /// Pops `a`, representing a memory address, from the top of the stack, then loads the
    /// first element of the word starting at that address, placing it on top of the stack.
    ///
    /// Traps if `a` >= 2^32
    MemLoad,
    /// Same as above, but the address is given as an immediate
    MemLoadImm(u32),
    /// Pops `a`, representing a memory address + offset pair, from the top of the stack, then
    /// loads the element at the given offset from the base of the word starting at that
    /// address, placing it on top of the stack.
    ///
    /// Traps if `a` >= 2^32
    ///
    /// NOTE: This instruction doesn't actually exist in Miden Assembly yet, it is a proposed
    /// extension of `MemLoad` which allows addressing all field elements of a word
    /// individually. It is here for testing.
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
    /// Pops `a, v` from the stack, where `a` represents a memory address + offset pair, and `v`
    /// the value to be stored, and stores `v` as the element at the given offset from the base
    /// of the word starting at that address. The remaining elements of the word are not
    /// modified.
    ///
    /// Traps if `a` >= 2^32
    ///
    /// NOTE: This instruction doesn't actually exist in Miden Assembly yet, it is a proposed
    /// extension of `MemStore` which allows addressing all field elements of a word
    /// individually. It is here for testing.
    MemStoreOffset,
    /// Same as above, but the address and offset are given as a immediates
    MemStoreOffsetImm(u32, u8),
    /// Pops `a, V` from the stack, where `a` represents a memory address, and `V` is a word to be
    /// stored at that location, and overwrites the word located at `a`.
    ///
    /// Traps if `a` >= 2^32
    MemStorew,
    /// Same as above, but the address is given as an immediate
    MemStorewImm(u32),
    /// Read two sequential words from memory starting at `a`, overwriting the first two words on
    /// the stack, and advancing `a` to the next address following the two that were loaded
    /// [C, B, A, a] <- [*a, *(a + 1), A, a + 2]
    MemStream,
    /// Pops the next two words from the advice stack, overwrites the
    /// top of the operand stack with them, and also writes these words
    /// into memory at `a` and `a + 1`
    ///
    /// [C, B, A, a] <- [*a, *(a + 1), A, a + 2]
    AdvPipe,
    /// Pops `n` elements from the advice stack, and pushes them on the operand stack
    ///
    /// Fails if less than `n` elements are available.
    ///
    /// Valid values of `n` fall in the range 1..=16
    AdvPush(u8),
    /// TODO
    AdvLoadw,
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
    /// Pops `N` args off the stack, executes the procedure in the root context, results will be
    /// placed on the stack
    Syscall(FunctionIdent),
    /// Pops the address (MAST root hash) of a callee off the stack, and dynamically `exec` the
    /// function
    DynExec,
    /// TODO
    DynCall,
    /// Pushes the address (MAST root hash) of the given function on the stack, to be used by
    /// `dynexec` or `dyncall`
    ProcRef(FunctionIdent),
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
    /// Computes the base 2 logarithm of `a`, rounded down, and places it on the advice stack.
    Ilog2,
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
    /// When called via `syscall`, this pushes the hash of the caller's MAST root on the stack
    Caller,
    /// Pushes the current value of the cycle counter (clock) on the stack
    Clk,
    /// Peeks `a` from the top of the stack, and places the 1 on the stack if `a < 2^32`, else 0
    U32Test,
    /// Peeks `A` from the top of the stack, and places the 1 on the stack if `forall a : A, a <
    /// 2^32`, else 0
    U32Testw,
    /// Peeks `a` from the top of the stack, and traps if `a >= 2^32`
    U32Assert,
    /// Peeks `a` from the top of the stack, and traps if `a >= 2^32`, raising the given error code
    U32AssertWithError(u32),
    /// Peeks `b, a` from the top of the stack, and traps if either `a` or `b` is >= 2^32
    U32Assert2,
    /// Peeks `b, a` from the top of the stack, and traps if either `a` or `b` is >= 2^32, raising
    /// the given error code
    U32Assert2WithError(u32),
    /// Peeks `A` from the top of the stack, and traps unless `forall a : A, a < 2^32`, else 0
    U32Assertw,
    /// Peeks `A` from the top of the stack, and traps unless `forall a : A, a < 2^32`, else 0,
    /// raising the given error code
    U32AssertwWithError(u32),
    /// Pops `a` from the top of the stack, and places the result of `a mod 2^32` on the stack
    ///
    /// This is used to cast a field element to the u32 range
    U32Cast,
    /// Pops `a` from the top of the stack, and splits it into upper and lower 32-bit values,
    /// placing them back on the stack. The lower part is calculated as `a mod 2^32`,
    /// and the higher part as `a / 2^32`. The higher part will be on top of the stack after.
    U32Split,
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
    /// This operation traps if `b` is zero.
    ///
    /// This operation is unchecked, so the result is undefined if the operands are not valid u32
    U32Div,
    /// Same as above, except `b` is provided by the immediate
    U32DivImm(u32),
    /// Pops `b, a` off the stack, and pushes `a mod b` on the stack.
    ///
    /// This operation traps if `b` is zero.
    ///
    /// This operation is unchecked, so the result is undefined if the operands are not valid u32
    U32Mod,
    /// Same as above, except `b` is provided by the immediate
    U32ModImm(u32),
    /// Pops `b, a` off the stack, and first pushes `a / b` on the stack, followed by `a mod b`.
    ///
    /// This operation traps if `b` is zero.
    ///
    /// This operation is unchecked, so the result is undefined if the operands are not valid u32
    U32DivMod,
    /// Same as above, except `b` is provided by the immediate
    U32DivModImm(u32),
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
    /// Truncates if the shift would cause overflow.
    ///
    /// This operation is unchecked, so the result is undefined if the operands are not valid u32
    U32Shl,
    /// Same as above, except `b` is provided by the immediate
    U32ShlImm(u32),
    /// Pops `b, a` off the stack, and places the result of `a / 2^b` on the stack.
    ///
    /// Truncates if the shift would cause overflow.
    ///
    /// This operation is unchecked, so the result is undefined if the operands are not valid u32
    U32Shr,
    /// Same as above, except `b` is provided by the immediate
    U32ShrImm(u32),
    /// Pops `b, a` off the stack, and places the result of rotating the 32-bit
    /// representation of `a` to the left by `b` bits.
    ///
    /// This operation is unchecked, so the result is undefined if the operands are not valid u32
    U32Rotl,
    /// Same as above, except `b` is provided by the immediate
    U32RotlImm(u32),
    /// Pops `b, a` off the stack, and places the result of rotating the 32-bit
    /// representation of `a` to the right by `b` bits.
    ///
    /// This operation is unchecked, so the result is undefined if the operands are not valid u32
    U32Rotr,
    /// Same as above, except `b` is provided by the immediate
    U32RotrImm(u32),
    /// Pops `a` off the stack, and places the number of set bits in `a` (it's hamming weight).
    ///
    /// This operation is unchecked, so the result is undefined if the operands are not valid u32
    U32Popcnt,
    /// Computes the number of leading zero bits in `a`, and places it on the advice stack
    ///
    /// This operation is unchecked, so the result is undefined if the operands are not valid u32
    U32Clz,
    /// Computes the number of trailing zero bits in `a`, and places it on the advice stack
    ///
    /// This operation is unchecked, so the result is undefined if the operands are not valid u32
    U32Ctz,
    /// Computes the number of leading one bits in `a`, and places it on the advice stack
    ///
    /// This operation is unchecked, so the result is undefined if the operands are not valid u32
    U32Clo,
    /// Computes the number of trailing one bits in `a`, and places it on the advice stack
    ///
    /// This operation is unchecked, so the result is undefined if the operands are not valid u32
    U32Cto,
    /// Pops `b, a` from the stack, and places 1 on the stack if `a < b`, else 0
    ///
    /// This operation is unchecked, so the result is undefined if the operands are not valid u32
    U32Lt,
    /// Pops `b, a` from the stack, and places 1 on the stack if `a <= b`, else 0
    ///
    /// This operation is unchecked, so the result is undefined if the operands are not valid u32
    U32Lte,
    /// Pops `b, a` from the stack, and places 1 on the stack if `a > b`, else 0
    ///
    /// This operation is unchecked, so the result is undefined if the operands are not valid u32
    U32Gt,
    /// Pops `b, a` from the stack, and places 1 on the stack if `a >= b`, else 0
    ///
    /// This operation is unchecked, so the result is undefined if the operands are not valid u32
    U32Gte,
    /// Pops `b, a` from the stack, and places `a` back on the stack if `a < b`, else `b`
    ///
    /// This operation is unchecked, so the result is undefined if the operands are not valid u32
    U32Min,
    /// Pops `b, a` from the stack, and places `a` back on the stack if `a > b`, else `b`
    ///
    /// This operation is unchecked, so the result is undefined if the operands are not valid u32
    U32Max,
}
impl MasmOp {
    pub fn has_regions(&self) -> bool {
        matches!(self, Self::If(_, _) | Self::While(_) | Self::Repeat(_, _))
    }

    pub fn from_masm(
        ix: miden_assembly::ast::Instruction,
        locals: &[FunctionIdent],
        imported: &miden_assembly::ast::ModuleImports,
    ) -> SmallVec<[Self; 2]> {
        use miden_assembly::ast::Instruction;

        use crate::{StarkField, Symbol};

        let op = match ix {
            Instruction::Assert => Self::Assert,
            Instruction::AssertWithError(code) => Self::AssertWithError(code),
            Instruction::AssertEq => Self::AssertEq,
            Instruction::AssertEqWithError(code) => Self::AssertEqWithError(code),
            Instruction::AssertEqw => Self::AssertEqw,
            Instruction::AssertEqwWithError(code) => Self::AssertEqwWithError(code),
            Instruction::Assertz => Self::Assertz,
            Instruction::AssertzWithError(code) => Self::AssertzWithError(code),
            Instruction::Add => Self::Add,
            Instruction::AddImm(imm) => Self::AddImm(imm),
            Instruction::Sub => Self::Sub,
            Instruction::SubImm(imm) => Self::SubImm(imm),
            Instruction::Mul => Self::Mul,
            Instruction::MulImm(imm) => Self::MulImm(imm),
            Instruction::Div => Self::Div,
            Instruction::DivImm(imm) => Self::DivImm(imm),
            Instruction::Neg => Self::Neg,
            Instruction::Inv => Self::Inv,
            Instruction::Incr => Self::Incr,
            //Instruction::Ilog2 => Self::ILog2,
            Instruction::Pow2 => Self::Pow2,
            Instruction::Exp => Self::Exp,
            Instruction::ExpImm(imm) => {
                Self::ExpImm(imm.as_int().try_into().expect("invalid exponent"))
            }
            Instruction::ExpBitLength(imm) => Self::ExpImm(imm),
            Instruction::Not => Self::Not,
            Instruction::And => Self::And,
            Instruction::Or => Self::Or,
            Instruction::Xor => Self::Xor,
            Instruction::Eq => Self::Eq,
            Instruction::EqImm(imm) => Self::EqImm(imm),
            Instruction::Neq => Self::Neq,
            Instruction::NeqImm(imm) => Self::NeqImm(imm),
            Instruction::Eqw => Self::Eqw,
            Instruction::Lt => Self::Lt,
            Instruction::Lte => Self::Lte,
            Instruction::Gt => Self::Gt,
            Instruction::Gte => Self::Gte,
            Instruction::IsOdd => Self::IsOdd,
            Instruction::Ext2Add
            | Instruction::Ext2Sub
            | Instruction::Ext2Mul
            | Instruction::Ext2Div
            | Instruction::Ext2Neg
            | Instruction::Ext2Inv => unimplemented!(),
            Instruction::U32Test => Self::U32Test,
            Instruction::U32TestW => Self::U32Testw,
            Instruction::U32Assert => Self::U32Assert,
            Instruction::U32AssertWithError(code) => Self::U32AssertWithError(code),
            Instruction::U32Assert2 => Self::U32Assert2,
            Instruction::U32Assert2WithError(code) => Self::U32Assert2WithError(code),
            Instruction::U32AssertW => Self::U32Assertw,
            Instruction::U32AssertWWithError(code) => Self::U32AssertwWithError(code),
            Instruction::U32Split => Self::U32Split,
            Instruction::U32Cast => Self::U32Cast,
            Instruction::U32WrappingAdd => Self::U32WrappingAdd,
            Instruction::U32WrappingAddImm(imm) => Self::U32WrappingAddImm(imm),
            Instruction::U32OverflowingAdd => Self::U32OverflowingAdd,
            Instruction::U32OverflowingAddImm(imm) => Self::U32OverflowingAddImm(imm),
            Instruction::U32OverflowingAdd3 => Self::U32OverflowingAdd3,
            Instruction::U32WrappingAdd3 => Self::U32WrappingAdd3,
            Instruction::U32WrappingSub => Self::U32WrappingSub,
            Instruction::U32WrappingSubImm(imm) => Self::U32WrappingSubImm(imm),
            Instruction::U32OverflowingSub => Self::U32OverflowingSub,
            Instruction::U32OverflowingSubImm(imm) => Self::U32OverflowingSubImm(imm),
            Instruction::U32WrappingMul => Self::U32WrappingMul,
            Instruction::U32WrappingMulImm(imm) => Self::U32WrappingMulImm(imm),
            Instruction::U32OverflowingMul => Self::U32OverflowingMul,
            Instruction::U32OverflowingMulImm(imm) => Self::U32OverflowingMulImm(imm),
            Instruction::U32OverflowingMadd => Self::U32OverflowingMadd,
            Instruction::U32WrappingMadd => Self::U32WrappingMadd,
            Instruction::U32Div => Self::U32Div,
            Instruction::U32DivImm(imm) => Self::U32DivImm(imm),
            Instruction::U32Mod => Self::U32Mod,
            Instruction::U32ModImm(imm) => Self::U32ModImm(imm),
            Instruction::U32DivMod => Self::U32DivMod,
            Instruction::U32DivModImm(imm) => Self::U32DivModImm(imm),
            Instruction::U32And => Self::U32And,
            Instruction::U32Or => Self::U32Or,
            Instruction::U32Xor => Self::U32Xor,
            Instruction::U32Not => Self::U32Not,
            Instruction::U32Shr => Self::U32Shr,
            Instruction::U32ShrImm(imm) => Self::U32ShrImm(imm as u32),
            Instruction::U32Shl => Self::U32Shl,
            Instruction::U32ShlImm(imm) => Self::U32ShlImm(imm as u32),
            Instruction::U32Rotr => Self::U32Rotr,
            Instruction::U32RotrImm(imm) => Self::U32RotrImm(imm as u32),
            Instruction::U32Rotl => Self::U32Rotl,
            Instruction::U32RotlImm(imm) => Self::U32RotlImm(imm as u32),
            Instruction::U32Popcnt => Self::U32Popcnt,
            //Instruction::U32Clz => Self::U32Clz,
            //Instruction::U32Ctz => Self::U32Ctz,
            //Instruction::U32Clo => Self::U32Clo,
            //Instruction::U32Cto => Self::U32Cto,
            Instruction::U32Lt => Self::U32Lt,
            Instruction::U32Lte => Self::U32Lte,
            Instruction::U32Gt => Self::U32Gt,
            Instruction::U32Gte => Self::U32Gte,
            Instruction::U32Min => Self::U32Min,
            Instruction::U32Max => Self::U32Max,
            Instruction::Drop => Self::Drop,
            Instruction::DropW => Self::Dropw,
            Instruction::PadW => Self::Padw,
            Instruction::Dup0 => Self::Dup(0),
            Instruction::Dup1 => Self::Dup(1),
            Instruction::Dup2 => Self::Dup(2),
            Instruction::Dup3 => Self::Dup(3),
            Instruction::Dup4 => Self::Dup(4),
            Instruction::Dup5 => Self::Dup(5),
            Instruction::Dup6 => Self::Dup(6),
            Instruction::Dup7 => Self::Dup(7),
            Instruction::Dup8 => Self::Dup(8),
            Instruction::Dup9 => Self::Dup(9),
            Instruction::Dup10 => Self::Dup(10),
            Instruction::Dup11 => Self::Dup(11),
            Instruction::Dup12 => Self::Dup(12),
            Instruction::Dup13 => Self::Dup(13),
            Instruction::Dup14 => Self::Dup(14),
            Instruction::Dup15 => Self::Dup(15),
            Instruction::DupW0 => Self::Dupw(0),
            Instruction::DupW1 => Self::Dupw(1),
            Instruction::DupW2 => Self::Dupw(2),
            Instruction::DupW3 => Self::Dupw(3),
            Instruction::Swap1 => Self::Swap(1),
            Instruction::Swap2 => Self::Swap(2),
            Instruction::Swap3 => Self::Swap(3),
            Instruction::Swap4 => Self::Swap(4),
            Instruction::Swap5 => Self::Swap(5),
            Instruction::Swap6 => Self::Swap(6),
            Instruction::Swap7 => Self::Swap(7),
            Instruction::Swap8 => Self::Swap(8),
            Instruction::Swap9 => Self::Swap(9),
            Instruction::Swap10 => Self::Swap(10),
            Instruction::Swap11 => Self::Swap(11),
            Instruction::Swap12 => Self::Swap(12),
            Instruction::Swap13 => Self::Swap(13),
            Instruction::Swap14 => Self::Swap(14),
            Instruction::Swap15 => Self::Swap(15),
            Instruction::SwapW1 => Self::Swapw(1),
            Instruction::SwapW2 => Self::Swapw(2),
            Instruction::SwapW3 => Self::Swapw(3),
            Instruction::SwapDw => unimplemented!("swap double-word"),
            Instruction::MovUp2 => Self::Movup(2),
            Instruction::MovUp3 => Self::Movup(3),
            Instruction::MovUp4 => Self::Movup(4),
            Instruction::MovUp5 => Self::Movup(5),
            Instruction::MovUp6 => Self::Movup(6),
            Instruction::MovUp7 => Self::Movup(7),
            Instruction::MovUp8 => Self::Movup(8),
            Instruction::MovUp9 => Self::Movup(9),
            Instruction::MovUp10 => Self::Movup(10),
            Instruction::MovUp11 => Self::Movup(11),
            Instruction::MovUp12 => Self::Movup(12),
            Instruction::MovUp13 => Self::Movup(13),
            Instruction::MovUp14 => Self::Movup(14),
            Instruction::MovUp15 => Self::Movup(15),
            Instruction::MovUpW2 => Self::Movupw(2),
            Instruction::MovUpW3 => Self::Movupw(3),
            Instruction::MovDn2 => Self::Movdn(2),
            Instruction::MovDn3 => Self::Movdn(3),
            Instruction::MovDn4 => Self::Movdn(4),
            Instruction::MovDn5 => Self::Movdn(5),
            Instruction::MovDn6 => Self::Movdn(6),
            Instruction::MovDn7 => Self::Movdn(7),
            Instruction::MovDn8 => Self::Movdn(8),
            Instruction::MovDn9 => Self::Movdn(9),
            Instruction::MovDn10 => Self::Movdn(10),
            Instruction::MovDn11 => Self::Movdn(11),
            Instruction::MovDn12 => Self::Movdn(12),
            Instruction::MovDn13 => Self::Movdn(13),
            Instruction::MovDn14 => Self::Movdn(14),
            Instruction::MovDn15 => Self::Movdn(15),
            Instruction::MovDnW2 => Self::Movdnw(2),
            Instruction::MovDnW3 => Self::Movdnw(3),
            Instruction::CSwap => Self::Cswap,
            Instruction::CSwapW => Self::Cswapw,
            Instruction::CDrop => Self::Cdrop,
            Instruction::CDropW => Self::Cdropw,
            Instruction::PushU8(elem) => Self::PushU8(elem),
            Instruction::PushU16(elem) => Self::PushU32(elem as u32),
            Instruction::PushU32(elem) => Self::PushU32(elem),
            Instruction::PushFelt(elem) => Self::Push(elem),
            Instruction::PushWord(word) => Self::Pushw(word),
            Instruction::PushU8List(u8s) => return u8s.into_iter().map(Self::PushU8).collect(),
            Instruction::PushU16List(u16s) => {
                return u16s.into_iter().map(|i| Self::PushU32(i as u32)).collect()
            }
            Instruction::PushU32List(u32s) => return u32s.into_iter().map(Self::PushU32).collect(),
            Instruction::PushFeltList(felts) => return felts.into_iter().map(Self::Push).collect(),
            Instruction::Locaddr(id) => {
                Self::LocAddr(LocalId::from_u8(id.try_into().expect("invalid local id")))
            }
            Instruction::LocStore(id) => {
                Self::LocStore(LocalId::from_u8(id.try_into().expect("invalid local id")))
            }
            Instruction::LocStoreW(id) => {
                Self::LocStorew(LocalId::from_u8(id.try_into().expect("invalid local id")))
            }
            Instruction::Clk => Self::Clk,
            Instruction::MemLoad => Self::MemLoad,
            Instruction::MemLoadImm(addr) => Self::MemLoadImm(addr),
            Instruction::MemLoadW => Self::MemLoadw,
            Instruction::MemLoadWImm(addr) => Self::MemLoadwImm(addr),
            Instruction::MemStore => Self::MemStore,
            Instruction::MemStoreImm(addr) => Self::MemStoreImm(addr),
            Instruction::MemStoreW => Self::MemStorew,
            Instruction::MemStoreWImm(addr) => Self::MemStorewImm(addr),
            Instruction::LocLoad(_) | Instruction::LocLoadW(_) => {
                unimplemented!("load by local id")
            }
            Instruction::MemStream => Self::MemStream,
            Instruction::AdvPipe => Self::AdvPipe,
            Instruction::AdvPush(byte) => Self::AdvPush(byte),
            Instruction::AdvLoadW => Self::AdvLoadw,
            Instruction::AdvInject(_) => unimplemented!("adv_inject"),
            Instruction::Hash
            | Instruction::HMerge
            | Instruction::HPerm
            | Instruction::MTreeGet
            | Instruction::MTreeSet
            | Instruction::MTreeMerge
            | Instruction::MTreeVerify => unimplemented!("cryptographic operations"),
            Instruction::ExecLocal(local_index) => Self::Exec(locals[local_index as usize]),
            Instruction::ExecImported(ref proc_id) => {
                let module = imported
                    .get_procedure_path(proc_id)
                    .expect("reference to import that doesn't exist")
                    .last();
                let name = imported
                    .get_procedure_name(proc_id)
                    .expect("reference to import that doesn't exist");
                Self::Exec(FunctionIdent {
                    module: Ident::with_empty_span(Symbol::intern(module)),
                    function: Ident::with_empty_span(Symbol::intern(name.as_ref())),
                })
            }
            Instruction::CallLocal(_)
            | Instruction::CallMastRoot(_)
            | Instruction::CallImported(_) => unimplemented!("contract calls"),
            Instruction::SysCall(ref proc_id) => {
                let module = imported
                    .get_procedure_path(proc_id)
                    .expect("reference to import that doesn't exist")
                    .last();
                let name = imported
                    .get_procedure_name(proc_id)
                    .expect("reference to import that doesn't exist");
                Self::Syscall(FunctionIdent {
                    module: Ident::with_empty_span(Symbol::intern(module)),
                    function: Ident::with_empty_span(Symbol::intern(name.as_ref())),
                })
            }
            Instruction::DynExec => Self::DynExec,
            Instruction::DynCall => Self::DynCall,
            Instruction::ProcRefLocal(local_index) => Self::ProcRef(locals[local_index as usize]),
            Instruction::ProcRefImported(ref proc_id) => {
                let module = imported
                    .get_procedure_path(proc_id)
                    .expect("reference to import that doesn't exist")
                    .last();
                let name = imported
                    .get_procedure_name(proc_id)
                    .expect("reference to import that doesn't exist");
                Self::ProcRef(FunctionIdent {
                    module: Ident::with_empty_span(Symbol::intern(module)),
                    function: Ident::with_empty_span(Symbol::intern(name.as_ref())),
                })
            }
            Instruction::Caller => Self::Caller,
            Instruction::Sdepth
            | Instruction::FriExt2Fold4
            | Instruction::Breakpoint
            | Instruction::Emit(_)
            | Instruction::Debug(_)
            | Instruction::Trace(_) => unimplemented!("miscellaneous instructions"),
        };
        smallvec![op]
    }

    pub fn into_node(
        self,
        _codemap: &miden_diagnostics::CodeMap,
        imports: &super::ModuleImportInfo,
        local_ids: &FxHashMap<FunctionIdent, u16>,
        proc_ids: &FxHashMap<FunctionIdent, miden_assembly::ProcedureId>,
    ) -> SmallVec<[miden_assembly::ast::Node; 2]> {
        use miden_assembly::ast::{Instruction, Node};
        let node = match self {
            Self::Padw => Instruction::PadW,
            Self::Push(v) => Instruction::PushFelt(v),
            Self::Push2([a, b]) => {
                return smallvec![
                    Node::Instruction(Instruction::PushFelt(a)),
                    Node::Instruction(Instruction::PushFelt(b))
                ]
            }
            Self::Pushw(word) => Instruction::PushWord(word),
            Self::PushU8(v) => Instruction::PushFelt(Felt::new(v as u64)),
            Self::PushU16(v) => Instruction::PushFelt(Felt::new(v as u64)),
            Self::PushU32(v) => Instruction::PushFelt(Felt::new(v as u64)),
            Self::Drop => Instruction::Drop,
            Self::Dropw => Instruction::DropW,
            Self::Dup(0) => Instruction::Dup0,
            Self::Dup(1) => Instruction::Dup1,
            Self::Dup(2) => Instruction::Dup2,
            Self::Dup(3) => Instruction::Dup3,
            Self::Dup(4) => Instruction::Dup4,
            Self::Dup(5) => Instruction::Dup5,
            Self::Dup(6) => Instruction::Dup6,
            Self::Dup(7) => Instruction::Dup7,
            Self::Dup(8) => Instruction::Dup8,
            Self::Dup(9) => Instruction::Dup9,
            Self::Dup(10) => Instruction::Dup10,
            Self::Dup(11) => Instruction::Dup11,
            Self::Dup(12) => Instruction::Dup12,
            Self::Dup(13) => Instruction::Dup13,
            Self::Dup(14) => Instruction::Dup14,
            Self::Dup(15) => Instruction::Dup15,
            Self::Dup(n) => {
                panic!("invalid dup instruction, valid index range is 0..=15, got {n}")
            }
            Self::Dupw(0) => Instruction::DupW0,
            Self::Dupw(1) => Instruction::DupW1,
            Self::Dupw(2) => Instruction::DupW2,
            Self::Dupw(3) => Instruction::DupW3,
            Self::Dupw(n) => {
                panic!("invalid dupw instruction, valid index range is 0..=3, got {n}")
            }
            Self::Swap(1) => Instruction::Swap1,
            Self::Swap(2) => Instruction::Swap2,
            Self::Swap(3) => Instruction::Swap3,
            Self::Swap(4) => Instruction::Swap4,
            Self::Swap(5) => Instruction::Swap5,
            Self::Swap(6) => Instruction::Swap6,
            Self::Swap(7) => Instruction::Swap7,
            Self::Swap(8) => Instruction::Swap8,
            Self::Swap(9) => Instruction::Swap9,
            Self::Swap(10) => Instruction::Swap10,
            Self::Swap(11) => Instruction::Swap11,
            Self::Swap(12) => Instruction::Swap12,
            Self::Swap(13) => Instruction::Swap13,
            Self::Swap(14) => Instruction::Swap14,
            Self::Swap(15) => Instruction::Swap15,
            Self::Swap(n) => {
                panic!("invalid swap instruction, valid index range is 1..=15, got {n}")
            }
            Self::Swapw(1) => Instruction::SwapW1,
            Self::Swapw(2) => Instruction::SwapW2,
            Self::Swapw(3) => Instruction::SwapW3,
            Self::Swapw(n) => {
                panic!("invalid swapw instruction, valid index range is 1..=3, got {n}")
            }
            Self::Movup(2) => Instruction::MovUp2,
            Self::Movup(3) => Instruction::MovUp3,
            Self::Movup(4) => Instruction::MovUp4,
            Self::Movup(5) => Instruction::MovUp5,
            Self::Movup(6) => Instruction::MovUp6,
            Self::Movup(7) => Instruction::MovUp7,
            Self::Movup(8) => Instruction::MovUp8,
            Self::Movup(9) => Instruction::MovUp9,
            Self::Movup(10) => Instruction::MovUp10,
            Self::Movup(11) => Instruction::MovUp11,
            Self::Movup(12) => Instruction::MovUp12,
            Self::Movup(13) => Instruction::MovUp13,
            Self::Movup(14) => Instruction::MovUp14,
            Self::Movup(15) => Instruction::MovUp15,
            Self::Movup(n) => {
                panic!("invalid movup instruction, valid index range is 2..=15, got {n}")
            }
            Self::Movupw(2) => Instruction::MovUpW2,
            Self::Movupw(3) => Instruction::MovUpW3,
            Self::Movupw(n) => {
                panic!("invalid movupw instruction, valid index range is 2..=3, got {n}")
            }
            Self::Movdn(2) => Instruction::MovDn2,
            Self::Movdn(3) => Instruction::MovDn3,
            Self::Movdn(4) => Instruction::MovDn4,
            Self::Movdn(5) => Instruction::MovDn5,
            Self::Movdn(6) => Instruction::MovDn6,
            Self::Movdn(7) => Instruction::MovDn7,
            Self::Movdn(8) => Instruction::MovDn8,
            Self::Movdn(9) => Instruction::MovDn9,
            Self::Movdn(10) => Instruction::MovDn10,
            Self::Movdn(11) => Instruction::MovDn11,
            Self::Movdn(12) => Instruction::MovDn12,
            Self::Movdn(13) => Instruction::MovDn13,
            Self::Movdn(14) => Instruction::MovDn14,
            Self::Movdn(15) => Instruction::MovDn15,
            Self::Movdn(n) => {
                panic!("invalid movdn instruction, valid index range is 2..=15, got {n}")
            }
            Self::Movdnw(2) => Instruction::MovDnW2,
            Self::Movdnw(3) => Instruction::MovDnW3,
            Self::Movdnw(n) => {
                panic!("invalid movdnw instruction, valid index range is 2..=3, got {n}")
            }
            Self::Cswap => Instruction::CSwap,
            Self::Cswapw => Instruction::CSwapW,
            Self::Cdrop => Instruction::CDrop,
            Self::Cdropw => Instruction::CDropW,
            Self::Assert => Instruction::Assert,
            Self::AssertWithError(code) => Instruction::AssertWithError(code),
            Self::Assertz => Instruction::Assertz,
            Self::AssertzWithError(code) => Instruction::AssertzWithError(code),
            Self::AssertEq => Instruction::AssertEq,
            Self::AssertEqWithError(code) => Instruction::AssertEqWithError(code),
            Self::AssertEqw => Instruction::AssertEqw,
            Self::AssertEqwWithError(code) => Instruction::AssertEqwWithError(code),
            Self::LocAddr(id) => Instruction::Locaddr(id.as_usize() as u16),
            Self::LocStore(id) => Instruction::LocStore(id.as_usize() as u16),
            Self::LocStorew(id) => Instruction::LocStoreW(id.as_usize() as u16),
            Self::MemLoad => Instruction::MemLoad,
            Self::MemLoadImm(addr) => Instruction::MemLoadImm(addr),
            Self::MemLoadw => Instruction::MemLoadW,
            Self::MemLoadwImm(addr) => Instruction::MemLoadWImm(addr),
            Self::MemStore => Instruction::MemStore,
            Self::MemStoreImm(addr) => Instruction::MemStoreImm(addr),
            Self::MemStorew => Instruction::MemStoreW,
            Self::MemStorewImm(addr) => Instruction::MemStoreWImm(addr),
            Self::MemLoadOffset
            | Self::MemLoadOffsetImm(..)
            | Self::MemStoreOffset
            | Self::MemStoreOffsetImm(..) => unimplemented!(
                "this is an experimental instruction that is not supported by the Miden VM"
            ),
            Self::MemStream => Instruction::MemStream,
            Self::AdvPipe => Instruction::AdvPipe,
            Self::AdvPush(n) => Instruction::AdvPush(n),
            Self::AdvLoadw => Instruction::AdvLoadW,
            Self::If(..) | Self::While(_) | Self::Repeat(..) => {
                panic!("control flow instructions are meant to be handled specially by the caller")
            }
            Self::Exec(ref callee) => {
                if let Some(idx) = local_ids.get(callee).copied() {
                    Instruction::ExecLocal(idx)
                } else {
                    let aliased = if let Some(alias) = imports.alias(&callee.module) {
                        FunctionIdent {
                            module: alias,
                            function: callee.function,
                        }
                    } else {
                        let module_as_import = super::MasmImport::try_from(callee.module)
                            .expect("invalid module name");
                        FunctionIdent {
                            module: Ident::with_empty_span(module_as_import.alias),
                            function: callee.function,
                        }
                    };
                    let id = proc_ids
                        .get(&aliased)
                        .copied()
                        .unwrap_or_else(|| miden_assembly::ProcedureId::new(aliased.to_string()));
                    Instruction::ExecImported(id)
                }
            }
            Self::Syscall(ref callee) => {
                let aliased = if let Some(alias) = imports.alias(&callee.module) {
                    FunctionIdent {
                        module: alias,
                        function: callee.function,
                    }
                } else {
                    let module_as_import =
                        super::MasmImport::try_from(callee.module).expect("invalid module name");
                    FunctionIdent {
                        module: Ident::with_empty_span(module_as_import.alias),
                        function: callee.function,
                    }
                };
                let id = proc_ids
                    .get(&aliased)
                    .copied()
                    .unwrap_or_else(|| miden_assembly::ProcedureId::new(aliased.to_string()));
                Instruction::SysCall(id)
            }
            Self::DynExec => Instruction::DynExec,
            Self::DynCall => Instruction::DynCall,
            Self::ProcRef(ref callee) => {
                if let Some(idx) = local_ids.get(callee).copied() {
                    Instruction::ProcRefLocal(idx)
                } else {
                    let aliased = if let Some(alias) = imports.alias(&callee.module) {
                        FunctionIdent {
                            module: alias,
                            function: callee.function,
                        }
                    } else {
                        let module_as_import = super::MasmImport::try_from(callee.module)
                            .expect("invalid module name");
                        FunctionIdent {
                            module: Ident::with_empty_span(module_as_import.alias),
                            function: callee.function,
                        }
                    };
                    let id = proc_ids
                        .get(&aliased)
                        .copied()
                        .unwrap_or_else(|| miden_assembly::ProcedureId::new(aliased.to_string()));
                    Instruction::ProcRefImported(id)
                }
            }
            Self::Add => Instruction::Add,
            Self::AddImm(imm) => Instruction::AddImm(imm),
            Self::Sub => Instruction::Sub,
            Self::SubImm(imm) => Instruction::SubImm(imm),
            Self::Mul => Instruction::Mul,
            Self::MulImm(imm) => Instruction::MulImm(imm),
            Self::Div => Instruction::Div,
            Self::DivImm(imm) => Instruction::DivImm(imm),
            Self::Neg => Instruction::Neg,
            Self::Inv => Instruction::Inv,
            Self::Incr => Instruction::Incr,
            Self::Ilog2 => todo!("Instruction::ILog2"),
            Self::Pow2 => Instruction::Pow2,
            Self::Exp => Instruction::Exp,
            Self::ExpImm(imm) => Instruction::ExpBitLength(imm),
            Self::Not => Instruction::Not,
            Self::And => Instruction::And,
            Self::AndImm(imm) => {
                return smallvec![
                    Node::Instruction(Instruction::PushFelt(Felt::new(imm as u64))),
                    Node::Instruction(Instruction::And)
                ]
            }
            Self::Or => Instruction::Or,
            Self::OrImm(imm) => {
                return smallvec![
                    Node::Instruction(Instruction::PushFelt(Felt::new(imm as u64))),
                    Node::Instruction(Instruction::Or)
                ]
            }
            Self::Xor => Instruction::Xor,
            Self::XorImm(imm) => {
                return smallvec![
                    Node::Instruction(Instruction::PushFelt(Felt::new(imm as u64))),
                    Node::Instruction(Instruction::Xor)
                ]
            }
            Self::Eq => Instruction::Eq,
            Self::EqImm(imm) => Instruction::EqImm(imm),
            Self::Neq => Instruction::Neq,
            Self::NeqImm(imm) => Instruction::NeqImm(imm),
            Self::Gt => Instruction::Gt,
            Self::GtImm(imm) => {
                return smallvec![
                    Node::Instruction(Instruction::PushFelt(imm)),
                    Node::Instruction(Instruction::Gt)
                ]
            }
            Self::Gte => Instruction::Gte,
            Self::GteImm(imm) => {
                return smallvec![
                    Node::Instruction(Instruction::PushFelt(imm)),
                    Node::Instruction(Instruction::Gte)
                ]
            }
            Self::Lt => Instruction::Lt,
            Self::LtImm(imm) => {
                return smallvec![
                    Node::Instruction(Instruction::PushFelt(imm)),
                    Node::Instruction(Instruction::Lt)
                ]
            }
            Self::Lte => Instruction::Lte,
            Self::LteImm(imm) => {
                return smallvec![
                    Node::Instruction(Instruction::PushFelt(imm)),
                    Node::Instruction(Instruction::Lte)
                ]
            }
            Self::IsOdd => Instruction::IsOdd,
            Self::Eqw => Instruction::Eqw,
            Self::Clk => Instruction::Clk,
            Self::Caller => Instruction::Caller,
            Self::U32Test => Instruction::U32Test,
            Self::U32Testw => Instruction::U32TestW,
            Self::U32Assert => Instruction::U32Assert,
            Self::U32AssertWithError(code) => Instruction::U32AssertWithError(code),
            Self::U32Assert2 => Instruction::U32Assert2,
            Self::U32Assert2WithError(code) => Instruction::U32Assert2WithError(code),
            Self::U32Assertw => Instruction::U32AssertW,
            Self::U32AssertwWithError(code) => Instruction::U32AssertWWithError(code),
            Self::U32Cast => Instruction::U32Cast,
            Self::U32Split => Instruction::U32Split,
            Self::U32OverflowingAdd => Instruction::U32OverflowingAdd,
            Self::U32OverflowingAddImm(imm) => Instruction::U32OverflowingAddImm(imm),
            Self::U32WrappingAdd => Instruction::U32WrappingAdd,
            Self::U32WrappingAddImm(imm) => Instruction::U32WrappingAddImm(imm),
            Self::U32OverflowingAdd3 => Instruction::U32OverflowingAdd3,
            Self::U32WrappingAdd3 => Instruction::U32WrappingAdd3,
            Self::U32OverflowingSub => Instruction::U32OverflowingSub,
            Self::U32OverflowingSubImm(imm) => Instruction::U32OverflowingSubImm(imm),
            Self::U32WrappingSub => Instruction::U32WrappingSub,
            Self::U32WrappingSubImm(imm) => Instruction::U32WrappingSubImm(imm),
            Self::U32OverflowingMul => Instruction::U32OverflowingMul,
            Self::U32OverflowingMulImm(imm) => Instruction::U32OverflowingMulImm(imm),
            Self::U32WrappingMul => Instruction::U32WrappingMul,
            Self::U32WrappingMulImm(imm) => Instruction::U32WrappingMulImm(imm),
            Self::U32OverflowingMadd => Instruction::U32OverflowingMadd,
            Self::U32WrappingMadd => Instruction::U32WrappingMadd,
            Self::U32Div => Instruction::U32Div,
            Self::U32DivImm(imm) => Instruction::U32DivImm(imm),
            Self::U32Mod => Instruction::U32Mod,
            Self::U32ModImm(imm) => Instruction::U32ModImm(imm),
            Self::U32DivMod => Instruction::U32DivMod,
            Self::U32DivModImm(imm) => Instruction::U32DivModImm(imm),
            Self::U32And => Instruction::U32And,
            Self::U32Or => Instruction::U32Or,
            Self::U32Xor => Instruction::U32Xor,
            Self::U32Not => Instruction::U32Not,
            Self::U32Shl => Instruction::U32Shl,
            Self::U32ShlImm(imm) => {
                Instruction::U32ShlImm(imm.try_into().expect("invalid rotation"))
            }
            Self::U32Shr => Instruction::U32Shr,
            Self::U32ShrImm(imm) => {
                Instruction::U32ShrImm(imm.try_into().expect("invalid rotation"))
            }
            Self::U32Rotl => Instruction::U32Rotl,
            Self::U32RotlImm(imm) => {
                Instruction::U32RotlImm(imm.try_into().expect("invalid rotation"))
            }
            Self::U32Rotr => Instruction::U32Rotr,
            Self::U32RotrImm(imm) => {
                Instruction::U32RotrImm(imm.try_into().expect("invalid rotation"))
            }
            Self::U32Popcnt => Instruction::U32Popcnt,
            Self::U32Clz => todo!("Instruction::U32Clz"),
            Self::U32Ctz => todo!("Instruction::U32Ctz"),
            Self::U32Clo => todo!("Instruction::U32Clo"),
            Self::U32Cto => todo!("Instruction::U32Cto"),
            Self::U32Lt => Instruction::U32Lt,
            Self::U32Lte => Instruction::U32Lte,
            Self::U32Gt => Instruction::U32Gt,
            Self::U32Gte => Instruction::U32Gte,
            Self::U32Min => Instruction::U32Min,
            Self::U32Max => Instruction::U32Max,
        };
        smallvec![Node::Instruction(node)]
    }
}

/// This implementation displays the opcode name for the given [MasmOp]
impl fmt::Display for MasmOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Padw => f.write_str("padw"),
            Self::Push(_)
            | Self::Push2(_)
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
            Self::AssertWithError(code) => write!(f, "assert.err={code}"),
            Self::Assertz => f.write_str("assertz"),
            Self::AssertzWithError(code) => write!(f, "assertz.err={code}"),
            Self::AssertEq => f.write_str("assert_eq"),
            Self::AssertEqWithError(code) => write!(f, "assert_eq.err={code}"),
            Self::AssertEqw => f.write_str("assert_eqw"),
            Self::AssertEqwWithError(code) => write!(f, "assert_eqw.err={code}"),
            Self::LocAddr(_) => f.write_str("locaddr"),
            Self::LocStore(_) => f.write_str("loc_store"),
            Self::LocStorew(_) => f.write_str("loc_storew"),
            Self::MemLoad
            | Self::MemLoadOffset
            | Self::MemLoadImm(_)
            | Self::MemLoadOffsetImm(..) => f.write_str("mem_load"),
            Self::MemLoadw | Self::MemLoadwImm(_) => f.write_str("mem_loadw"),
            Self::MemStore
            | Self::MemStoreOffset
            | Self::MemStoreImm(_)
            | Self::MemStoreOffsetImm(..) => f.write_str("mem_store"),
            Self::MemStorew | Self::MemStorewImm(_) => f.write_str("mem_storew"),
            Self::MemStream => f.write_str("mem_stream"),
            Self::AdvPipe => f.write_str("adv_pipe"),
            Self::AdvPush(_) => f.write_str("adv_push"),
            Self::AdvLoadw => f.write_str("adv_loadw"),
            Self::If(..) => f.write_str("if.true"),
            Self::While(_) => f.write_str("while.true"),
            Self::Repeat(..) => f.write_str("repeat"),
            Self::Exec(_) => f.write_str("exec"),
            Self::Syscall(_) => f.write_str("syscall"),
            Self::DynExec => f.write_str("dynexec"),
            Self::DynCall => f.write_str("dyncall"),
            Self::ProcRef(_) => f.write_str("procref"),
            Self::Add | Self::AddImm(_) => f.write_str("add"),
            Self::Sub | Self::SubImm(_) => f.write_str("sub"),
            Self::Mul | Self::MulImm(_) => f.write_str("mul"),
            Self::Div | Self::DivImm(_) => f.write_str("div"),
            Self::Neg => f.write_str("neg"),
            Self::Inv => f.write_str("inv"),
            Self::Incr => f.write_str("add.1"),
            Self::Ilog2 => f.write_str("ilog2"),
            Self::Pow2 => f.write_str("pow2"),
            Self::Exp => f.write_str("exp"),
            Self::ExpImm(imm) => write!(f, "exp.u{imm}"),
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
            Self::Caller => f.write_str("caller"),
            Self::U32Test => f.write_str("u32test"),
            Self::U32Testw => f.write_str("u32testw"),
            Self::U32Assert => f.write_str("u32assert"),
            Self::U32AssertWithError(code) => write!(f, "u32assert.err={code}"),
            Self::U32Assert2 => f.write_str("u32assert2"),
            Self::U32Assert2WithError(code) => write!(f, "u32assert2.err={code}"),
            Self::U32Assertw => f.write_str("u32assertw"),
            Self::U32AssertwWithError(code) => write!(f, "u32assertw.err={code}"),
            Self::U32Cast => f.write_str("u32cast"),
            Self::U32Split => f.write_str("u32split"),
            Self::U32OverflowingAdd | Self::U32OverflowingAddImm(_) => {
                f.write_str("u32overflowing_add")
            }
            Self::U32WrappingAdd | Self::U32WrappingAddImm(_) => f.write_str("u32wrapping_add"),
            Self::U32OverflowingAdd3 => f.write_str("u32overflowing_add3"),
            Self::U32WrappingAdd3 => f.write_str("u32wrapping_add3"),
            Self::U32OverflowingSub | Self::U32OverflowingSubImm(_) => {
                f.write_str("u32overflowing_sub")
            }
            Self::U32WrappingSub | Self::U32WrappingSubImm(_) => f.write_str("u32wrapping_sub"),
            Self::U32OverflowingMul | Self::U32OverflowingMulImm(_) => {
                f.write_str("u32overflowing_mul")
            }
            Self::U32WrappingMul | Self::U32WrappingMulImm(_) => f.write_str("u32wrapping_mul"),
            Self::U32OverflowingMadd => f.write_str("u32overflowing_madd"),
            Self::U32WrappingMadd => f.write_str("u32wrapping_madd"),
            Self::U32Div | Self::U32DivImm(_) => f.write_str("u32div"),
            Self::U32Mod | Self::U32ModImm(_) => f.write_str("u32mod"),
            Self::U32DivMod | Self::U32DivModImm(_) => f.write_str("u32divmod"),
            Self::U32And => f.write_str("u32and"),
            Self::U32Or => f.write_str("u32or"),
            Self::U32Xor => f.write_str("u32xor"),
            Self::U32Not => f.write_str("u32not"),
            Self::U32Shl | Self::U32ShlImm(_) => f.write_str("u32shl"),
            Self::U32Shr | Self::U32ShrImm(_) => f.write_str("u32shr"),
            Self::U32Rotl | Self::U32RotlImm(_) => f.write_str("u32rotl"),
            Self::U32Rotr | Self::U32RotrImm(_) => f.write_str("u32rotr"),
            Self::U32Popcnt => f.write_str("u32popcnt"),
            Self::U32Clz => f.write_str("u32clz"),
            Self::U32Ctz => f.write_str("u32ctz"),
            Self::U32Clo => f.write_str("u32clo"),
            Self::U32Cto => f.write_str("u32cto"),
            Self::U32Lt => f.write_str("u32lt"),
            Self::U32Lte => f.write_str("u32lte"),
            Self::U32Gt => f.write_str("u32gt"),
            Self::U32Gte => f.write_str("u32gte"),
            Self::U32Min => f.write_str("u32min"),
            Self::U32Max => f.write_str("u32max"),
        }
    }
}
