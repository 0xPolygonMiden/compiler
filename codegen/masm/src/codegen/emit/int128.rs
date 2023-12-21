use crate::masm::Op;

use super::OpEmitter;

#[allow(unused)]
impl<'a> OpEmitter<'a> {
    /// Checks if the i128 value on the stack has its sign bit set.
    #[inline(always)]
    pub fn is_signed_i128(&mut self) {
        self.is_signed_int32()
    }

    /// Assert that the i128 value on the stack does not have its sign bit set.
    #[inline(always)]
    pub fn assert_unsigned_i128(&mut self) {
        // Assert that the sign bit is unset
        self.assert_unsigned_int32()
    }

    /// Push an i128 value on the operand stack
    ///
    /// An i128 value consists of 4 32-bit limbs
    pub fn push_i128(&mut self, value: i128) {
        let bytes = value.to_le_bytes();
        let hi = u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]);
        let lo = u64::from_le_bytes([
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
        ]);
        self.push_u64(lo);
        self.push_u64(hi);
    }

    /// Convert an i128 value to a field element value.
    ///
    /// This is different than `trunc_i128_to_felt`, as this function performs a
    /// range check on the input value to ensure that it will fit in a felt.
    ///
    /// This consumes the input value, and leaves a felt value on the stack.
    /// Execution traps if the input value cannot fit in a field element.
    ///
    /// NOTE: This function does not validate the i128, the caller is expected to
    /// have already validated that the top of the stack holds a valid i128.
    pub fn i128_to_felt(&mut self) {
        // First, convert to u64
        self.i128_to_u64();
        // Then convert the u64 to felt
        self.u64_to_felt();
    }

    /// Convert an i128 value to u64
    ///
    /// This is different than `trunc_i128_to_u64`, as this function performs a
    /// range check on the input value to ensure that it will fit in a u64.
    ///
    /// This consumes the input value, and leaves a u64 value on the stack.
    ///
    /// NOTE: This function does not validate the i128, the caller is expected to
    /// have already validated that the top of the stack holds a valid i128.
    pub fn i128_to_u64(&mut self) {
        // Assert the first element is equal to 0 without consuming it
        //
        // This has the effect of validating not only that the value is
        // unsigned, but that the value is small enough to fit in an i96
        self.assert_felt_is_zero();
        // Assert that the high 64 bits are all zero
        //
        // This extends the range check above, validating that the input
        // is small enough to fit in an i64
        //
        // What remains on the stack at this point are the low 64-bits,
        // which is also our result.
        self.emit(Op::AssertEq);
    }

    /// Convert an i128 value to i64
    ///
    /// This is different than `trunc_i128_to_i64`, as this function performs a
    /// range check on the input value to ensure that it will fit in a i64.
    ///
    /// This consumes the input value, and leaves an i64 value on the stack.
    ///
    /// NOTE: This function does not validate the i128, the caller is expected to
    /// have already validated that the top of the stack holds a valid i128.
    pub fn i128_to_i64(&mut self) {
        // Determine if this value is signed or not
        self.is_signed_int32();
        // Preserving the is_signed flag, select the expected hi bits value
        self.emit(Op::Dup(0));
        self.select_int32(u32::MAX, 0);
        // Move the most significant 64 bits to top of stack
        self.move_int64_up(2);
        // Move expected value to top of stack
        self.emit(Op::Movup(2));
        // Assert the most significant 32 bits match, without consuming them
        self.assert_eq_u32();
        self.emit_all(&[
            // Assert that both 32-bit limbs of the most significant 64 bits match,
            // consuming them in the process
            Op::AssertEq,
            // At this point, the stack is: [is_signed, x1, x0]
            //
            // Select an expected value for the sign bit based on the is_signed flag
            Op::Swap(1),
        ]);
        // [is_sign_bit_set, x1, is_signed, x0]
        self.is_const_flag_set_u32(1 << 31);
        self.emit_all(&[
            // [is_signed, is_sign_bit_set, x1, x0]
            Op::Movup(2),
            // Assert that the flags are equal: either the input was signed and the
            // sign bit was set, or the input was unsigned, and the sign bit was unset,
            // any other combination will trap.
            //
            // [x1, x0]
            Op::AssertEq,
        ]);
    }

    /// Truncate this i128 value to a felt value
    ///
    /// This consumes the input value, and leaves a felt value on the stack.
    ///
    /// NOTE: This function does not validate the i128, that is left up to the caller.
    #[inline]
    pub fn trunc_i128_to_felt(&mut self) {
        self.emit_n(2, Op::Drop);
        self.trunc_int64_to_felt();
    }

    /// Truncate this i128 value to N bits, where N is <= 64
    ///
    /// This consumes the input value, and leaves an N-bit value on the stack,
    /// where the the value is assumed to be represented using 32-bit limbs.
    /// For example, a 64-bit value will consist of two 32-bit values on the
    /// stack.
    ///
    /// NOTE: This function does not validate the i128 value, that is left up to the caller.
    #[inline]
    pub fn trunc_i128(&mut self, n: u32) {
        assert_valid_integer_size!(n, 1, 64);
        match n {
            64 => {
                self.emit_n(2, Op::Drop);
            }
            32 => {
                self.emit_n(3, Op::Drop);
            }
            n => {
                self.trunc_int32(n);
            }
        }
    }

    /// Pop two i128 values, `b` and `a`, off the operand stack, and place the result of `a == b` on the stack.
    #[inline]
    pub fn eq_i128(&mut self) {
        self.emit_all(&[
            Op::Eqw,
            // Move the boolean below the elements we're going to drop
            Op::Movdn(8),
            // Drop both i128 values
            Op::Dropw,
            Op::Dropw,
        ]);
    }

    /// Pop two i128 values, `b` and `a`, off the operand stack, and place the result of `a == b` on the stack.
    #[inline]
    pub fn neq_i128(&mut self) {
        self.eq_i128();
        self.emit(Op::Not);
    }
}
