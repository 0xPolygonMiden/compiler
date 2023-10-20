use super::*;

/// The types of values which can be represented in an AirScript program
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// The singleton type
    Unit,
    /// The empty type
    Never,
    /// The type of a single bit, i.e., the boolean type
    I1,
    /// Signed 8-bit integers
    I8,
    /// Unsigned 8-bit integers
    U8,
    /// Signed 16-bit integers
    I16,
    /// Unsigned 16-bit integers
    U16,
    /// Signed 32-bit integers
    I32,
    /// Unsigned 32-bit integers
    U32,
    /// Signed 64-bit integers
    I64,
    /// Unsigned 64-bit integers
    U64,
    /// Signed 128-bit integers
    I128,
    /// Unsigned 128-bit integers
    U128,
    /// Unsigned 256-bit integers
    U256,
    /// Signed integers of size equal to the native architecture word size
    ISize,
    /// Unsigned integers of size equal to the native architecture word size
    USize,
    /// 64-bit floats
    F64,
    /// Field elements
    Felt,
    /// Pointers to values of the inner type
    Ptr(Box<Type>),
    /// Native pointers to values of the inner type
    NativePtr(Box<Type>),
    /// Structs containing field values of the specified types in the specified order.
    /// The empty struct is a legal type.
    Struct(Vec<Type>),
    /// Arrays of the specified length, containing values of the specified type.
    Array(Box<Type>, u128),
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unit => f.write_str("()"),
            Self::Never => f.write_str("!"),
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
            Self::ISize => f.write_str("isize"),
            Self::USize => f.write_str("usize"),
            Self::F64 => f.write_str("f64"),
            Self::Felt => f.write_str("felt"),
            Self::Ptr(inner) => write!(f, "*mut {}", inner),
            Self::NativePtr(inner) => write!(f, "&mut {}", inner),
            Self::Struct(types) => {
                f.write_str("{ ")?;
                for (i, t) in types.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", {}", t)?;
                    } else {
                        write!(f, "{}", t)?;
                    }
                    f.write_str(" }")?;
                }
                Ok(())
            }
            Self::Array(inner, length) => write!(f, "[ {} ; {} ]", inner, length),
        }
    }
}
