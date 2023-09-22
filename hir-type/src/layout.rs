use alloc::{alloc::Layout, vec::Vec};
use core::{fmt, num::NonZeroU8};

use super::Type;

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

impl Type {
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

    /// Returns the minimum alignment, in bytes, of this type
    pub fn min_alignment(&self) -> usize {
        match self {
            // These types don't have a meaningful alignment, so choose byte-aligned
            Self::Unknown | Self::Unit | Self::Never => 1,
            // Felts must be naturally aligned
            Self::Felt => 8,
            // 256-bit and 128-bit integers must be word-aligned
            Self::U256 | Self::I128 | Self::U128 => 32,
            // 64-bit integers and floats must be element-aligned
            Self::I64 | Self::U64 | Self::F64 => 8,
            // 32-bit integers and pointers must be element-aligned
            Self::I32
            | Self::U32
            | Self::Isize
            | Self::Usize
            | Self::Ptr(_)
            | Self::NativePtr(_) => 8,
            // 16-bit integers can be naturally aligned
            Self::I16 | Self::U16 => 4,
            // 8-bit integers and booleans can be naturally aligned
            Self::I8 | Self::U8 | Self::I1 => 1,
            // Structs use the minimum alignment of their first field, or 1 if a zero-sized type
            Self::Struct(ref fields) => fields.first().map(|f| f.min_alignment()).unwrap_or(1),
            // Arrays use the minimum alignment of their element type
            Self::Array(ref element_ty, _) => element_ty.min_alignment(),
        }
    }

    /// Returns the size in bytes of this type, without alignment padding.
    pub fn size_in_bytes(&self) -> usize {
        match self {
            // These types have no representation in memory
            Self::Unknown | Self::Unit | Self::Never => 0,
            // Booleans consume a full byte, and 8-bit integers are naturally sized
            Self::I1 | Self::I8 | Self::U8 => 1,
            // 16-bit integers require 2 bytes
            Self::I16 | Self::U16 => 2,
            // 32-bit integers require 4 bytes
            Self::I32 | Self::U32 | Self::Isize | Self::Usize => 4,
            // Pointers, which are 32-bits, require 4 bytes
            Self::Ptr(_) | Self::NativePtr(_) => 4,
            // 64-bit integers/floats (and field elements, which are 64 bits) require 8 bytes
            Self::I64 | Self::U64 | Self::F64 | Self::Felt => 8,
            // 128-bit integers require 16 bytes
            Self::I128 | Self::U128 => 16,
            // 256-bit integers require 32 bytes
            Self::U256 => 32,
            // Zero-sized types have no size
            Self::Struct(ref fields) if fields.is_empty() => 0,
            Self::Struct(ref fields) => {
                let mut size = 0;
                for (i, ty) in fields.iter().enumerate() {
                    // Add alignment padding for all but the first field
                    if i > 0 {
                        let min_align = ty.min_alignment();
                        size += size.align_offset(min_align);
                    }
                    size += ty.size_in_bytes();
                }
                size
            }
            // Empty arrays have no size
            Self::Array(_, 0) => 0,
            // Single-element arrays are equivalent in size to their element type
            Self::Array(ref element_ty, 1) => element_ty.size_in_bytes(),
            // All other arrays are the size of their element type multiplied by the
            // size of the array, with sufficient padding to all but the first element
            // to ensure that each element in the array meets its required alignment
            Self::Array(ref element_ty, n) => {
                let min_align = element_ty.min_alignment();
                let element_size = element_ty.size_in_bytes();
                let padded_element_size = element_size.align_up(min_align);
                element_size + (padded_element_size * (n - 1))
            }
        }
    }

    /// Returns the size in bytes of this type, with padding to account for alignment
    pub fn aligned_size_in_bytes(&self) -> usize {
        let align = self.min_alignment();
        let size = self.size_in_bytes();

        // Assuming that a pointer is allocated with the worst possible alignment,
        // i.e. it is not aligned on a power-of-two boundary, we can ensure that there
        // is enough space to align the pointer to the required minimum alignment and
        // still fit it in the allocated block of memory without overflowing its bounds,
        // by adding `align` to size.
        //
        // We panic if padding the size overflows `usize`.
        //
        // So let's say we have a type with a min alignment of 16, and size of 24. If
        // we add 16 to 24, we get 40. We then allocate a block of memory of 40 bytes,
        // the pointer of which happens to be at address 0x01. If we align that pointer
        // to 0x10 (the next closest aligned address within the block we allocated),
        // that consumes 15 bytes of the 40 we have, leaving us with 25 bytes to hold
        // our 24 byte value.
        size.checked_add(align)
            .expect("type cannot meet its minimum alignment requirement due to its size")
    }

    /// Returns the size in field elements of this type
    pub fn size_in_felts(&self) -> usize {
        let bytes = self.size_in_bytes();
        let trailing = bytes % FELT_SIZE;
        (bytes / FELT_SIZE) + ((trailing > 0) as usize)
    }

    /// Returns the size in words of this type
    pub fn size_in_words(&self) -> usize {
        let bytes = self.size_in_bytes();
        let trailing = bytes % WORD_SIZE;
        (bytes / WORD_SIZE) + ((trailing > 0) as usize)
    }

    /// Returns the layout of this type in memory
    pub fn layout(&self) -> Layout {
        Layout::from_size_align(self.size_in_bytes(), self.min_alignment())
            .expect("invalid layout: the size, when padded for alignment, overflows isize")
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
}

/// This trait represents an alignable primitive integer value representing an address
pub trait Alignable {
    /// This function computes the offset, in bytes, needed to align `self` upwards so that
    /// it is aligned to `align` bytes.
    ///
    /// The following must be true, or this function will panic:
    ///
    /// * `align` is non-zero
    /// * `align` is a power of two
    fn align_offset(self, align: Self) -> Self;
    /// This function aligns `self` to the specified alignment (in bytes), aligning upwards.
    ///
    /// The following must be true, or this function will panic:
    ///
    /// * `align` is non-zero
    /// * `align` is a power of two
    /// * `self` + `align` must be less than `Self::MAX`
    fn align_up(self, align: Self) -> Self;

    /// Compute the smallest value greater than or equal to `self` that is a multiple of `m`
    ///
    /// The following must be true, or this function will panic:
    ///
    /// * `m` must be non-zero
    /// * `self` + `m` must be less than `Self::MAX`
    ///
    /// TODO: Replace this with the standard library `next_multiple_of` when 1.73 drops.
    fn next_multiple_of(self, m: Self) -> Self;
}

macro_rules! alignable {
    ($($ty:ty),+) => {
        $(
            alignable_impl!($ty);
        )*
    };
}

macro_rules! alignable_impl {
    ($ty:ty) => {
        #[allow(unstable_name_collisions)]
        impl Alignable for $ty {
            #[inline]
            fn align_offset(self, align: Self) -> Self {
                assert!(align.is_power_of_two());
                self.next_multiple_of(align) - self
            }

            #[inline]
            fn align_up(self, align: Self) -> Self {
                assert!(self.saturating_add(align) < Self::MAX);
                self.next_multiple_of(align)
            }

            #[inline]
            fn next_multiple_of(self, m: Self) -> Self {
                // The offset in from the last multiple of `m` to reach `n`
                let offset = self % m;
                // If `n` is a multiple of `m`, this is 0, else 1
                let is_not_multiple = (offset > 0) as $ty;
                // Apply offset to `n` to reach the next nearest multiple  of `m`
                //
                // If `n` is already a multiple of `m`, the offset is 0
                self + ((m - offset) * is_not_multiple)
            }
        }
    };
}

alignable!(u8, u16, u32, u64, usize);
