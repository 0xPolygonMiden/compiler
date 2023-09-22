use std::{collections::BTreeMap, fmt};

use cranelift_entity::{entity_impl, EntityRef};

pub trait IntoBytes {
    fn into_bytes(self) -> Vec<u8>;
}
impl IntoBytes for Vec<u8> {
    #[inline(always)]
    fn into_bytes(self) -> Vec<u8> {
        self
    }
}
impl IntoBytes for i8 {
    #[inline]
    fn into_bytes(self) -> Vec<u8> {
        vec![self as u8]
    }
}
impl IntoBytes for i16 {
    #[inline]
    fn into_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

/// A handle to a constant
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Constant(u32);
entity_impl!(Constant, "const");

/// This type represents the raw data of a constant.
///
/// The data is expected to be in little-endian order.
#[derive(Debug, Clone, PartialEq, Eq, Default, PartialOrd, Ord, Hash)]
pub struct ConstantData(Vec<u8>);
impl ConstantData {
    /// Return the number of bytes in the constant.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if the constant contains any bytes.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return the data as a slice.
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    /// Append bytes to this constant
    pub fn append(mut self, bytes: impl IntoBytes) -> Self {
        let mut bytes = bytes.into_bytes();
        self.0.append(&mut bytes);
        self
    }

    /// Grow the size of the constant data in bytes to `expected_size`, zero-extending
    /// the data by writing zeroes to the newly-added high-order bytes.
    pub fn zext(mut self, expected_size: usize) -> Self {
        assert!(
            self.len() <= expected_size,
            "the constant is already larger than {} bytes",
            expected_size
        );
        self.0.resize(expected_size, 0);
        self
    }
}
impl FromIterator<u8> for ConstantData {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
impl From<Vec<u8>> for ConstantData {
    fn from(v: Vec<u8>) -> Self {
        Self(v)
    }
}
impl<const N: usize> From<[u8; N]> for ConstantData {
    fn from(v: [u8; N]) -> Self {
        Self(v.to_vec())
    }
}
impl From<&[u8]> for ConstantData {
    fn from(v: &[u8]) -> Self {
        Self(v.to_vec())
    }
}
impl fmt::Display for ConstantData {
    /// Print the constant data in hexadecimal format, e.g. 0x000102030405060708090a0b0c0d0e0f.
    ///
    /// The printed form of the constant renders the bytes in big-endian order, for readability.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.is_empty() {
            write!(f, "0x")?;
            for b in self.0.iter().rev() {
                write!(f, "{:02x}", b)?;
            }
        }
        Ok(())
    }
}

/// This maintains the storage for constants used within a function
#[derive(Default)]
pub struct ConstantPool {
    /// This mapping maintains the insertion order as long as Constants are created with
    /// sequentially increasing integers.
    ///
    /// It is important that, by construction, no entry in that list gets removed. If that ever
    /// need to happen, don't forget to update the `Constant` generation scheme.
    constants: BTreeMap<Constant, ConstantData>,

    /// Mapping of hashed `ConstantData` to the index into the other hashmap.
    ///
    /// This allows for deduplication of entries into the `handles_to_values` mapping.
    cache: BTreeMap<ConstantData, Constant>,
}
impl ConstantPool {
    /// Returns the number of constants in this pool
    pub fn len(&self) -> usize {
        self.constants.len()
    }

    /// Retrieve the constant data given a handle.
    pub fn get(&self, id: Constant) -> &ConstantData {
        &self.constants[&id]
    }

    /// Returns true if this pool contains the given constant data
    pub fn contains(&self, data: &ConstantData) -> bool {
        self.cache.contains_key(data)
    }

    /// Insert constant data into the pool, returning a handle for later referencing; when constant
    /// data is inserted that is a duplicate of previous constant data, the existing handle will be
    /// returned.
    pub fn insert(&mut self, data: ConstantData) -> Constant {
        if let Some(cst) = self.cache.get(&data) {
            return *cst;
        }

        let id = Constant::new(self.len());
        self.constants.insert(id, data.clone());
        self.cache.insert(data, id);
        id
    }
}
