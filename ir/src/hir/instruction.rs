use core::fmt;
use core::ops::{Deref, DerefMut};

use cranelift_entity::entity_impl;
use intrusive_collections::{intrusive_adapter, LinkedListLink, UnsafeRef};
use smallvec::SmallVec;

use miden_diagnostics::{Span, Spanned};

use crate::types::Type;

use super::*;

/// A handle to a single instruction
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Inst(u32);
entity_impl!(Inst, "inst");

/// Represents the data associated with an `Inst`.
///
/// Specifically, this represents a leaf node in the control flow graph of
/// a function, i.e. it links a specific instruction in to the sequence of
/// instructions belonging to a specific block.
#[derive(Clone, Spanned)]
pub struct InstNode {
    pub link: LinkedListLink,
    pub key: Inst,
    pub block: Block,
    #[span]
    pub data: Span<Instruction>,
}
impl InstNode {
    pub fn new(key: Inst, block: Block, data: Span<Instruction>) -> Self {
        Self {
            link: LinkedListLink::default(),
            key,
            block,
            data,
        }
    }
}
impl Deref for InstNode {
    type Target = Span<Instruction>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl DerefMut for InstNode {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

intrusive_adapter!(pub InstAdapter = UnsafeRef<InstNode>: InstNode { link: LinkedListLink });

/// Represents the type of instruction associated with a particular opcode
#[derive(Debug, Clone)]
pub enum Instruction {
    BinaryOp(BinaryOp),
    BinaryOpImm(BinaryOpImm),
    UnaryOp(UnaryOp),
    UnaryOpImm(UnaryOpImm),
    Call(Call),
    Br(Br),
    CondBr(CondBr),
    Switch(Switch),
    Ret(Ret),
    MemCpy(MemCpy),
    PrimOp(PrimOp),
    PrimOpImm(PrimOpImm),
    Test(Test),
    InlineAsm(InlineAsm),
}
impl Instruction {
    pub fn opcode(&self) -> Opcode {
        match self {
            Self::BinaryOp(BinaryOp { ref op, .. })
            | Self::BinaryOpImm(BinaryOpImm { ref op, .. })
            | Self::UnaryOp(UnaryOp { ref op, .. })
            | Self::UnaryOpImm(UnaryOpImm { ref op, .. })
            | Self::Call(Call { ref op, .. })
            | Self::Br(Br { ref op, .. })
            | Self::CondBr(CondBr { ref op, .. })
            | Self::Switch(Switch { ref op, .. })
            | Self::Ret(Ret { ref op, .. })
            | Self::MemCpy(MemCpy { ref op, .. })
            | Self::PrimOp(PrimOp { ref op, .. })
            | Self::PrimOpImm(PrimOpImm { ref op, .. })
            | Self::Test(Test { ref op, .. })
            | Self::InlineAsm(InlineAsm { ref op, .. }) => *op,
        }
    }

    pub fn arguments<'a>(&'a self, pool: &'a ValueListPool) -> &[Value] {
        match self {
            Self::BinaryOp(BinaryOp { ref args, .. }) => args.as_slice(),
            Self::BinaryOpImm(BinaryOpImm { ref arg, .. }) => core::slice::from_ref(arg),
            Self::UnaryOp(UnaryOp { ref arg, .. }) => core::slice::from_ref(arg),
            Self::UnaryOpImm(UnaryOpImm { .. }) => &[],
            Self::Call(Call { ref args, .. }) => args.as_slice(pool),
            Self::Br(Br { ref args, .. }) => args.as_slice(pool),
            Self::CondBr(CondBr { ref cond, .. }) => core::slice::from_ref(cond),
            Self::Switch(Switch { ref arg, .. }) => core::slice::from_ref(arg),
            Self::Ret(Ret { ref args, .. }) => args.as_slice(pool),
            Self::MemCpy(MemCpy { ref args, .. }) => args.as_slice(),
            Self::PrimOp(PrimOp { ref args, .. }) => args.as_slice(pool),
            Self::PrimOpImm(PrimOpImm { ref args, .. }) => args.as_slice(pool),
            Self::Test(Test { ref arg, .. }) => core::slice::from_ref(arg),
            Self::InlineAsm(InlineAsm { ref args, .. }) => args.as_slice(pool),
        }
    }

    pub fn arguments_mut<'a>(&'a mut self, pool: &'a mut ValueListPool) -> &mut [Value] {
        match self {
            Self::BinaryOp(BinaryOp { ref mut args, .. }) => args.as_mut_slice(),
            Self::BinaryOpImm(BinaryOpImm { ref mut arg, .. }) => core::slice::from_mut(arg),
            Self::UnaryOp(UnaryOp { ref mut arg, .. }) => core::slice::from_mut(arg),
            Self::UnaryOpImm(UnaryOpImm { .. }) => &mut [],
            Self::Call(Call { ref mut args, .. }) => args.as_mut_slice(pool),
            Self::Br(Br { ref mut args, .. }) => args.as_mut_slice(pool),
            Self::CondBr(CondBr { ref mut cond, .. }) => core::slice::from_mut(cond),
            Self::Switch(Switch { ref mut arg, .. }) => core::slice::from_mut(arg),
            Self::Ret(Ret { ref mut args, .. }) => args.as_mut_slice(pool),
            Self::MemCpy(MemCpy { ref mut args, .. }) => args.as_mut_slice(),
            Self::PrimOp(PrimOp { ref mut args, .. }) => args.as_mut_slice(pool),
            Self::PrimOpImm(PrimOpImm { ref mut args, .. }) => args.as_mut_slice(pool),
            Self::Test(Test { ref mut arg, .. }) => core::slice::from_mut(arg),
            Self::InlineAsm(InlineAsm { ref mut args, .. }) => args.as_mut_slice(pool),
        }
    }

    pub fn analyze_branch<'a>(&'a self, pool: &'a ValueListPool) -> BranchInfo<'a> {
        match self {
            Self::Br(ref b) if b.op == Opcode::Br => {
                BranchInfo::SingleDest(b.destination, b.args.as_slice(pool))
            }
            Self::Br(ref b) => BranchInfo::SingleDest(b.destination, &b.args.as_slice(pool)[1..]),
            Self::CondBr(CondBr {
                ref then_dest,
                ref else_dest,
                ..
            }) => BranchInfo::MultiDest(vec![
                JumpTable::new(then_dest.0, then_dest.1.as_slice(pool)),
                JumpTable::new(else_dest.0, else_dest.1.as_slice(pool)),
            ]),
            Self::Switch(Switch {
                ref arms,
                ref default,
                ..
            }) => {
                let mut targets = arms
                    .iter()
                    .map(|(_, b)| JumpTable::new(*b, &[]))
                    .collect::<Vec<_>>();
                targets.push(JumpTable::new(*default, &[]));
                BranchInfo::MultiDest(targets)
            }
            _ => BranchInfo::NotABranch,
        }
    }

    pub fn analyze_call<'a>(&'a self, pool: &'a ValueListPool) -> CallInfo<'a> {
        match self {
            Self::Call(ref c) => CallInfo::Direct(c.callee, c.args.as_slice(pool)),
            _ => CallInfo::NotACall,
        }
    }
}

pub enum BranchInfo<'a> {
    NotABranch,
    SingleDest(Block, &'a [Value]),
    MultiDest(Vec<JumpTable<'a>>),
}

pub struct JumpTable<'a> {
    pub destination: Block,
    pub args: &'a [Value],
}
impl<'a> JumpTable<'a> {
    pub fn new(destination: Block, args: &'a [Value]) -> Self {
        Self { destination, args }
    }
}

pub enum CallInfo<'a> {
    NotACall,
    Direct(FuncRef, &'a [Value]),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Opcode {
    /// Asserts the given value is 1
    Assert,
    /// Asserts the given value is 0
    Assertz,
    /// Asserts the two given values are equal
    AssertEq,
    /// Same as `Test`, but does not return a value, instead it traps on failure
    AssertTest,
    /// Represents an immediate integer value
    ImmInt,
    /// Represents an immediate floating-point value
    ImmFloat,
    /// Represents an immediate "null" value, where all bytes of the representation are zeroed
    ImmNull,
    /// Loads the address of a given value into memory
    AddrOf,
    /// Loads a value from a pointer to memory
    Load,
    /// Stores a value to a pointer to memory
    Store,
    /// Copies `n` values of a given type from a source pointer to a destination pointer
    MemCpy,
    /// Casts a pointer value to an integral type
    PtrToInt,
    /// Casts an integral type to a pointer value
    IntToPtr,
    /// Casts from a field element type to an integral type
    ///
    /// It is not valid to perform a cast on any value other than a field element, see
    /// `Trunc`, `Zext`, and `Sext` for casts between machine integer types.
    Cast,
    /// Truncates a larger integral type to a smaller integral type, e.g. i64 -> i32
    Trunc,
    /// Zero-extends a smaller unsigned integral type to a larger unsigned integral type, e.g. u32 -> u64
    Zext,
    /// Sign-extends a smaller signed integral type to a larger signed integral type, e.g. i32 -> i64
    Sext,
    /// Returns true if argument fits in the given integral type, e.g. u32, otherwise false
    Test,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    DivMod,
    Neg,
    Inv,
    Pow2,
    Exp,
    Not,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Rotl,
    Rotr,
    Popcnt,
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    IsOdd,
    Min,
    Max,
    Call,
    Br,
    CondBr,
    Switch,
    Ret,
    InlineAsm,
}
impl Opcode {
    pub fn is_terminator(&self) -> bool {
        match self {
            Self::Br | Self::CondBr | Self::Switch | Self::Ret => true,
            _ => false,
        }
    }

    pub fn num_fixed_args(&self) -> usize {
        match self {
            Self::Assert | Self::Assertz | Self::AssertTest => 1,
            Self::AssertEq => 2,
            // Immediates/constants have none
            Self::ImmInt | Self::ImmFloat | Self::ImmNull => 0,
            // Binary ops always have two
            Self::Store
            | Self::Add
            | Self::Sub
            | Self::Mul
            | Self::Div
            | Self::Mod
            | Self::DivMod
            | Self::Exp
            | Self::And
            | Self::Or
            | Self::Xor
            | Self::Shl
            | Self::Shr
            | Self::Rotl
            | Self::Rotr
            | Self::Eq
            | Self::Neq
            | Self::Gt
            | Self::Gte
            | Self::Lt
            | Self::Lte
            | Self::Min
            | Self::Max => 2,
            // Unary ops always have one
            Self::AddrOf
            | Self::Load
            | Self::PtrToInt
            | Self::IntToPtr
            | Self::Cast
            | Self::Trunc
            | Self::Zext
            | Self::Sext
            | Self::Test
            | Self::Neg
            | Self::Inv
            | Self::Pow2
            | Self::Popcnt
            | Self::Not
            | Self::IsOdd => 1,
            // MemCpy requires source, destination, and arity
            Self::MemCpy => 3,
            // Calls are entirely variable
            Self::Call => 0,
            // Unconditional branches have no fixed arguments
            Self::Br => 0,
            // Ifs have a single argument, the conditional
            Self::CondBr => 1,
            // Switches have a single argument, the input value
            Self::Switch => 1,
            // Returns require at least one argument
            Self::Ret => 1,
            // The following require no arguments
            Self::InlineAsm => 0,
        }
    }

    pub(super) fn results(&self, ctrl_ty: Type, args: &[Type]) -> SmallVec<[Type; 1]> {
        use smallvec::smallvec;

        match self {
            // These ops have no results
            Self::Assert
            | Self::Assertz
            | Self::AssertEq
            | Self::AssertTest
            | Self::Store
            | Self::MemCpy
            | Self::Br
            | Self::CondBr
            | Self::Switch
            | Self::InlineAsm => smallvec![],
            // These ops have fixed result types
            Self::Test | Self::IsOdd => smallvec![Type::I1],
            // For these ops, the controlling type variable determines the type for the op
            Self::ImmInt
            | Self::ImmFloat
            | Self::ImmNull
            | Self::PtrToInt
            | Self::IntToPtr
            | Self::Cast
            | Self::Trunc
            | Self::Zext
            | Self::Sext
            | Self::Ret => {
                smallvec![ctrl_ty]
            }
            // The result type of addrof is derived from the value type
            Self::AddrOf => {
                assert_eq!(args.len(), 1);
                smallvec![Type::Ptr(Box::new(args[0].clone()))]
            }
            // The result type of a load is derived from the pointee type
            Self::Load => {
                assert_eq!(args.len(), 1);
                debug_assert!(
                    args[0].is_pointer(),
                    "expected pointer type, got {:#?}",
                    &args[0]
                );
                smallvec![args[0].pointee().unwrap()]
            }
            // These ops are unary operators whose result type depends on the argument type, which must be integral
            Self::Neg | Self::Inv | Self::Pow2 | Self::Popcnt | Self::Not => {
                assert_eq!(args.len(), 1);
                assert!(args[0].is_integer());
                smallvec![args[0].clone()]
            }
            // These ops are binary operators whose result type depends on the type of the arguments,
            // and those arguments must be the same type
            Self::Add
            | Self::Sub
            | Self::Mul
            | Self::Div
            | Self::Eq
            | Self::Neq
            | Self::Gt
            | Self::Gte
            | Self::Lt
            | Self::Lte
            | Self::Min
            | Self::Max => {
                assert_eq!(args.len(), 2);
                assert_eq!(&args[0], &args[1], "type mismatch: expected operator to have matching operand types, got: {:?} vs {:?}", &args[0], &args[1]);
                smallvec![args[0].clone()]
            }
            // Same as above, but the type must be integral
            Self::Mod
            | Self::DivMod
            | Self::Exp
            | Self::And
            | Self::Or
            | Self::Xor
            | Self::Shl
            | Self::Shr
            | Self::Rotl
            | Self::Rotr => {
                assert_eq!(args.len(), 2);
                assert_eq!(&args[0], &args[1], "type mismatch: expected operator to have matching operand types, got: {:?} vs {:?}", &args[0], &args[1]);
                assert!(
                    args[0].is_integer(),
                    "invalid operand type: expected integral type, got: {:#?}",
                    &args[0]
                );
                smallvec![args[0].clone()]
            }
            // Call results are handled separately
            Self::Call => unreachable!(),
        }
    }
}
impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Assert => f.write_str("assert"),
            Self::Assertz => f.write_str("assertz"),
            Self::AssertEq => f.write_str("assert.eq"),
            Self::AssertTest => f.write_str("assert.test"),
            Self::ImmInt => f.write_str("const.int"),
            Self::ImmFloat => f.write_str("const.float"),
            Self::ImmNull => f.write_str("const.null"),
            Self::AddrOf => f.write_str("addrof"),
            Self::Load => f.write_str("load"),
            Self::Store => f.write_str("store"),
            Self::MemCpy => f.write_str("memcpy"),
            Self::PtrToInt => f.write_str("ptrtoint"),
            Self::IntToPtr => f.write_str("inttoptr"),
            Self::Cast => f.write_str("cast"),
            Self::Trunc => f.write_str("trunc"),
            Self::Zext => f.write_str("zext"),
            Self::Sext => f.write_str("sext"),
            Self::Br => f.write_str("br"),
            Self::CondBr => f.write_str("condbr"),
            Self::Switch => f.write_str("switch"),
            Self::Call => f.write_str("call"),
            Self::Ret => f.write_str("ret"),
            Self::Test => f.write_str("test"),
            Self::Add => f.write_str("add"),
            Self::Sub => f.write_str("sub"),
            Self::Mul => f.write_str("mul"),
            Self::Div => f.write_str("div"),
            Self::Mod => f.write_str("mod"),
            Self::DivMod => f.write_str("divmod"),
            Self::Exp => f.write_str("exp"),
            Self::Neg => f.write_str("neg"),
            Self::Inv => f.write_str("inv"),
            Self::Pow2 => f.write_str("pow2"),
            Self::Not => f.write_str("not"),
            Self::And => f.write_str("and"),
            Self::Or => f.write_str("or"),
            Self::Xor => f.write_str("xor"),
            Self::Shl => f.write_str("shl"),
            Self::Shr => f.write_str("shr"),
            Self::Rotl => f.write_str("rotl"),
            Self::Rotr => f.write_str("rotr"),
            Self::Popcnt => f.write_str("popcnt"),
            Self::Eq => f.write_str("eq"),
            Self::Neq => f.write_str("neq"),
            Self::Gt => f.write_str("gt"),
            Self::Gte => f.write_str("gte"),
            Self::Lt => f.write_str("lt"),
            Self::Lte => f.write_str("lte"),
            Self::IsOdd => f.write_str("is_odd"),
            Self::Min => f.write_str("min"),
            Self::Max => f.write_str("max"),
            Self::InlineAsm => f.write_str("asm"),
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub enum Overflow {
    #[default]
    Unchecked,
    Checked,
    Wrapping,
    Overflowing,
}

#[derive(Debug, Clone)]
pub struct BinaryOp {
    pub op: Opcode,
    pub overflow: Overflow,
    pub args: [Value; 2],
}

#[derive(Debug, Clone)]
pub struct BinaryOpImm {
    pub op: Opcode,
    pub overflow: Overflow,
    pub arg: Value,
    pub imm: Immediate,
}

#[derive(Debug, Clone)]
pub struct UnaryOp {
    pub op: Opcode,
    pub overflow: Overflow,
    pub arg: Value,
}

#[derive(Debug, Clone)]
pub struct UnaryOpImm {
    pub op: Opcode,
    pub overflow: Overflow,
    pub imm: Immediate,
}

#[derive(Debug, Clone)]
pub struct Call {
    pub op: Opcode,
    pub callee: FuncRef,
    pub args: ValueList,
}

/// Branch
#[derive(Debug, Clone)]
pub struct Br {
    pub op: Opcode,
    pub destination: Block,
    pub args: ValueList,
}

/// Conditional Branch
#[derive(Debug, Clone)]
pub struct CondBr {
    pub op: Opcode,
    pub cond: Value,
    pub then_dest: (Block, ValueList),
    pub else_dest: (Block, ValueList),
}

/// Multi-way Branch w/Selector
#[derive(Debug, Clone)]
pub struct Switch {
    pub op: Opcode,
    pub arg: Value,
    pub arms: Vec<(u32, Block)>,
    pub default: Block,
}

/// Return
#[derive(Debug, Clone)]
pub struct Ret {
    pub op: Opcode,
    pub args: ValueList,
}

/// Test and AssertTest
#[derive(Debug, Clone)]
pub struct Test {
    pub op: Opcode,
    pub arg: Value,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct MemCpy {
    pub op: Opcode,
    pub args: [Value; 3],
    pub ty: Type,
}

/// A primop/intrinsic that takes a variable number of arguments
#[derive(Debug, Clone)]
pub struct PrimOp {
    pub op: Opcode,
    pub args: ValueList,
}

/// A primop that takes an immediate for its first argument, followed by a variable number of
/// arguments
#[derive(Debug, Clone)]
pub struct PrimOpImm {
    pub op: Opcode,
    pub imm: Immediate,
    pub args: ValueList,
}

#[derive(Debug, Clone)]
pub struct InlineAsm {
    pub op: Opcode,
    pub body: Vec<AsmInstruction>,
    pub args: ValueList,
}

#[derive(Debug, Clone)]
pub struct AsmInstruction {
    pub name: String,
}
