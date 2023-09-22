#![no_std]

extern crate alloc;

mod layout;

pub use self::layout::{Alignable, TypeRepr};

use alloc::{boxed::Box, vec::Vec};
use core::fmt;

/// Represents the type of a value
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// This indicates a failure to type a value, or a value which is untypable
    Unknown,
    /// This type is used to indicate the absence of a value, such as a function which returns
    /// nothing
    Unit,
    /// This type is the bottom type, and represents divergence, akin to Rust's Never/! type
    Never,
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
    Isize,
    Usize,
    F64,
    /// Field element
    Felt,
    /// A pointer to a value in memory, assuming a byte-addressable address space
    ///
    /// This is the pointer type that most high-level programming languages will lower to,
    /// but may be less efficient as it requires translation to a native pointer type.
    Ptr(Box<Type>),
    /// A pointer to a value in memory, assuming a word-addressable address space
    ///
    /// This is the native pointer type on the Miden VM, and uses a larger value representation
    /// to support offsets for the `mem_load` and `mem_store` instructions.
    NativePtr(Box<Type>),
    /// A compound type of fixed shape and size
    Struct(Vec<Type>),
    /// A vector of fixed size
    Array(Box<Type>, usize),
}
impl Type {
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Self::I1
                | Self::I8
                | Self::U8
                | Self::I16
                | Self::U16
                | Self::I32
                | Self::U32
                | Self::I64
                | Self::U64
                | Self::I128
                | Self::U128
                | Self::U256
                | Self::Isize
                | Self::Usize
                | Self::F64
                | Self::Felt
        )
    }

    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            Self::I1
                | Self::I8
                | Self::U8
                | Self::I16
                | Self::U16
                | Self::I32
                | Self::U32
                | Self::I64
                | Self::U64
                | Self::I128
                | Self::U128
                | Self::U256
                | Self::Isize
                | Self::Usize
                | Self::Felt
        )
    }

    pub fn is_signed_integer(&self) -> bool {
        matches!(
            self,
            Self::I8 | Self::I16 | Self::I32 | Self::I64 | Self::I128 | Self::Isize
        )
    }

    pub fn is_unsigned_integer(&self) -> bool {
        matches!(
            self,
            Self::U8 | Self::U16 | Self::U32 | Self::U64 | Self::U128 | Self::Usize
        )
    }

    #[inline]
    pub fn is_float(&self) -> bool {
        matches!(self, Self::F64)
    }

    #[inline]
    pub fn is_felt(&self) -> bool {
        matches!(self, Self::Felt)
    }

    #[inline]
    pub fn is_pointer(&self) -> bool {
        matches!(self, Self::Ptr(_) | Self::NativePtr(_))
    }

    #[inline]
    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct(_))
    }

    #[inline]
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_, _))
    }

    /// Returns true if `self` and `other` are compatible operand types for a binary operator, e.g. `add`
    ///
    /// In short, the rules are as follows:
    ///
    /// * The operand order is assumed to be `self <op> other`, i.e. `op` is being applied
    ///   to `self` using `other`. The left-hand operand is used as the "controlling" type
    ///   for the operator, i.e. it determines what instruction will be used to perform the
    ///   operation.
    /// * The operand types must be numeric, or support being manipulated numerically
    /// * If the controlling type is unsigned, it is never compatible with signed types, because Miden
    ///   instructions for unsigned types use a simple unsigned binary encoding, thus they will not handle
    ///   signed operands using two's complement correctly.
    /// * If the controlling type is signed, it is compatible with both signed and unsigned types, as long
    ///   as the values fit in the range of the controlling type, e.g. adding a `u16` to an `i32` is fine,
    ///   but adding a `u32` to an `i32` is not.
    /// * Pointer types are permitted to be the controlling type, and since they are represented using u32,
    ///   they have the same compatibility set as u32 does. In all other cases, pointer types are treated
    ///   the same as any other non-numeric type.
    /// * Non-numeric types are always incompatible, since no operators support these types
    pub fn is_compatible_operand(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::I1, Type::I1) => true,
            (Type::I8, Type::I8) => true,
            (Type::U8, Type::U8) => true,
            (Type::I16, Type::I8 | Type::U8 | Type::I16) => true,
            (Type::U16, Type::U8 | Type::U16) => true,
            (
                Type::I32 | Type::Isize,
                Type::I8 | Type::U8 | Type::I16 | Type::U16 | Type::I32 | Type::Isize,
            ) => true,
            (Type::U32 | Type::Usize, Type::U8 | Type::U16 | Type::U32 | Type::Usize) => true,
            (
                Type::Felt,
                Type::I8
                | Type::U8
                | Type::I16
                | Type::U16
                | Type::I32
                | Type::U32
                | Type::Isize
                | Type::Usize
                | Type::Felt,
            ) => true,
            (
                Type::I64,
                Type::I8
                | Type::U8
                | Type::I16
                | Type::U16
                | Type::I32
                | Type::U32
                | Type::Isize
                | Type::Usize
                | Type::Felt
                | Type::I64,
            ) => true,
            (Type::U64, Type::U8 | Type::U16 | Type::U32 | Type::Usize | Type::U64) => true,
            (
                Type::I128,
                Type::I8
                | Type::U8
                | Type::I16
                | Type::U16
                | Type::I32
                | Type::U32
                | Type::Isize
                | Type::Usize
                | Type::Felt
                | Type::I64
                | Type::U64
                | Type::I128,
            ) => true,
            (
                Type::U128,
                Type::U8 | Type::U16 | Type::U32 | Type::Usize | Type::U64 | Type::U128,
            ) => true,
            (Type::U256, rty) => rty.is_integer(),
            (Type::F64, Type::F64) => true,
            (Type::Ptr(_) | Type::NativePtr(_), Type::U8 | Type::U16 | Type::U32 | Type::Usize) => {
                true
            }
            _ => false,
        }
    }

    #[inline]
    pub fn pointee(&self) -> Option<&Type> {
        use core::ops::Deref;
        match self {
            Self::Ptr(ty) | Self::NativePtr(ty) => Some(ty.deref()),
            _ => None,
        }
    }
}

impl fmt::Display for Type {
    /// Print this type for display using the provided module context
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use core::fmt::Write;
        match self {
            Self::Unknown => f.write_str("?"),
            Self::Unit => f.write_str("()"),
            Self::Never => f.write_char('!'),
            Self::I1 => f.write_str("i1"),
            Self::I8 => f.write_str("i8"),
            Self::U8 => f.write_str("u8"),
            Self::I16 => f.write_str("i16"),
            Self::U16 => f.write_str("u16"),
            Self::I32 => f.write_str("i32"),
            Self::U32 => f.write_str("u32"),
            Self::I64 => f.write_str("i64"),
            Self::U64 => f.write_str("u64"),
            Self::I128 => f.write_str("i128"),
            Self::U128 => f.write_str("u128"),
            Self::U256 => f.write_str("u256"),
            Self::Isize => f.write_str("isize"),
            Self::Usize => f.write_str("usize"),
            Self::F64 => f.write_str("f64"),
            Self::Felt => f.write_str("felt"),
            Self::Ptr(inner) => write!(f, "*mut {}", &inner),
            Self::NativePtr(inner) => write!(f, "&mut {}", inner),
            Self::Struct(fields) => {
                f.write_str("{")?;
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", {}", field)?;
                    } else {
                        write!(f, "{}", field)?;
                    }
                }
                f.write_str("}")
            }
            Self::Array(element_ty, arity) => write!(f, "[{}; {}]", &element_ty, arity),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct FunctionType {
    pub results: Vec<Type>,
    pub params: Vec<Type>,
}
impl FunctionType {
    pub fn new(params: Vec<Type>, results: Vec<Type>) -> Self {
        Self { results, params }
    }

    pub fn arity(&self) -> usize {
        self.params.len()
    }

    pub fn results(&self) -> &[Type] {
        self.results.as_slice()
    }

    pub fn params(&self) -> &[Type] {
        self.params.as_slice()
    }
}
impl fmt::Display for FunctionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use core::fmt::Write;

        f.write_str("fn (")?;
        for (i, ty) in self.params.iter().enumerate() {
            if i > 0 {
                write!(f, ", {}", ty)?;
            } else {
                write!(f, "{}", ty)?;
            }
        }
        f.write_str(" -> (")?;
        for (i, ty) in self.results.iter().enumerate() {
            if i > 0 {
                write!(f, ", {}", ty)?;
            } else {
                write!(f, "{}", ty)?;
            }
        }
        f.write_char(')')
    }
}
