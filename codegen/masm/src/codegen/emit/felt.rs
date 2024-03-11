use miden_hir::{Felt, FieldElement};

use super::OpEmitter;
use crate::masm::Op;

/// The value zero, as a field element
pub const ZERO: Felt = Felt::ZERO;

/// The value 2^32, as a field element
pub const U32_FIELD_MODULUS: Felt = Felt::new(2u64.pow(32));

#[allow(unused)]
impl<'a> OpEmitter<'a> {
    /// This operation checks if the field element on top of the stack is zero.
    ///
    /// This operation does not consume the input, and pushes a boolean value on the stack.
    ///
    /// # Stack effects
    ///
    /// `[a, ..] => [a == 0, a, ..]`
    #[inline(always)]
    pub fn felt_is_zero(&mut self) {
        self.emit_all(&[Op::Dup(0), Op::EqImm(ZERO)]);
    }

    /// This operation asserts the field element on top of the stack is zero.
    ///
    /// This operation does not consume the input.
    ///
    /// # Stack effects
    ///
    /// `[a, ..] => [a, ..]`
    #[inline(always)]
    pub fn assert_felt_is_zero(&mut self) {
        self.emit_all(&[Op::Dup(0), Op::Assertz]);
    }

    /// Convert a field element to i128 by zero-extension.
    ///
    /// This consumes the field element on top of the stack.
    ///
    /// # Stack effects
    ///
    /// `[a, ..] => [0, 0, a_hi, a_lo]`
    #[inline]
    pub fn felt_to_i128(&mut self) {
        self.emit_all(&[Op::U32Split, Op::Push2([ZERO, ZERO])]);
    }

    /// Convert a field element to u64 by zero-extension.
    ///
    /// This consumes the field element on top of the stack.
    ///
    /// # Stack effects
    ///
    /// `[a, ..] => [a_hi, a_lo]`
    #[inline(always)]
    pub fn felt_to_u64(&mut self) {
        self.emit(Op::U32Split);
    }

    /// Convert a field element to i64 by zero-extension.
    ///
    /// Asserts if the field element is too large to represent as an i64.
    ///
    /// This consumes the field element on top of the stack.
    ///
    /// # Stack effects
    ///
    /// `[a, ..] => [a_hi, a_lo]`
    #[inline(always)]
    pub fn felt_to_i64(&mut self) {
        self.felt_to_u64();
        self.assert_unsigned_int64();
    }

    /// Convert a field element value to an unsigned N-bit integer, where N <= 32
    ///
    /// Conversion will trap if the input value is too large to fit in an unsigned N-bit integer.
    pub fn felt_to_uint(&mut self, n: u32) {
        assert_valid_integer_size!(n, 1, 32);
        self.emit_all(&[
            // Split into u32 limbs
            Op::U32Split,
            // Assert most significant 32 bits are unused
            Op::EqImm(ZERO),
            Op::Assert,
        ]);
        if n < 32 {
            // Convert to N-bit integer
            self.int32_to_uint(n);
        }
    }

    /// Convert a field element value to a signed N-bit integer, where N <= 32
    ///
    /// Conversion will trap if the input value is too large to fit in a signed N-bit integer.
    pub fn felt_to_int(&mut self, n: u32) {
        assert_valid_integer_size!(n, 1, 32);
        self.emit_all(&[
            // Split into u32 limbs
            Op::U32Split,
            // Assert most significant 32 bits are unused
            Op::EqImm(ZERO),
            Op::Assert,
        ]);
        // Assert the sign bit isn't set
        self.assert_unsigned_int32();
        if n < 32 {
            // Convert to signed N-bit integer
            self.int32_to_int(n);
        }
    }

    /// Zero-extend a field element value to N-bits, where N >= 64
    ///
    /// N must be a power of two, or this function will panic.
    pub fn zext_felt(&mut self, n: u32) {
        assert_valid_integer_size!(n, 64, 256);
        match n {
            64 => self.felt_to_u64(),
            128 => self.felt_to_i128(),
            n => {
                // Convert to u64 and zero-extend
                self.felt_to_u64();
                self.zext_int64(n);
            }
        }
    }

    /// Emits code to sign-extend a field element value to an N-bit integer, where N >= 64
    ///
    /// Field elements are unsigned, so sign-extension here is indicating that the target
    /// integer type is a signed type, so we have one less bit available to use.
    ///
    /// N must be a power of two, or this function will panic.
    pub fn sext_felt(&mut self, n: u32) {
        assert_valid_integer_size!(n, 64, 256);
        match n {
            64 => self.felt_to_i64(),
            128 => self.felt_to_i128(),
            n => {
                // Convert to i64 and sign-extend
                self.felt_to_i64();
                self.sext_int64(n);
            }
        }
    }

    /// Truncates a field element on top of the stack to an N-bit integer, where N <= 32.
    ///
    /// Truncation on field elements is not well-defined, because field elements do not have
    /// a specified bitwise representation. To implement semantics equivalent to the other types
    /// which _do_ have a specified representation, we first convert the input field element to u32,
    /// and then masking out any additional unused bits of the u32 representation.
    ///
    /// This should produce outputs which are identical to equivalent u64 values, i.e. the same
    /// value in both u64 and felt representation will be truncated to the same u32 value.
    #[inline]
    pub fn trunc_felt(&mut self, n: u32) {
        // Apply a field modulus of 2^32, i.e. `a mod 2^32`, converting
        // the field element into the u32 range. Miden defines values in
        // this range as having a standard unsigned binary representation.
        self.emit(Op::U32Cast);
        self.trunc_int32(n);
    }

    /// Make `n` copies of the element on top of the stack
    #[inline(always)]
    pub fn dup_felt(&mut self, count: u8) {
        self.emit_n(count as usize, Op::Dup(0));
    }
}
