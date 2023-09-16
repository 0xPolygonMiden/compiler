use std::hash::{Hash, Hasher};

use intrusive_collections::{intrusive_adapter, LinkedList, LinkedListLink, UnsafeRef};

intrusive_adapter!(pub DataSegmentAdapter = UnsafeRef<DataSegment>: DataSegment { link: LinkedListLink });

use super::{Constant, Offset};

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
}

/// Similar to [GlobalVariableTable], this structure is used to track data segments in a module or program.
#[derive(Default)]
pub struct DataSegmentTable {
    segments: LinkedList<DataSegmentAdapter>,
}
impl DataSegmentTable {
    /// Returns true if the table has no segments defined
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Try to insert a new [DataSegment] in the table, with the given offset, size, and data.
    ///
    /// Returns `Err` if the proposed segment overlaps with an existing segment.
    ///
    /// Data segments are ordered by the address at which they are allocated.
    pub fn insert(
        &mut self,
        offset: Offset,
        size: u32,
        init: Constant,
        readonly: bool,
    ) -> Result<(), DataSegmentError> {
        // Make sure this segment does not overlap with another segment
        let end = offset
            .checked_add(size)
            .ok_or_else(|| DataSegmentError::OutOfBounds { offset, size })?;
        let mut cursor = self.segments.front_mut();
        while let Some(segment) = cursor.get() {
            let segment_end = segment.offset + segment.size;
            // If this segment starts after the segment we're declaring,
            // we do not need to continue searching for conflicts, and
            // can go a head and perform the insert
            if segment.offset >= end {
                let segment = Box::new(DataSegment::new(offset, size, init, readonly));
                cursor.insert_before(UnsafeRef::from_box(segment));
                return Ok(());
            }
            // If this segment starts at the same place as the one we're
            // declaring that's a guaranteed conflict
            if segment.offset == offset {
                // If the two segments have the same size and offset, then
                // if they match in all other respects, we're done. If they
                // don't match, then we raise a mismatch error.
                if segment.size == size {
                    if segment.init == init && segment.readonly == readonly {
                        return Ok(());
                    }
                }
                return Err(DataSegmentError::Mismatch(offset));
            }
            // This segment starts before the segment we're declaring,
            // make sure that this segment ends before our segment starts
            if segment_end > offset {
                return Err(DataSegmentError::OverlappingSegments {
                    offset1: offset,
                    size1: size,
                    offset2: segment.offset,
                    size2: segment.size,
                });
            }
        }

        // If we reach here, we didn't find any conflicts, and all segments
        // that were previously declared occur before the offset at which this
        // segment is allocated
        let segment = Box::new(DataSegment::new(offset, size, init, readonly));
        self.segments.push_back(UnsafeRef::from_box(segment));
        Ok(())
    }

    /// Traverse the data segments in the table in ascending order by offset
    pub fn iter<'a, 'b: 'a>(
        &'b self,
    ) -> intrusive_collections::linked_list::Iter<'a, DataSegmentAdapter> {
        self.segments.iter()
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
#[derive(Debug, Clone)]
pub struct DataSegment {
    link: LinkedListLink,
    /// The offset from the start of linear memory where this segment starts
    pub offset: Offset,
    /// The size, in bytes, of this data segment.
    ///
    /// By default this will be the same size as `init`, unless explicitly given.
    pub size: u32,
    /// The data to initialize this segment with, may not be larger than `size`
    pub init: Constant,
    /// Whether or not this segment is intended to be read-only data
    pub readonly: bool,
}
impl DataSegment {
    pub fn new(offset: Offset, size: u32, init: Constant, readonly: bool) -> Self {
        Self {
            link: Default::default(),
            offset,
            size,
            init,
            readonly,
        }
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
