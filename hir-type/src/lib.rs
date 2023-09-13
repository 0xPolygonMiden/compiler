#![no_std]

extern crate alloc;

use alloc::{alloc::Layout, boxed::Box, vec::Vec};
use core::{fmt, num::NonZeroU8};

const FELT_SIZE: usize = core::mem::size_of::<u64>();
const WORD_SIZE: usize = core::mem::size_of::<[u64; 4]>();

/// This enum represents a [Type] decorated with the way in which it should be represented
/// on the operand stack of the Miden VM.
///
/// We don't use this representation when laying out types in memory however, since we must
/// preserve the semantics of a byte-addressable address space for consumers of the IR.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeRepr {
    /// This value is a zero-sized type, and is not actually reified on the operand stack
    Zst(Type),
    /// This value is a sized type that fits in a single element on the operand stack
    Default(Type),
    /// This value is a sized type that is split across multiple elements on the operand stack,
    /// for example, the representation of u64 in Miden uses two field elements, rather than
    /// packing it in a single element.
    ///
    /// We call these "sparse" because the value is split along arbitrary lines, rather than
    /// due to binary representation requirements.
    Sparse(Type, NonZeroU8),
    /// This value is a sized type which is encoded into one or more field elements as follows:
    ///
    /// * Each element is logically split into multiple u32 values
    /// * The type is encoded into its binary representation, and spread across as many u32
    /// values as necessary to hold it
    /// * The number of u32 values is then rounded up to the nearest multiple of two
    /// * The u32 values are packed into field elements using the inverse of `u32.split`
    /// * The packed field elements are pushed on the stack such that the lowest bits of
    /// the binary representation are nearest to the top of the stack.
    Packed(Type),
}
impl TypeRepr {
    /// Returns the size in field elements of this type, in the given representation
    pub fn size(&self) -> usize {
        match self {
            Self::Zst(_) => 0,
            Self::Default(_) => 1,
            Self::Sparse(_, n) => n.get() as usize,
            Self::Packed(ref ty) => ty.size_in_felts(),
        }
    }

    /// Returns true if this type is a zero-sized type
    pub fn is_zst(&self) -> bool {
        matches!(self, Self::Zst(_))
    }

    /// Returns true if this type is sparsely encoded
    pub fn is_sparse(&self) -> bool {
        matches!(self, Self::Sparse(_, _))
    }

    /// Returns true if this type is densely encoded (packed)
    pub fn is_packed(&self) -> bool {
        matches!(self, Self::Packed(_))
    }

    /// Returns a reference to the underlying [Type]
    pub fn ty(&self) -> &Type {
        match self {
            Self::Zst(ref ty)
            | Self::Default(ref ty)
            | Self::Sparse(ref ty, _)
            | Self::Packed(ref ty) => ty,
        }
    }
}
impl fmt::Display for TypeRepr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.ty(), f)
    }
}

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

    /// Returns the [TypeRepr] corresponding to this type.
    ///
    /// If the type is unknown, returns `None`.
    pub fn repr(&self) -> Option<TypeRepr> {
        match self {
            Type::Unknown | Type::Never => None,
            // The unit type is a zero-sized type
            Type::Unit => Some(TypeRepr::Zst(Type::Unit)),
            // All numeric types < 64 bits in size use the default representation
            ty @ (Type::I1
            | Type::I8
            | Type::U8
            | Type::I16
            | Type::U16
            | Type::I32
            | Type::U32
            | Type::Isize
            | Type::Usize
            | Type::F64
            | Type::Felt) => Some(TypeRepr::Default(ty.clone())),
            // 64-bit integers are represented sparsely, as two field elements
            ty @ (Type::I64 | Type::U64) => Some(TypeRepr::Sparse(ty.clone(), unsafe {
                NonZeroU8::new_unchecked(2)
            })),
            // 128-bit integers are represented sparsely, as three field elements
            ty @ (Type::I128 | Type::U128) => Some(TypeRepr::Sparse(ty.clone(), unsafe {
                NonZeroU8::new_unchecked(3)
            })),
            // 256-bit integers are represented sparsely, as five field elements
            ty @ Type::U256 => Some(TypeRepr::Sparse(ty.clone(), unsafe {
                NonZeroU8::new_unchecked(5)
            })),
            // All pointer types use a single field element
            ty @ (Type::Ptr(_) | Type::NativePtr(_)) => Some(TypeRepr::Default(ty.clone())),
            // Empty structs are zero-sized by definition
            Type::Struct(ref fields) if fields.is_empty() => {
                Some(TypeRepr::Zst(Type::Struct(Vec::new())))
            }
            // Structs are "packed" across one or more field elements
            Type::Struct(ref fields) => {
                match fields.as_slice() {
                    // Single-field structs have transparent representation
                    [field_ty] => field_ty.repr(),
                    fields => Some(TypeRepr::Packed(Type::Struct(fields.to_vec()))),
                }
            }
            // Zero-sized arrays are treated as zero-sized types
            ty @ Type::Array(_, 0) => Some(TypeRepr::Zst(ty.clone())),
            // Single-element arrays have transparent representation
            Type::Array(ref element_ty, 1) => element_ty.repr(),
            // N-ary arrays are "packed" across one or more field elements
            ty @ Type::Array(_, _) => Some(TypeRepr::Packed(ty.clone())),
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

    /// Returns the size in field elements of this type, including necessary alignment padding
    pub fn size_in_felts(&self) -> usize {
        let bytes = self.size_in_bytes();
        let trailing = bytes % FELT_SIZE;
        (bytes / FELT_SIZE) + ((trailing > 0) as usize)
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
