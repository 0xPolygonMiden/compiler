use core::{fmt, mem};

use num_bigint::BigInt;

use super::LexicalError;
use crate::Symbol;

/// The token type produced by [Lexer]
#[derive(Debug, Clone)]
pub enum Token {
    Eof,
    Comment,
    Error(LexicalError),
    /// A module or value identifier
    Ident(Symbol),
    /// A possibly-qualified function identifier
    FunctionIdent((Symbol, Symbol)),
    /// A value is an identifier of the form `v(0|[1-9][0-9]*)`
    Value(crate::Value),
    /// A block is an identifer of the form `(blk(0|[1-9][0-9]*))`
    Block(crate::Block),
    /// Fixed-width integers smaller than or equal to the platform native integer width
    Int(isize),
    /// Represents large integer types, such as i128 or u256
    BigInt(BigInt),
    /// Hex strings are used to initialize global variables
    Hex(crate::ConstantData),
    Kernel,
    Module,
    Internal,
    Odr,
    Extern,
    External,
    Pub,
    Fn,
    Cc,
    Fast,
    Sret,
    Zext,
    Sext,
    Trunc,
    Ret,
    Call,
    Syscall,
    Br,
    CondBr,
    Switch,
    Test,
    Load,
    MemCpy,
    Asm,
    MemoryGrow,
    AddUnchecked,
    AddChecked,
    AddOverflowing,
    AddWrapping,
    SubUnchecked,
    SubChecked,
    SubOverflowing,
    SubWrapping,
    MulUnchecked,
    MulChecked,
    MulOverflowing,
    MulWrapping,
    DivChecked,
    DivUnchecked,
    ModUnchecked,
    ModChecked,
    DivModUnchecked,
    DivModChecked,
    Min,
    Max,
    Exp,
    And,
    BAnd,
    Or,
    BOr,
    Xor,
    BXor,
    ShlUnchecked,
    ShlChecked,
    ShrUnchecked,
    ShrChecked,
    RotlUnchecked,
    RotlChecked,
    RotrUnchecked,
    RotrChecked,
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    Store,
    Inv,
    IncrUnchecked,
    IncrChecked,
    Pow2,
    Not,
    BNot,
    PopCnt,
    IsOdd,
    Cast,
    PtrToInt,
    IntToPtr,
    Neg,
    ConstI1,
    ConstI8,
    ConstU8,
    ConstI16,
    ConstU16,
    ConstI32,
    ConstU32,
    ConstI64,
    ConstU64,
    ConstFelt,
    Select,
    Assert,
    Assertz,
    AssertEq,
    Alloca,
    Unreachable,
    Global,
    GlobalSymbol,
    GlobalLoad,
    GlobalIAdd,
    As,
    Id,
    Symbol,
    IAdd,
    I1,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    I128,
    U128,
    U256,
    F64,
    Felt,
    Mut,
    DoubleQuote,
    Colon,
    Semicolon,
    Comma,
    Dot,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Equal,
    RDoubleArrow,
    Plus,
    Minus,
    RArrow,
    Star,
    Ampersand,
    Bang,
    At,
}
impl Token {
    pub fn from_keyword_or_ident(s: &str) -> Self {
        match s {
            "kernel" => Self::Kernel,
            "module" => Self::Module,
            "internal" => Self::Internal,
            "odr" => Self::Odr,
            "extern" => Self::Extern,
            "external" => Self::External,
            "pub" => Self::Pub,
            "fn" => Self::Fn,
            "cc" => Self::Cc,
            "fast" => Self::Fast,
            "sret" => Self::Sret,
            "zext" => Self::Zext,
            "sext" => Self::Sext,
            "trunc" => Self::Trunc,
            "ret" => Self::Ret,
            "call" => Self::Call,
            "syscall" => Self::Syscall,
            "br" => Self::Br,
            "condbr" => Self::CondBr,
            "switch" => Self::Switch,
            "test" => Self::Test,
            "load" => Self::Load,
            "memcpy" => Self::MemCpy,
            "asm" => Self::Asm,
            "memory.grow" => Self::MemoryGrow,
            "add.unchecked" => Self::AddUnchecked,
            "add.checked" => Self::AddChecked,
            "add.overflowing" => Self::AddOverflowing,
            "add.wrapping" => Self::AddWrapping,
            "sub.unchecked" => Self::SubUnchecked,
            "sub.checked" => Self::SubChecked,
            "sub.overflowing" => Self::SubOverflowing,
            "sub.wrapping" => Self::SubWrapping,
            "mul.unchecked" => Self::MulUnchecked,
            "mul.checked" => Self::MulChecked,
            "mul.overflowing" => Self::MulOverflowing,
            "mul.wrapping" => Self::MulWrapping,
            "div.unchecked" => Self::DivUnchecked,
            "div.checked" => Self::DivChecked,
            "mod.unchecked" => Self::ModUnchecked,
            "mod.checked" => Self::ModChecked,
            "divmod.unchecked" => Self::DivModUnchecked,
            "divmod.checked" => Self::DivModChecked,
            "min" => Self::Min,
            "max" => Self::Max,
            "exp" => Self::Exp,
            "and" => Self::And,
            "band" => Self::BAnd,
            "or" => Self::Or,
            "bor" => Self::BOr,
            "xor" => Self::Xor,
            "bxor" => Self::BXor,
            "shl.unchecked" => Self::ShlUnchecked,
            "shl.checked" => Self::ShlChecked,
            "shr.unchecked" => Self::ShrUnchecked,
            "shr.checked" => Self::ShrChecked,
            "rotl.unchecked" => Self::RotlUnchecked,
            "rotl.checked" => Self::RotlChecked,
            "rotr.unchecked" => Self::RotrUnchecked,
            "rotr.checked" => Self::RotrChecked,
            "eq" => Self::Eq,
            "neq" => Self::Neq,
            "gt" => Self::Gt,
            "gte" => Self::Gte,
            "lt" => Self::Lt,
            "lte" => Self::Lte,
            "store" => Self::Store,
            "inv" => Self::Inv,
            "incr.unchecked" => Self::IncrUnchecked,
            "incr.checked" => Self::IncrChecked,
            "pow2" => Self::Pow2,
            "not" => Self::Not,
            "bnot" => Self::BNot,
            "popcnt" => Self::PopCnt,
            "is_odd" => Self::IsOdd,
            "cast" => Self::Cast,
            "ptrtoint" => Self::PtrToInt,
            "inttoprt" => Self::IntToPtr,
            "neg" => Self::Neg,
            "const.i1" => Self::ConstI1,
            "const.i8" => Self::ConstI8,
            "const.u8" => Self::ConstU8,
            "const.i16" => Self::ConstI16,
            "const.u16" => Self::ConstU16,
            "const.i32" => Self::ConstI32,
            "const.u32" => Self::ConstU32,
            "const.i64" => Self::ConstI64,
            "const.u64" => Self::ConstU64,
            "select" => Self::Select,
            "assert" => Self::Assert,
            "assertz" => Self::Assertz,
            "assert.eq" => Self::AssertEq,
            "alloca" => Self::Alloca,
            "unreachable" => Self::Unreachable,
            "as" => Self::As,
            "id" => Self::Id,
            "global" => Self::Global,
            "global.symbol" => Self::GlobalSymbol,
            "global.load" => Self::GlobalLoad,
            "global.iadd" => Self::GlobalIAdd,
            "symbol" => Self::Symbol,
            "iadd" => Self::IAdd,
            "i1" => Self::I1,
            "i8" => Self::I8,
            "u8" => Self::U8,
            "i16" => Self::I16,
            "u16" => Self::U16,
            "i32" => Self::I32,
            "u32" => Self::U32,
            "i64" => Self::I64,
            "u64" => Self::U64,
            "i128" => Self::I128,
            "u128" => Self::U128,
            "u256" => Self::U256,
            "f64" => Self::F64,
            "felt" => Self::Felt,
            "mut" => Self::Mut,
            other => Self::Ident(Symbol::intern(other)),
        }
    }
}
impl Eq for Token {}
impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        match self {
            Self::Int(i) => {
                if let Self::Int(i2) = other {
                    return *i == *i2;
                }
            }
            Self::BigInt(i) => {
                if let Self::BigInt(i2) = other {
                    return *i == *i2;
                }
            }
            Self::Error(_) => {
                if let Self::Error(_) = other {
                    return true;
                }
            }
            Self::Ident(i) => {
                if let Self::Ident(i2) = other {
                    return i == i2;
                }
            }
            Self::FunctionIdent((m1, f1)) => {
                if let Self::FunctionIdent((m2, f2)) = other {
                    return m1 == m2 && f1 == f2;
                }
            }
            Self::Hex(a) => {
                if let Self::Hex(b) = other {
                    return a == b;
                }
            }
            Self::Value(a) => {
                if let Self::Value(b) = other {
                    return a == b;
                }
            }
            Self::Block(a) => {
                if let Self::Block(b) = other {
                    return a == b;
                }
            }
            _ => return mem::discriminant(self) == mem::discriminant(other),
        }
        false
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Eof => write!(f, "EOF"),
            Self::Comment => write!(f, "COMMENT"),
            Self::Error(_) => write!(f, "ERROR"),
            Self::Ident(ref id) => write!(f, "{id}"),
            Self::FunctionIdent((ref module, ref function)) => write!(f, "{module}::{function}"),
            Self::Value(ref id) => write!(f, "{id}"),
            Self::Block(ref id) => write!(f, "{id}"),
            Self::Int(ref i) => write!(f, "{i}"),
            Self::BigInt(ref i) => write!(f, "{i}"),
            Self::Hex(ref data) => write!(f, "{data}"),
            Self::Kernel => write!(f, "kernel"),
            Self::Module => write!(f, "module"),
            Self::Internal => write!(f, "internal"),
            Self::Odr => write!(f, "odr"),
            Self::Extern => write!(f, "extern"),
            Self::External => write!(f, "external"),
            Self::Pub => write!(f, "pub"),
            Self::Fn => write!(f, "fn"),
            Self::Cc => write!(f, "cc"),
            Self::Fast => write!(f, "fast"),
            Self::Sret => write!(f, "sret"),
            Self::Zext => write!(f, "zext"),
            Self::Sext => write!(f, "sext"),
            Self::Trunc => write!(f, "trunc"),
            Self::Ret => write!(f, "ret"),
            Self::Call => write!(f, "call"),
            Self::Syscall => write!(f, "syscall"),
            Self::Br => write!(f, "br"),
            Self::CondBr => write!(f, "condbr"),
            Self::Switch => write!(f, "switch"),
            Self::Test => write!(f, "test"),
            Self::Load => write!(f, "load"),
            Self::MemCpy => write!(f, "memcpy"),
            Self::Asm => write!(f, "asm"),
            Self::MemoryGrow => write!(f, "memory.grow"),
            Self::AddUnchecked => write!(f, "add.unchecked"),
            Self::AddChecked => write!(f, "add.checked"),
            Self::AddOverflowing => write!(f, "add.overflowing"),
            Self::AddWrapping => write!(f, "add.wrapping"),
            Self::SubUnchecked => write!(f, "sub.unchecked"),
            Self::SubChecked => write!(f, "sub.checked"),
            Self::SubOverflowing => write!(f, "sub.overflowing"),
            Self::SubWrapping => write!(f, "sub.wrapping"),
            Self::MulUnchecked => write!(f, "mul.unchecked"),
            Self::MulChecked => write!(f, "mul.checked"),
            Self::MulOverflowing => write!(f, "mul.overflowing"),
            Self::MulWrapping => write!(f, "mul.wrapping"),
            Self::DivUnchecked => write!(f, "div.unchecked"),
            Self::DivChecked => write!(f, "div.checked"),
            Self::ModUnchecked => write!(f, "mod.unchecked"),
            Self::ModChecked => write!(f, "mod.checked"),
            Self::DivModUnchecked => write!(f, "divmod.unchecked"),
            Self::DivModChecked => write!(f, "divmod.checked"),
            Self::Min => write!(f, "min"),
            Self::Max => write!(f, "max"),
            Self::Exp => write!(f, "exp"),
            Self::And => write!(f, "and"),
            Self::BAnd => write!(f, "band"),
            Self::Or => write!(f, "or"),
            Self::BOr => write!(f, "bor"),
            Self::Xor => write!(f, "xor"),
            Self::BXor => write!(f, "bxor"),
            Self::ShlUnchecked => write!(f, "shl.unchecked"),
            Self::ShlChecked => write!(f, "shl.checked"),
            Self::ShrUnchecked => write!(f, "shr.unchecked"),
            Self::ShrChecked => write!(f, "shr.checked"),
            Self::RotlUnchecked => write!(f, "rotl.unchecked"),
            Self::RotlChecked => write!(f, "rotl.checked"),
            Self::RotrUnchecked => write!(f, "rotr.unchecked"),
            Self::RotrChecked => write!(f, "rotr.checked"),
            Self::Eq => write!(f, "eq"),
            Self::Neq => write!(f, "neq"),
            Self::Gt => write!(f, "gt"),
            Self::Gte => write!(f, "gte"),
            Self::Lt => write!(f, "lt"),
            Self::Lte => write!(f, "lte"),
            Self::Store => write!(f, "store"),
            Self::Inv => write!(f, "inv"),
            Self::IncrUnchecked => write!(f, "incr.unchecked"),
            Self::IncrChecked => write!(f, "incr.checked"),
            Self::Pow2 => write!(f, "pow2"),
            Self::Not => write!(f, "not"),
            Self::BNot => write!(f, "bnot"),
            Self::PopCnt => write!(f, "popcnt"),
            Self::IsOdd => write!(f, "is_odd"),
            Self::Cast => write!(f, "cast"),
            Self::PtrToInt => write!(f, "ptrtoint"),
            Self::IntToPtr => write!(f, "inttoptr"),
            Self::Neg => write!(f, "neg"),
            Self::ConstI1 => write!(f, "const.i1"),
            Self::ConstI8 => write!(f, "const.i8"),
            Self::ConstU8 => write!(f, "const.u8"),
            Self::ConstI16 => write!(f, "const.i16"),
            Self::ConstU16 => write!(f, "const.u16"),
            Self::ConstI32 => write!(f, "const.i32"),
            Self::ConstU32 => write!(f, "const.u32"),
            Self::ConstI64 => write!(f, "const.i64"),
            Self::ConstU64 => write!(f, "const.u64"),
            Self::ConstFelt => write!(f, "const.felt"),
            Self::Select => write!(f, "select"),
            Self::Assert => write!(f, "assert"),
            Self::Assertz => write!(f, "assertz"),
            Self::AssertEq => write!(f, "assert.eq"),
            Self::Alloca => write!(f, "alloca"),
            Self::Unreachable => write!(f, "unreachable"),
            Self::Global => write!(f, "global"),
            Self::GlobalSymbol => write!(f, "global.symbol"),
            Self::GlobalLoad => write!(f, "global.load"),
            Self::GlobalIAdd => write!(f, "global.iadd"),
            Self::As => write!(f, "as"),
            Self::Id => write!(f, "id"),
            Self::Symbol => write!(f, "symbol"),
            Self::IAdd => write!(f, "iadd"),
            Self::I1 => write!(f, "i1"),
            Self::I8 => write!(f, "i8"),
            Self::U8 => write!(f, "u8"),
            Self::I16 => write!(f, "i16"),
            Self::U16 => write!(f, "u16"),
            Self::I32 => write!(f, "i32"),
            Self::U32 => write!(f, "u32"),
            Self::I64 => write!(f, "i64"),
            Self::U64 => write!(f, "u64"),
            Self::I128 => write!(f, "i128"),
            Self::U128 => write!(f, "u128"),
            Self::U256 => write!(f, "u256"),
            Self::F64 => write!(f, "f64"),
            Self::Felt => write!(f, "felt"),
            Self::Mut => write!(f, "mut"),
            Self::DoubleQuote => write!(f, "\""),
            Self::Colon => write!(f, ":"),
            Self::Semicolon => write!(f, ";"),
            Self::Comma => write!(f, ","),
            Self::Dot => write!(f, "."),
            Self::LParen => write!(f, "("),
            Self::RParen => write!(f, ")"),
            Self::LBracket => write!(f, "["),
            Self::RBracket => write!(f, "]"),
            Self::LBrace => write!(f, "{{"),
            Self::RBrace => write!(f, "}}"),
            Self::Equal => write!(f, "="),
            Self::RDoubleArrow => write!(f, "=>"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::RArrow => write!(f, "->"),
            Self::Star => write!(f, "*"),
            Self::Ampersand => write!(f, "&"),
            Self::Bang => write!(f, "!"),
            Self::At => write!(f, "@"),
        }
    }
}
