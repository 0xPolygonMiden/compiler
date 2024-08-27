use std::{collections::BTreeMap, fmt, str::FromStr, sync::Arc};

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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConstantData(#[cfg_attr(feature = "serde", serde(with = "serde_bytes"))] Vec<u8>);
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

    /// Attempt to convert this constant data to a `u32` value
    pub fn as_u32(&self) -> Option<u32> {
        let bytes = self.as_slice();
        if bytes.len() != 4 {
            return None;
        }
        let bytes = bytes.as_ptr() as *const [u8; 4];
        Some(u32::from_le_bytes(unsafe { bytes.read() }))
    }
}
impl From<ConstantData> for Vec<u8> {
    #[inline(always)]
    fn from(data: ConstantData) -> Self {
        data.0
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
        fmt::LowerHex::fmt(self, f)
    }
}
impl fmt::LowerHex for ConstantData {
    /// Print the constant data in hexadecimal format, e.g. 0x000102030405060708090a0b0c0d0e0f.
    ///
    /// The printed form of the constant renders the bytes in the same order as the data.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.is_empty() {
            if f.alternate() {
                f.write_str("0x")?;
            }
            for byte in self.0.iter().rev() {
                write!(f, "{byte:02x}")?;
            }
        }
        Ok(())
    }
}
impl FromStr for ConstantData {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_be(s).map_err(|_| ())
    }
}
impl ConstantData {
    pub fn from_str_be(s: &str) -> Result<Self, &'static str> {
        const NOT_EVEN: &str = "invalid hex-encoded data: expected an even number of hex digits";
        const NOT_HEX: &str = "invalid hex-encoded data: contains invalid hex digits";

        let s = s.strip_prefix("0x").unwrap_or(s);
        let len = s.len();
        if len % 2 != 0 {
            return Err(NOT_EVEN);
        }
        // Parse big-endian
        let pairs = len / 2;
        let mut data = Vec::with_capacity(pairs);
        let mut chars = s.chars();
        while let Some(a) = chars.next() {
            let a = a.to_digit(16).ok_or(NOT_HEX)?;
            let b = chars.next().unwrap().to_digit(16).ok_or(NOT_HEX)?;
            data.push(((a << 4) + b) as u8);
        }

        Ok(Self(data))
    }

    pub fn from_str_le(s: &str) -> Result<Self, &'static str> {
        let mut data = Self::from_str_be(s)?;
        // Make little-endian
        data.0.reverse();
        Ok(data)
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
    constants: BTreeMap<Constant, Arc<ConstantData>>,

    /// Mapping of hashed `ConstantData` to the index into the other hashmap.
    ///
    /// This allows for deduplication of entries into the `handles_to_values` mapping.
    cache: BTreeMap<Arc<ConstantData>, Constant>,
}
impl ConstantPool {
    /// Returns true if the pool is empty
    pub fn is_empty(&self) -> bool {
        self.constants.is_empty()
    }

    /// Returns the number of constants in this pool
    pub fn len(&self) -> usize {
        self.constants.len()
    }

    /// Retrieve the constant data as a reference-counted pointer, given a handle.
    pub fn get(&self, id: Constant) -> Arc<ConstantData> {
        Arc::clone(&self.constants[&id])
    }

    /// Retrieve the constant data by reference given a handle.
    pub fn get_by_ref(&self, id: Constant) -> &ConstantData {
        self.constants[&id].as_ref()
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

        let data = Arc::new(data);
        let id = Constant::new(self.len());
        self.constants.insert(id, Arc::clone(&data));
        self.cache.insert(data, id);
        id
    }

    /// Same as [ConstantPool::insert], but for data already allocated in an [Arc].
    pub fn insert_arc(&mut self, data: Arc<ConstantData>) -> Constant {
        if let Some(cst) = self.cache.get(data.as_ref()) {
            return *cst;
        }

        let id = Constant::new(self.len());
        self.constants.insert(id, Arc::clone(&data));
        self.cache.insert(data, id);
        id
    }

    /// Traverse the contents of the pool
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (Constant, Arc<ConstantData>)> + '_ {
        self.constants.iter().map(|(k, v)| (*k, Arc::clone(v)))
    }
}
