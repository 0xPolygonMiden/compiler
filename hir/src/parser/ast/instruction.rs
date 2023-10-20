use crate::{FunctionIdent, Ident, Overflow, Type, };
use super::*;

/// Represents a value in Miden IR.
///
/// All intermediate values are named, and have an associated [Value].
/// Value identifiers must be globally unique.
pub struct Value {
    pub name: Ident,
}
impl Value {
    pub fn new(name: Ident) -> Self {
        Self { name }
    }
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Immediates are converted at a later stage
pub enum Immediate {
    Pos(u128),
    Neg(u128),
}
impl fmt::Display for Immediate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Pos(0) | Self::Neg(0) => f.write_str("0"),
            Self::Pos(v) => write!(f, "{}", v),
            Self::Neg(v) => write!(f, "-{}", v),
        }
    }
}

/// Represents a single instruction.
///
/// An instruction consists of a single operation, and a number of values that
/// represent the results of the operation. Additionally, the instruction contains
/// the types of the produced results
#[derive(Spanned)]
pub struct Instruction {
    #[span]
    pub span: SourceSpan,
    pub values: Vec<Value>,
    pub op: Operation,
    pub types: Vec<Type>,
}
impl Instruction {
    pub fn new(span: SourceSpan, values: Vec<Value>, op: Operation, types: Vec<Type>) -> Self {
        Self {
            span,
            values,
            op,
            types,
        }
    }
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.values.len() == 0 {
            write!(f, "{}", self.op)?;
        } else {
            for (i, v) in self.values.iter().enumerate() {
                if i != 0 {
                    write!(f, ", {}", v)?;
                } else {
                    write!(f, "{}", v)?;
                }
            }
            write!(f, " = {} : ", self.op)?;
            for (i, t) in self.types.iter().enumerate() {
                if i != 0 {
                    write!(f, ", {}", t)?;
                } else {
                    write!(f, "{}", t)?;
                }
            }
        }
        Ok(())
    }
}

/// Represents a operation and its arguments
pub enum Operation {
    BinaryOp(BinaryOpCode, Value, Value),
    BinaryImmOp(BinaryImmOpCode, Value, Immediate),
    UnaryOp(UnaryOpCode, Value),
    UnaryImmOp(UnaryImmOpCode, Immediate),
    ReturnOp(Vec<Value>),
    CallOp(CallOp, FunctionIdent, Vec<Value>),
    CondOp(Value, Destination, Destination),
    BranchOp(Destination),
    SwitchOp(Value, Vec<SwitchBranch>),
    TestOp(Type, Value),
    PrimOp(PrimOpCode, Vec<Value>),
    LoadOp(Value),
    MemCpyOp(Type, Value, Value, Value),
    GlobalValueOp(GlobalValueOp),
}
impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BinaryOp(op, v1, v2) => {
                write!(f, "{} {} {}", op, v1, v2)
            }
            Self::BinaryImmOp(op, v, i) => {
                write!(f, "{} {} {}", op, v, i)
            }
            Self::UnaryOp(op, v) => {
                write!(f, "{} {}", op, v)
            }
            Self::UnaryImmOp(op, i) => {
                write!(f, "{} {}", op, i)
            }
            Self::ReturnOp(vs) => {
                f.write_str("ret")?;
                for (i, v) in vs.iter().enumerate() {
                    if i > 0 {
                        f.write_str(",")?;
                    }
                    write!(f, " {}", v)?;
                }
                Ok(())
            }
            Self::CallOp(op, id, vs) => {
                write!(f, "{} {} (", op, id)?;
                for (i, v) in vs.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                f.write_str(")")
            }
            Self::CondOp(v, dest1, dest2) => {
                write!(f, "cond {}, {}, {}", v, dest1, dest2)
            }
            Self::BranchOp(dest) => {
                write!(f, "branch {}", dest)
            }
            Self::SwitchOp(v, branches) => {
                writeln!(f, "switch {} {{", v)?;
                for (i, b) in branches.iter().enumerate() {
                    if i > 0 {
                        f.write_str(",\n")?;
                    }
                    //TODO: Indentation
                    write!(f, "{}", b)?;
                }
                //TODO: Indentation
                f.write_str("\n}}")
            }
            Self::TestOp(t, v) => {
                write!(f, "test.{} {}", t, v)
            }
            Self::PrimOp(op, vs) => {
                write!(f, "{}", op)?;
                for (i, v) in vs.iter().enumerate() {
                    if i > 0 {
                        f.write_str(",")?;
                    }
                    write!(f, " {}", v)?;
                }
                Ok(())
            }
            Self::LoadOp(v) => {
                write!(f, "load {}", v)
            }
            Self::MemCpyOp(t, v1, v2, v3) => {
                write!(f, "memcpy.{} {}, {}, {}", t, v1, v2, v3)
            }
            Self::GlobalValueOp(op) => {
                write!(f, "{}", op)
            }
        }
    }
}

/// Used to distinguish between user calls and kernel calls
pub enum CallOp {
    Call,
    SysCall,
}
impl fmt::Display for CallOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Call => f.write_str("call"),
            Self::SysCall => f.write_str("syscall"),
        }
    }
}

/// Used to distinguish between binary operations
pub enum BinaryOpCode {
    Add(Overflow),
    Sub(Overflow),
    Mul(Overflow),
    Div(Overflow),
    Min(Overflow),
    Max(Overflow),
    Mod(Overflow),
    DivMod(Overflow),
    Exp(Overflow),
    And,
    BAnd(Overflow),
    Or,
    BOr(Overflow),
    Xor,
    BXor(Overflow),
    Shl(Overflow),
    Shr(Overflow),
    Rotl(Overflow),
    Rotr(Overflow),
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    Store,
}
impl fmt::Display for BinaryOpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Add(overflow) => {
                write!(f, "add.{}", overflow)
            }
            Self::Sub(overflow) => {
                write!(f, "sub.{}", overflow)
            }
            Self::Mul(overflow) => {
                write!(f, "mul.{}", overflow)
            }
            Self::Div(overflow) => {
                write!(f, "div.{}", overflow)
            }
            Self::Min(overflow) => {
                write!(f, "min.{}", overflow)
            }
            Self::Max(overflow) => {
                write!(f, "max.{}", overflow)
            }
            Self::Mod(overflow) => {
                write!(f, "mod.{}", overflow)
            }
            Self::DivMod(overflow) => {
                write!(f, "divmod.{}", overflow)
            }
            Self::Exp(overflow) => {
                write!(f, "exp.{}", overflow)
            }
            Self::And => f.write_str("and"),
            Self::BAnd(overflow) => {
                write!(f, "band.{}", overflow)
            }
            Self::Or => f.write_str("or"),
            Self::BOr(overflow) => {
                write!(f, "bor.{}", overflow)
            }
            Self::Xor => f.write_str("xor"),
            Self::BXor(overflow) => {
                write!(f, "bxor.{}", overflow)
            }
            Self::Shl(overflow) => {
                write!(f, "shl.{}", overflow)
            }
            Self::Shr(overflow) => {
                write!(f, "shr.{}", overflow)
            }
            Self::Rotl(overflow) => {
                write!(f, "rotl.{}", overflow)
            }
            Self::Rotr(overflow) => {
                write!(f, "rotr.{}", overflow)
            }
            Self::Eq => f.write_str("eq"),
            Self::Neq => f.write_str("neq"),
            Self::Gt => f.write_str("gt"),
            Self::Gte => f.write_str("gte"),
            Self::Lt => f.write_str("lt"),
            Self::Lte => f.write_str("lte"),
            Self::Store => f.write_str("store"),
        }
    }
}

/// Used to distinguish between immediate binary operations
pub enum BinaryImmOpCode {
    AddImm(Overflow),
    SubImm(Overflow),
    MulImm(Overflow),
    DivImm(Overflow),
    MinImm(Overflow),
    MaxImm(Overflow),
    ModImm(Overflow),
    DivModImm(Overflow),
    ExpImm(Overflow),
    AndImm,
    BAndImm(Overflow),
    OrImm,
    BOrImm(Overflow),
    XorImm,
    BXorImm(Overflow),
    ShlImm(Overflow),
    ShrImm(Overflow),
    RotlImm(Overflow),
    RotrImm(Overflow),
}
impl fmt::Display for BinaryImmOpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AddImm(overflow) => {
                write!(f, "add_imm.{}", overflow)
            }
            Self::SubImm(overflow) => {
                write!(f, "sub_imm.{}", overflow)
            }
            Self::MulImm(overflow) => {
                write!(f, "mul_imm.{}", overflow)
            }
            Self::DivImm(overflow) => {
                write!(f, "div_imm.{}", overflow)
            }
            Self::MinImm(overflow) => {
                write!(f, "min_imm.{}", overflow)
            }
            Self::MaxImm(overflow) => {
                write!(f, "max_imm.{}", overflow)
            }
            Self::ModImm(overflow) => {
                write!(f, "mod_imm.{}", overflow)
            }
            Self::DivModImm(overflow) => {
                write!(f, "divmod_imm.{}", overflow)
            }
            Self::ExpImm(overflow) => {
                write!(f, "exp_imm.{}", overflow)
            }
            Self::AndImm => f.write_str("and"),
            Self::BAndImm(overflow) => {
                write!(f, "band_imm.{}", overflow)
            }
            Self::OrImm => f.write_str("or"),
            Self::BOrImm(overflow) => {
                write!(f, "bor_imm.{}", overflow)
            }
            Self::XorImm => f.write_str("xor"),
            Self::BXorImm(overflow) => {
                write!(f, "bxor_imm.{}", overflow)
            }
            Self::ShlImm(overflow) => {
                write!(f, "shl_imm.{}", overflow)
            }
            Self::ShrImm(overflow) => {
                write!(f, "shr_imm.{}", overflow)
            }
            Self::RotlImm(overflow) => {
                write!(f, "rotl_imm.{}", overflow)
            }
            Self::RotrImm(overflow) => {
                write!(f, "rotr_imm.{}", overflow)
            }
        }
    }
}

/// Used to distinguish between unary operations
pub enum UnaryOpCode {
    Inv,
    Incr,
    Pow2,
    Not,
    BNot,
    PopCnt,
    IsOdd,
    Cast,
    PtrToInt,
    IntToPtr,
    TruncW,
    Zext,
    Sext,
    Neg,
}
impl fmt::Display for UnaryOpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Inv => f.write_str("inv"),
            Self::Incr => f.write_str("incr"),
            Self::Pow2 => f.write_str("pow2"),
            Self::Not => f.write_str("not"),
            Self::BNot => f.write_str("bnot"),
            Self::PopCnt => f.write_str("popcnt"),
            Self::IsOdd => f.write_str("is_odd"),
            Self::Cast => f.write_str("cast"),
            Self::PtrToInt => f.write_str("ptrtoint"),
            Self::IntToPtr => f.write_str("inttoptr"),
            Self::TruncW => f.write_str("truncw"),
            Self::Zext => f.write_str("zext"),
            Self::Sext => f.write_str("sext"),
            Self::Neg => f.write_str("neg"),
        }
    }
}

/// Used to distinguish between immediate unary operations
pub enum UnaryImmOpCode {
    I1,
    I8,
    I16,
    I32,
    I64,
    Felt,
    F64,
}
impl fmt::Display for UnaryImmOpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("const.")?;
        match self {
            Self::I1 => f.write_str("i1"),
            Self::I8 => f.write_str("i8"),
            Self::I16 => f.write_str("i16"),
            Self::I32 => f.write_str("i32"),
            Self::I64 => f.write_str("i64"),
            Self::Felt => f.write_str("felt"),
            Self::F64 => f.write_str("f64"),
        }
    }
}

/// Used to distinguish between primary operations
pub enum PrimOpCode {
    Select,
    Assert,
    Assertz,
    AssertEq,
    Alloca,
    Unreachable,
}
impl fmt::Display for PrimOpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Select => f.write_str("select"),
            Self::Assert => f.write_str("assert"),
            Self::Assertz => f.write_str("assertz"),
            Self::AssertEq => f.write_str("asserteq"),
            Self::Alloca => f.write_str("alloca"),
            Self::Unreachable => f.write_str("unreachable"),
        }
    }
}

/// Memory offset for global variable reads.
/// Conversion to i32 happens during transformation to hir.
pub enum Offset {
    Pos(u128),
    Neg(u128),
}
impl Offset {
    pub fn is_zero(&self) -> bool {
        match self {
            Self::Pos(offset) | Self::Neg(offset) => offset == &0,
        }
    }
}
impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Pos(offset) => {
                if offset > &0 {
                    write!(f, "+{}", offset)?;
                }
            }
            Self::Neg(offset) => {
                if offset > &0 {
                    write!(f, "-{}", offset)?;
                }
            }
        }
        Ok(())
    }
}

/// Used to distinguish between nested global value operations
pub enum GlobalValueOpNested {
    Symbol(Ident, Offset),
    Load(Box<GlobalValueOpNested>, Offset),
    Cast(Box<GlobalValueOpNested>, Offset, Type),
}
impl fmt::Display for GlobalValueOpNested {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Symbol(id, offset) => {
                write!(f, "@ {} {}", id, offset)
            }
            Self::Load(nested, offset) => {
                f.write_str("* ")?;
                if offset.is_zero() {
                    write!(f, "{}", nested)?;
                } else {
                    write!(f, "({}){}", nested, offset)?;
                }
                Ok(())
            }
            Self::Cast(nested, offset, ty) => {
                write!(f, "* ({}) {} as {}", nested, offset, ty)
            }
        }
    }
}

/// Used to distinguish between top-level global value operations
pub enum GlobalValueOp {
    Symbol(Ident, Offset),
    Load(GlobalValueOpNested, Offset),
    Cast(GlobalValueOpNested, Offset, Type),
    IAddImm(u128, Type, GlobalValueOpNested),
}
impl fmt::Display for GlobalValueOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("global.")?;
        match self {
            Self::Symbol(id, offset) => {
                write!(f, "symbol @ {} {}", id, offset)
            }
            Self::Load(nested, offset) => {
                f.write_str("load ")?;
                if offset.is_zero() {
                    write!(f, "{}", nested)?;
                } else {
                    write!(f, "({}) {}", nested, offset)?;
                }
                Ok(())
            }
            Self::Cast(nested, offset, ty) => {
                write!(f, "load ({}) {} {}", nested, offset, ty)
            }
            Self::IAddImm(i, ty, nested) => {
                write!(f, "iadd.{}.{} {}", i, ty, nested)
            }
        }
    }
}

/// The destination of a branch/jump
pub struct Destination {
    pub label: Label,
    pub args: Vec<BlockArgument>,
}
impl Destination {
    pub fn new(label: Label, args: Vec<BlockArgument>) -> Self {
        Self { label, args }
    }
}
impl fmt::Display for Destination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.label)?;
        if self.args.len() > 0 {
            f.write_str(" (")?;
            for (i, arg) in self.args.iter().enumerate() {
                if i > 0 {
                    write!(f, ", {}", arg)?;
                } else {
                    write!(f, "{}", arg)?;
                }
            }
            f.write_str(")")?;
        }
        Ok(())
    }
}

/// A branch of a switch operation
pub enum SwitchBranch {
    Test(u128, Label),
    Default(Label),
}
impl fmt::Display for SwitchBranch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Test(test, label) => {
                write!(f, "{} => {}", test, label)
            }
            Self::Default(label) => {
                write!(f, "{}", label)
            }
        }
    }
}
