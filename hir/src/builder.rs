use cranelift_entity::packed_option::PackedOption;

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
    ) -> Result<FunctionIdent, SymbolConflictError> {
        let module = Ident::with_empty_span(Symbol::intern(module.as_ref()));
        let function = Ident::new(Symbol::intern(function.as_ref()), span);
        self.func.dfg.import_function(module, function, signature)
    }

    pub fn ins<'a, 'b: 'a>(&'b mut self) -> DefaultInstBuilder<'a> {
        let block = self.position.expect("must be in a block to insert instructions");
        DefaultInstBuilder::new(&mut self.func.dfg, block)
    }
}

pub struct DefaultInstBuilder<'f> {
    dfg: &'f mut DataFlowGraph,
    ip: InsertionPoint,
}
impl<'f> DefaultInstBuilder<'f> {
    pub(crate) fn new(dfg: &'f mut DataFlowGraph, block: Block) -> Self {
        assert!(dfg.is_block_linked(block));

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
                    !self.dfg.is_block_terminated(blk),
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
    /// Get a default instruction builder using the dataflow graph and insertion point of the
    /// current builder
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

macro_rules! require_unsigned_integer {
    ($this:ident, $val:ident) => {{
        let ty = $this.data_flow_graph().value_type($val);
        assert!(
            ty.is_unsigned_integer(),
            "expected {} to be an unsigned integral type",
            stringify!($val)
        );
        ty
    }};
}

macro_rules! require_integer {
    ($this:ident, $val:ident) => {{
        let ty = $this.data_flow_graph().value_type($val);
        assert!(ty.is_integer(), "expected {} to be of integral type", stringify!($val));
        ty
    }};

    ($this:ident, $val:ident, $ty:expr) => {{
        let ty = $this.data_flow_graph().value_type($val);
        let expected_ty = $ty;
        assert!(ty.is_integer(), "expected {} to be of integral type", stringify!($val));
        assert_eq!(ty, &expected_ty, "expected {} to be a {}", stringify!($val), &expected_ty);
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
            fn [<$name _imm>](self, lhs: Value, imm: Immediate, span: SourceSpan) -> Value {
                let lty = assert_integer_operands!(self, lhs);
                into_first_result!(self.BinaryImm($op, lty, lhs, imm, span))
            }
        }
    };
}

macro_rules! binary_int_op_with_overflow {
    ($name:ident, $op:expr) => {
        paste::paste! {
            fn [<$name _checked>](self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
                let lty = assert_integer_operands!(self, lhs, rhs);
                into_first_result!(self.BinaryWithOverflow($op, lty, lhs, rhs, Overflow::Checked, span))
            }
            fn [<$name _unchecked>](self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
                let lty = assert_integer_operands!(self, lhs, rhs);
                into_first_result!(self.BinaryWithOverflow($op, lty, lhs, rhs, Overflow::Unchecked, span))
            }
            fn [<$name _wrapping>](self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
                let lty = assert_integer_operands!(self, lhs, rhs);
                into_first_result!(self.BinaryWithOverflow($op, lty, lhs, rhs, Overflow::Wrapping, span))
            }
            fn [<$name _overflowing>](self, lhs: Value, rhs: Value, span: SourceSpan) -> Inst {
                let lty = assert_integer_operands!(self, lhs, rhs);
                self.BinaryWithOverflow($op, lty, lhs, rhs, Overflow::Overflowing, span).0
            }
            fn [<$name _imm_checked>](self, lhs: Value, imm: Immediate, span: SourceSpan) -> Value {
                let lty = assert_integer_operands!(self, lhs);
                into_first_result!(self.BinaryImmWithOverflow($op, lty, lhs, imm, Overflow::Checked, span))
            }
            fn [<$name _imm_unchecked>](self, lhs: Value, imm: Immediate, span: SourceSpan) -> Value {
                let lty = assert_integer_operands!(self, lhs);
                into_first_result!(self.BinaryImmWithOverflow($op, lty, lhs, imm, Overflow::Unchecked, span))
            }
            fn [<$name _imm_wrapping>](self, lhs: Value, imm: Immediate, span: SourceSpan) -> Value {
                let lty = assert_integer_operands!(self, lhs);
                into_first_result!(self.BinaryImmWithOverflow($op, lty, lhs, imm, Overflow::Wrapping, span))
            }
            fn [<$name _imm_overflowing>](self, lhs: Value, imm: Immediate, span: SourceSpan) -> Inst {
                let lty = assert_integer_operands!(self, lhs);
                self.BinaryImmWithOverflow($op, lty, lhs, imm, Overflow::Overflowing, span).0
            }
        }
    }
}

macro_rules! checked_binary_int_op {
    ($name:ident, $op:expr) => {
        paste::paste! {
            fn [<$name _checked>](self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
                let lty = assert_integer_operands!(self, lhs, rhs);
                into_first_result!(self.BinaryWithOverflow($op, lty, lhs, rhs, Overflow::Checked, span))
            }
            fn [<$name _unchecked>](self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
                let lty = assert_integer_operands!(self, lhs, rhs);
                into_first_result!(self.BinaryWithOverflow($op, lty, lhs, rhs, Overflow::Unchecked, span))
            }
            fn [<$name _imm_checked>](self, lhs: Value, imm: Immediate, span: SourceSpan) -> Value {
                let lty = assert_integer_operands!(self, lhs);
                into_first_result!(self.BinaryImmWithOverflow($op, lty, lhs, imm, Overflow::Checked, span))
            }
            fn [<$name _imm_unchecked>](self, lhs: Value, imm: Immediate, span: SourceSpan) -> Value {
                let lty = assert_integer_operands!(self, lhs);
                into_first_result!(self.BinaryImmWithOverflow($op, lty, lhs, imm, Overflow::Unchecked, span))
            }
        }
    };
}

macro_rules! binary_boolean_op {
    ($name:ident, $op:expr) => {
        paste::paste! {
            fn $name(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
                let lty = require_matching_operands!(self, lhs, rhs).clone();
                assert_eq!(lty, Type::I1, concat!(stringify!($name), " requires boolean operands"));
                into_first_result!(self.Binary($op, lty, lhs, rhs, span))
            }
            fn [<$name _imm>](self, lhs: Value, imm: Immediate, span: SourceSpan) -> Value {
                let lty = require_integer!(self, lhs, Type::I1).clone();
                assert_eq!(lty, Type::I1, concat!(stringify!($name), " requires boolean operands"));
                into_first_result!(self.BinaryImm($op, lty, lhs, imm, span))
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

macro_rules! unary_int_op_with_overflow {
    ($name:ident, $op:expr) => {
        paste::paste! {
            fn [<$name _checked>](self, rhs: Value, span: SourceSpan) -> Value {
                let rty = assert_integer_operands!(self, rhs);
                into_first_result!(self.UnaryWithOverflow($op, rty, rhs, Overflow::Checked, span))
            }
            fn [<$name _unchecked>](self, rhs: Value, span: SourceSpan) -> Value {
                let rty = assert_integer_operands!(self, rhs);
                into_first_result!(self.UnaryWithOverflow($op, rty, rhs, Overflow::Unchecked, span))
            }
            fn [<$name _wrapping>](self, rhs: Value, span: SourceSpan) -> Value {
                let rty = assert_integer_operands!(self, rhs);
                into_first_result!(self.UnaryWithOverflow($op, rty, rhs, Overflow::Wrapping, span))
            }
            fn [<$name _overflowing>](self, rhs: Value, span: SourceSpan) -> Inst {
                let rty = assert_integer_operands!(self, rhs);
                self.UnaryWithOverflow($op, rty, rhs, Overflow::Overflowing, span).0
            }
        }
    };
}

macro_rules! unary_boolean_op {
    ($name:ident, $op:expr) => {
        fn $name(self, rhs: Value, span: SourceSpan) -> Value {
            let rty = require_integer!(self, rhs, Type::I1).clone();
            into_first_result!(self.Unary($op, rty, rhs, span))
        }
    };
}

macro_rules! integer_literal {
    ($width:literal) => {
        paste::paste! {
            unsigned_integer_literal!($width);
            signed_integer_literal!($width);
        }
    };

    ($width:ident) => {
        paste::paste! {
            unsigned_integer_literal!($width);
            signed_integer_literal!($width);
        }
    };

    ($width:literal, $ty:ty) => {
        paste::paste! {
            unsigned_integer_literal!($width, $ty);
            signed_integer_literal!($width, $ty);
        }
    };

    ($width:ident, $ty:ty) => {
        paste::paste! {
            unsigned_integer_literal!($width, $ty);
            signed_integer_literal!($width, $ty);
        }
    };
}

macro_rules! unsigned_integer_literal {
    ($width:literal) => {
        paste::paste! {
            unsigned_integer_literal!($width, [<u $width>]);
        }
    };

    ($width:ident) => {
        paste::paste! {
            unsigned_integer_literal!($width, [<u $width>]);
        }
    };

    ($width:literal, $ty:ty) => {
        paste::paste! {
            unsigned_integer_literal!([<u $width>], [<U $width>], $ty);
        }
    };

    ($width:ident, $ty:ty) => {
        paste::paste! {
            unsigned_integer_literal!([<u $width>], [<U $width>], $ty);
        }
    };

    ($name:ident, $suffix:ident, $ty:ty) => {
        paste::paste! {
            unsigned_integer_literal!($name, $suffix, $ty, [<Imm $suffix>]);
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

macro_rules! signed_integer_literal {
    ($width:literal) => {
        paste::paste! {
            signed_integer_literal!($width, [<i $width>]);
        }
    };

    ($width:ident) => {
        paste::paste! {
            signed_integer_literal!($width, [<i $width>]);
        }
    };

    ($width:literal, $ty:ty) => {
        paste::paste! {
            signed_integer_literal!([<i $width>], [<I $width>], $ty);
        }
    };

    ($width:ident, $ty:ty) => {
        paste::paste! {
            signed_integer_literal!([<i $width>], [<I $width>], $ty);
        }
    };

    ($name:ident, $suffix:ident, $ty:ty) => {
        paste::paste! {
            signed_integer_literal!($name, $suffix, $ty, [<Imm $suffix>]);
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
            vlist.extend([rhs, lhs], pool);
        }
        self.PrimOp(Opcode::AssertEq, Type::Unit, vlist, span).0
    }

    fn assert_eq_imm(mut self, lhs: Immediate, rhs: Value, span: SourceSpan) -> Inst {
        require_integer!(self, rhs);
        let mut vlist = ValueList::default();
        {
            let pool = &mut self.data_flow_graph_mut().value_lists;
            vlist.push(rhs, pool);
        }
        self.PrimOpImm(Opcode::AssertEq, Type::Unit, lhs, vlist, span).0
    }

    signed_integer_literal!(1, bool);
    integer_literal!(8);
    integer_literal!(16);
    integer_literal!(32);
    integer_literal!(64);

    fn felt(self, i: Felt, span: SourceSpan) -> Value {
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

    fn mem_grow(mut self, value: Value, span: SourceSpan) -> Value {
        require_integer!(self, value, Type::U32);
        let mut vlist = ValueList::default();
        {
            let pool = &mut self.data_flow_graph_mut().value_lists;
            vlist.push(value, pool);
        }
        into_first_result!(self.PrimOp(Opcode::MemGrow, Type::I32, vlist, span,))
    }

    /// Get a [GlobalValue] which represents the address of a global variable whose symbol is `name`
    ///
    /// On it's own, this does nothing, you must use the resulting [GlobalValue] with a builder
    /// that expects one as an argument, or use `global_value` to obtain a [Value] from it.
    fn symbol<S: AsRef<str>>(self, name: S, span: SourceSpan) -> GlobalValue {
        self.symbol_relative(name, 0, span)
    }

    /// Same semantics as `symbol`, but applies a constant offset to the address of the given
    /// symbol.
    ///
    /// If the offset is zero, this is equivalent to `symbol`
    fn symbol_relative<S: AsRef<str>>(
        mut self,
        name: S,
        offset: i32,
        span: SourceSpan,
    ) -> GlobalValue {
        self.data_flow_graph_mut().create_global_value(GlobalValueData::Symbol {
            name: Ident::new(Symbol::intern(name.as_ref()), span),
            offset,
        })
    }

    /// Get the address of a global variable whose symbol is `name`
    ///
    /// The type of the pointer produced is given as `ty`. It is up to the caller
    /// to ensure that loading memory from that pointer is valid for the provided
    /// type.
    fn symbol_addr<S: AsRef<str>>(self, name: S, ty: Type, span: SourceSpan) -> Value {
        self.symbol_relative_addr(name, 0, ty, span)
    }

    /// Same semantics as `symbol_addr`, but applies a constant offset to the address of the given
    /// symbol.
    ///
    /// If the offset is zero, this is equivalent to `symbol_addr`
    fn symbol_relative_addr<S: AsRef<str>>(
        mut self,
        name: S,
        offset: i32,
        ty: Type,
        span: SourceSpan,
    ) -> Value {
        assert!(ty.is_pointer(), "expected pointer type, got '{}'", &ty);
        let gv = self.data_flow_graph_mut().create_global_value(GlobalValueData::Symbol {
            name: Ident::new(Symbol::intern(name.as_ref()), span),
            offset,
        });
        into_first_result!(self.Global(gv, ty, span))
    }

    /// Loads a value of type `ty` from the global variable whose symbol is `name`.
    ///
    /// NOTE: There is no requirement that the memory contents at the given symbol
    /// contain a valid value of type `ty`. That is left entirely up the caller to
    /// guarantee at a higher level.
    fn load_symbol<S: AsRef<str>>(self, name: S, ty: Type, span: SourceSpan) -> Value {
        self.load_symbol_relative(name, ty, 0, span)
    }

    /// Same semantics as `load_symbol`, but a constant offset is applied to the address before
    /// issuing the load.
    fn load_symbol_relative<S: AsRef<str>>(
        mut self,
        name: S,
        ty: Type,
        offset: i32,
        span: SourceSpan,
    ) -> Value {
        let base = self.data_flow_graph_mut().create_global_value(GlobalValueData::Symbol {
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

    /// Same semantics as `load_global_relative`, but a constant offset is applied to the address
    /// before issuing the load.
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
            assert!(base_ty.is_pointer(), "expected global value to have pointer type");
            let base_ty = base_ty.clone();
            let base = self.ins().load_global(base, base_ty.clone(), span);
            let addr = self.ins().ptrtoint(base, Type::U32, span);
            let offset_addr = if offset >= 0 {
                self.ins().add_imm_checked(addr, Immediate::U32(offset as u32), span)
            } else {
                self.ins().sub_imm_checked(addr, Immediate::U32(offset.unsigned_abs()), span)
            };
            let ptr = self.ins().inttoptr(offset_addr, base_ty, span);
            self.load(ptr, span)
        } else {
            // The global address can be computed statically
            let gv = self.data_flow_graph_mut().create_global_value(GlobalValueData::Load {
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
            assert!(base_ty.is_pointer(), "expected global value to have pointer type");
            let base_ty = base_ty.clone();
            let base = self.ins().load_global(base, base_ty.clone(), span);
            let addr = self.ins().ptrtoint(base, Type::U32, span);
            let unit_size: i32 = unit_ty
                .size_in_bytes()
                .try_into()
                .expect("invalid type: size is larger than 2^32");
            let computed_offset = unit_size * offset;
            let offset_addr = if computed_offset >= 0 {
                self.ins().add_imm_checked(addr, Immediate::U32(offset as u32), span)
            } else {
                self.ins().sub_imm_checked(addr, Immediate::U32(offset.unsigned_abs()), span)
            };
            let ptr = self.ins().inttoptr(offset_addr, base_ty, span);
            self.load(ptr, span)
        } else {
            // The global address can be computed statically
            let gv = self.data_flow_graph_mut().create_global_value(GlobalValueData::IAddImm {
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
        into_first_result!(self.build(data, Type::Ptr(Box::new(ty)), span))
    }

    /// Stores `value` to the address given by `ptr`
    ///
    /// NOTE: This function will panic if the pointer and pointee types do not match
    fn store(mut self, ptr: Value, value: Value, span: SourceSpan) -> Inst {
        let pointee_ty = require_pointee!(self, ptr);
        let value_ty = self.data_flow_graph().value_type(value);
        assert_eq!(pointee_ty, value_ty, "expected value to be a {}, got {}", pointee_ty, value_ty);
        let mut vlist = ValueList::default();
        {
            let dfg = self.data_flow_graph_mut();
            vlist.extend([ptr, value], &mut dfg.value_lists);
        }
        self.PrimOp(Opcode::Store, Type::Unit, vlist, span).0
    }

    /// Copies `count` values from the memory at address `src`, to the memory at address `dst`.
    ///
    /// The unit size for `count` is determined by the `src` pointer type, i.e. a pointer to u8
    /// will copy one `count` bytes, a pointer to u16 will copy `count * 2` bytes, and so on.
    ///
    /// NOTE: The source and destination pointer types must match, or this function will panic.
    fn memcpy(mut self, src: Value, dst: Value, count: Value, span: SourceSpan) -> Inst {
        require_integer!(self, count);
        let src_ty = require_pointer!(self, src);
        let dst_ty = require_pointer!(self, dst);
        assert_eq!(src_ty, dst_ty, "the source and destination pointers must be the same type");
        let mut vlist = ValueList::default();
        {
            let dfg = self.data_flow_graph_mut();
            vlist.extend([src, dst, count], &mut dfg.value_lists);
        }
        self.PrimOp(Opcode::MemCpy, Type::Unit, vlist, span).0
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
        require_unsigned_integer!(self, arg);
        assert!(ty.is_pointer(), "expected pointer type, got {}", &ty);
        into_first_result!(self.Unary(Opcode::IntToPtr, ty, arg, span))
    }

    /// This is an intrinsic which derives a new pointer from an existing pointer to an aggregate.
    ///
    /// In short, this represents the common need to calculate a new pointer from an existing
    /// pointer, but without losing provenance of the original pointer. It is specifically
    /// intended for use in obtaining a pointer to an element/field of an array/struct, of the
    /// correct type, given a well typed pointer to the aggregate.
    ///
    /// This function will panic if the pointer is not to an aggregate type
    ///
    /// The new pointer is derived by statically navigating the structure of the pointee type, using
    /// `offsets` to guide the traversal. Initially, the first offset is relative to the original
    /// pointer, where `0` refers to the base/first field of the object. The second offset is then
    /// relative to the base of the object selected by the first offset, and so on. Offsets must
    /// remain in bounds, any attempt to index outside a type's boundaries will result in a
    /// panic.
    fn getelementptr(mut self, ptr: Value, mut indices: &[usize], span: SourceSpan) -> Value {
        let mut ty = require_pointee!(self, ptr);
        assert!(!indices.is_empty(), "getelementptr requires at least one index");

        // Calculate the offset in bytes from `ptr` to get the element pointer
        let mut offset = 0;
        while let Some((index, rest)) = indices.split_first().map(|(i, rs)| (*i, rs)) {
            indices = rest;
            match ty {
                Type::Array(ref element_ty, len) => {
                    assert!(
                        index < *len,
                        "invalid getelementptr: index of {} is out of bounds for {}",
                        index,
                        ty
                    );
                    let element_size = element_ty.size_in_bytes();
                    let min_align = element_ty.min_alignment();
                    let padded_element_size = element_size.align_up(min_align);
                    ty = element_ty;
                    offset += padded_element_size * index;
                }
                Type::Struct(ref struct_ty) => {
                    assert!(
                        index < struct_ty.len(),
                        "invalid getelementptr: index of {} is out of bounds for {}",
                        index,
                        ty
                    );
                    let field = struct_ty.get(index);
                    offset += field.offset as usize;
                    ty = &field.ty;
                }
                other => panic!("invalid getelementptr: cannot index values of type {}", other),
            }
        }

        // Emit the instruction sequence for offsetting the pointer
        let ty = Type::Ptr(Box::new(ty.clone()));
        // Cast the pointer to an integer
        let addr = self.ins().ptrtoint(ptr, Type::U32, span);
        // Add the element offset to the pointer
        let offset: u32 = offset.try_into().expect(
            "invalid getelementptr type: computed offset cannot possibly fit in linear memory",
        );
        let new_addr = self.ins().add_imm_checked(addr, Immediate::U32(offset), span);
        // Cast back to a pointer to the selected element type
        self.inttoptr(new_addr, ty, span)
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
            "invalid cast, expected source and target types to be of the same kind, where a kind \
             is either numeric or pointer: value is of type {}, and target type is {}",
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
            arg_ty.size_in_bits() <= ty.size_in_bits(),
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
            arg_ty.size_in_bits() <= ty.size_in_bits(),
            "invalid extension: target type ({:?}) is smaller than the argument type ({:?})",
            &ty,
            &arg_ty
        );
        into_first_result!(self.Unary(Opcode::Sext, ty, arg, span))
    }

    binary_int_op_with_overflow!(add, Opcode::Add);
    binary_int_op_with_overflow!(sub, Opcode::Sub);
    binary_int_op_with_overflow!(mul, Opcode::Mul);
    checked_binary_int_op!(div, Opcode::Div);
    binary_int_op!(min, Opcode::Min);
    binary_int_op!(max, Opcode::Max);
    checked_binary_int_op!(r#mod, Opcode::Mod);
    checked_binary_int_op!(divmod, Opcode::DivMod);
    binary_int_op!(exp, Opcode::Exp);
    binary_boolean_op!(and, Opcode::And);
    binary_int_op!(band, Opcode::Band);
    binary_boolean_op!(or, Opcode::Or);
    binary_int_op!(bor, Opcode::Bor);
    binary_boolean_op!(xor, Opcode::Xor);
    binary_int_op!(bxor, Opcode::Bxor);
    binary_int_op_with_overflow!(shl, Opcode::Shl);
    binary_int_op_with_overflow!(shr, Opcode::Shr);
    binary_int_op!(rotl, Opcode::Rotl);
    binary_int_op!(rotr, Opcode::Rotr);
    unary_int_op!(neg, Opcode::Neg);
    unary_int_op!(inv, Opcode::Inv);
    unary_int_op_with_overflow!(incr, Opcode::Incr);
    unary_int_op!(pow2, Opcode::Pow2);
    unary_boolean_op!(not, Opcode::Not);
    unary_int_op!(bnot, Opcode::Bnot);
    unary_int_op!(popcnt, Opcode::Popcnt);

    fn eq(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
        into_first_result!(self.Binary(Opcode::Eq, Type::I1, lhs, rhs, span))
    }

    fn eq_imm(self, lhs: Value, imm: Immediate, span: SourceSpan) -> Value {
        let lty = assert_integer_operands!(self, lhs);
        assert_eq!(
            lty,
            imm.ty(),
            "expected immediate to be the same type as non-immediate operand",
        );
        into_first_result!(self.BinaryImm(Opcode::Eq, Type::I1, lhs, imm, span))
    }

    fn neq(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
        into_first_result!(self.Binary(Opcode::Neq, Type::I1, lhs, rhs, span))
    }

    fn neq_imm(self, lhs: Value, imm: Immediate, span: SourceSpan) -> Value {
        let lty = assert_integer_operands!(self, lhs);
        assert_eq!(
            lty,
            imm.ty(),
            "expected immediate to be the same type as non-immediate operand",
        );
        into_first_result!(self.BinaryImm(Opcode::Neq, Type::I1, lhs, imm, span))
    }

    fn gt(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
        require_integer!(self, lhs);
        require_integer!(self, rhs);
        into_first_result!(self.Binary(Opcode::Gt, Type::I1, lhs, rhs, span))
    }

    fn gt_imm(self, lhs: Value, imm: Immediate, span: SourceSpan) -> Value {
        let lty = assert_integer_operands!(self, lhs);
        assert_eq!(
            lty,
            imm.ty(),
            "expected immediate to be the same type as non-immediate operand",
        );
        into_first_result!(self.BinaryImm(Opcode::Gt, Type::I1, lhs, imm, span))
    }

    fn gte(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
        require_integer!(self, lhs);
        require_integer!(self, rhs);
        into_first_result!(self.Binary(Opcode::Gte, Type::I1, lhs, rhs, span))
    }

    fn gte_imm(self, lhs: Value, imm: Immediate, span: SourceSpan) -> Value {
        let lty = assert_integer_operands!(self, lhs);
        assert_eq!(
            lty,
            imm.ty(),
            "expected immediate to be the same type as non-immediate operand",
        );
        into_first_result!(self.BinaryImm(Opcode::Gte, Type::I1, lhs, imm, span))
    }

    fn lt(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
        require_integer!(self, lhs);
        require_integer!(self, rhs);
        into_first_result!(self.Binary(Opcode::Lt, Type::I1, lhs, rhs, span))
    }

    fn lt_imm(self, lhs: Value, imm: Immediate, span: SourceSpan) -> Value {
        let lty = assert_integer_operands!(self, lhs);
        assert_eq!(
            lty,
            imm.ty(),
            "expected immediate to be the same type as non-immediate operand",
        );
        into_first_result!(self.BinaryImm(Opcode::Lt, Type::I1, lhs, imm, span))
    }

    fn lte(self, lhs: Value, rhs: Value, span: SourceSpan) -> Value {
        require_integer!(self, lhs);
        require_integer!(self, rhs);
        into_first_result!(self.Binary(Opcode::Lte, Type::I1, lhs, rhs, span))
    }

    fn lte_imm(self, lhs: Value, imm: Immediate, span: SourceSpan) -> Value {
        let lty = assert_integer_operands!(self, lhs);
        assert_eq!(
            lty,
            imm.ty(),
            "expected immediate to be the same type as non-immediate operand",
        );
        into_first_result!(self.BinaryImm(Opcode::Lte, Type::I1, lhs, imm, span))
    }

    #[allow(clippy::wrong_self_convention)]
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

    fn select(mut self, cond: Value, a: Value, b: Value, span: SourceSpan) -> Value {
        let mut vlist = ValueList::default();
        let ty = require_matching_operands!(self, a, b).clone();
        {
            let dfg = self.data_flow_graph_mut();
            assert_eq!(
                dfg.value_type(cond),
                &Type::I1,
                "select expect the type of the condition to be i1"
            );
            vlist.extend([cond, a, b], &mut dfg.value_lists);
        }
        into_first_result!(self.PrimOp(Opcode::Select, ty, vlist, span))
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
        self.CondBr(cond, then_dest, then_vlist, else_dest, else_vlist, span).0
    }

    fn switch(self, arg: Value, arms: Vec<(u32, Block)>, default: Block, span: SourceSpan) -> Inst {
        require_integer!(self, arg, Type::U32);
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

    fn inline_asm(
        self,
        args: &[Value],
        results: impl IntoIterator<Item = Type>,
        span: SourceSpan,
    ) -> MasmBuilder<Self> {
        MasmBuilder::new(self, args, results.into_iter().collect(), span)
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
        let data = Instruction::BinaryOp(BinaryOp {
            op,
            overflow: None,
            // We place arguments in stack order for more efficient codegen
            args: [rhs, lhs],
        });
        self.build(data, ty, span)
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
            overflow: Some(overflow),
            // We place arguments in stack order for more efficient codegen
            args: [rhs, lhs],
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
        let data = Instruction::BinaryOpImm(BinaryOpImm {
            op,
            overflow: None,
            arg,
            imm,
        });
        self.build(data, ty, span)
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
            overflow: Some(overflow),
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
        let data = Instruction::UnaryOp(UnaryOp {
            op,
            overflow: None,
            arg,
        });
        self.build(data, ty, span)
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
        let data = Instruction::UnaryOp(UnaryOp {
            op,
            overflow: Some(overflow),
            arg,
        });
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
        let data = Instruction::UnaryOpImm(UnaryOpImm {
            op,
            overflow: None,
            imm,
        });
        self.build(data, ty, span)
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
        let data = Instruction::UnaryOpImm(UnaryOpImm {
            op,
            overflow: Some(overflow),
            imm,
        });
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
