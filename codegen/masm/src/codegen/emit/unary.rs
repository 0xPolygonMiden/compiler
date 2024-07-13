use midenc_hir::{Felt, FieldElement, Overflow};

use super::*;

impl<'a> OpEmitter<'a> {
    /// Truncate an integral value of type `src` to type `dst`
    ///
    /// Truncation is defined in terms of the bitwise representation of the type.
    /// Specifically, the source value will have any excess bits above the bitwidth of
    /// the `dst` type either zeroed, or dropped, depending on the `src` type. For example,
    /// u64 is represented as two 32-bit limbs, each in a field element on the operand stack;
    /// while u16 is represented as 16 bits in a single field element. Truncating from u64
    /// to u16 results in dropping the 32-bit limb containing the most significant bits, and
    /// then masking out the most significant 16 bits of the remaining 32-bit limb, leaving
    /// a 16-bit value on the operand stack.
    ///
    /// NOTE: Field elements do not have a formal bitwise representation. When truncating to
    /// felt and the source value is negative, the resulting felt will be `Felt::ZERO`. When
    /// the value is non-negative, the source value will be mapped to the field element range
    /// using the field modulus of `2^64 - 2^32 + 1`, and then convert the representation to
    /// felt by multiplying the 32-bit limbs (the only values which can be truncated to felt
    /// are u64, i64, and i128, all of which use multiple 32-bit limbs).
    ///
    /// This function assumes that an integer value of type `src` is on top of the operand stack,
    /// and will ensure a value of type `dst` is on the operand stack after truncation, or that
    /// execution traps.
    pub fn trunc(&mut self, dst: &Type) {
        let arg = self.stack.pop().expect("operand stack is empty");
        let src = arg.ty();
        assert!(
            src.is_integer() && dst.is_integer(),
            "invalid truncation of {src} to {dst}: only integer-to-integer casts are supported"
        );
        let n = dst.size_in_bits() as u32;
        match (&src, dst) {
            // If the types are equivalent, it's a no-op
            (src, dst) if src == dst => (),
            (Type::Felt, _) if n <= 32 => self.trunc_felt(n),
            // Truncating to felt
            (Type::U128 | Type::I128, Type::Felt) => self.trunc_i128_to_felt(),
            // Truncating a 128-bit integer to 64 bits or smaller
            (Type::U128 | Type::I128, _) if n <= 64 => self.trunc_i128(n),
            // Truncating i64/u64 to felt
            (Type::I64 | Type::U64, Type::Felt) => self.trunc_int64_to_felt(),
            // Truncating a u64/i64 to 32 bits or smaller
            (Type::I64 | Type::U64, _) if n <= 32 => self.trunc_int64(n),
            // Truncating a felt to 32 bits or smaller
            (Type::Felt, _) if n <= 32 => self.trunc_felt(n),
            // Truncating an i32/u32 to smaller than 32 bits
            (Type::I32 | Type::U32, _) if n <= 32 => self.trunc_int32(n),
            // Truncating an i16/u16 to smaller than 16 bits
            (Type::I16 | Type::U16, _) if n <= 16 => self.trunc_int32(n),
            // Truncating an i8/u8 to smaller than 8 bits
            (Type::I8 | Type::U8, _) if n <= 8 => self.trunc_int32(n),
            (src, dst) => unimplemented!("unsupported truncation of {src} to {dst}"),
        }
        self.stack.push(dst.clone());
    }

    /// Zero-extend an unsigned integral value of type `src` to type `dst`
    ///
    /// This function will panic if the source or target types are not unsigned integral types.
    /// Despite its type name, i1 is an unsigned value, because it may only represent 1 or 0.
    ///
    /// Zero-extension is defined in terms of the bitwise representation of the type.
    /// Specifically, the source value will have any excess bits above the bitwidth of
    /// the `src` type either added as zeroes, or set to zero, depending on the `dst` type.
    /// For example, u16 is represented as 16 bits in a single field element, while u64 is
    /// represented as two 32-bit limbs, each in a separate field element. Zero-extending
    /// from u16 to u64 requires only pushing a new element of `Felt::ZERO` on the operand stack.
    /// Since the upper 16 bits of the original 32-bit field element value must already be zero,
    /// we only needed to pad out the representation with an extra zero element to obtain the
    /// corresponding u64.
    ///
    /// NOTE: Field elements do not have a formal bitwise representation. However, types with a
    /// bitwidth of 32 bits or smaller are transparently represented as field elements in the VM,
    /// so zero-extending to felt from such a type is a no-op. Even though a field element is
    /// notionally a 64-bit value in memory, it is not equivalent in range to a 64-bit integer,
    /// so 64-bit integers and above require the use of multiple 32-bit limbs, to provide a two's
    /// complement bitwise representation; so there are no types larger than 32-bits that are
    /// zero-extendable to felt, but are not representable as a felt transparently.
    ///
    /// This function assumes that an integer value of type `src` is on top of the operand stack,
    /// and will ensure a value of type `dst` is on the operand stack after truncation, or that
    /// execution traps.
    pub fn zext(&mut self, dst: &Type) {
        let arg = self.stack.pop().expect("operand stack is empty");
        let src = arg.ty();
        assert!(
            src.is_unsigned_integer() && dst.is_integer(),
            "invalid zero-extension of {src} to {dst}: only unsigned integer-to-integer casts are \
             supported"
        );
        let src_bits = src.size_in_bits() as u32;
        let dst_bits = dst.size_in_bits() as u32;
        assert!(
            src_bits <= dst_bits,
            "invalid zero-extension from {src} to {dst}: cannot zero-extend to a smaller type"
        );
        match (&src, dst) {
            // If the types are equivalent, it's a no-op, but only if they are integers
            (src, dst) if src == dst => (),
            // Zero-extending a u64 to i128 simply requires pushing a 0u64 on the stack
            (Type::U64, Type::U128 | Type::I128) => self.push_u64(0),
            (Type::Felt, Type::U64 | Type::U128 | Type::I128) => self.zext_felt(dst_bits),
            (Type::U32, Type::U64 | Type::I64 | Type::U128 | Type::I128) => {
                self.zext_int32(dst_bits)
            }
            (Type::I1 | Type::U8 | Type::U16, Type::U64 | Type::I64 | Type::U128 | Type::I128) => {
                self.zext_smallint(src_bits, dst_bits)
            }
            // Zero-extending to u32/i32 from smaller integers is a no-op
            (Type::I1 | Type::U8 | Type::U16, Type::U32 | Type::I32) => (),
            // Zero-extending to felt, from types that fit in felt, is a no-op
            (Type::I1 | Type::U8 | Type::U16 | Type::U32, Type::Felt) => (),
            (src, dst) if dst.is_signed_integer() => panic!(
                "invalid zero-extension from {src} to {dst}: value may not fit in range, use \
                 explicit cast instead"
            ),
            (src, dst) => panic!("unsupported zero-extension from {src} to {dst}"),
        }
        self.stack.push(dst.clone());
    }

    /// Sign-extend an integral value of type `src` to type `dst`
    ///
    /// This function will panic if the target type is not a signed integral type.
    /// To extend unsigned integer types to a larger unsigned integer type, use `zext`.
    /// To extend signed integer types to an equal or larger unsigned type, use `cast`.
    ///
    /// Sign-extension is defined in terms of the bitwise representation of the type.
    /// Specifically, the sign bit of the source value will be propagated to all excess
    /// bits added to the representation of `src` to represent `dst`.
    ///
    /// For example, i16 is represented as 16 bits in a single field element, while i64 is
    /// represented as two 32-bit limbs, each in a separate field element. Sign-extending
    /// the i16 value -128, to i64, requires propagating the sign bit value, 1 since -128
    /// is a negative number, to the most significant 32-bits of the input element, as well
    /// as pushing an additional element representing `u32::MAX` on the operand stack. This
    /// gives us a bitwise representation where the most significant 48 bits are all 1s, and
    /// the last 16 bits are the same as the original input value, giving us the i64
    /// representation of -128.
    ///
    /// NOTE: Field elements cannot be sign-extended to i64, you must an explicit cast, as the
    /// range of the field means that it is not guaranteed that the felt will fit in the i64
    /// range.
    ///
    /// This function assumes that an integer value of type `src` is on top of the operand stack,
    /// and will ensure a value of type `dst` is on the operand stack after truncation, or that
    /// execution traps.
    pub fn sext(&mut self, dst: &Type) {
        let arg = self.stack.pop().expect("operand stack is empty");
        let src = arg.ty();
        assert!(
            src.is_integer() && dst.is_signed_integer(),
            "invalid sign-extension of {src} to {dst}: only integer-to-signed-integer casts are \
             supported"
        );
        let src_bits = src.size_in_bits() as u32;
        let dst_bits = dst.size_in_bits() as u32;
        assert!(
            src_bits <= dst_bits,
            "invalid zero-extension from {src} to {dst}: cannot zero-extend to a smaller type"
        );
        match (&src, dst) {
            // If the types are equivalent, it's a no-op
            (src, dst) if src == dst => (),
            (Type::U64 | Type::I64, Type::I128) => self.sext_int64(128),
            (Type::Felt, Type::I64 | Type::I128) => self.sext_felt(dst_bits),
            (Type::I32 | Type::U32, Type::I64 | Type::I128) => self.sext_int32(dst_bits),
            (
                Type::I1 | Type::I8 | Type::U8 | Type::I16 | Type::U16,
                Type::I32 | Type::I64 | Type::I128,
            ) => self.sext_smallint(src_bits, dst_bits),
            (src, dst) => panic!("unsupported sign-extension from {src} to {dst}"),
        }
        self.stack.push(dst.clone());
    }

    pub fn bitcast(&mut self, dst: &Type) {
        let arg = self.stack.pop().expect("operand stack is empty");
        let src = arg.ty();
        assert!(
            src.is_integer() && dst.is_integer(),
            "invalid cast of {src} to {dst}: only integer-to-integer bitcasts are supported"
        );
        self.stack.push(dst.clone());
    }

    /// Convert between two integral types, given as `src` and `dst`,
    /// indicating the direction of the conversion.
    ///
    /// This function will panic if either type is not an integer.
    ///
    /// The specific semantics of a cast are dependent on the pair of types involved,
    /// but the general rules are as follows:
    ///
    /// * Any integer-to-integer cast is allowed
    /// * Casting a signed integer to an unsigned integer will assert that the
    /// input value is unsigned
    /// * Casting a type with a larger range to a type with a smaller one will
    /// assert that the input value fits within that range, e.g. u8 to i8, i16 to i8, etc.
    /// * Casting to a larger unsigned type will use zero-extension
    /// * Casting a signed type to a larger signed type will use sign-extension
    /// * Casting an unsigned type to a larger signed type will use zero-extension
    ///
    /// As a rule, the input value must be representable in the target type, or an
    /// assertion will be raised. Casts are intended to faithfully translate a value
    /// in one type to the same value in another type.
    ///
    /// This function assumes that an integer value of type `src` is on top of the operand stack,
    /// and will ensure a value of type `dst` is on the operand stack after truncation, or that
    /// execution traps.
    pub fn cast(&mut self, dst: &Type) {
        let arg = self.stack.pop().expect("operand stack is empty");
        let src = arg.ty();
        assert!(
            src.is_integer() && dst.is_integer(),
            "invalid cast of {src} to {dst}: only integer-to-integer casts are supported"
        );

        let src_bits = src.size_in_bits() as u32;
        let dst_bits = dst.size_in_bits() as u32;
        match (&src, dst) {
            // u128
            (Type::U128, Type::I128) => self.assert_unsigned_int128(),
            (Type::U128, Type::I64) => self.u128_to_i64(),
            (Type::U128 | Type::I128, Type::U64) => self.int128_to_u64(),
            (Type::U128 | Type::I128, Type::Felt) => self.int128_to_felt(),
            (Type::U128 | Type::I128, Type::U32) => self.int128_to_u32(),
            (Type::U128 | Type::I128, Type::U16 | Type::U8 | Type::I1) => {
                self.int128_to_u32();
                self.int32_to_uint(dst_bits);
            }
            (Type::U128, Type::I32) => {
                self.int128_to_u32();
                self.assert_unsigned_int32();
            }
            (Type::U128, Type::I16 | Type::I8) => {
                self.int128_to_u32();
                self.int32_to_int(dst_bits);
            }
            // i128
            (Type::I128, Type::I64) => self.i128_to_i64(),
            (Type::I128, Type::I32 | Type::I16 | Type::I8) => {
                self.i128_to_i64();
                self.i64_to_int(dst_bits);
            }
            // i64
            (Type::I64, Type::I128) => self.sext_int64(128),
            (Type::I64, Type::U128) => self.zext_int64(128),
            (Type::I64, Type::U64) => self.assert_unsigned_int64(),
            (Type::I64, Type::Felt) => self.i64_to_felt(),
            (Type::I64, Type::U32 | Type::U16 | Type::U8 | Type::I1) => {
                self.assert_unsigned_int64();
                self.u64_to_uint(dst_bits);
            }
            (Type::I64, Type::I32 | Type::I16 | Type::I8) => {
                self.i64_to_int(dst_bits);
            }
            // u64
            (Type::U64, Type::I128 | Type::U128) => self.zext_int64(128),
            (Type::U64, Type::I64) => self.assert_i64(),
            (Type::U64, Type::Felt) => self.u64_to_felt(),
            (Type::U64, Type::U32 | Type::U16 | Type::U8 | Type::I1) => {
                self.u64_to_uint(dst_bits);
            }
            (Type::U64, Type::I32 | Type::I16 | Type::I8) => {
                // Convert to N bits as unsigned
                self.u64_to_uint(dst_bits);
                // Verify that the input value is still unsigned
                self.assert_unsigned_smallint(dst_bits);
            }
            // felt
            (Type::Felt, Type::I64 | Type::I128) => self.sext_felt(dst_bits),
            (Type::Felt, Type::U128) => self.zext_felt(dst_bits),
            (Type::Felt, Type::U64) => self.felt_to_u64(),
            (Type::Felt, Type::U32 | Type::U16 | Type::U8 | Type::I1) => {
                self.felt_to_uint(dst_bits);
            }
            (Type::Felt, Type::I32 | Type::I16 | Type::I8) => {
                self.felt_to_int(dst_bits);
            }
            // u32
            (Type::U32, Type::I64 | Type::U64 | Type::I128) => self.zext_int32(dst_bits),
            (Type::U32, Type::I32) => self.assert_unsigned_int32(),
            (Type::U32, Type::U16 | Type::U8 | Type::I1) => {
                self.int32_to_uint(dst_bits);
            }
            (Type::U32, Type::I16 | Type::I8) => self.int32_to_int(dst_bits),
            // i32
            (Type::I32, Type::I64 | Type::I128) => self.sext_int32(dst_bits),
            (Type::I32, Type::U64) => {
                self.assert_unsigned_int32();
                self.emit(Op::PushU32(0));
            }
            (Type::I32, Type::U32) => {
                self.assert_unsigned_int32();
            }
            (Type::I32, Type::U16 | Type::U8 | Type::I1) => {
                self.int32_to_uint(dst_bits);
            }
            (Type::I32, Type::I16 | Type::I8) => self.int32_to_int(dst_bits),
            // i8/i16
            (Type::I8 | Type::I16, Type::I32 | Type::I64 | Type::I128) => {
                self.sext_smallint(src_bits, dst_bits);
            }
            (Type::I8 | Type::I16, Type::U32 | Type::U64) => {
                self.assert_unsigned_smallint(src_bits);
                self.zext_smallint(src_bits, dst_bits);
            }
            (Type::I16, Type::U16) | (Type::I8, Type::U8) => {
                self.assert_unsigned_smallint(src_bits);
            }
            (Type::I16, Type::U8 | Type::I1) => self.int32_to_int(dst_bits),
            (Type::I16, Type::I8) => self.int32_to_int(dst_bits),
            (Type::I8, Type::I1) => {
                self.emit_all(&[
                    // Assert that input is either 0 or 1
                    //
                    // NOTE: The comparison here is unsigned, so the sign
                    // bit being set will make the i8 larger than 0 or 1
                    Op::Dup(0),
                    Op::PushU32(2),
                    Op::Lt,
                    Op::Assert,
                ]);
            }
            // i1
            (Type::I1, _) => self.zext_smallint(src_bits, dst_bits),
            (src, dst) => unimplemented!("unsupported cast from {src} to {dst}"),
        }
        self.stack.push(dst.clone());
    }

    /// Cast `arg` to a pointer value
    pub fn inttoptr(&mut self, ty: &Type) {
        assert!(ty.is_pointer(), "exected pointer typed argument");
        // For now, we're strict about the types of values we'll allow casting from
        let arg = self.stack.pop().expect("operand stack is empty");
        match arg.ty() {
            // We allow i32 here because Wasm uses it
            Type::U32 | Type::I32 => {
                self.stack.push(ty.clone());
            }
            Type::Felt => {
                self.emit(Op::U32Assert);
                self.stack.push(ty.clone());
            }
            int => panic!("invalid inttoptr cast: cannot cast value of type {int} to {ty}"),
        }
    }

    /// Check if an integral value on the operand stack is an odd number.
    ///
    /// The result is placed on the stack as a boolean value.
    ///
    /// This operation consumes the input operand.
    pub fn is_odd(&mut self) {
        let arg = self.stack.pop().expect("operand stack is empty");
        match arg.ty() {
            // For both signed and unsigned types,
            // values <= bitwidth of a felt can use
            // the native instruction because the sign
            // bit does not change whether the value is
            // odd or not
            Type::I1
            | Type::U8
            | Type::I8
            | Type::U16
            | Type::I16
            | Type::U32
            | Type::I32
            | Type::Felt => {
                self.emit(Op::IsOdd);
            }
            // For i64/u64, we use the native instruction
            // on the lower limb to check for odd/even
            Type::I64 | Type::U64 => {
                self.emit_all(&[Op::Drop, Op::IsOdd]);
            }
            // For i128, same as above, but more elements are dropped
            Type::I128 | Type::U128 => {
                self.emit_n(3, Op::Drop);
                self.emit(Op::IsOdd);
            }
            Type::F64 => {
                unimplemented!("is_odd support for floating-point values is not yet implemented")
            }
            ty => panic!("expected integral type for is_odd opcode, got {ty}"),
        }
        self.stack.push(Type::I1);
    }

    /// Compute the integral base-2 logarithm of the value on top of the operand stack, and
    /// place the result back on the operand stack as a u32 value.
    ///
    /// This operation consumes the input operand.
    pub fn ilog2(&mut self) {
        let ty = self.stack.peek().expect("operand stack is empty").ty();
        match &ty {
            Type::Felt => self.emit(Op::Ilog2),
            Type::I128 | Type::U128 | Type::I64 | Type::U64 => {
                // Compute the number of leading zeros
                //
                // NOTE: This function handles popping the input and pushing
                // a u32 result on the stack for us, so we can omit any stack
                // manipulation here.
                self.clz();
                let bits = ty.size_in_bits();
                // ilog2 is bits - clz - 1
                self.emit_all(&[
                    Op::PushU8(bits as u8),
                    Op::Swap(1),
                    Op::Sub,
                    Op::U32OverflowingSubImm(1),
                    Op::Assertz,
                ]);
            }
            Type::I32 | Type::U32 | Type::I16 | Type::U16 | Type::I8 | Type::U8 => {
                let _ = self.stack.pop();
                self.emit_all(&[
                    // Compute ilog2 on the advice stack
                    Op::Ilog2,
                    // Drop the operand
                    Op::Drop,
                    // Move the result to the operand stack
                    Op::AdvPush(1),
                ]);
                self.stack.push(Type::U32);
            }
            Type::I1 => {
                // 2^0 == 1
                let _ = self.stack.pop();
                self.emit_all(&[Op::Drop, Op::PushU8(0)]);
                self.stack.push(Type::U32);
            }
            ty if !ty.is_integer() => {
                panic!("invalid ilog2 on {ty}: only integral types are supported")
            }
            ty => unimplemented!("ilog2 for {ty} is not supported"),
        }
    }

    /// Count the number of non-zero bits in the integral value on top of the operand stack,
    /// and place the count back on the stack as a u32 value.
    ///
    /// This operation consumes the input operand.
    pub fn popcnt(&mut self) {
        let arg = self.stack.pop().expect("operand stack is empty");
        match arg.ty() {
            Type::I128 | Type::U128 => {
                self.emit_all(&[
                    // [x3, x2, x1, x0]
                    Op::U32Popcnt,
                    // [popcnt3, x2, x1, x0]
                    Op::Swap(1),
                    // [x2, popcnt3, x1, x0]
                    Op::U32Popcnt,
                    // [popcnt2, popcnt3, x1, x0]
                    Op::Add,
                    // [popcnt_hi, x1, x0]
                    Op::Movdn(2),
                    // [x1, x0, popcnt]
                    Op::U32Popcnt,
                    // [popcnt1, x0, popcnt]
                    Op::Swap(1),
                    // [x0, popcnt1, popcnt]
                    Op::U32Popcnt,
                    // [popcnt0, popcnt1, popcnt]
                    //
                    // This last instruction adds all three values together mod 2^32
                    Op::U32WrappingAdd3,
                ]);
            }
            Type::I64 | Type::U64 => {
                self.emit_all(&[
                    // Get popcnt of high bits
                    Op::U32Popcnt,
                    // Swap to low bits and repeat
                    Op::Swap(1),
                    Op::U32Popcnt,
                    // Add both counts to get the total count
                    Op::Add,
                ]);
            }
            Type::I32 | Type::U32 | Type::I16 | Type::U16 | Type::I8 | Type::U8 | Type::I1 => {
                self.emit(Op::U32Popcnt);
            }
            ty if !ty.is_integer() => {
                panic!("invalid popcnt on {ty}: only integral types are supported")
            }
            ty => unimplemented!("popcnt for {ty} is not supported"),
        }
        self.stack.push(Type::U32);
    }

    /// Count the number of leading zero bits in the integral value on top of the operand stack,
    /// and place the count back on the stack as a u32 value.
    ///
    /// This operation is implemented so that it consumes the input operand.
    pub fn clz(&mut self) {
        let arg = self.stack.pop().expect("operand stack is empty");
        match arg.ty() {
            Type::I128 | Type::U128 => {
                let u64_clz = "::std::math::u64::clz".parse().unwrap();
                // We decompose the 128-bit value into two 64-bit limbs, and use the standard
                // library intrinsics to get the count for those limbs. We then add the count
                // for the low bits to that of the high bits, if the high bits are all zero,
                // otherwise we take just the high bit count.
                self.emit_all(&[
                    // Count leading zeros in the high bits
                    Op::Exec(u64_clz), // [hi_clz, lo_hi, lo_lo]
                    // Count leading zeros in the low bits
                    Op::Movup(2),      // [lo_lo, hi_clz, lo_hi]
                    Op::Movup(2),      // [lo_hi, lo_lo, hi_clz]
                    Op::Exec(u64_clz), // [lo_clz, hi_clz]
                    // Add the low bit leading zeros to those of the high bits, if the high bits
                    // are all zeros; otherwise return only the high bit count
                    Op::PushU32(0),           // [0, lo_clz, hi_clz]
                    Op::Dup(2),               // [hi_clz, 0, lo_clz, hi_clz]
                    Op::LtImm(Felt::new(32)), // [hi_clz < 32, 0, lo_clz, hi_clz]
                    Op::Cdrop,                // [hi_clz < 32 ? 0 : lo_clz, hi_clz]
                    Op::Add,
                ]);
            }
            Type::I64 | Type::U64 => {
                self.emit(Op::Exec("::std::math::u64::clz".parse().unwrap()));
            }
            Type::I32 | Type::U32 => {
                self.emit(Op::U32Clz);
            }
            Type::I16 | Type::U16 => {
                // There are always 16 leading zeroes from the perspective of the
                // MASM u32clz instruction for values of (i|u)16 type, so subtract
                // that from the count
                self.emit_all(&[
                    Op::U32Clz,
                    // Subtract the excess bits from the count
                    Op::U32WrappingSubImm(16),
                ]);
            }
            Type::I8 | Type::U8 => {
                // There are always 24 leading zeroes from the perspective of the
                // MASM u32clz instruction for values of (i|u)8 type, so subtract
                // that from the count
                self.emit_all(&[
                    Op::U32Clz,
                    // Subtract the excess bits from the count
                    Op::U32WrappingSubImm(24),
                ]);
            }
            Type::I1 => {
                // There is exactly one leading zero if false, or zero if true
                self.emit(Op::EqImm(Felt::ZERO));
            }
            ty if !ty.is_integer() => {
                panic!("invalid clz on {ty}: only integral types are supported")
            }
            ty => unimplemented!("clz for {ty} is not supported"),
        }
        self.stack.push(Type::U32);
    }

    /// Count the number of leading one bits in the integral value on top of the operand stack,
    /// and place the count back on the stack as a u32 value.
    ///
    /// This operation is implemented so that it consumes the input operand.
    pub fn clo(&mut self) {
        let arg = self.stack.pop().expect("operand stack is empty");
        match arg.ty() {
            // The implementation here is effectively the same as `clz`, just with minor adjustments
            Type::I128 | Type::U128 => {
                let u64_clo = "::std::math::u64::clo".parse().unwrap();
                // We decompose the 128-bit value into two 64-bit limbs, and use the standard
                // library intrinsics to get the count for those limbs. We then add the count
                // for the low bits to that of the high bits, if the high bits are all one,
                // otherwise we take just the high bit count.
                self.emit_all(&[
                    // Count leading ones in the high bits
                    Op::Exec(u64_clo), // [hi_clo, lo_hi, lo_lo]
                    // Count leading ones in the low bits
                    Op::Movup(2),      // [lo_lo, hi_clo, lo_hi]
                    Op::Movup(2),      // [lo_hi, lo_lo, hi_clo]
                    Op::Exec(u64_clo), // [lo_clo, hi_clo]
                    // Add the low bit leading ones to those of the high bits, if the high bits
                    // are all one; otherwise return only the high bit count
                    Op::PushU32(0),           // [0, lo_clo, hi_clo]
                    Op::Dup(2),               // [hi_clo, 0, lo_clo, hi_clo]
                    Op::LtImm(Felt::new(32)), // [hi_clo < 32, 0, lo_clo, hi_clo]
                    Op::Cdrop,                // [hi_clo < 32 ? 0 : lo_clo, hi_clo]
                    Op::Add,
                ]);
            }
            Type::I64 | Type::U64 => self.emit(Op::Exec("::std::math::u64::clo".parse().unwrap())),
            Type::I32 | Type::U32 => {
                self.emit(Op::U32Clo);
            }
            Type::I16 | Type::U16 => {
                // There are always 16 leading zeroes from the perspective of the
                // MASM u32clo instruction for values of (i|u)16 type, so to get
                // the correct count, we need to bitwise-OR in a 16 bits of leading
                // ones, then subtract that from the final count.
                self.emit_all(&[
                    // OR in the leading 16 ones
                    Op::PushU32(u32::MAX << 16),
                    Op::U32Or,
                    // Obtain the count
                    Op::U32Clo,
                    // Subtract the leading bits we added from the count
                    Op::U32WrappingSubImm(16),
                ]);
            }
            Type::I8 | Type::U8 => {
                // There are always 24 leading zeroes from the perspective of the
                // MASM u32clo instruction for values of (i|u)8 type, so as with the
                // 16-bit values, we need to bitwise-OR in 24 bits of leading ones,
                // then subtract them from the final count.
                self.emit_all(&[
                    // OR in the leading 24 ones
                    Op::PushU32(u32::MAX << 8),
                    Op::U32Or,
                    // Obtain the count
                    Op::U32Clo,
                    // Subtract the excess bits from the count
                    Op::U32WrappingSubImm(24),
                ]);
            }
            Type::I1 => {
                // There is exactly one leading one if true, or zero if false
                self.emit(Op::EqImm(Felt::ONE));
            }
            ty if !ty.is_integer() => {
                panic!("invalid clo on {ty}: only integral types are supported")
            }
            ty => unimplemented!("clo for {ty} is not supported"),
        }
        self.stack.push(Type::U32);
    }

    /// Count the number of trailing zero bits in the integral value on top of the operand stack,
    /// and place the count back on the stack as a u32 value.
    ///
    /// This operation is implemented so that it consumes the input operand.
    pub fn ctz(&mut self) {
        let arg = self.stack.pop().expect("operand stack is empty");
        match arg.ty() {
            Type::I128 | Type::U128 => {
                let u64_ctz = "::std::math::u64::ctz".parse().unwrap();
                // We decompose the 128-bit value into two 64-bit limbs, and use the standard
                // library intrinsics to get the count for those limbs. We then add the count
                // for the low bits to that of the high bits, if the high bits are all one,
                // otherwise we take just the high bit count.
                self.emit_all(&[
                    // Count trailing zeros in the high bits
                    Op::Exec(u64_ctz), // [hi_ctz, lo_hi, lo_lo]
                    // Count trailing zeros in the low bits
                    Op::Movup(2),      // [lo_lo, hi_ctz, lo_hi]
                    Op::Movup(2),      // [lo_hi, lo_lo, hi_ctz]
                    Op::Exec(u64_ctz), // [lo_ctz, hi_ctz]
                    // Add the high bit trailing zeros to those of the low bits, if the low bits
                    // are all zero; otherwise return only the low bit count
                    Op::Swap(1),
                    Op::PushU32(0),           // [0, hi_ctz, lo_ctz]
                    Op::Dup(2),               // [lo_ctz, 0, hi_ctz, lo_ctz]
                    Op::LtImm(Felt::new(32)), // [lo_ctz < 32, 0, hi_ctz, lo_ctz]
                    Op::Cdrop,                // [lo_ctz < 32 ? 0 : hi_ctz, lo_ctz]
                    Op::Add,
                ]);
            }
            Type::I64 | Type::U64 => self.emit(Op::Exec("::std::math::u64::ctz".parse().unwrap())),
            Type::I32 | Type::U32 => self.emit(Op::U32Ctz),
            Type::I16 | Type::U16 => {
                // Clamp the total number of trailing zeros to 16
                self.emit_all(&[
                    // Obtain the count
                    Op::U32Ctz,
                    // Clamp to 16
                    //   operand_stack: [16, ctz]
                    Op::PushU8(16),
                    //   operand_stack: [ctz, 16, ctz]
                    Op::Dup(1),
                    //   operand_stack: [ctz >= 16, 16, ctz]
                    Op::GteImm(Felt::new(16)),
                    //   operand_stack: [actual_ctz]
                    Op::Cdrop,
                ]);
            }
            Type::I8 | Type::U8 => {
                // Clamp the total number of trailing zeros to 8
                self.emit_all(&[
                    // Obtain the count
                    Op::U32Ctz,
                    // Clamp to 8
                    //   operand_stack: [8, ctz]
                    Op::PushU8(8),
                    //   operand_stack: [ctz, 8, ctz]
                    Op::Dup(1),
                    //   operand_stack: [ctz >= 8, 8, ctz]
                    Op::GteImm(Felt::new(8)),
                    //   operand_stack: [actual_ctz]
                    Op::Cdrop,
                ]);
            }
            Type::I1 => {
                // There is exactly one trailing zero if false, or zero if true
                self.emit(Op::EqImm(Felt::ZERO));
            }
            ty if !ty.is_integer() => {
                panic!("invalid ctz on {ty}: only integral types are supported")
            }
            ty => unimplemented!("ctz for {ty} is not supported"),
        }
        self.stack.push(Type::U32);
    }

    /// Count the number of trailing one bits in the integral value on top of the operand stack,
    /// and place the count back on the stack as a u32 value.
    ///
    /// This operation is implemented so that it consumes the input operand.
    pub fn cto(&mut self) {
        let arg = self.stack.pop().expect("operand stack is empty");
        match arg.ty() {
            Type::I128 | Type::U128 => {
                let u64_cto = "::std::math::u64::cto".parse().unwrap();
                // We decompose the 128-bit value into two 64-bit limbs, and use the standard
                // library intrinsics to get the count for those limbs. We then add the count
                // for the low bits to that of the high bits, if the high bits are all one,
                // otherwise we take just the high bit count.
                self.emit_all(&[
                    // Count trailing ones in the high bits
                    Op::Exec(u64_cto), // [hi_cto, lo_hi, lo_lo]
                    // Count trailing ones in the low bits
                    Op::Movup(2),      // [lo_lo, hi_cto, lo_hi]
                    Op::Movup(2),      // [lo_hi, lo_lo, hi_cto]
                    Op::Exec(u64_cto), // [lo_cto, hi_cto]
                    // Add the high bit trailing ones to those of the low bits, if the low bits
                    // are all one; otherwise return only the low bit count
                    Op::Swap(1),
                    Op::PushU32(0),           // [0, hi_cto, lo_cto]
                    Op::Dup(2),               // [lo_cto, 0, hi_cto, lo_cto]
                    Op::LtImm(Felt::new(32)), // [lo_cto < 32, 0, hi_cto, lo_cto]
                    Op::Cdrop,                // [lo_cto < 32 ? 0 : hi_cto, lo_cto]
                    Op::Add,
                ]);
            }
            Type::I64 | Type::U64 => self.emit(Op::Exec("::std::math::u64::cto".parse().unwrap())),
            Type::I32 | Type::U32 | Type::I16 | Type::U16 | Type::I8 | Type::U8 => {
                // The number of trailing ones is de-facto clamped by the bitwidth of
                // the value, since all of the padding bits are leading zeros.
                self.emit(Op::U32Cto)
            }
            Type::I1 => {
                // There is exactly one trailing one if true, or zero if false
                self.emit(Op::EqImm(Felt::ONE));
            }
            ty if !ty.is_integer() => {
                panic!("invalid cto on {ty}: only integral types are supported")
            }
            ty => unimplemented!("cto for {ty} is not supported"),
        }
        self.stack.push(Type::U32);
    }

    /// Invert the bitwise representation of the integral value on top of the operand stack.
    ///
    /// This has the effect of changing all 1 bits to 0s, and all 0 bits to 1s.
    ///
    /// This operation consumes the input operand.
    pub fn bnot(&mut self) {
        let arg = self.stack.pop().expect("operand stack is empty");
        let ty = arg.ty();
        match &ty {
            Type::I1 => self.emit(Op::Not),
            Type::I8
            | Type::U8
            | Type::I16
            | Type::U16
            | Type::I32
            | Type::U32
            | Type::I64
            | Type::U64
            | Type::I128
            | Type::U128 => {
                let num_elements = ty.size_in_bits() / 32;
                match num_elements {
                    0 | 1 => {
                        self.emit(Op::U32Not);
                    }
                    2 => {
                        self.emit_repeat(2, &[Op::Swap(1), Op::U32Not]);
                    }
                    n => {
                        self.emit_template(n, |n| [Op::Movup(n as u8), Op::U32Not]);
                    }
                }
            }
            ty if !ty.is_integer() => {
                panic!("invalid bnot on {ty}, only integral types are supported")
            }
            ty => unimplemented!("bnot for {ty} is not supported"),
        }
        self.stack.push(ty);
    }

    /// Invert the boolean value on top of the operand stack.
    ///
    /// This operation consumes the input operand.
    pub fn not(&mut self) {
        let arg = self.stack.pop().expect("operand stack is empty");
        assert_eq!(arg.ty(), Type::I1, "logical NOT requires a boolean value");
        self.emit(Op::Not);
        self.stack.push(Type::I1);
    }

    /// Compute 2^N, where N is the integral value on top of the operand stack, as
    /// a value of the same type as the input.
    ///
    /// The input value must be < 64, or execution will trap.
    ///
    /// This operation consumes the input operand.
    pub fn pow2(&mut self) {
        let arg = self.stack.pop().expect("operand stack is empty");
        let ty = arg.ty();
        match &ty {
            Type::U64 => {
                self.emit_all(&[
                    // Assert that the high bits are zero
                    Op::Assertz,
                    // This asserts if value > 63, thus result is guaranteed to fit in u64
                    Op::Pow2,
                    // Obtain the u64 representation by splitting the felt result
                    Op::U32Split,
                ]);
            }
            Type::Felt => {
                self.emit(Op::Pow2);
            }
            Type::U32 => {
                self.emit_all(&[Op::Pow2, Op::U32Assert]);
            }
            Type::I32 => {
                self.emit(Op::Exec("::intrinsics::i32::pow2".parse().unwrap()));
            }
            Type::U8 | Type::U16 => {
                self.emit_all(&[Op::Pow2, Op::U32Assert]);
                // Cast u32 to u8/u16
                self.int32_to_uint(ty.size_in_bits() as u32);
            }
            ty if !ty.is_unsigned_integer() => {
                panic!(
                    "invalid unary operand: pow2 only permits unsigned integer operands, got {ty}"
                )
            }
            ty => unimplemented!("pow2 for {ty} is not supported"),
        }
        self.stack.push(ty);
    }

    /// Increment the operand on top of the stack by 1.
    ///
    /// The input value must be an integer, and overflow has wrapping semantics.
    ///
    /// This operation consumes the input operand.
    pub fn incr(&mut self) {
        let arg = self.stack.pop().expect("operand stack is empty");
        let ty = arg.ty();
        match &ty {
            // For this specific case, wrapping u64 arithmetic works for both i64/u64
            Type::I64 | Type::U64 => {
                self.push_u64(1);
                self.add_u64(Overflow::Wrapping);
            }
            Type::Felt => {
                self.emit(Op::Incr);
            }
            // For this specific case, wrapping u32 arithmetic works for both i32/u32
            Type::I32 | Type::U32 => {
                self.add_imm_u32(1, Overflow::Wrapping);
            }
            // We need to wrap the result for smallint types
            Type::I8 | Type::U8 | Type::I16 | Type::U16 => {
                let bits = ty.size_in_bits() as u32;
                self.add_imm_u32(1, Overflow::Wrapping);
                self.unchecked_mod_imm_u32(2u32.pow(bits));
            }
            ty if !ty.is_integer() => {
                panic!("invalid unary operand: incr requires an integer operand, got {ty}")
            }
            ty => unimplemented!("incr for {ty} is not supported"),
        }
        self.stack.push(ty);
    }

    /// Compute the modular multiplicative inverse of the operand on top of the stack, `n`, i.e.
    /// `n^-1 mod P`.
    ///
    /// This operation consumes the input operand.
    pub fn inv(&mut self) {
        let arg = self.pop().expect("operand stack is empty");
        let ty = arg.ty();
        match &ty {
            Type::Felt => {
                self.emit(Op::Inv);
            }
            ty if !ty.is_integer() => {
                panic!("invalid unary operand: inv requires an integer, got {ty}")
            }
            ty => unimplemented!("inv for {ty} is not supported"),
        }
        self.push(ty);
    }

    /// Compute the modular negation of the operand on top of the stack, `n`, i.e. `-n mod P`.
    ///
    /// This operation consumes the input operand.
    pub fn neg(&mut self) {
        let arg = self.pop().expect("operand stack is empty");
        let ty = arg.ty();
        match &ty {
            Type::Felt => {
                self.emit(Op::Neg);
            }
            ty if !ty.is_integer() => {
                panic!("invalid unary operand: neg requires an integer, got {ty}")
            }
            ty => unimplemented!("neg for {ty} is not supported"),
        }
        self.push(ty);
    }
}
