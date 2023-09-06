#![no_std]

extern crate alloc;

use alloc::{alloc::Layout, boxed::Box, vec::Vec};
use core::fmt;

const WORD_SIZE: usize = core::mem::size_of::<[u64; 4]>();

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
        match self {
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
            | Self::Felt => true,
            _ => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
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
            | Self::Felt => true,
            _ => false,
        }
    }

    pub fn is_signed_integer(&self) -> bool {
        match self {
            Self::I1 | Self::I8 | Self::I16 | Self::I32 | Self::I64 | Self::I128 | Self::Isize => {
                true
            }
            _ => false,
        }
    }

    #[inline]
    pub fn is_float(&self) -> bool {
        match self {
            Self::F64 => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_felt(&self) -> bool {
        match self {
            Self::Felt => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_pointer(&self) -> bool {
        match self {
            Self::Ptr(_) | Self::NativePtr(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_struct(&self) -> bool {
        match self {
            Self::Struct(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_array(&self) -> bool {
        match self {
            Self::Array(_, _) => true,
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

    /// Returns true if this type can be loaded on to the operand stack
    ///
    /// The rule for "loadability" is a bit arbitrary, but the purpose is to
    /// force users of the IR to either pass large values by reference, or calculate
    /// the addresses of the individual fields needed from a large structure or array,
    /// and issue loads/stores against those instead.
    ///
    /// In effect, we reject loads of values that are larger than a single word, as that
    /// is the largest value which can be worked with on the operand stack of the Miden VM.
    pub fn is_loadable(&self) -> bool {
        self.size_in_words() <= WORD_SIZE
    }

    /// Returns the size in bits of this type
    ///
    /// It is intended for this to be used with integral types
    pub fn bitwidth(&self) -> usize {
        match self {
            Self::I1 => 1,
            _ => self.size_in_bytes() * 8,
        }
    }

    /// Returns the size in bytes of this type, including necessary alignment padding
    pub fn size_in_bytes(&self) -> usize {
        self.layout().pad_to_align().size()
    }

    /// Returns the size in words of this type, including necessary alignment padding
    pub fn size_in_words(&self) -> usize {
        let bytes = self.size_in_bytes();
        let trailing = bytes % WORD_SIZE;
        (bytes / WORD_SIZE) + ((trailing > 0) as usize)
    }

    /// Returns the layout of this type in memory
    pub fn layout(&self) -> Layout {
        match self {
            Self::Unknown | Self::Unit | Self::Never => Layout::new::<()>(),
            Self::I1 | Self::I8 | Self::U8 => Layout::new::<i8>(),
            Self::I16 | Self::U16 => Layout::new::<i16>(),
            Self::I32 | Self::U32 | Self::Isize | Self::Usize | Self::Ptr(_) => {
                Layout::new::<i32>()
            }
            Self::I64 | Self::U64 | Self::F64 | Self::Felt | Self::NativePtr(_) => {
                Layout::new::<i64>()
            }
            Self::I128 | Self::U128 => Layout::new::<i128>(),
            Self::U256 => Layout::new::<[u64; 4]>(),
            Self::Struct(ref tys) => {
                if let Some(ty) = tys.first() {
                    let mut layout = ty.layout();
                    for ty in tys.iter().skip(1) {
                        let (new_layout, _field_offset) = layout
                            .extend(ty.layout())
                            .expect("invalid type: layout too large");
                        layout = new_layout;
                    }
                    layout.pad_to_align()
                } else {
                    Layout::new::<()>()
                }
            }
            Self::Array(ty, len) => {
                let layout = ty.layout().pad_to_align();
                let size = layout.size();
                let align = layout.align();
                Layout::from_size_align(size * len, align).expect("invalid type: layout too large")
            }
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
