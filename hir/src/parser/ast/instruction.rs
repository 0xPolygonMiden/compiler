use core::fmt;

use miden_diagnostics::Span;

use crate::{FunctionIdent, Ident, Opcode, Overflow, Type};

use super::*;

/// Represents a single instruction.
#[derive(Spanned)]
pub struct Inst {
    #[span]
    pub span: SourceSpan,
    /// The specific type of instruction and its data
    pub ty: InstType,
    /// If the instruction produces outputs, this will contain them, otherwise it is empty
    pub outputs: Vec<TypedValue>,
}
impl Inst {
    pub fn new(span: SourceSpan, ty: InstType, outputs: Vec<TypedValue>) -> Self {
        Self { span, ty, outputs }
    }
}
impl fmt::Debug for Inst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Inst")
            .field("ty", &self.ty)
            .field("outputs", &self.outputs)
            .finish()
    }
}
impl PartialEq for Inst {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.outputs == other.outputs
    }
}

/// This represents the various types of instructions which we can parse
#[derive(Debug, PartialEq, Eq)]
pub enum InstType {
    BinaryOp {
        opcode: Opcode,
        overflow: Option<Overflow>,
        operands: [Operand; 2],
    },
    UnaryOp {
        opcode: Opcode,
        overflow: Option<Overflow>,
        operand: Operand,
    },
    Br {
        opcode: Opcode,
        successor: Successor,
    },
    CondBr {
        opcode: Opcode,
        cond: Span<crate::Value>,
        then_dest: Successor,
        else_dest: Successor,
    },
    Switch {
        opcode: Opcode,
        input: Span<crate::Value>,
        successors: Vec<Span<(u32, Successor)>>,
        fallback: Successor,
    },
    Ret {
        opcode: Opcode,
        operands: Vec<Operand>,
    },
    Call {
        opcode: Opcode,
        callee: FunctionIdent,
        operands: Vec<Span<crate::Value>>,
    },
    CallIndirect {
        opcode: Opcode,
        operands: Vec<Operand>,
    },
    PrimOp {
        opcode: Opcode,
        operands: Vec<Operand>,
    },
    GlobalValue {
        opcode: Opcode,
        expr: GlobalValueExpr,
    },
}

/// An operand is an argument to an instruction
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operand {
    Value(Span<crate::Value>),
    /// A small integer type, e.g. u32
    Int(Span<isize>),
    /// A large integer type, e.g. i128 or u256
    BigInt(Span<num_bigint::BigInt>),
}
impl Operand {
    pub fn span(&self) -> SourceSpan {
        match self {
            Self::Value(ref spanned) => spanned.span(),
            Self::Int(ref spanned) => spanned.span(),
            Self::BigInt(ref spanned) => spanned.span(),
        }
    }

    pub fn as_value(&self) -> Option<Span<crate::Value>> {
        match self {
            Self::Value(v) => Some(*v),
            _ => None,
        }
    }
}

/// Represents a value/type pair where applicable in the AST
#[derive(Spanned)]
pub struct TypedValue {
    #[span]
    pub span: SourceSpan,
    pub id: crate::Value,
    pub ty: Type,
}
impl TypedValue {
    pub fn new(span: SourceSpan, id: crate::Value, ty: Type) -> Self {
        Self { span, id, ty }
    }
}
impl fmt::Display for TypedValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
impl fmt::Debug for TypedValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TypedValue")
            .field("id", &self.id)
            .field("ty", &self.ty)
            .finish()
    }
}
impl Eq for TypedValue {}
impl PartialEq for TypedValue {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.ty == other.ty
    }
}

/// This represents a branch destination and arguments
#[derive(Spanned)]
pub struct Successor {
    #[span]
    pub span: SourceSpan,
    pub id: crate::Block,
    pub args: Vec<Span<crate::Value>>,
}
impl fmt::Debug for Successor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Successor")
            .field("id", &format_args!("{}", &self.id))
            .field(
                "args",
                &format_args!("{}", crate::display::DisplayValues::new(self.args.iter())),
            )
            .finish()
    }
}
impl Eq for Successor {}
impl PartialEq for Successor {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.args == other.args
    }
}

/// This represents access or a relative operation to a global variable
#[derive(Debug, PartialEq, Eq)]
pub enum GlobalValueExpr {
    Symbol {
        symbol: Ident,
        offset: i32,
        span: SourceSpan,
    },
    Load {
        base: Box<GlobalValueExpr>,
        offset: i32,
        ty: Option<Type>,
        span: SourceSpan,
    },
    IAddImm {
        base: Box<GlobalValueExpr>,
        offset: i32,
        ty: Type,
        span: SourceSpan,
    },
}
impl GlobalValueExpr {
    pub fn ty(&self) -> Option<Type> {
        match self {
            Self::Symbol { .. } => None,
            Self::Load { ref ty, .. } => ty.clone(),
            Self::IAddImm { ref base, .. } => base.ty(),
        }
    }

    pub fn span(&self) -> SourceSpan {
        match self {
            Self::Symbol { span, .. } | Self::Load { span, .. } | Self::IAddImm { span, .. } => {
                *span
            }
        }
    }
}
