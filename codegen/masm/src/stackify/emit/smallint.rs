//! A "smallint" is an `N`-bit, signed or unsigned integer in two's complement representation,
//! where `N` is defined as being 32 bits or smaller, and a power of two.
//!
//! In Miden, unsigned smallint operations are largely handled with the native u32 operations,
//! however we perform range checks on checked operations to ensure the value is still a valid
//! `N`-bit integer.
//!
//! For signed smallint operations, we implement them in terms of a two's complement representation,
//! using a set of common primitives. The only thing that changes are which bits are considered by
//! those primitives.
use crate::masm::Op;
use miden_hir::Overflow;

use super::OpEmitter;

#[allow(unused)]
impl<'a> OpEmitter<'a> {
    /// Check that a u32 value on the stack can fit in the unsigned N-bit integer range
    #[inline(always)]
    pub fn is_valid_uint(&mut self, n: u32) {
        // Use fallible conversion from u32
        self.try_int32_to_uint(n);
    }

    /// Check that the 32-bit value on the stack can fit in the signed N-bit integer range
    #[inline(always)]
    pub fn is_valid_int(&mut self, n: u32) {
        self.try_int32_to_int(n);
    }

    /// Check if the sign bit of an N-bit integer on the stack, is set.
    #[inline]
    pub fn is_signed_smallint(&mut self, n: u32) {
        assert_valid_integer_size!(n, 1, 32);
        match n {
            // i1 is never signed
            1 => self.emit(Op::PushU32(0)),
            n => self.is_const_flag_set_u32(1 << (n - 1)),
        }
    }

    /// Asserts the N-bit integer on the stack does not have its sign bit set.
    #[inline]
    pub fn assert_unsigned_smallint(&mut self, n: u32) {
        match n {
            // i1 is always unsigned
            1 => (),
            n => {
                self.is_signed_smallint(n);
                self.emit(Op::Assert);
            }
        }
    }

    /// Convert a signed N-bit integer to a field element
    #[inline(always)]
    pub fn int_to_felt(&mut self, n: u32) {
        self.assert_unsigned_smallint(n);
    }

    /// Convert an unsigned N-bit integer to a field element
    #[inline(always)]
    pub fn uint_to_felt(&mut self, n: u32) {
        // Conversion to felt is a no-op
        assert_valid_integer_size!(n, 1, 32);
    }

    /// Convert a signed N-bit integer to u64
    ///
    /// This operation will trap if the value has the sign bit set.
    #[inline]
    pub fn int_to_u64(&mut self, n: u32) {
        self.assert_unsigned_smallint(n);
        self.emit(Op::PushU32(0));
    }

    /// Convert an unsigned N-bit integer to u64
    #[inline(always)]
    pub fn uint_to_u64(&mut self, _: u32) {
        self.emit(Op::PushU32(0));
    }

    /// Convert a signed N-bit integer to i128
    #[inline]
    pub fn int_to_i128(&mut self, n: u32) {
        self.sext_smallint(n, 128);
    }

    /// Convert an unsigned N-bit integer to i128
    #[inline(always)]
    pub fn uint_to_i128(&mut self, _n: u32) {
        // zero-extend to i128
        self.emit_n(3, Op::PushU32(0));
    }

    /// Sign-extend the N-bit value on the stack to M-bits, where M is >= N and <= 256.
    ///
    /// This assumes the value on the stack is a valid N-bit integer in two's complement
    /// representation, i.e. the most significant bit is the sign bit.
    pub fn sext_smallint(&mut self, n: u32, m: u32) {
        assert_valid_integer_size!(n, n, 256);
        // No-op
        if n == m {
            return;
        }
        // The number of sign bits are the number of bits between N and 32
        let num_sign_bits = 32 - n;
        // By subtracting 1 from 2^(32 - N), we get a value that is all 1s,
        // shifting it left by N, and bitwise-OR'ing it with the input value,
        // we produce the sign-extended value
        let sign_bits = ((1 << num_sign_bits) - 1) << n;
        // We optimize larger extensions by re-using the is_signed flag
        let is_large = m > 32;
        // Get the value of the sign bit
        self.is_signed_smallint(n);
        if is_large {
            // Make a copy for selecting padding later
            self.emit(Op::Dup(0));
            self.select_int32(sign_bits, 0);
            self.emit_all(&[
                // Move the input value to the top of the stack
                Op::Movup(2),
                // Sign-extend to i32
                Op::U32Or,
                // Move the is_signed flag back to the top
                Op::Swap(1),
            ]);
            // Select the padding element value
            self.select_int32(u32::MAX, 0);
            // Pad out to M bits
            self.pad_int32(m);
        } else {
            self.select_int32(sign_bits, 0);
            // Sign-extend to i32
            self.emit(Op::U32Or);
        }
    }

    /// Zero-extend the N-bit value on the stack to M-bits, where M is >= N and <= 256.
    ///
    /// This assumes the value on the stack is a valid N-bit integer.
    #[inline]
    pub fn zext_smallint(&mut self, n: u32, m: u32) {
        assert_valid_integer_size!(n, n, 256);
        // No-op
        if n == m {
            return;
        }
        self.zext_int32(m);
    }

    pub fn add_uint(&mut self, n: u32, overflow: Overflow) {
        match overflow {
            Overflow::Unchecked => self.add_u32(Overflow::Unchecked),
            overflow => {
                self.add_u32(Overflow::Checked);
                self.handle_uint_overflow(n, overflow)
            }
        }
    }

    pub fn add_imm_uint(&mut self, imm: u32, n: u32, overflow: Overflow) {
        match overflow {
            Overflow::Unchecked => self.add_imm_u32(imm, Overflow::Unchecked),
            overflow => {
                self.add_imm_u32(imm, Overflow::Checked);
                self.handle_uint_overflow(n, overflow)
            }
        }
    }

    /// Pops two u32 values off the stack, `b` and `a`, and performs `a - b`.
    ///
    /// See the [Overflow] type for how overflow semantics can change the operation.
    pub fn sub_uint(&mut self, n: u32, overflow: Overflow) {
        match overflow {
            Overflow::Unchecked => self.sub_u32(overflow),
            overflow => {
                self.sub_u32(overflow);
                self.handle_uint_overflow(n, overflow);
            }
        }
    }

    /// Pops a u32 value off the stack, `a`, and performs `a - <imm>`.
    ///
    /// See the [Overflow] type for how overflow semantics can change the operation.
    ///
    /// Subtracting zero is a no-op.
    #[inline]
    pub fn sub_imm_uint(&mut self, imm: u32, n: u32, overflow: Overflow) {
        if imm == 0 {
            return;
        }
        match overflow {
            Overflow::Unchecked => self.sub_imm_u32(imm, overflow),
            overflow => {
                self.sub_imm_u32(imm, overflow);
                self.handle_uint_overflow(n, overflow);
            }
        }
    }

    pub fn mul_uint(&mut self, n: u32, overflow: Overflow) {
        match overflow {
            Overflow::Unchecked => self.mul_u32(Overflow::Unchecked),
            overflow => {
                self.mul_u32(Overflow::Checked);
                self.handle_uint_overflow(n, overflow)
            }
        }
    }

    pub fn mul_imm_uint(&mut self, imm: u32, n: u32, overflow: Overflow) {
        match overflow {
            Overflow::Unchecked => self.mul_imm_u32(imm, Overflow::Unchecked),
            overflow => {
                self.mul_imm_u32(imm, Overflow::Checked);
                self.handle_uint_overflow(n, overflow)
            }
        }
    }

    #[inline]
    pub fn checked_div_uint(&mut self, n: u32) {
        self.checked_div_u32();
        self.int32_to_uint(n);
    }

    #[inline]
    pub fn checked_div_imm_uint(&mut self, imm: u32, n: u32) {
        self.checked_div_imm_u32(imm);
        self.int32_to_uint(n);
    }

    #[inline(always)]
    pub fn unchecked_div_uint(&mut self, _n: u32) {
        self.unchecked_div_u32();
    }

    #[inline(always)]
    pub fn unchecked_div_imm_uint(&mut self, imm: u32, _n: u32) {
        self.unchecked_div_imm_u32(imm);
    }

    /// Pops two u32 values off the stack, `b` and `a`, and performs `a % b`.
    ///
    /// This operation is checked, so if the operands or result are not valid u32, execution traps.
    #[inline(always)]
    pub fn checked_mod_uint(&mut self, _n: u32) {
        self.checked_mod_u32();
    }

    /// Pops a u32 value off the stack, `a`, and performs `a % <imm>`.
    ///
    /// This function will panic if the divisor is zero.
    ///
    /// This operation is checked, so if the operand or result are not valid u32, execution traps.
    #[inline(always)]
    pub fn checked_mod_imm_uint(&mut self, imm: u32, _n: u32) {
        self.checked_mod_imm_u32(imm);
    }

    /// Pops two u32 values off the stack, `b` and `a`, and performs `a % b`.
    ///
    /// This operation is unchecked, so the result is not guaranteed to be a valid u32
    #[inline(always)]
    pub fn unchecked_mod_uint(&mut self, _n: u32) {
        self.unchecked_mod_u32();
    }

    /// Pops a u32 value off the stack, `a`, and performs `a % <imm>`.
    ///
    /// This function will panic if the divisor is zero.
    #[inline(always)]
    pub fn unchecked_mod_imm_uint(&mut self, imm: u32, _n: u32) {
        self.unchecked_mod_imm_u32(imm);
    }

    /// Pops two u32 values off the stack, `b` and `a`, and pushes `a / b`, then `a % b` on the stack.
    ///
    /// This operation is checked, so if the operands or result are not valid u32, execution traps.
    #[inline(always)]
    pub fn checked_divmod_uint(&mut self, _n: u32) {
        self.checked_divmod_u32();
    }

    /// Pops a u32 value off the stack, `a`, and pushes `a / <imm>`, then `a % <imm>` on the stack.
    ///
    /// This operation is checked, so if the operands or result are not valid u32, execution traps.
    #[inline(always)]
    pub fn checked_divmod_imm_uint(&mut self, imm: u32, _n: u32) {
        self.checked_divmod_imm_u32(imm);
    }

    /// Pops two u32 values off the stack, `b` and `a`, and pushes `a / b`, then `a % b` on the stack.
    ///
    /// This operation is unchecked, so the result is not guaranteed to be a valid u32
    #[inline(always)]
    pub fn unchecked_divmod_uint(&mut self, _n: u32) {
        self.unchecked_divmod_u32();
    }

    /// Pops a u32 value off the stack, `a`, and pushes `a / <imm>`, then `a % <imm>` on the stack.
    ///
    /// This operation is unchecked, so the result is not guaranteed to be a valid u32
    #[inline(always)]
    pub fn unchecked_divmod_imm_uint(&mut self, imm: u32, _n: u32) {
        self.unchecked_divmod_imm_u32(imm)
    }

    pub fn handle_uint_overflow(&mut self, n: u32, overflow: Overflow) {
        match overflow {
            Overflow::Unchecked => (),
            Overflow::Checked => self.int32_to_uint(n),
            Overflow::Wrapping => self.emit(Op::U32CheckedModImm(2u32.pow(n))),
            Overflow::Overflowing => {
                self.try_int32_to_uint(n);
                self.emit_all(&[
                    // move result to top, and wrap it at 2^n
                    Op::Swap(1),
                    Op::U32CheckedModImm(2u32.pow(n)),
                    // move is_valid flag to top, and invert it
                    Op::Swap(1),
                    Op::Not,
                ]);
            }
        }
    }
}
