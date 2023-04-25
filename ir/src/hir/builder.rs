use cranelift_entity::packed_option::PackedOption;

use miden_diagnostics::SourceSpan;

use crate::types::Type;

use super::*;

pub struct FunctionBuilder<'a> {
    pub func: &'a mut Function,
    pub entry: Block,
    position: PackedOption<Block>,
}
impl<'a> FunctionBuilder<'a> {
    pub fn new(func: &'a mut Function) -> Self {
        let entry = func.dfg.make_block();
        let position = Some(entry);

        Self {
            func,
            entry,
            position: position.into(),
        }
    }

    #[inline]
    pub fn current_block(&self) -> Block {
        self.position.expand().unwrap()
    }

    #[inline]
    pub fn switch_to_block(&mut self, block: Block) {
        self.position = PackedOption::from(block);
    }

    #[inline]
    pub fn create_block(&mut self) -> Block {
        self.func.dfg.make_block()
    }

    pub fn remove_block(&mut self, block: Block) {
        assert_ne!(
            block,
            self.current_block(),
            "cannot remove block the builder is currently inserting in"
        );
        self.func.dfg.remove_block(block);
    }

    #[inline]
    pub fn block_params(&self, block: Block) -> &[Value] {
        self.func.dfg.block_params(block)
    }

    #[inline]
    pub fn append_block_param(&mut self, block: Block, ty: Type, span: SourceSpan) -> Value {
        self.func.dfg.append_block_param(block, ty, span)
    }

    #[inline]
    pub fn inst_results(&self, inst: Inst) -> &[Value] {
        self.func.dfg.inst_results(inst)
    }

    #[inline]
    pub fn first_result(&self, inst: Inst) -> Value {
        self.func.dfg.first_result(inst)
    }

    #[inline]
    pub fn get_callee(&self, name: &str) -> Option<FuncRef> {
        self.func.dfg.get_callee(name)
    }

    #[inline]
    pub fn register_callee(&self, name: String, signature: Signature) -> FuncRef {
        self.func.dfg.register_callee(name, signature)
    }

    pub fn ins<'short>(&'short mut self) -> FuncInstBuilder<'short, 'a> {
        let block = self
            .position
            .expect("must be in a block to insert instructions");
        FuncInstBuilder::new(self, block)
    }

    pub(super) fn is_block_terminated(&self, block: Block) -> bool {
        if let Some(inst) = self.func.dfg.last_inst(block) {
            let data = &self.func.dfg[inst];
            data.opcode().is_terminator()
        } else {
            false
        }
    }
}

pub struct FuncInstBuilder<'a, 'b: 'a> {
    builder: &'a mut FunctionBuilder<'b>,
    block: Block,
}
impl<'a, 'b> FuncInstBuilder<'a, 'b> {
    fn new(builder: &'a mut FunctionBuilder<'b>, block: Block) -> Self {
        assert!(builder.func.dfg.is_block_inserted(block));

        Self { builder, block }
    }
}
impl<'a, 'b> InstBuilderBase<'a> for FuncInstBuilder<'a, 'b> {
    fn data_flow_graph(&self) -> &DataFlowGraph {
        &self.builder.func.dfg
    }

    fn data_flow_graph_mut(&mut self) -> &mut DataFlowGraph {
        &mut self.builder.func.dfg
    }

    fn build(self, data: Instruction, ty: Type, span: SourceSpan) -> (Inst, &'a mut DataFlowGraph) {
        debug_assert!(
            !self.builder.is_block_terminated(self.block),
            "cannot append an instruction to a block that is already terminated"
        );

        let inst = self.builder.func.dfg.push_inst(self.block, data, span);
        self.builder.func.dfg.make_inst_results(inst, ty);

        (inst, &mut self.builder.func.dfg)
    }
}

pub trait InstBuilderBase<'f>: Sized {
    fn data_flow_graph(&self) -> &DataFlowGraph;
    fn data_flow_graph_mut(&mut self) -> &mut DataFlowGraph;
    fn build(
        self,
        data: Instruction,
        ctrl_ty: Type,
        span: SourceSpan,
    ) -> (Inst, &'f mut DataFlowGraph);
}

macro_rules! binary_numeric_op {
    ($name:ident, $op:expr) => {
        fn $name(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
            let lty = self.data_flow_graph().value_type(lhs);
            let rty = self.data_flow_graph().value_type(rhs);
            assert_eq!(
                lty, rty,
                "expected binary op operands to be of the same type"
            );
            assert!(
                lty.is_numeric(),
                "expected binary op operands to be of numeric type"
            );
            let (inst, dfg) = self.Binary($op, lty, lhs, rhs, span);
            dfg.first_result(inst)
        }
    };
}

macro_rules! binary_int_op {
    ($name:ident, $op:expr) => {
        paste::paste! {
            fn $name(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
                let lty = self.data_flow_graph().value_type(lhs);
                let rty = self.data_flow_graph().value_type(rhs);
                assert_eq!(
                    lty, rty,
                    "expected binary op operands to be of the same type"
                );
                assert!(
                    lty.is_integer(),
                    "expected binary op operands to be of integral type"
                );
                let (inst, dfg) = self.Binary($op, lty, lhs, rhs, span);
                dfg.first_result(inst)
            }
            fn [<$name _checked>](self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
                let lty = self.data_flow_graph().value_type(lhs);
                let rty = self.data_flow_graph().value_type(rhs);
                assert_eq!(
                    lty, rty,
                    "expected binary op operands to be of the same type"
                );
                assert!(
                    lty.is_integer(),
                    "expected binary op operands to be of integral type"
                );
                let (inst, dfg) = self.BinaryWithOverflow($op, lty, lhs, rhs, Overflow::Checked, span);
                dfg.first_result(inst)
            }
            fn [<$name _wrapping>](self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
                let lty = self.data_flow_graph().value_type(lhs);
                let rty = self.data_flow_graph().value_type(rhs);
                assert_eq!(
                    lty, rty,
                    "expected binary op operands to be of the same type"
                );
                assert!(
                    lty.is_integer(),
                    "expected binary op operands to be of integral type"
                );
                let (inst, dfg) = self.BinaryWithOverflow($op, lty, lhs, rhs, Overflow::Wrapping, span);
                dfg.first_result(inst)
            }
            fn [<$name _overflowing>](self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
                let lty = self.data_flow_graph().value_type(lhs);
                let rty = self.data_flow_graph().value_type(rhs);
                assert_eq!(
                    lty, rty,
                    "expected binary op operands to be of the same type"
                );
                assert!(
                    lty.is_integer(),
                    "expected binary op operands to be of integral type"
                );
                let (inst, dfg) = self.BinaryWithOverflow($op, lty, lhs, rhs, Overflow::Overflowing, span);
                dfg.first_result(inst)
            }
        }
    };
}

macro_rules! unary_numeric_op {
    ($name:ident, $op:expr) => {
        fn $name(self, rhs: Value, span: SourceSpan) -> Value {
            let rty = self.data_flow_graph().value_type(rhs);
            assert!(
                rty.is_numeric(),
                "expected binary op operands to be of numeric type"
            );
            let (inst, dfg) = self.Unary($op, rty, rhs, span);
            dfg.first_result(inst)
        }
    };
}

macro_rules! unary_int_op {
    ($name:ident, $op:expr) => {
        fn $name(self, rhs: Value, span: SourceSpan) -> Value {
            let rty = self.data_flow_graph().value_type(rhs);
            assert!(
                rty.is_integer(),
                "expected binary op operands to be of integral type"
            );
            let (inst, dfg) = self.Unary($op, rty, rhs, span);
            dfg.first_result(inst)
        }
    };
}

pub trait InstBuilder<'f>: InstBuilderBase<'f> {
    fn assert(mut self, value: Value, span: SourceSpan) -> Inst {
        let mut vlist = ValueList::default();
        {
            let pool = &mut self.data_flow_graph_mut().value_lists;
            vlist.push(value, pool);
        }
        self.PrimOp(Opcode::Assert, Type::Unit, vlist, span).0
    }

    fn assertz(mut self, value: Value, span: SourceSpan) -> Inst {
        let mut vlist = ValueList::default();
        {
            let pool = &mut self.data_flow_graph_mut().value_lists;
            vlist.push(value, pool);
        }
        self.PrimOp(Opcode::Assertz, Type::Unit, vlist, span).0
    }

    fn assert_eq(mut self, lhs: Value, rhs: Value, span: SourceSpan) -> Inst {
        let mut vlist = ValueList::default();
        {
            let pool = &mut self.data_flow_graph_mut().value_lists;
            vlist.push(lhs, pool);
            vlist.push(rhs, pool);
        }
        self.PrimOp(Opcode::AssertEq, Type::Unit, vlist, span).0
    }

    fn assert_test(self, value: Value, ty: Type, span: SourceSpan) -> Inst {
        self.Test(Opcode::Test, Type::Unit, value, ty, span).0
    }

    fn i1(self, i: bool, span: SourceSpan) -> Value {
        let (inst, dfg) = self.UnaryImm(Opcode::ImmInt, Type::I1, Immediate::I1(i), span);
        dfg.first_result(inst)
    }

    fn i8(self, i: i8, span: SourceSpan) -> Value {
        let (inst, dfg) = self.UnaryImm(Opcode::ImmInt, Type::I8, Immediate::I8(i), span);
        dfg.first_result(inst)
    }

    fn i16(self, i: i16, span: SourceSpan) -> Value {
        let (inst, dfg) = self.UnaryImm(Opcode::ImmInt, Type::I16, Immediate::I16(i), span);
        dfg.first_result(inst)
    }

    fn i32(self, i: i32, span: SourceSpan) -> Value {
        let (inst, dfg) = self.UnaryImm(Opcode::ImmInt, Type::I32, Immediate::I32(i), span);
        dfg.first_result(inst)
    }

    fn i64(self, i: i64, span: SourceSpan) -> Value {
        let (inst, dfg) = self.UnaryImm(Opcode::ImmInt, Type::I64, Immediate::I64(i), span);
        dfg.first_result(inst)
    }

    fn i128(self, i: i128, span: SourceSpan) -> Value {
        let (inst, dfg) = self.UnaryImm(Opcode::ImmInt, Type::I128, Immediate::I128(i), span);
        dfg.first_result(inst)
    }

    fn isize(self, i: isize, span: SourceSpan) -> Value {
        let (inst, dfg) = self.UnaryImm(Opcode::ImmInt, Type::Isize, Immediate::Isize(i), span);
        dfg.first_result(inst)
    }

    fn felt(self, i: u64, span: SourceSpan) -> Value {
        let (inst, dfg) = self.UnaryImm(Opcode::ImmInt, Type::Felt, Immediate::Felt(i), span);
        dfg.first_result(inst)
    }

    fn f64(self, f: f64, span: SourceSpan) -> Value {
        let (inst, dfg) = self.UnaryImm(Opcode::ImmFloat, Type::F64, Immediate::F64(f), span);
        dfg.first_result(inst)
    }

    fn character(self, c: char, span: SourceSpan) -> Value {
        self.i32((c as u32) as i32, span)
    }

    fn null(self, ty: Type, span: SourceSpan) -> Value {
        let (inst, dfg) = self.UnaryImm(Opcode::ImmNull, ty, Immediate::I1(false), span);
        dfg.first_result(inst)
    }

    fn addrof(self, value: Value, span: SourceSpan) -> Value {
        let ty = self.data_flow_graph().value_type(value);
        let (inst, dfg) = self.Unary(Opcode::AddrOf, Type::Ptr(Box::new(ty)), value, span);
        dfg.first_result(inst)
    }

    fn load(self, ptr: Value, span: SourceSpan) -> Value {
        let ty = self.data_flow_graph().value_type(ptr);
        let ty = ty
            .pointee()
            .expect("expected load operand to be a pointer value");
        let (inst, dfg) = self.Unary(Opcode::Load, ty, ptr, span);
        dfg.first_result(inst)
    }

    fn store(self, ptr: Value, value: Value, span: SourceSpan) -> Inst {
        let ptr_ty = self.data_flow_graph().value_type(ptr);
        let pointee_ty = ptr_ty
            .pointee()
            .expect("expected first store operand to be a pointer");
        let value_ty = self.data_flow_graph().value_type(value);
        assert_eq!(
            pointee_ty, value_ty,
            "type mismatch between pointer and value to be stored"
        );
        self.Binary(Opcode::Store, Type::Unit, ptr, value, span).0
    }

    fn memcpy(self, src: Value, dst: Value, count: Value, ty: Type, span: SourceSpan) -> Inst {
        let data = Instruction::MemCpy(MemCpy {
            op: Opcode::MemCpy,
            args: [src, dst, count],
            ty,
        });
        self.build(data, Type::Unit, span).0
    }

    fn ptrtoint(self, arg: Value, ty: Type, span: SourceSpan) -> Value {
        assert!(ty.is_integer(), "expected integral type, got {:#?}", &ty);
        let arg_ty = self.data_flow_graph().value_type(arg);
        assert!(
            arg_ty.is_pointer(),
            "expected operand to be a pointer value, got {:#?}",
            &arg_ty
        );
        let (inst, dfg) = self.Unary(Opcode::PtrToInt, ty, arg, span);
        dfg.first_result(inst)
    }

    fn inttoptr(self, arg: Value, ty: Type, span: SourceSpan) -> Value {
        assert!(ty.is_pointer(), "expected pointer type, got {:#?}", &ty);
        let arg_ty = self.data_flow_graph().value_type(arg);
        assert!(
            arg_ty.is_integer(),
            "expected operand to be an integer value, got {:#?}",
            &arg_ty
        );
        let (inst, dfg) = self.Unary(Opcode::IntToPtr, ty, arg, span);
        dfg.first_result(inst)
    }

    fn cast(self, arg: Value, ty: Type, span: SourceSpan) -> Value {
        let (inst, dfg) = self.Unary(Opcode::Cast, ty, arg, span);
        dfg.first_result(inst)
    }

    fn trunc(self, arg: Value, ty: Type, span: SourceSpan) -> Value {
        assert!(ty.is_integer());
        let (inst, dfg) = self.Unary(Opcode::Trunc, ty, arg, span);
        dfg.first_result(inst)
    }

    fn zext(self, arg: Value, ty: Type, span: SourceSpan) -> Value {
        assert!(ty.is_integer());
        let (inst, dfg) = self.Unary(Opcode::Zext, ty, arg, span);
        dfg.first_result(inst)
    }

    fn sext(self, arg: Value, ty: Type, span: SourceSpan) -> Value {
        assert!(ty.is_integer());
        let (inst, dfg) = self.Unary(Opcode::Sext, ty, arg, span);
        dfg.first_result(inst)
    }

    fn test(self, arg: Value, ty: Type, span: SourceSpan) -> Value {
        assert!(ty.is_integer());
        let (inst, dfg) = self.Test(Opcode::Test, Type::I1, arg, ty, span);
        dfg.first_result(inst)
    }

    binary_numeric_op!(add, Opcode::Add);
    binary_numeric_op!(sub, Opcode::Sub);
    binary_numeric_op!(mul, Opcode::Mul);
    binary_numeric_op!(div, Opcode::Div);
    binary_numeric_op!(min, Opcode::Min);
    binary_numeric_op!(max, Opcode::Max);
    binary_int_op!(r#mod, Opcode::Mod);
    binary_int_op!(divmod, Opcode::DivMod);
    binary_int_op!(pow2, Opcode::Pow2);
    binary_int_op!(exp, Opcode::Exp);
    binary_int_op!(and, Opcode::And);
    binary_int_op!(or, Opcode::Or);
    binary_int_op!(xor, Opcode::Xor);
    binary_int_op!(shl, Opcode::Shl);
    binary_int_op!(shr, Opcode::Shr);
    binary_int_op!(rotl, Opcode::Rotl);
    binary_int_op!(rotr, Opcode::Rotr);
    unary_numeric_op!(neg, Opcode::Neg);
    unary_numeric_op!(inv, Opcode::Inv);
    unary_int_op!(not, Opcode::Not);
    unary_int_op!(popcnt, Opcode::Popcnt);

    fn eq(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
        let (inst, dfg) = self.Binary(Opcode::Eq, Type::I1, lhs, rhs, span);
        dfg.first_result(inst)
    }

    fn neq(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
        let (inst, dfg) = self.Binary(Opcode::Neq, Type::I1, lhs, rhs, span);
        dfg.first_result(inst)
    }

    fn gt(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
        let (inst, dfg) = self.Binary(Opcode::Gt, Type::I1, lhs, rhs, span);
        dfg.first_result(inst)
    }

    fn gte(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
        let (inst, dfg) = self.Binary(Opcode::Gte, Type::I1, lhs, rhs, span);
        dfg.first_result(inst)
    }

    fn lt(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
        let (inst, dfg) = self.Binary(Opcode::Lt, Type::I1, lhs, rhs, span);
        dfg.first_result(inst)
    }

    fn lte(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
        let (inst, dfg) = self.Binary(Opcode::Lte, Type::I1, lhs, rhs, span);
        dfg.first_result(inst)
    }

    fn is_odd(self, value: Value, span: SourceSpan) -> Value {
        let (inst, dfg) = self.Unary(Opcode::IsOdd, Type::I1, value, span);
        dfg.first_result(inst)
    }

    fn call(mut self, callee: FuncRef, args: &[Value], span: SourceSpan) -> Inst {
        let mut vlist = ValueList::default();
        {
            let pool = &mut self.data_flow_graph_mut().value_lists;
            vlist.extend(args.iter().copied(), pool);
        }
        self.Call(Opcode::Call, callee, vlist, span).0
    }

    fn br(mut self, block: Block, args: &[Value], span: SourceSpan) -> Inst {
        let mut vlist = ValueList::default();
        {
            let pool = &mut self.data_flow_graph_mut().value_lists;
            vlist.extend(args.iter().copied(), pool);
        }
        self.Br(Opcode::Br, Type::Unit, block, vlist, span).0
    }

    fn cond_br(
        mut self,
        cond: Value,
        then_dest: Block,
        then_args: &[Value],
        else_dest: Block,
        else_args: &[Value],
        span: SourceSpan,
    ) -> Inst {
        let mut then_vlist = ValueList::default();
        let mut else_vlist = ValueList::default();
        {
            let pool = &mut self.data_flow_graph_mut().value_lists;
            then_vlist.extend(then_args.iter().copied(), pool);
            else_vlist.extend(else_args.iter().copied(), pool);
        }
        self.CondBr(cond, then_dest, then_vlist, else_dest, else_vlist, span)
            .0
    }

    fn switch(self, arg: Value, arms: Vec<(u32, Block)>, default: Block, span: SourceSpan) -> Inst {
        self.Switch(arg, arms, default, span).0
    }

    fn ret(mut self, returning: Value, span: SourceSpan) -> Inst {
        let mut vlist = ValueList::default();
        {
            let pool = &mut self.data_flow_graph_mut().value_lists;
            vlist.push(returning, pool);
        }
        self.Ret(vlist, span).0
    }

    #[allow(non_snake_case)]
    fn CondBr(
        self,
        cond: Value,
        then_dest: Block,
        then_args: ValueList,
        else_dest: Block,
        else_args: ValueList,
        span: SourceSpan,
    ) -> (Inst, &'f mut DataFlowGraph) {
        let data = Instruction::CondBr(CondBr {
            op: Opcode::CondBr,
            cond,
            then_dest: (then_dest, then_args),
            else_dest: (else_dest, else_args),
        });
        self.build(data, Type::Unit, span)
    }

    #[allow(non_snake_case)]
    fn Br(
        self,
        op: Opcode,
        ty: Type,
        destination: Block,
        args: ValueList,
        span: SourceSpan,
    ) -> (Inst, &'f mut DataFlowGraph) {
        let data = Instruction::Br(Br {
            op,
            destination,
            args,
        });
        self.build(data, ty, span)
    }

    #[allow(non_snake_case)]
    fn Switch(
        self,
        arg: Value,
        arms: Vec<(u32, Block)>,
        default: Block,
        span: SourceSpan,
    ) -> (Inst, &'f mut DataFlowGraph) {
        let data = Instruction::Switch(Switch {
            op: Opcode::Switch,
            arg,
            arms,
            default,
        });
        self.build(data, Type::Unit, span)
    }

    #[allow(non_snake_case)]
    fn Ret(self, args: ValueList, span: SourceSpan) -> (Inst, &'f mut DataFlowGraph) {
        let data = Instruction::Ret(Ret {
            op: Opcode::Ret,
            args,
        });
        self.build(data, Type::Unit, span)
    }

    #[allow(non_snake_case)]
    fn Call(
        self,
        op: Opcode,
        callee: FuncRef,
        args: ValueList,
        span: SourceSpan,
    ) -> (Inst, &'f mut DataFlowGraph) {
        let data = Instruction::Call(Call { op, callee, args });
        self.build(data, Type::Unit, span)
    }

    #[allow(non_snake_case)]
    fn Binary(
        self,
        op: Opcode,
        ty: Type,
        lhs: Value,
        rhs: Value,
        span: SourceSpan,
    ) -> (Inst, &'f mut DataFlowGraph) {
        self.BinaryWithOverflow(op, ty, lhs, rhs, Default::default(), span)
    }

    #[allow(non_snake_case)]
    fn BinaryWithOverflow(
        self,
        op: Opcode,
        ty: Type,
        lhs: Value,
        rhs: Value,
        overflow: Overflow,
        span: SourceSpan,
    ) -> (Inst, &'f mut DataFlowGraph) {
        let data = Instruction::BinaryOp(BinaryOp {
            op,
            overflow,
            args: [lhs, rhs],
        });
        self.build(data, ty, span)
    }

    #[allow(non_snake_case)]
    fn BinaryImm(
        self,
        op: Opcode,
        ty: Type,
        arg: Value,
        imm: Immediate,
        span: SourceSpan,
    ) -> (Inst, &'f mut DataFlowGraph) {
        self.BinaryImmWithOverflow(op, ty, arg, imm, Default::default(), span)
    }

    #[allow(non_snake_case)]
    fn BinaryImmWithOverflow(
        self,
        op: Opcode,
        ty: Type,
        arg: Value,
        imm: Immediate,
        overflow: Overflow,
        span: SourceSpan,
    ) -> (Inst, &'f mut DataFlowGraph) {
        let data = Instruction::BinaryOpImm(BinaryOpImm {
            op,
            overflow,
            arg,
            imm,
        });
        self.build(data, ty, span)
    }

    #[allow(non_snake_case)]
    fn Unary(
        self,
        op: Opcode,
        ty: Type,
        arg: Value,
        span: SourceSpan,
    ) -> (Inst, &'f mut DataFlowGraph) {
        self.UnaryWithOverflow(op, ty, arg, Default::default(), span)
    }

    #[allow(non_snake_case)]
    fn UnaryWithOverflow(
        self,
        op: Opcode,
        ty: Type,
        arg: Value,
        overflow: Overflow,
        span: SourceSpan,
    ) -> (Inst, &'f mut DataFlowGraph) {
        let data = Instruction::UnaryOp(UnaryOp { op, overflow, arg });
        self.build(data, ty, span)
    }

    #[allow(non_snake_case)]
    fn UnaryImm(
        self,
        op: Opcode,
        ty: Type,
        imm: Immediate,
        span: SourceSpan,
    ) -> (Inst, &'f mut DataFlowGraph) {
        self.UnaryImmWithOverflow(op, ty, imm, Default::default(), span)
    }

    #[allow(non_snake_case)]
    fn UnaryImmWithOverflow(
        self,
        op: Opcode,
        ty: Type,
        imm: Immediate,
        overflow: Overflow,
        span: SourceSpan,
    ) -> (Inst, &'f mut DataFlowGraph) {
        let data = Instruction::UnaryOpImm(UnaryOpImm { op, overflow, imm });
        self.build(data, ty, span)
    }

    #[allow(non_snake_case)]
    fn Test(
        self,
        op: Opcode,
        ret: Type,
        arg: Value,
        ty: Type,
        span: SourceSpan,
    ) -> (Inst, &'f mut DataFlowGraph) {
        let data = Instruction::Test(Test { op, arg, ty });
        self.build(data, ret, span)
    }

    #[allow(non_snake_case)]
    fn PrimOp(
        self,
        op: Opcode,
        ty: Type,
        args: ValueList,
        span: SourceSpan,
    ) -> (Inst, &'f mut DataFlowGraph) {
        let data = Instruction::PrimOp(PrimOp { op, args });
        self.build(data, ty, span)
    }

    #[allow(non_snake_case)]
    fn PrimOpImm(
        self,
        op: Opcode,
        ty: Type,
        imm: Immediate,
        args: ValueList,
        span: SourceSpan,
    ) -> (Inst, &'f mut DataFlowGraph) {
        let data = Instruction::PrimOpImm(PrimOpImm { op, imm, args });
        self.build(data, ty, span)
    }
}

impl<'f, T: InstBuilderBase<'f>> InstBuilder<'f> for T {}
