use std::{
    fmt,
    hash::{Hash, Hasher},
};

use intrusive_collections::{intrusive_adapter, LinkedList, LinkedListLink, UnsafeRef};

intrusive_adapter!(pub DataSegmentAdapter = UnsafeRef<DataSegment>: DataSegment { link: LinkedListLink });

use super::{Alignable, ConstantData, Offset};

/// This error is raised when attempting to declare a [DataSegment]
/// that in some way conflicts with previously declared data segments.
#[derive(Debug, thiserror::Error)]
pub enum DataSegmentError {
    /// The current segment overlaps with a previously allocated segment
    #[error("invalid data segment: segment of {size1} bytes at {offset1:#x} overlaps with segment of {size2} bytes at {offset2:#x}")]
    OverlappingSegments {
        offset1: Offset,
        size1: u32,
        offset2: Offset,
        size2: u32,
    },
    /// The current segment and a previous definition of that segment do
    /// not agree on the data or read/write properties of the memory they
    /// represent.
    #[error("invalid data segment: segment at {0:#x} conflicts with a previous segment declaration at this address")]
    Mismatch(Offset),
    /// The current segment and size do not fall in the boundaries of the heap
    /// which is allocatable to globals and other heap allocations.
    ///
    /// For example, Miden reserves some amount of memory for procedure locals
    /// at a predetermined address, and we do not permit segments to be allocated
    /// past that point.
    #[error("invalid data segment: segment of {size} bytes at {offset:#x} would extend beyond the end of the usable heap")]
    OutOfBounds { offset: Offset, size: u32 },
    /// The initializer for the current segment has a size greater than `u32::MAX` bytes
    #[error("invalid data segment: segment at {0:#x} was declared with an initializer larger than 2^32 bytes")]
    InitTooLarge(Offset),
    /// The initializer for the current segment has a size greater than the declared segment size
    #[error("invalid data segment: segment of {size} bytes at {offset:#x} has an initializer of {actual} bytes")]
    InitOutOfBounds {
        offset: Offset,
        size: u32,
        actual: u32,
    },
}

/// Similar to [GlobalVariableTable], this structure is used to track data segments in a module or program.
#[derive(Default)]
pub struct DataSegmentTable {
    segments: LinkedList<DataSegmentAdapter>,
}
impl Clone for DataSegmentTable {
    fn clone(&self) -> Self {
        let mut table = Self::default();
        for segment in self.segments.iter() {
            table
                .segments
                .push_back(UnsafeRef::from_box(Box::new(segment.clone())));
        }
        table
    }
}
impl DataSegmentTable {
    /// Returns true if the table has no segments defined
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Returns the offset in linear memory where the last data segment ends
    pub fn next_available_offset(&self) -> u32 {
        if let Some(last_segment) = self.last() {
            let next_offset = last_segment.offset() + last_segment.size();
            // Ensure the start of the globals segment is word-aligned
            next_offset.align_up(32)
        } else {
            0
        }
    }

    /// Declare a new [DataSegment], with the given offset, size, and data.
    ///
    /// Returns `Err` if the declared segment overlaps/conflicts with an existing segment.
    pub fn declare(
        &mut self,
        offset: Offset,
        size: u32,
        init: ConstantData,
        readonly: bool,
    ) -> Result<(), DataSegmentError> {
        self.insert(Box::new(DataSegment::new(offset, size, init, readonly)?))
    }

    /// Insert a [DataSegment] into this table, while preserving the order of the table.
    ///
    /// This will fail if the segment is invalid, or overlaps/conflicts with an existing segment.
    pub fn insert(&mut self, segment: Box<DataSegment>) -> Result<(), DataSegmentError> {
        let mut cursor = self.segments.front_mut();
        let end = segment.offset + segment.size;
        while let Some(current_segment) = cursor.get() {
            let segment_end = current_segment.offset + current_segment.size;
            // If this segment starts after the segment we're declaring,
            // we do not need to continue searching for conflicts, and
            // can go a head and perform the insert
            if current_segment.offset >= end {
                cursor.insert_before(UnsafeRef::from_box(segment));
                return Ok(());
            }
            // If this segment starts at the same place as the one we're
            // declaring that's a guaranteed conflict
            if current_segment.offset == segment.offset {
                // If the two segments have the same size and offset, then
                // if they match in all other respects, we're done. If they
                // don't match, then we raise a mismatch error.
                if current_segment.size == segment.size
                    && current_segment.init == segment.init
                    && current_segment.readonly == segment.readonly
                {
                    return Ok(());
                }
                return Err(DataSegmentError::Mismatch(segment.offset));
            }
            // This segment starts before the segment we're declaring,
            // make sure that this segment ends before our segment starts
            if segment_end > segment.offset {
                return Err(DataSegmentError::OverlappingSegments {
                    offset1: segment.offset,
                    size1: segment.size,
                    offset2: current_segment.offset,
                    size2: current_segment.size,
                });
            }

            cursor.move_next();
        }

        self.segments.push_back(UnsafeRef::from_box(segment));

        Ok(())
    }

    /// Traverse the data segments in the table in ascending order by offset
    pub fn iter<'a, 'b: 'a>(
        &'b self,
    ) -> intrusive_collections::linked_list::Iter<'a, DataSegmentAdapter> {
        self.segments.iter()
    }

    /// Remove the first data segment from the table
    #[inline]
    pub fn pop_front(&mut self) -> Option<Box<DataSegment>> {
        self.segments
            .pop_front()
            .map(|unsafe_ref| unsafe { UnsafeRef::into_box(unsafe_ref) })
    }

    /// Return a reference to the last [DataSegment] in memory
    #[inline]
    pub fn last(&self) -> Option<&DataSegment> {
        self.segments.back().get()
    }
}
impl fmt::Debug for DataSegmentTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.segments.iter()).finish()
    }
}

/// A [DataSegment] represents a region of linear memory that should be initialized
/// with a given vector of bytes.
///
/// This is distinct from [GlobalVariableData], which can be referenced by name,
/// and participates in linkage. Furthermore, [GlobalVariableData] is only as large
/// as it's type/initializer and alignment require, they cannot be arbitrarily sized.
///
/// A data segment has an offset from the start of linear memory, i.e. address 0x0,
/// and a fixed size, which must be at least as large as the initializer data for
/// the segment. If the size is larger than the initializer data, then it is implied
/// that the remaining bytes will be zeroed.
///
/// A read-only data segment is used to determine whether a given operation is permitted
/// on addresses falling in that segment - e.g. loads are allowed, stores are not. Miden
/// currently does not have any form of memory protection, so this validation is best
/// effort.
#[derive(Clone)]
pub struct DataSegment {
    link: LinkedListLink,
    /// The offset from the start of linear memory where this segment starts
    offset: Offset,
    /// The size, in bytes, of this data segment.
    ///
    /// By default this will be the same size as `init`, unless explicitly given.
    size: u32,
    /// The data to initialize this segment with, may not be larger than `size`
    init: ConstantData,
    /// Whether or not this segment is intended to be read-only data
    readonly: bool,
}
impl fmt::Debug for DataSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DataSegment")
            .field("offset", &self.offset)
            .field("size", &self.size)
            .field("init", &format_args!("{}", &self.init))
            .field("readonly", &self.readonly)
            .finish()
    }
}
impl DataSegment {
    /// Create a new [DataSegment] with the given offset, size, initializer, and readonly flag.
    ///
    /// If the declared size and the size of the initializer differ, then the greater of the two is
    /// used. However, if the declared size is smaller than the initializer, an error is returned.
    ///
    /// If the offset and/or size are invalid, an error is returned.
    pub(crate) fn new(
        offset: Offset,
        size: u32,
        init: ConstantData,
        readonly: bool,
    ) -> Result<Self, DataSegmentError> {
        // Require the initializer data to be no larger than 2^32 bytes
        let init_size = init
            .len()
            .try_into()
            .map_err(|_| DataSegmentError::InitTooLarge(offset))?;

        // Require the initializer to fit within the declared bounds
        if size < init_size {
            return Err(DataSegmentError::InitOutOfBounds {
                offset,
                size,
                actual: init_size,
            });
        }

        // Require the entire segment to fit within the linear memory address space
        let size = core::cmp::max(size, init_size);
        offset
            .checked_add(size)
            .ok_or(DataSegmentError::OutOfBounds { offset, size })?;

        Ok(Self {
            link: Default::default(),
            offset,
            size,
            init,
            readonly,
        })
    }

    /// Get the offset from the base of linear memory where this segment starts
    pub const fn offset(&self) -> Offset {
        self.offset
    }

    /// Get the size, in bytes, of this segment
    pub const fn size(&self) -> u32 {
        self.size
    }

    /// Get a reference to this segment's initializer data
    pub const fn init(&self) -> &ConstantData {
        &self.init
    }

    /// Returns true if this segment is intended to be read-only
    pub const fn is_readonly(&self) -> bool {
        self.readonly
    }
}
impl Eq for DataSegment {}
impl PartialEq for DataSegment {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset
            && self.size == other.size
            && self.init == other.init
            && self.readonly == other.readonly
    }
}
impl Hash for DataSegment {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.offset.hash(state);
        self.size.hash(state);
        self.init.hash(state);
        self.readonly.hash(state);
    }
}
