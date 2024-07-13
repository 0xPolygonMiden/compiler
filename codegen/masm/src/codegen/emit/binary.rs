use midenc_hir::{assert_matches, Felt, Immediate, Overflow, Type};

use super::OpEmitter;
use crate::masm::Op;

impl<'a> OpEmitter<'a> {
    pub fn eq(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected eq operands to be the same type");
        match &ty {
            Type::I128 | Type::U128 => {
                self.eq_i128();
            }
            Type::I64 | Type::U64 => {
                self.eq_int64();
            }
            Type::Felt
            | Type::Ptr(_)
            | Type::U32
            | Type::I32
            | Type::U16
            | Type::I16
            | Type::I8
            | Type::U8
            | Type::I1 => {
                self.emit(Op::Eq);
            }
            ty => unimplemented!("eq is not yet implemented for {ty}"),
        }
        self.push(Type::I1);
    }

    pub fn eq_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected eq operands to be the same type");
        match &ty {
            Type::I128 | Type::U128 => {
                self.push_immediate(imm);
                self.eq_i128();
            }
            Type::I64 | Type::U64 => {
                self.push_immediate(imm);
                self.eq_int64();
            }
            Type::Felt | Type::Ptr(_) | Type::U32 | Type::U16 | Type::U8 | Type::I1 => {
                self.emit(Op::EqImm(imm.as_felt().unwrap()));
            }
            Type::I32 | Type::I16 | Type::I8 => {
                self.emit(Op::EqImm(Felt::new(imm.as_i32().unwrap() as u32 as u64)));
            }
            ty => unimplemented!("eq is not yet implemented for {ty}"),
        }
        self.push(Type::I1);
    }

    pub fn neq(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected neq operands to be the same type");
        match &ty {
            Type::I128 | Type::U128 => {
                self.neq_i128();
            }
            Type::I64 | Type::U64 => self.neq_int64(),
            Type::Felt
            | Type::Ptr(_)
            | Type::U32
            | Type::I32
            | Type::U16
            | Type::I16
            | Type::I8
            | Type::U8
            | Type::I1 => {
                self.emit(Op::Neq);
            }
            ty => unimplemented!("neq is not yet implemented for {ty}"),
        }
        self.push(Type::I1);
    }

    pub fn neq_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected neq operands to be the same type");
        match &ty {
            Type::I128 | Type::U128 => {
                self.push_immediate(imm);
                self.neq_i128();
            }
            Type::I64 | Type::U64 => {
                self.push_immediate(imm);
                self.neq_int64()
            }
            Type::Felt | Type::Ptr(_) | Type::U32 | Type::U16 | Type::U8 | Type::I1 => {
                self.emit(Op::NeqImm(imm.as_felt().unwrap()));
            }
            Type::I32 | Type::I16 | Type::I8 => {
                self.emit(Op::NeqImm(Felt::new(imm.as_i32().unwrap() as u32 as u64)));
            }
            ty => unimplemented!("neq is not yet implemented for {ty}"),
        }
        self.push(Type::I1);
    }

    pub fn gt(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected gt operands to be the same type");
        match &ty {
            Type::Felt => {
                self.emit(Op::Gt);
            }
            Type::I64 | Type::U64 => {
                self.gt_u64();
            }
            Type::U32 | Type::U16 | Type::U8 | Type::I1 => {
                self.emit(Op::U32Gt);
            }
            Type::I32 => self.emit(Op::Exec("intrinsics::i32::is_gt".parse().unwrap())),
            ty => unimplemented!("gt is not yet implemented for {ty}"),
        }
        self.push(Type::I1);
    }

    pub fn gt_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected gt operands to be the same type");
        match &ty {
            Type::Felt => {
                self.emit(Op::GtImm(imm.as_felt().unwrap()));
            }
            Type::U64 => {
                self.push_u64(imm.as_u64().unwrap());
                self.gt_u64();
            }
            Type::I64 => {
                self.push_i64(imm.as_i64().unwrap());
                self.gt_u64();
            }
            Type::U32 | Type::U16 | Type::U8 | Type::I1 => {
                self.emit_all(&[Op::PushU32(imm.as_u32().unwrap()), Op::U32Gt]);
            }
            Type::I32 => {
                self.emit_all(&[
                    Op::PushU32(imm.as_i32().unwrap() as u32),
                    Op::Exec("intrinsics::i32::is_gt".parse().unwrap()),
                ]);
            }
            ty => unimplemented!("gt is not yet implemented for {ty}"),
        }
        self.push(Type::I1);
    }

    pub fn gte(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected gte operands to be the same type");
        match &ty {
            Type::Felt => {
                self.emit(Op::Gte);
            }
            Type::I64 | Type::U64 => {
                self.gte_u64();
            }
            Type::U32 | Type::U16 | Type::U8 | Type::I1 => {
                self.emit(Op::U32Gte);
            }
            Type::I32 => self.emit(Op::Exec("intrinsics::i32::is_gte".parse().unwrap())),
            ty => unimplemented!("gte is not yet implemented for {ty}"),
        }
        self.push(Type::I1);
    }

    pub fn gte_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected gte operands to be the same type");
        match &ty {
            Type::Felt => {
                self.emit(Op::GteImm(imm.as_felt().unwrap()));
            }
            Type::U64 => {
                self.push_u64(imm.as_u64().unwrap());
                self.gte_u64();
            }
            Type::I64 => {
                self.push_i64(imm.as_i64().unwrap());
                self.gte_u64();
            }
            Type::U32 | Type::U16 | Type::U8 | Type::I1 => {
                self.emit_all(&[Op::PushU32(imm.as_u32().unwrap()), Op::U32Gte]);
            }
            Type::I32 => {
                self.emit_all(&[
                    Op::PushU32(imm.as_i32().unwrap() as u32),
                    Op::Exec("intrinsics::i32::is_gte".parse().unwrap()),
                ]);
            }
            ty => unimplemented!("gte is not yet implemented for {ty}"),
        }
        self.push(Type::I1);
    }

    pub fn lt(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected lt operands to be the same type");
        match &ty {
            Type::Felt => {
                self.emit(Op::Lt);
            }
            Type::I64 | Type::U64 => {
                self.lt_u64();
            }
            Type::U32 | Type::U16 | Type::U8 | Type::I1 => {
                self.emit(Op::U32Lt);
            }
            Type::I32 => self.emit(Op::Exec("intrinsics::i32::is_lt".parse().unwrap())),
            ty => unimplemented!("lt is not yet implemented for {ty}"),
        }
        self.push(Type::I1);
    }

    pub fn lt_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected lt operands to be the same type");
        match &ty {
            Type::Felt => {
                self.emit(Op::LtImm(imm.as_felt().unwrap()));
            }
            Type::U64 => {
                self.push_u64(imm.as_u64().unwrap());
                self.lt_u64();
            }
            Type::I64 => {
                self.push_i64(imm.as_i64().unwrap());
                self.lt_u64();
            }
            Type::U32 | Type::U16 | Type::U8 | Type::I1 => {
                self.emit_all(&[Op::PushU32(imm.as_u32().unwrap()), Op::U32Lt]);
            }
            Type::I32 => {
                self.emit_all(&[
                    Op::PushU32(imm.as_i32().unwrap() as u32),
                    Op::Exec("intrinsics::i32::is_lt".parse().unwrap()),
                ]);
            }
            ty => unimplemented!("lt is not yet implemented for {ty}"),
        }
        self.push(Type::I1);
    }

    pub fn lte(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected lte operands to be the same type");
        match &ty {
            Type::Felt => {
                self.emit(Op::Lte);
            }
            Type::I64 | Type::U64 => {
                self.lte_u64();
            }
            Type::U32 | Type::U16 | Type::U8 | Type::I1 => {
                self.emit(Op::U32Lte);
            }
            Type::I32 => self.emit(Op::Exec("intrinsics::i32::is_lte".parse().unwrap())),
            ty => unimplemented!("lte is not yet implemented for {ty}"),
        }
        self.push(Type::I1);
    }

    pub fn lte_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected lte operands to be the same type");
        match &ty {
            Type::Felt => {
                self.emit(Op::LteImm(imm.as_felt().unwrap()));
            }
            Type::U64 => {
                self.push_u64(imm.as_u64().unwrap());
                self.lte_u64();
            }
            Type::I64 => {
                self.push_i64(imm.as_i64().unwrap());
                self.lte_u64();
            }
            Type::U32 | Type::U16 | Type::U8 | Type::I1 => {
                self.emit_all(&[Op::PushU32(imm.as_u32().unwrap()), Op::U32Lte]);
            }
            Type::I32 => {
                self.emit_all(&[
                    Op::PushU32(imm.as_i32().unwrap() as u32),
                    Op::Exec("intrinsics::i32::is_lte".parse().unwrap()),
                ]);
            }
            ty => unimplemented!("lte is not yet implemented for {ty}"),
        }
        self.push(Type::I1);
    }

    pub fn add(&mut self, overflow: Overflow) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected add operands to be the same type");
        match &ty {
            Type::Felt => {
                self.emit(Op::Add);
            }
            Type::U64 => {
                self.add_u64(overflow);
            }
            Type::U32 => {
                self.add_u32(overflow);
            }
            Type::I32 => {
                self.add_i32(overflow);
            }
            ty @ (Type::U16 | Type::U8 | Type::I1) => {
                self.add_uint(ty.size_in_bits() as u32, overflow);
            }
            ty => unimplemented!("add is not yet implemented for {ty}"),
        }
        self.push(ty);
        if overflow.is_overflowing() {
            self.push(Type::I1);
        }
    }

    pub fn add_imm(&mut self, imm: Immediate, overflow: Overflow) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected add operands to be the same type");
        match &ty {
            Type::Felt if imm == 1 => self.emit(Op::Incr),
            Type::Felt => {
                self.emit(Op::AddImm(imm.as_felt().unwrap()));
            }
            Type::U64 => {
                self.push_immediate(imm);
                self.add_u64(overflow);
            }
            Type::U32 => {
                self.add_imm_u32(imm.as_u32().unwrap(), overflow);
            }
            Type::I32 => {
                self.add_imm_i32(imm.as_i32().unwrap(), overflow);
            }
            ty @ (Type::U16 | Type::U8 | Type::I1) => {
                self.add_imm_uint(imm.as_u32().unwrap(), ty.size_in_bits() as u32, overflow);
            }
            ty => unimplemented!("add is not yet implemented for {ty}"),
        }
        self.push(ty);
        if overflow.is_overflowing() {
            self.push(Type::I1);
        }
    }

    pub fn sub(&mut self, overflow: Overflow) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected sub operands to be the same type");
        match &ty {
            Type::Felt => {
                self.emit(Op::Sub);
            }
            Type::U64 => {
                self.sub_u64(overflow);
            }
            Type::U32 => {
                self.sub_u32(overflow);
            }
            Type::I32 => {
                self.sub_i32(overflow);
            }
            ty @ (Type::U16 | Type::U8 | Type::I1) => {
                self.sub_uint(ty.size_in_bits() as u32, overflow);
            }
            ty => unimplemented!("sub is not yet implemented for {ty}"),
        }
        self.push(ty);
        if overflow.is_overflowing() {
            self.push(Type::I1);
        }
    }

    pub fn sub_imm(&mut self, imm: Immediate, overflow: Overflow) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected sub operands to be the same type");
        match &ty {
            Type::Felt => {
                self.emit(Op::SubImm(imm.as_felt().unwrap()));
            }
            Type::U64 => {
                self.push_immediate(imm);
                self.sub_u64(overflow);
            }
            Type::U32 => {
                self.sub_imm_u32(imm.as_u32().unwrap(), overflow);
            }
            Type::I32 => {
                self.sub_imm_i32(imm.as_i32().unwrap(), overflow);
            }
            ty @ (Type::U16 | Type::U8 | Type::I1) => {
                self.sub_imm_uint(imm.as_u32().unwrap(), ty.size_in_bits() as u32, overflow);
            }
            ty => unimplemented!("sub is not yet implemented for {ty}"),
        }
        self.push(ty);
        if overflow.is_overflowing() {
            self.push(Type::I1);
        }
    }

    pub fn mul(&mut self, overflow: Overflow) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected mul operands to be the same type");
        match &ty {
            Type::I128 | Type::U128 => {
                // We can use the Karatsuba algorithm for multiplication here:
                //
                // x = x_hi * 2^63 + x_lo
                // y = y_hi * 2^63 + x_lo
                //
                // z2 = x_hi * y_hi
                // z0 = x_lo * y_lo
                // z1 = (x_hi + x_lo) * (y_hi + y_lo) - z2 - z0
                //
                // z = z2 * (2^63)^2 + z1 * 2^63 + z0
                //
                // We assume the stack holds two words representing x and y, with y on top of the
                // stack
                todo!()
            }
            Type::U64 => self.mul_u64(overflow),
            Type::Felt => {
                assert_matches!(
                    overflow,
                    Overflow::Unchecked | Overflow::Wrapping,
                    "only unchecked or wrapping semantics are supported for felt"
                );
                self.emit(Op::Mul);
            }
            Type::U32 => self.mul_u32(overflow),
            Type::I32 => self.mul_i32(overflow),
            ty @ (Type::U16 | Type::U8) => {
                self.mul_uint(ty.size_in_bits() as u32, overflow);
            }
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: mul expects integer operands, got {ty}")
            }
            ty => unimplemented!("mul for {ty} is not supported"),
        }
        self.push(ty);
        if overflow.is_overflowing() {
            self.push(Type::I1);
        }
    }

    pub fn mul_imm(&mut self, imm: Immediate, overflow: Overflow) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected mul operands to be the same type");
        match &ty {
            Type::U64 => {
                self.push_immediate(imm);
                self.mul_u64(overflow);
            }
            Type::Felt => {
                assert_matches!(
                    overflow,
                    Overflow::Unchecked | Overflow::Wrapping,
                    "only unchecked or wrapping semantics are supported for felt"
                );
                self.emit(Op::MulImm(imm.as_felt().unwrap()));
            }
            Type::U32 => self.mul_imm_u32(imm.as_u32().unwrap(), overflow),
            Type::I32 => self.mul_imm_i32(imm.as_i32().unwrap(), overflow),
            ty @ (Type::U16 | Type::U8) => {
                self.mul_imm_uint(imm.as_u32().unwrap(), ty.size_in_bits() as u32, overflow);
            }
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: mul expects integer operands, got {ty}")
            }
            ty => unimplemented!("mul for {ty} is not supported"),
        }
        self.push(ty);
        if overflow.is_overflowing() {
            self.push(Type::I1);
        }
    }

    pub fn checked_div(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected div operands to be the same type");
        match &ty {
            Type::U64 => self.checked_div_u64(),
            Type::Felt => {
                self.emit(Op::Div);
            }
            Type::U32 => self.checked_div_u32(),
            Type::I32 => self.checked_div_i32(),
            ty @ (Type::U16 | Type::U8) => {
                self.checked_div_uint(ty.size_in_bits() as u32);
            }
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: div expects integer operands, got {ty}")
            }
            ty => unimplemented!("div for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn checked_div_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected div operands to be the same type");
        match &ty {
            Type::U64 => {
                assert_ne!(imm.as_u64().unwrap(), 0, "invalid division by zero");
                self.push_immediate(imm);
                self.checked_div_u64();
            }
            Type::Felt => {
                self.emit(Op::Div);
            }
            Type::U32 => self.checked_div_imm_u32(imm.as_u32().unwrap()),
            Type::I32 => self.checked_div_imm_i32(imm.as_i32().unwrap()),
            ty @ (Type::U16 | Type::U8) => {
                self.checked_div_imm_uint(imm.as_u32().unwrap(), ty.size_in_bits() as u32);
            }
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: div expects integer operands, got {ty}")
            }
            ty => unimplemented!("div for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn unchecked_div(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected div operands to be the same type");
        match &ty {
            Type::U64 => self.unchecked_div_u64(),
            Type::Felt => {
                self.emit(Op::Div);
            }
            Type::U32 | Type::U16 | Type::U8 => self.unchecked_div_u32(),
            Type::I32 => self.checked_div_i32(),
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: div expects integer operands, got {ty}")
            }
            ty => unimplemented!("div for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn unchecked_div_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected div operands to be the same type");
        match &ty {
            Type::U64 => {
                assert_ne!(imm.as_u64().unwrap(), 0, "invalid division by zero");
                self.push_immediate(imm);
                self.unchecked_div_u64();
            }
            Type::Felt => {
                self.emit(Op::Div);
            }
            Type::U32 => self.unchecked_div_imm_u32(imm.as_u32().unwrap()),
            Type::I32 => self.checked_div_imm_i32(imm.as_i32().unwrap()),
            ty @ (Type::U16 | Type::U8) => {
                self.unchecked_div_imm_uint(imm.as_u32().unwrap(), ty.size_in_bits() as u32);
            }
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: div expects integer operands, got {ty}")
            }
            ty => unimplemented!("div for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn checked_mod(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected mod operands to be the same type");
        match &ty {
            Type::U64 => self.checked_mod_u64(),
            Type::U32 => self.checked_mod_u32(),
            ty @ (Type::U16 | Type::U8) => {
                self.checked_mod_uint(ty.size_in_bits() as u32);
            }
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: mod expects integer operands, got {ty}")
            }
            ty => unimplemented!("mod for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn checked_mod_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected mod operands to be the same type");
        match &ty {
            Type::U64 => {
                assert_ne!(imm.as_u64().unwrap(), 0, "invalid division by zero");
                self.push_immediate(imm);
                self.checked_mod_u64();
            }
            Type::U32 => self.checked_mod_imm_u32(imm.as_u32().unwrap()),
            ty @ (Type::U16 | Type::U8) => {
                self.checked_mod_imm_uint(imm.as_u32().unwrap(), ty.size_in_bits() as u32);
            }
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: mod expects integer operands, got {ty}")
            }
            ty => unimplemented!("mod for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn unchecked_mod(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected mod operands to be the same type");
        match &ty {
            Type::U64 => self.unchecked_mod_u64(),
            Type::U32 => self.unchecked_mod_u32(),
            ty @ (Type::U16 | Type::U8) => {
                self.unchecked_mod_uint(ty.size_in_bits() as u32);
            }
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: mod expects integer operands, got {ty}")
            }
            ty => unimplemented!("mod for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn unchecked_mod_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected mod operands to be the same type");
        match &ty {
            Type::U64 => {
                assert_ne!(imm.as_u64().unwrap(), 0, "invalid division by zero");
                self.push_immediate(imm);
                self.unchecked_mod_u64();
            }
            Type::U32 => self.unchecked_mod_imm_u32(imm.as_u32().unwrap()),
            ty @ (Type::U16 | Type::U8) => {
                self.unchecked_mod_imm_uint(imm.as_u32().unwrap(), ty.size_in_bits() as u32);
            }
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: mod expects integer operands, got {ty}")
            }
            ty => unimplemented!("mod for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn checked_divmod(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected divmod operands to be the same type");
        match &ty {
            Type::U64 => self.checked_divmod_u64(),
            Type::U32 => self.checked_divmod_u32(),
            ty @ (Type::U16 | Type::U8) => {
                self.checked_divmod_uint(ty.size_in_bits() as u32);
            }
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: divmod expects integer operands, got {ty}")
            }
            ty => unimplemented!("divmod for {ty} is not supported"),
        }
        self.push(ty.clone());
        self.push(ty);
    }

    pub fn checked_divmod_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected divmod operands to be the same type");
        match &ty {
            Type::U64 => {
                assert_ne!(imm.as_u64().unwrap(), 0, "invalid division by zero");
                self.push_immediate(imm);
                self.checked_divmod_u64();
            }
            Type::U32 => self.checked_divmod_imm_u32(imm.as_u32().unwrap()),
            ty @ (Type::U16 | Type::U8) => {
                self.checked_divmod_imm_uint(imm.as_u32().unwrap(), ty.size_in_bits() as u32);
            }
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: divmod expects integer operands, got {ty}")
            }
            ty => unimplemented!("divmod for {ty} is not supported"),
        }
        self.push(ty.clone());
        self.push(ty);
    }

    pub fn unchecked_divmod(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected divmod operands to be the same type");
        match &ty {
            Type::U64 => self.unchecked_divmod_u64(),
            Type::U32 => self.unchecked_divmod_u32(),
            ty @ (Type::U16 | Type::U8) => {
                self.unchecked_divmod_uint(ty.size_in_bits() as u32);
            }
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: divmod expects integer operands, got {ty}")
            }
            ty => unimplemented!("divmod for {ty} is not supported"),
        }
        self.push(ty.clone());
        self.push(ty);
    }

    pub fn unchecked_divmod_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected divmod operands to be the same type");
        match &ty {
            Type::U64 => {
                assert_ne!(imm.as_u64().unwrap(), 0, "invalid division by zero");
                self.push_immediate(imm);
                self.unchecked_divmod_u64();
            }
            Type::U32 => self.unchecked_divmod_imm_u32(imm.as_u32().unwrap()),
            ty @ (Type::U16 | Type::U8) => {
                self.unchecked_divmod_imm_uint(imm.as_u32().unwrap(), ty.size_in_bits() as u32);
            }
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: divmod expects integer operands, got {ty}")
            }
            ty => unimplemented!("divmod for {ty} is not supported"),
        }
        self.push(ty.clone());
        self.push(ty);
    }

    pub fn exp(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected exp operands to be the same type");
        match &ty {
            Type::U64 => todo!("exponentiation by squaring"),
            Type::Felt => {
                self.emit(Op::Exp);
            }
            Type::U32 => {
                self.emit_all(&[Op::Exp, Op::U32Assert]);
            }
            Type::I32 => {
                self.emit(Op::Exec("intrinsics::i32::ipow".parse().unwrap()));
            }
            ty @ (Type::U16 | Type::U8) => {
                self.emit_all(&[Op::Exp, Op::U32Assert]);
                self.int32_to_uint(ty.size_in_bits() as u32);
            }
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: exp expects integer operands, got {ty}")
            }
            ty => unimplemented!("mod for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn exp_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected exp operands to be the same type");
        let exp: u8 =
            imm.as_u64().unwrap().try_into().expect("invalid exponent: must be value < 64");
        match &ty {
            Type::U64 => todo!("exponentiation by squaring"),
            Type::Felt => {
                self.emit(Op::ExpImm(exp));
            }
            Type::U32 => {
                self.emit_all(&[Op::ExpImm(exp), Op::U32Assert]);
            }
            Type::I32 => {
                self.emit_all(&[
                    Op::PushU8(exp),
                    Op::Exec("intrinsics::i32::ipow".parse().unwrap()),
                ]);
            }
            ty @ (Type::U16 | Type::U8) => {
                self.emit_all(&[Op::ExpImm(exp), Op::U32Assert]);
                self.int32_to_uint(ty.size_in_bits() as u32);
            }
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: exp expects integer operands, got {ty}")
            }
            ty => unimplemented!("mod for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn and(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected and operands to be the same type");
        assert_eq!(ty, Type::I1, "expected and operands to be of boolean type");
        self.emit(Op::And);
        self.push(ty);
    }

    pub fn and_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected and operands to be the same type");
        assert_eq!(ty, Type::I1, "expected and operands to be of boolean type");
        self.emit(Op::AndImm(imm.as_bool().unwrap()));
        self.push(ty);
    }

    pub fn or(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected or operands to be the same type");
        assert_eq!(ty, Type::I1, "expected or operands to be of boolean type");
        self.emit(Op::Or);
        self.push(ty);
    }

    pub fn or_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected or operands to be the same type");
        assert_eq!(ty, Type::I1, "expected or operands to be of boolean type");
        self.emit(Op::OrImm(imm.as_bool().unwrap()));
        self.push(ty);
    }

    pub fn xor(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected xor operands to be the same type");
        assert_eq!(ty, Type::I1, "expected xor operands to be of boolean type");
        self.emit(Op::Xor);
        self.push(ty);
    }

    pub fn xor_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected xor operands to be the same type");
        assert_eq!(ty, Type::I1, "expected xor operands to be of boolean type");
        self.emit(Op::XorImm(imm.as_bool().unwrap()));
        self.push(ty);
    }

    pub fn band(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected band operands to be the same type");
        match &ty {
            Type::U128 | Type::I128 => {
                // AND the high bits
                //
                // [b_hi_hi, b_hi_lo, b_lo_hi, b_lo_lo, a_hi_hi, ..]
                self.emit_all(&[
                    // [a_hi_hi, a_hi_lo, b_hi_hi, b_hi_lo, ..]
                    Op::Movup(5),
                    Op::Movup(5),
                ]);
                self.band_int64(); // [band_hi_hi, band_hi_lo, b_lo_hi, b_lo_lo, a_lo_hi, a_lo_lo]
                                   // AND the low bits
                self.emit_all(&[
                    // [b_lo_hi, b_lo_lo, a_lo_hi, a_lo_lo, band_hi_hi, band_hi_lo]
                    Op::Movdn(5),
                    Op::Movdn(5),
                ]);
                self.band_int64(); // [band_lo_hi, band_lo_lo, band_hi_hi, band_hi_lo]
                self.emit_all(&[
                    // [band_hi_hi, band_hi_lo, band_lo_hi, band_lo_lo]
                    Op::Movup(3),
                    Op::Movup(3),
                ]);
            }
            Type::U64 | Type::I64 => self.band_int64(),
            Type::U32 | Type::I32 | Type::U16 | Type::I16 | Type::U8 | Type::I8 => self.band_u32(),
            Type::I1 => self.emit(Op::And),
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: band expects integer operands, got {ty}")
            }
            ty => unimplemented!("band for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn band_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected band operands to be the same type");
        match &ty {
            Type::U128 | Type::I128 => {
                self.push_immediate(imm);
                // AND the high bits
                //
                // [b_hi_hi, b_hi_lo, b_lo_hi, b_lo_lo, a_hi_hi, ..]
                self.emit_all(&[
                    // [a_hi_hi, a_hi_lo, b_hi_hi, b_hi_lo, ..]
                    Op::Movup(5),
                    Op::Movup(5),
                ]);
                self.band_int64(); // [band_hi_hi, band_hi_lo, b_lo_hi, b_lo_lo, a_lo_hi, a_lo_lo]
                                   // AND the low bits
                self.emit_all(&[
                    // [b_lo_hi, b_lo_lo, a_lo_hi, a_lo_lo, band_hi_hi, band_hi_lo]
                    Op::Movdn(5),
                    Op::Movdn(5),
                ]);
                self.band_int64(); // [band_lo_hi, band_lo_lo, band_hi_hi, band_hi_lo]
                self.emit_all(&[
                    // [band_hi_hi, band_hi_lo, band_lo_hi, band_lo_lo]
                    Op::Movup(3),
                    Op::Movup(3),
                ]);
            }
            Type::U64 | Type::I64 => {
                self.push_immediate(imm);
                self.band_int64();
            }
            Type::U32 | Type::U16 | Type::U8 => self.band_imm_u32(imm.as_u32().unwrap()),
            Type::I32 | Type::I16 | Type::I8 => {
                self.band_imm_u32(imm.as_i64().unwrap() as u64 as u32)
            }
            Type::I1 => self.emit(Op::AndImm(imm.as_bool().unwrap())),
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: band expects integer operands, got {ty}")
            }
            ty => unimplemented!("band for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn bor(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected bor operands to be the same type");
        match &ty {
            Type::U128 | Type::I128 => {
                // OR the high bits
                //
                // [b_hi_hi, b_hi_lo, b_lo_hi, b_lo_lo, a_hi_hi, ..]
                self.emit_all(&[
                    // [a_hi_hi, a_hi_lo, b_hi_hi, b_hi_lo, ..]
                    Op::Movup(5),
                    Op::Movup(5),
                ]);
                self.bor_int64(); // [band_hi_hi, band_hi_lo, b_lo_hi, b_lo_lo, a_lo_hi, a_lo_lo]
                                  // OR the low bits
                self.emit_all(&[
                    // [b_lo_hi, b_lo_lo, a_lo_hi, a_lo_lo, band_hi_hi, band_hi_lo]
                    Op::Movdn(5),
                    Op::Movdn(5),
                ]);
                self.bor_int64(); // [band_lo_hi, band_lo_lo, band_hi_hi, band_hi_lo]
                self.emit_all(&[
                    // [band_hi_hi, band_hi_lo, band_lo_hi, band_lo_lo]
                    Op::Movup(3),
                    Op::Movup(3),
                ]);
            }
            Type::U64 | Type::I64 => self.bor_int64(),
            Type::U32 | Type::I32 | Type::U16 | Type::I16 | Type::U8 | Type::I8 => self.bor_u32(),
            Type::I1 => self.emit(Op::Or),
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: bor expects integer operands, got {ty}")
            }
            ty => unimplemented!("bor for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn bor_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected bor operands to be the same type");
        match &ty {
            Type::U128 | Type::I128 => {
                self.push_immediate(imm);
                // OR the high bits
                //
                // [b_hi_hi, b_hi_lo, b_lo_hi, b_lo_lo, a_hi_hi, ..]
                self.emit_all(&[
                    // [a_hi_hi, a_hi_lo, b_hi_hi, b_hi_lo, ..]
                    Op::Movup(5),
                    Op::Movup(5),
                ]);
                self.bor_int64(); // [band_hi_hi, band_hi_lo, b_lo_hi, b_lo_lo, a_lo_hi, a_lo_lo]
                                  // OR the low bits
                self.emit_all(&[
                    // [b_lo_hi, b_lo_lo, a_lo_hi, a_lo_lo, band_hi_hi, band_hi_lo]
                    Op::Movdn(5),
                    Op::Movdn(5),
                ]);
                self.bor_int64(); // [band_lo_hi, band_lo_lo, band_hi_hi, band_hi_lo]
                self.emit_all(&[
                    // [band_hi_hi, band_hi_lo, band_lo_hi, band_lo_lo]
                    Op::Movup(3),
                    Op::Movup(3),
                ]);
            }
            Type::U64 | Type::I64 => {
                self.push_immediate(imm);
                self.bor_int64();
            }
            Type::U32 | Type::U16 | Type::U8 => self.bor_imm_u32(imm.as_u32().unwrap()),
            Type::I32 | Type::I16 | Type::I8 => {
                self.bor_imm_u32(imm.as_i64().unwrap() as u64 as u32)
            }
            Type::I1 => self.emit(Op::AndImm(imm.as_bool().unwrap())),
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: bor expects integer operands, got {ty}")
            }
            ty => unimplemented!("bor for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn bxor(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected bxor operands to be the same type");
        match &ty {
            Type::U128 | Type::I128 => {
                // XOR the high bits
                //
                // [b_hi_hi, b_hi_lo, b_lo_hi, b_lo_lo, a_hi_hi, ..]
                self.emit_all(&[
                    // [a_hi_hi, a_hi_lo, b_hi_hi, b_hi_lo, ..]
                    Op::Movup(5),
                    Op::Movup(5),
                ]);
                self.bxor_int64(); // [band_hi_hi, band_hi_lo, b_lo_hi, b_lo_lo, a_lo_hi, a_lo_lo]
                                   // XOR the low bits
                self.emit_all(&[
                    // [b_lo_hi, b_lo_lo, a_lo_hi, a_lo_lo, band_hi_hi, band_hi_lo]
                    Op::Movdn(5),
                    Op::Movdn(5),
                ]);
                self.bxor_int64(); // [band_lo_hi, band_lo_lo, band_hi_hi, band_hi_lo]
                self.emit_all(&[
                    // [band_hi_hi, band_hi_lo, band_lo_hi, band_lo_lo]
                    Op::Movup(3),
                    Op::Movup(3),
                ]);
            }
            Type::U64 | Type::I64 => self.bxor_int64(),
            Type::U32 | Type::I32 => self.bxor_u32(),
            ty @ (Type::U16 | Type::I16 | Type::U8 | Type::I8) => {
                self.bxor_u32();
                self.trunc_int32(ty.size_in_bits() as u32);
            }
            Type::I1 => self.emit(Op::Xor),
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: bxor expects integer operands, got {ty}")
            }
            ty => unimplemented!("bxor for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn bxor_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected bxor operands to be the same type");
        match &ty {
            Type::U128 | Type::I128 => {
                self.push_immediate(imm);
                // XOR the high bits
                //
                // [b_hi_hi, b_hi_lo, b_lo_hi, b_lo_lo, a_hi_hi, ..]
                self.emit_all(&[
                    // [a_hi_hi, a_hi_lo, b_hi_hi, b_hi_lo, ..]
                    Op::Movup(5),
                    Op::Movup(5),
                ]);
                self.bxor_int64(); // [band_hi_hi, band_hi_lo, b_lo_hi, b_lo_lo, a_lo_hi, a_lo_lo]
                                   // XOR the low bits
                self.emit_all(&[
                    // [b_lo_hi, b_lo_lo, a_lo_hi, a_lo_lo, band_hi_hi, band_hi_lo]
                    Op::Movdn(5),
                    Op::Movdn(5),
                ]);
                self.bxor_int64(); // [band_lo_hi, band_lo_lo, band_hi_hi, band_hi_lo]
                self.emit_all(&[
                    // [band_hi_hi, band_hi_lo, band_lo_hi, band_lo_lo]
                    Op::Movup(3),
                    Op::Movup(3),
                ]);
            }
            Type::U64 | Type::I64 => {
                self.push_immediate(imm);
                self.bxor_int64();
            }
            Type::U32 => self.bxor_imm_u32(imm.as_u32().unwrap()),
            Type::I32 => self.bxor_imm_u32(imm.as_i64().unwrap() as u64 as u32),
            ty @ (Type::U16 | Type::U8) => {
                self.bxor_imm_u32(imm.as_u32().unwrap());
                self.trunc_int32(ty.size_in_bits() as u32);
            }
            ty @ (Type::I16 | Type::I8) => {
                self.bxor_imm_u32(imm.as_i64().unwrap() as u64 as u32);
                self.trunc_int32(ty.size_in_bits() as u32);
            }
            Type::I1 => self.emit(Op::XorImm(imm.as_bool().unwrap())),
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: bxor expects integer operands, got {ty}")
            }
            ty => unimplemented!("bxor for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn shl(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected shl operands to be the same type");
        match &ty {
            Type::U64 => self.shl_u64(),
            Type::U32 | Type::I32 => self.shl_u32(),
            ty @ (Type::U16 | Type::U8) => {
                self.shl_u32();
                self.trunc_int32(ty.size_in_bits() as u32);
            }
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: shl expects integer operands, got {ty}")
            }
            ty => unimplemented!("shl for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn shl_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected shl operands to be the same type");
        match &ty {
            Type::U64 => {
                assert!(imm.as_u64().unwrap() < 64, "invalid shift value: must be < 64");
                self.push_immediate(imm);
                self.shl_u64();
            }
            Type::U32 => self.shl_imm_u32(imm.as_u32().unwrap()),
            Type::I32 => self.shl_imm_u32(imm.as_i32().unwrap() as u32),
            ty @ (Type::U16 | Type::U8) => {
                self.shl_imm_u32(imm.as_u32().unwrap());
                self.trunc_int32(ty.size_in_bits() as u32);
            }
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: shl expects integer operands, got {ty}")
            }
            ty => unimplemented!("shl for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn shr(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected shr operands to be the same type");
        match &ty {
            Type::U64 => self.shr_u64(),
            Type::U32 | Type::U16 | Type::U8 => self.shr_u32(),
            Type::I32 => self.shr_i32(),
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: shr expects integer operands, got {ty}")
            }
            ty => unimplemented!("shr for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn shr_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected shr operands to be the same type");
        match &ty {
            Type::U64 => {
                let shift = imm.as_u64().unwrap();
                assert!(shift < 64, "invalid shift value: must be < 64, got {shift}");
                self.push_immediate(imm);
                self.shr_u64();
            }
            Type::U32 | Type::U16 | Type::U8 => self.shr_imm_u32(imm.as_u32().unwrap()),
            Type::I32 => self.shr_imm_i32(imm.as_i32().unwrap()),
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: shr expects integer operands, got {ty}")
            }
            ty => unimplemented!("shr for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn rotl(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected rotl operands to be the same type");
        match &ty {
            Type::U64 => self.rotl_u64(),
            Type::U32 => self.rotl_u32(),
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: rotl expects integer operands, got {ty}")
            }
            ty => unimplemented!("rotl for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn rotl_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected rotl operands to be the same type");
        match &ty {
            Type::U64 => {
                self.push_immediate(imm);
                self.rotl_u64();
            }
            Type::U32 => self.rotl_imm_u32(imm.as_u32().unwrap()),
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: rotl expects integer operands, got {ty}")
            }
            ty => unimplemented!("rotl for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn rotr(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected rotr operands to be the same type");
        match &ty {
            Type::U64 => self.rotr_u64(),
            Type::U32 => self.rotr_u32(),
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: rotr expects integer operands, got {ty}")
            }
            ty => unimplemented!("rotr for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn rotr_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected rotr operands to be the same type");
        match &ty {
            Type::U64 => {
                self.push_immediate(imm);
                self.rotr_u64();
            }
            Type::U32 => self.rotr_imm_u32(imm.as_u32().unwrap()),
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: rotr expects integer operands, got {ty}")
            }
            ty => unimplemented!("rotr for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn min(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected min operands to be the same type");
        match &ty {
            Type::U64 => self.min_u64(),
            Type::U32 | Type::U16 | Type::U8 | Type::I1 => self.min_u32(),
            Type::I32 => self.min_i32(),
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: min expects integer operands, got {ty}")
            }
            ty => unimplemented!("min for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn min_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected min operands to be the same type");
        match &ty {
            Type::U64 => {
                self.push_immediate(imm);
                self.min_u64();
            }
            Type::U32 | Type::U16 | Type::U8 | Type::I1 => self.min_imm_u32(imm.as_u32().unwrap()),
            Type::I32 => self.min_imm_i32(imm.as_i32().unwrap()),
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: min expects integer operands, got {ty}")
            }
            ty => unimplemented!("min for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn max(&mut self) {
        let rhs = self.pop().expect("operand stack is empty");
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, rhs.ty(), "expected max operands to be the same type");
        match &ty {
            Type::U64 => self.max_u64(),
            Type::U32 | Type::U16 | Type::U8 | Type::I1 => self.max_u32(),
            Type::I32 => self.max_i32(),
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: max expects integer operands, got {ty}")
            }
            ty => unimplemented!("max for {ty} is not supported"),
        }
        self.push(ty);
    }

    pub fn max_imm(&mut self, imm: Immediate) {
        let lhs = self.pop().expect("operand stack is empty");
        let ty = lhs.ty();
        assert_eq!(ty, imm.ty(), "expected max operands to be the same type");
        match &ty {
            Type::U64 => {
                self.push_immediate(imm);
                self.max_u64();
            }
            Type::U32 | Type::U16 | Type::U8 | Type::I1 => self.max_imm_u32(imm.as_u32().unwrap()),
            Type::I32 => self.max_imm_i32(imm.as_i32().unwrap()),
            ty if !ty.is_integer() => {
                panic!("invalid binary operand: max expects integer operands, got {ty}")
            }
            ty => unimplemented!("max for {ty} is not supported"),
        }
        self.push(ty);
    }
}
