use cranelift_entity::packed_option::PackedOption;

use miden_diagnostics::SourceSpan;

use super::*;

pub struct FunctionBuilder<'f> {
    pub func: &'f mut Function,
    position: PackedOption<Block>,
}
impl<'f> FunctionBuilder<'f> {
    pub fn new(func: &'f mut Function) -> Self {
        let entry = func.dfg.entry_block();
        let position = func.dfg.last_block().unwrap_or(entry);

        Self {
            func,
            position: position.into(),
        }
    }

    pub fn entry_block(&self) -> Block {
        self.func.dfg.entry_block()
    }

    #[inline]
    pub fn current_block(&self) -> Block {
        self.position.expand().unwrap()
    }

    #[inline]
    pub fn switch_to_block(&mut self, block: Block) {
        self.position = PackedOption::from(block);
    }

    pub fn create_block(&mut self) -> Block {
        self.func.dfg.create_block()
    }

    pub fn detach_block(&mut self, block: Block) {
        assert_ne!(
            block,
            self.current_block(),
            "cannot remove block the builder is currently inserting in"
        );
        self.func.dfg.detach_block(block);
    }

    pub fn block_params(&self, block: Block) -> &[Value] {
        self.func.dfg.block_params(block)
    }

    pub fn append_block_param(&mut self, block: Block, ty: Type, span: SourceSpan) -> Value {
        self.func.dfg.append_block_param(block, ty, span)
    }

    pub fn inst_results(&self, inst: Inst) -> &[Value] {
        self.func.dfg.inst_results(inst)
    }

    pub fn first_result(&self, inst: Inst) -> Value {
        self.func.dfg.first_result(inst)
    }

    pub fn create_global_value(&mut self, data: GlobalValueData) -> GlobalValue {
        self.func.dfg.create_global_value(data)
    }

    pub fn get_import(&self, id: &FunctionIdent) -> Option<&ExternalFunction> {
        self.func.dfg.get_import(id)
    }

    #[inline]
    pub fn import_function<M: AsRef<str>, F: AsRef<str>>(
        &mut self,
        module: M,
        function: F,
        signature: Signature,
        span: SourceSpan,
    ) -> Result<FunctionIdent, ()> {
        let module = Ident::with_empty_span(Symbol::intern(module.as_ref()));
        let function = Ident::new(Symbol::intern(function.as_ref()), span);
        self.func.dfg.import_function(module, function, signature)
    }

    pub fn ins<'a, 'b: 'a>(&'b mut self) -> DefaultInstBuilder<'a> {
        let block = self
            .position
            .expect("must be in a block to insert instructions");
        DefaultInstBuilder::new(&mut self.func.dfg, block)
    }
}

pub struct DefaultInstBuilder<'f> {
    dfg: &'f mut DataFlowGraph,
    ip: InsertionPoint,
}
impl<'f> DefaultInstBuilder<'f> {
    pub(crate) fn new(dfg: &'f mut DataFlowGraph, block: Block) -> Self {
        assert!(dfg.is_block_inserted(block));

        Self {
            dfg,
            ip: InsertionPoint::after(ProgramPoint::Block(block)),
        }
    }

    fn at(dfg: &'f mut DataFlowGraph, ip: InsertionPoint) -> Self {
        Self { dfg, ip }
    }
}
impl<'f> InstBuilderBase<'f> for DefaultInstBuilder<'f> {
    fn data_flow_graph(&self) -> &DataFlowGraph {
        self.dfg
    }

    fn data_flow_graph_mut(&mut self) -> &mut DataFlowGraph {
        self.dfg
    }

    fn insertion_point(&self) -> InsertionPoint {
        self.ip
    }

    fn build(self, data: Instruction, ty: Type, span: SourceSpan) -> (Inst, &'f mut DataFlowGraph) {
        if cfg!(debug_assertions) {
            if let InsertionPoint {
                at: ProgramPoint::Block(blk),
                action: Insert::After,
            } = self.ip
            {
                debug_assert!(
                    self.dfg.is_block_terminated(blk),
                    "cannot append an instruction to a block that is already terminated"
                );
            }
        }

        let inst = self.dfg.insert_inst(self.ip, data, ty, span);
        (inst, self.dfg)
    }
}

/// Instruction builder that replaces an existing instruction.
///
/// The inserted instruction will have the same `Inst` number as the old one.
///
/// If the old instruction still has result values attached, it is assumed that the new instruction
/// produces the same number and types of results. The old result values are preserved. If the
/// replacement instruction format does not support multiple results, the builder panics. It is a
/// bug to leave result values dangling.
pub struct ReplaceBuilder<'f> {
    dfg: &'f mut DataFlowGraph,
    inst: Inst,
}

impl<'f> ReplaceBuilder<'f> {
    /// Create a `ReplaceBuilder` that will overwrite `inst`.
    pub fn new(dfg: &'f mut DataFlowGraph, inst: Inst) -> Self {
        Self { dfg, inst }
    }
}

impl<'f> InstBuilderBase<'f> for ReplaceBuilder<'f> {
    fn data_flow_graph(&self) -> &DataFlowGraph {
        self.dfg
    }

    fn data_flow_graph_mut(&mut self) -> &mut DataFlowGraph {
        self.dfg
    }

    fn insertion_point(&self) -> InsertionPoint {
        InsertionPoint::before(ProgramPoint::Inst(self.inst))
    }

    fn build(
        self,
        data: Instruction,
        ctrl_typevar: Type,
        span: SourceSpan,
    ) -> (Inst, &'f mut DataFlowGraph) {
        use miden_diagnostics::Span;

        // Splat the new instruction on top of the old one.
        self.dfg.insts[self.inst].replace(Span::new(span, data));
        // The old result values, if any, were either detached or non-existent.
        // Construct new ones.
        self.dfg.replace_results(self.inst, ctrl_typevar);

        (self.inst, self.dfg)
    }
}

pub trait InstBuilderBase<'f>: Sized {
    /// Get a reference to the underlying [DataFlowGraph] for this builder
    fn data_flow_graph(&self) -> &DataFlowGraph;
    /// Get a mutable reference to the underlying [DataFlowGraph] for this builder
    fn data_flow_graph_mut(&mut self) -> &mut DataFlowGraph;
    /// Return the insertion point of this builder
    fn insertion_point(&self) -> InsertionPoint;
    /// Build the given instruction, returing it's handle and the inner [DataFlowGraph]
    fn build(
        self,
        data: Instruction,
        ctrl_ty: Type,
        span: SourceSpan,
    ) -> (Inst, &'f mut DataFlowGraph);
    /// Get a default instruction builder using the dataflow graph and insertion point of the current builder
    fn ins<'a, 'b: 'a>(&'b mut self) -> DefaultInstBuilder<'a> {
        let ip = self.insertion_point();
        DefaultInstBuilder::at(self.data_flow_graph_mut(), ip)
    }
}

macro_rules! into_first_result {
    ($built:expr) => {{
        let (inst, dfg) = $built;
        dfg.first_result(inst)
    }};
}

macro_rules! assert_integer_operands {
    ($this:ident, $lhs:ident, $rhs:ident) => {{
        let lty = require_matching_operands!($this, $lhs, $rhs);
        assert!(
            lty.is_integer(),
            "expected {} and {} to be of integral type",
            stringify!($lhs),
            stringify!($rhs)
        );
        lty.clone()
    }};

    ($this:ident, $lhs:ident) => {{
        let ty = require_integer!($this, $lhs);
        ty.clone()
    }};
}

macro_rules! require_matching_operands {
    ($this:ident, $lhs:ident, $rhs:ident) => {{
        let lty = $this.data_flow_graph().value_type($lhs);
        let rty = $this.data_flow_graph().value_type($rhs);
        assert_eq!(
            lty,
            rty,
            "expected {} and {} to be of the same type",
            stringify!($lhs),
            stringify!($rhs)
        );
        lty
    }};
}

macro_rules! require_integer {
    ($this:ident, $val:ident) => {{
        let ty = $this.data_flow_graph().value_type($val);
        assert!(
            ty.is_integer(),
            "expected {} to be of integral type",
            stringify!($val)
        );
        ty
    }};

    ($this:ident, $val:ident, $ty:expr) => {{
        let ty = $this.data_flow_graph().value_type($val);
        let expected_ty = $ty;
        assert!(
            ty.is_integer(),
            "expected {} to be of integral type",
            stringify!($val)
        );
        assert_eq!(
            ty,
            &expected_ty,
            "expected {} to be a {}",
            stringify!($val),
            &expected_ty
        );
        ty
    }};
}

macro_rules! require_pointer {
    ($this:ident, $val:ident) => {{
        let ty = $this.data_flow_graph().value_type($val);
        assert!(
            ty.is_pointer(),
            "expected {} to be of pointer type, got {}",
            stringify!($val),
            &ty
        );
        ty
    }};
}

macro_rules! require_pointee {
    ($this:ident, $val:ident) => {{
        let ty = $this.data_flow_graph().value_type($val);
        let pointee_ty = ty.pointee();
        assert!(
            pointee_ty.is_some(),
            "expected {} to be of pointer type, got {}",
            stringify!($val),
            &ty
        );
        pointee_ty.unwrap()
    }};
}

macro_rules! binary_int_op {
    ($name:ident, $op:expr) => {
        paste::paste! {
            fn $name(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
                let lty = assert_integer_operands!(self, lhs, rhs);
                into_first_result!(self.Binary($op, lty, lhs, rhs, span))
            }
            fn [<$name _checked>](self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
                let lty = assert_integer_operands!(self, lhs, rhs);
                into_first_result!(self.BinaryWithOverflow($op, lty, lhs, rhs, Overflow::Checked, span))
            }
            fn [<$name _wrapping>](self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
                let lty = assert_integer_operands!(self, lhs, rhs);
                into_first_result!(self.BinaryWithOverflow($op, lty, lhs, rhs, Overflow::Wrapping, span))
            }
            fn [<$name _overflowing>](self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
                let lty = assert_integer_operands!(self, lhs, rhs);
                into_first_result!(self.BinaryWithOverflow($op, lty, lhs, rhs, Overflow::Overflowing, span))
            }
            fn [<$name _imm>](self, lhs: Value, imm: Immediate, span: SourceSpan) -> Value {
                let lty = assert_integer_operands!(self, lhs);
                into_first_result!(self.BinaryImm($op, lty, lhs, imm, span))
            }
            fn [<$name _imm_checked>](self, lhs: Value, imm: Immediate, span: SourceSpan) -> Value {
                let lty = assert_integer_operands!(self, lhs);
                into_first_result!(self.BinaryImmWithOverflow($op, lty, lhs, imm, Overflow::Checked, span))
            }
            fn [<$name _imm_wrapping>](self, lhs: Value, imm: Immediate, span: SourceSpan) -> Value {
                let lty = assert_integer_operands!(self, lhs);
                into_first_result!(self.BinaryImmWithOverflow($op, lty, lhs, imm, Overflow::Wrapping, span))
            }
            fn [<$name _imm_overflowing>](self, lhs: Value, imm: Immediate, span: SourceSpan) -> Value {
                let lty = assert_integer_operands!(self, lhs);
                into_first_result!(self.BinaryImmWithOverflow($op, lty, lhs, imm, Overflow::Overflowing, span))
            }
        }
    };
}

macro_rules! unary_int_op {
    ($name:ident, $op:expr) => {
        fn $name(self, rhs: Value, span: SourceSpan) -> Value {
            let rty = assert_integer_operands!(self, rhs);
            into_first_result!(self.Unary($op, rty, rhs, span))
        }
    };
}

macro_rules! integer_literal {
    ($width:literal) => {
        paste::paste! {
            integer_literal!($width, [<i $width>]);
        }
    };

    ($width:ident) => {
        paste::paste! {
            integer_literal!($width, [<i $width>]);
        }
    };

    ($width:literal, $ty:ty) => {
        paste::paste! {
            integer_literal!([<i $width>], [<I $width>], $ty);
        }
    };

    ($width:ident, $ty:ty) => {
        paste::paste! {
            integer_literal!([<i $width>], [<I $width>], $ty);
        }
    };

    ($name:ident, $suffix:ident, $ty:ty) => {
        paste::paste! {
            integer_literal!($name, $suffix, $ty, [<Imm $suffix>]);
        }
    };

    ($name:ident, $suffix:ident, $ty:ty, $op:ident) => {
        paste::paste! {
            fn $name(self, imm: $ty, span: SourceSpan) -> Value {
                into_first_result!(self.UnaryImm(Opcode::$op, Type::$suffix, Immediate::$suffix(imm), span))
            }
        }
    };
}

pub trait InstBuilder<'f>: InstBuilderBase<'f> {
    fn assert(mut self, value: Value, span: SourceSpan) -> Inst {
        require_integer!(self, value, Type::I1);
        let mut vlist = ValueList::default();
        {
            let pool = &mut self.data_flow_graph_mut().value_lists;
            vlist.push(value, pool);
        }
        self.PrimOp(Opcode::Assert, Type::Unit, vlist, span).0
    }

    fn assertz(mut self, value: Value, span: SourceSpan) -> Inst {
        require_integer!(self, value, Type::I1);
        let mut vlist = ValueList::default();
        {
            let pool = &mut self.data_flow_graph_mut().value_lists;
            vlist.push(value, pool);
        }
        self.PrimOp(Opcode::Assertz, Type::Unit, vlist, span).0
    }

    fn assert_eq(mut self, lhs: Value, rhs: Value, span: SourceSpan) -> Inst {
        require_matching_operands!(self, lhs, rhs);
        let mut vlist = ValueList::default();
        {
            let pool = &mut self.data_flow_graph_mut().value_lists;
            vlist.push(lhs, pool);
            vlist.push(rhs, pool);
        }
        self.PrimOp(Opcode::AssertEq, Type::Unit, vlist, span).0
    }

    integer_literal!(1, bool);
    integer_literal!(8);
    integer_literal!(16);
    integer_literal!(32);
    integer_literal!(64);
    integer_literal!(size);

    fn felt(self, i: u64, span: SourceSpan) -> Value {
        into_first_result!(self.UnaryImm(Opcode::ImmFelt, Type::Felt, Immediate::Felt(i), span))
    }

    fn f64(self, f: f64, span: SourceSpan) -> Value {
        into_first_result!(self.UnaryImm(Opcode::ImmF64, Type::F64, Immediate::F64(f), span))
    }

    fn character(self, c: char, span: SourceSpan) -> Value {
        self.i32((c as u32) as i32, span)
    }

    fn alloca(self, ty: Type, span: SourceSpan) -> Value {
        into_first_result!(self.PrimOp(
            Opcode::Alloca,
            Type::Ptr(Box::new(ty)),
            ValueList::default(),
            span,
        ))
    }

    /// Get the address of a global variable whose symbol is `name`
    ///
    /// The type of the value will be `*mut u8`, i.e. a raw pointer to the first byte of the memory
    /// where the symbol is allocated.
    fn symbol_addr<S: AsRef<str>>(self, name: S, span: SourceSpan) -> Value {
        self.symbol_relative_addr(name, 0, span)
    }

    /// Same semantics as `symbol_addr`, but applies a constant offset to the address of the given symbol.
    ///
    /// If the offset is zero, this is equivalent to `symbol_addr`
    fn symbol_relative_addr<S: AsRef<str>>(
        mut self,
        name: S,
        offset: i32,
        span: SourceSpan,
    ) -> Value {
        let gv = self
            .data_flow_graph_mut()
            .create_global_value(GlobalValueData::Symbol {
                name: Ident::new(Symbol::intern(name.as_ref()), span),
                offset,
            });
        into_first_result!(self.Global(gv, Type::Ptr(Box::new(Type::I8)), span))
    }

    /// Loads a value of type `ty` from the global variable whose symbol is `name`.
    ///
    /// NOTE: There is no requirement that the memory contents at the given symbol
    /// contain a valid value of type `ty`. That is left entirely up the caller to
    /// guarantee at a higher level.
    fn load_symbol<S: AsRef<str>>(self, name: S, ty: Type, span: SourceSpan) -> Value {
        self.load_symbol_relative(name, ty, 0, span)
    }

    /// Same semantics as `load_symbol`, but a constant offset is applied to the address before issuing the load.
    fn load_symbol_relative<S: AsRef<str>>(
        mut self,
        name: S,
        ty: Type,
        offset: i32,
        span: SourceSpan,
    ) -> Value {
        let base = self
            .data_flow_graph_mut()
            .create_global_value(GlobalValueData::Symbol {
                name: Ident::new(Symbol::intern(name.as_ref()), span),
                offset: 0,
            });
        self.load_global_relative(base, ty, offset, span)
    }

    /// Loads a value of type `ty` from the address represented by `addr`
    ///
    /// NOTE: There is no requirement that the memory contents at the given symbol
    /// contain a valid value of type `ty`. That is left entirely up the caller to
    /// guarantee at a higher level.
    fn load_global(self, addr: GlobalValue, ty: Type, span: SourceSpan) -> Value {
        self.load_global_relative(addr, ty, 0, span)
    }

    /// Same semantics as `load_global_relative`, but a constant offset is applied to the address before issuing the load.
    fn load_global_relative(
        mut self,
        base: GlobalValue,
        ty: Type,
        offset: i32,
        span: SourceSpan,
    ) -> Value {
        if let GlobalValueData::Load {
            ty: ref base_ty, ..
        } = self.data_flow_graph().global_value(base)
        {
            // If the base global is a load, the target address cannot be computed until runtime,
            // so expand this to the appropriate sequence of instructions to do so in that case
            assert!(
                base_ty.is_pointer(),
                "expected global value to have pointer type"
            );
            let base_ty = base_ty.clone();
            let base = self.ins().load_global(base, base_ty.clone(), span);
            let addr = self.ins().ptrtoint(base, Type::I32, span);
            let offset_addr = self
                .ins()
                .add_imm_checked(addr, Immediate::I32(offset), span);
            let ptr = self.ins().inttoptr(offset_addr, base_ty, span);
            self.load(ptr, span)
        } else {
            // The global address can be computed statically
            let gv = self
                .data_flow_graph_mut()
                .create_global_value(GlobalValueData::Load {
                    base,
                    offset,
                    ty: ty.clone(),
                });
            into_first_result!(self.Global(gv, ty, span))
        }
    }

    /// Computes an address relative to the pointer produced by `base`, by applying an offset
    /// given by multiplying `offset` * the size in bytes of `unit_ty`.
    ///
    /// The type of the pointer produced is the same as the type of the pointer given by `base`
    ///
    /// This is useful in some scenarios where `load_global_relative` is not, namely when computing
    /// the effective address of an element of an array stored in a global variable.
    fn global_addr_offset(
        mut self,
        base: GlobalValue,
        offset: i32,
        unit_ty: Type,
        span: SourceSpan,
    ) -> Value {
        if let GlobalValueData::Load {
            ty: ref base_ty, ..
        } = self.data_flow_graph().global_value(base)
        {
            // If the base global is a load, the target address cannot be computed until runtime,
            // so expand this to the appropriate sequence of instructions to do so in that case
            assert!(
                base_ty.is_pointer(),
                "expected global value to have pointer type"
            );
            let base_ty = base_ty.clone();
            let base = self.ins().load_global(base, base_ty.clone(), span);
            let addr = self.ins().ptrtoint(base, Type::I32, span);
            let computed_offset = unit_ty.layout().size() as isize * (offset as isize);
            let offset_addr = if computed_offset < 0 {
                let offset: i32 = computed_offset
                    .abs()
                    .try_into()
                    .expect("invalid offset: out of range for i32 immediates");
                self.ins()
                    .sub_imm_checked(addr, Immediate::I32(offset), span)
            } else {
                let offset: i32 = computed_offset
                    .try_into()
                    .expect("invalid offset: out of range for i32 immediates");
                self.ins()
                    .add_imm_checked(addr, Immediate::I32(offset), span)
            };
            let ptr = self.ins().inttoptr(offset_addr, base_ty, span);
            self.load(ptr, span)
        } else {
            // The global address can be computed statically
            let gv = self
                .data_flow_graph_mut()
                .create_global_value(GlobalValueData::IAddImm {
                    base,
                    offset,
                    ty: unit_ty.clone(),
                });
            let ty = self.data_flow_graph().global_type(gv);
            into_first_result!(self.Global(gv, ty, span))
        }
    }

    /// Loads a value of the type pointed to by the given pointer, on to the stack
    ///
    /// NOTE: This function will panic if `ptr` is not a pointer typed value
    fn load(self, addr: Value, span: SourceSpan) -> Value {
        let ty = require_pointee!(self, addr).clone();
        let data = Instruction::Load(LoadOp {
            op: Opcode::Load,
            addr,
            ty: ty.clone(),
        });
        into_first_result!(self.build(data, ty, span))
    }

    /// Stores `value` to the address given by `ptr`
    ///
    /// NOTE: This function will panic if the pointer and pointee types do not match
    fn store(self, ptr: Value, value: Value, span: SourceSpan) -> Inst {
        let pointee_ty = require_pointee!(self, ptr);
        let value_ty = self.data_flow_graph().value_type(value);
        assert_eq!(
            pointee_ty, value_ty,
            "expected value to be a {}, got {}",
            pointee_ty, value_ty
        );
        self.Binary(Opcode::Store, Type::Unit, ptr, value, span).0
    }

    /// Copies `count` values of type `ty` from the memory at address `src`, to the
    /// memory at address `dst`.
    fn memcpy(self, src: Value, dst: Value, count: Value, ty: Type, span: SourceSpan) -> Inst {
        require_pointer!(self, src);
        require_pointer!(self, dst);
        require_integer!(self, count);
        let data = Instruction::MemCpy(MemCpy {
            op: Opcode::MemCpy,
            args: [src, dst, count],
            ty,
        });
        self.build(data, Type::Unit, span).0
    }

    /// This is a cast operation that permits performing arithmetic on pointer values
    /// by casting a pointer to a specified integral type.
    fn ptrtoint(self, arg: Value, ty: Type, span: SourceSpan) -> Value {
        require_pointer!(self, arg);
        assert!(ty.is_integer(), "expected integral type, got {}", &ty);
        into_first_result!(self.Unary(Opcode::PtrToInt, ty, arg, span))
    }

    /// This is the inverse of `ptrtoint`, used to recover a pointer that was
    /// previously cast to an integer type. It may also be used to cast arbitrary
    /// integer values to pointers.
    ///
    /// In both cases, use of the resulting pointer must not violate the semantics
    /// of the higher level language being represented in Miden IR.
    fn inttoptr(self, arg: Value, ty: Type, span: SourceSpan) -> Value {
        require_integer!(self, arg);
        assert!(ty.is_pointer(), "expected pointer type, got {}", &ty);
        into_first_result!(self.Unary(Opcode::IntToPtr, ty, arg, span))
    }

    /// Cast `arg` to a value of type `ty`
    ///
    /// NOTE: This is only valid for numeric to numeric, or pointer to pointer casts.
    /// For numeric to pointer, or pointer to numeric casts, use `inttoptr` and `ptrtoint`
    /// respectively.
    fn cast(self, arg: Value, ty: Type, span: SourceSpan) -> Value {
        let arg_ty = self.data_flow_graph().value_type(arg);
        let both_numeric = arg_ty.is_numeric() && ty.is_numeric();
        let both_pointers = arg_ty.is_pointer() && ty.is_pointer();
        let kind_matches = both_numeric || both_pointers;
        assert!(
            kind_matches,
            "invalid cast, expected source and target types to be of the same kind, where a kind is either numeric or pointer: value is of type {}, and target type is {}",
            &arg_ty, &ty
        );
        into_first_result!(self.Unary(Opcode::Cast, ty, arg, span))
    }

    /// Truncates an integral value as necessary to fit in `ty`.
    ///
    /// NOTE: Truncating a value into a larger type has undefined behavior, it is
    /// equivalent to extending a value without doing anything with the new high-order
    /// bits of the resulting value.
    fn trunc(self, arg: Value, ty: Type, span: SourceSpan) -> Value {
        require_integer!(self, arg);
        assert!(ty.is_integer(), "expected integer type, got {}", &ty);
        into_first_result!(self.Unary(Opcode::Trunc, ty, arg, span))
    }

    /// Extends an integer into a larger integeral type, by zero-extending the value,
    /// i.e. the new high-order bits of the resulting value will be all zero.
    ///
    /// NOTE: This function will panic if `ty` is smaller than `arg`.
    ///
    /// If `arg` is the same type as `ty`, `arg` is returned as-is
    fn zext(self, arg: Value, ty: Type, span: SourceSpan) -> Value {
        let arg_ty = require_integer!(self, arg);
        assert!(ty.is_integer(), "expected integer type, got {}", &ty);
        if arg_ty == &ty {
            return arg;
        }
        assert!(
            arg_ty.bitwidth() <= ty.bitwidth(),
            "invalid extension: target type ({:?}) is smaller than the argument type ({:?})",
            &ty,
            &arg_ty
        );
        into_first_result!(self.Unary(Opcode::Zext, ty, arg, span))
    }

    /// Extends an integer into a larger integeral type, by sign-extending the value,
    /// i.e. the new high-order bits of the resulting value will all match the sign bit.
    ///
    /// NOTE: This function will panic if `ty` is smaller than `arg`.
    ///
    /// If `arg` is the same type as `ty`, `arg` is returned as-is
    fn sext(self, arg: Value, ty: Type, span: SourceSpan) -> Value {
        let arg_ty = require_integer!(self, arg);
        assert!(ty.is_integer(), "expected integer type, got {}", &ty);
        if arg_ty == &ty {
            return arg;
        }
        assert!(
            arg_ty.bitwidth() <= ty.bitwidth(),
            "invalid extension: target type ({:?}) is smaller than the argument type ({:?})",
            &ty,
            &arg_ty
        );
        into_first_result!(self.Unary(Opcode::Sext, ty, arg, span))
    }

    binary_int_op!(add, Opcode::Add);
    binary_int_op!(sub, Opcode::Sub);
    binary_int_op!(mul, Opcode::Mul);
    binary_int_op!(div, Opcode::Div);
    binary_int_op!(min, Opcode::Min);
    binary_int_op!(max, Opcode::Max);
    binary_int_op!(r#mod, Opcode::Mod);
    binary_int_op!(divmod, Opcode::DivMod);
    binary_int_op!(exp, Opcode::Exp);
    binary_int_op!(and, Opcode::And);
    binary_int_op!(or, Opcode::Or);
    binary_int_op!(xor, Opcode::Xor);
    binary_int_op!(shl, Opcode::Shl);
    binary_int_op!(shr, Opcode::Shr);
    binary_int_op!(rotl, Opcode::Rotl);
    binary_int_op!(rotr, Opcode::Rotr);
    unary_int_op!(neg, Opcode::Neg);
    unary_int_op!(inv, Opcode::Inv);
    unary_int_op!(incr, Opcode::Incr);
    unary_int_op!(pow2, Opcode::Pow2);
    unary_int_op!(not, Opcode::Not);
    unary_int_op!(popcnt, Opcode::Popcnt);

    fn eq(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
        into_first_result!(self.Binary(Opcode::Eq, Type::I1, lhs, rhs, span))
    }

    fn neq(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
        into_first_result!(self.Binary(Opcode::Neq, Type::I1, lhs, rhs, span))
    }

    fn gt(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
        require_integer!(self, lhs);
        require_integer!(self, rhs);
        into_first_result!(self.Binary(Opcode::Gt, Type::I1, lhs, rhs, span))
    }

    fn gte(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
        require_integer!(self, lhs);
        require_integer!(self, rhs);
        into_first_result!(self.Binary(Opcode::Gte, Type::I1, lhs, rhs, span))
    }

    fn lt(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
        require_integer!(self, lhs);
        require_integer!(self, rhs);
        into_first_result!(self.Binary(Opcode::Lt, Type::I1, lhs, rhs, span))
    }

    fn lte(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
        require_integer!(self, lhs);
        require_integer!(self, rhs);
        into_first_result!(self.Binary(Opcode::Lte, Type::I1, lhs, rhs, span))
    }

    fn is_odd(self, value: Value, span: SourceSpan) -> Value {
        require_integer!(self, value);
        into_first_result!(self.Unary(Opcode::IsOdd, Type::I1, value, span))
    }

    fn call(mut self, callee: FunctionIdent, args: &[Value], span: SourceSpan) -> Inst {
        let mut vlist = ValueList::default();
        {
            let dfg = self.data_flow_graph_mut();
            assert!(
                dfg.get_import(&callee).is_some(),
                "must import callee ({}) before calling it",
                &callee
            );
            vlist.extend(args.iter().copied(), &mut dfg.value_lists);
        }
        self.Call(Opcode::Call, callee, vlist, span).0
    }

    fn syscall(mut self, callee: FunctionIdent, args: &[Value], span: SourceSpan) -> Inst {
        let mut vlist = ValueList::default();
        {
            let dfg = self.data_flow_graph_mut();
            assert!(
                dfg.get_import(&callee).is_some(),
                "must import callee ({}) before calling it",
                &callee
            );
            vlist.extend(args.iter().copied(), &mut dfg.value_lists);
        }
        self.Call(Opcode::Syscall, callee, vlist, span).0
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
        require_integer!(self, cond, Type::I1);
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
        require_integer!(self, arg, Type::I32);
        self.Switch(arg, arms, default, span).0
    }

    fn ret(mut self, returning: Option<Value>, span: SourceSpan) -> Inst {
        let mut vlist = ValueList::default();
        if let Some(value) = returning {
            let pool = &mut self.data_flow_graph_mut().value_lists;
            vlist.push(value, pool);
        }
        self.Ret(vlist, span).0
    }

    fn ret_imm(self, arg: Immediate, span: SourceSpan) -> Inst {
        let data = Instruction::RetImm(RetImm {
            op: Opcode::Ret,
            arg,
        });
        self.build(data, Type::Unit, span).0
    }

    fn unreachable(self, span: SourceSpan) -> Inst {
        let data = Instruction::PrimOp(PrimOp {
            op: Opcode::Unreachable,
            args: ValueList::default(),
        });
        self.build(data, Type::Never, span).0
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
        callee: FunctionIdent,
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

    #[allow(non_snake_case)]
    fn Global(
        self,
        global: GlobalValue,
        ty: Type,
        span: SourceSpan,
    ) -> (Inst, &'f mut DataFlowGraph) {
        let data = Instruction::GlobalValue(GlobalValueOp {
            op: Opcode::GlobalValue,
            global,
        });
        self.build(data, ty, span)
    }
}

impl<'f, T: InstBuilderBase<'f>> InstBuilder<'f> for T {}
