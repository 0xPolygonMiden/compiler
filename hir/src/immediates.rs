use std::{
    fmt,
    hash::{Hash, Hasher},
};

use super::Type;

#[derive(Debug, Copy, Clone)]
pub enum Immediate {
    I1(bool),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    I128(i128),
    F64(f64),
    Felt(u64),
}
impl Immediate {
    pub fn ty(&self) -> Type {
        match self {
            Self::I1(_) => Type::I1,
            Self::U8(_) => Type::U8,
            Self::I8(_) => Type::I8,
            Self::U16(_) => Type::U16,
            Self::I16(_) => Type::I16,
            Self::U32(_) => Type::U32,
            Self::I32(_) => Type::I32,
            Self::U64(_) => Type::U64,
            Self::I64(_) => Type::I64,
            Self::I128(_) => Type::I128,
            Self::F64(_) => Type::F64,
            Self::Felt(_) => Type::Felt,
        }
    }

    /// Returns true if this immediate is an odd integer, otherwise false
    ///
    /// If the immediate is not an integer, returns `None`
    pub fn is_odd(self) -> Option<bool> {
        match self {
            Self::I1(b) => Some(b),
            Self::U8(i) => Some(i % 2 == 0),
            Self::I8(i) => Some(i % 2 == 0),
            Self::U16(i) => Some(i % 2 == 0),
            Self::I16(i) => Some(i % 2 == 0),
            Self::U32(i) => Some(i % 2 == 0),
            Self::I32(i) => Some(i % 2 == 0),
            Self::U64(i) => Some(i % 2 == 0),
            Self::I64(i) => Some(i % 2 == 0),
            Self::Felt(i) => Some(i % 2 == 0),
            Self::I128(i) => Some(i % 2 == 0),
            Self::F64(_) => None,
        }
    }

    /// Returns true if this immediate is a non-zero integer, otherwise false
    ///
    /// If the immediate is not an integer, returns `None`
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::I1(b) => Some(*b),
            Self::U8(i) => Some(*i != 0),
            Self::I8(i) => Some(*i != 0),
            Self::U16(i) => Some(*i != 0),
            Self::I16(i) => Some(*i != 0),
            Self::U32(i) => Some(*i != 0),
            Self::I32(i) => Some(*i != 0),
            Self::U64(i) => Some(*i != 0),
            Self::I64(i) => Some(*i != 0),
            Self::Felt(i) => Some(*i != 0),
            Self::I128(i) => Some(*i != 0),
            Self::F64(_) => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::I1(b) => Some(*b as i64),
            Self::U8(i) => Some(*i as i64),
            Self::I8(i) => Some(*i as i64),
            Self::U16(i) => Some(*i as i64),
            Self::I16(i) => Some(*i as i64),
            Self::U32(i) => Some(*i as i64),
            Self::I32(i) => Some(*i as i64),
            Self::U64(i) => (*i).try_into().ok(),
            Self::I64(i) => Some(*i),
            Self::Felt(i) => (*i).try_into().ok(),
            Self::I128(i) => (*i).try_into().ok(),
            Self::F64(_) => None,
        }
    }
}
impl fmt::Display for Immediate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::I1(i) => write!(f, "{}", i),
            Self::U8(i) => write!(f, "{}", i),
            Self::I8(i) => write!(f, "{}", i),
            Self::U16(i) => write!(f, "{}", i),
            Self::I16(i) => write!(f, "{}", i),
            Self::U32(i) => write!(f, "{}", i),
            Self::I32(i) => write!(f, "{}", i),
            Self::U64(i) => write!(f, "{}", i),
            Self::I64(i) => write!(f, "{}", i),
            Self::I128(i) => write!(f, "{}", i),
            Self::F64(n) => write!(f, "{}", n),
            Self::Felt(i) => write!(f, "{}", i),
        }
    }
}
impl Hash for Immediate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let d = std::mem::discriminant(self);
        d.hash(state);
        match self {
            Self::I1(i) => i.hash(state),
            Self::U8(i) => i.hash(state),
            Self::I8(i) => i.hash(state),
            Self::U16(i) => i.hash(state),
            Self::I16(i) => i.hash(state),
            Self::U32(i) => i.hash(state),
            Self::I32(i) => i.hash(state),
            Self::U64(i) => i.hash(state),
            Self::I64(i) => i.hash(state),
            Self::I128(i) => i.hash(state),
            Self::F64(f) => {
                let bytes = f.to_be_bytes();
                bytes.hash(state)
            }
            Self::Felt(i) => i.hash(state),
        }
    }
}
impl Eq for Immediate {}
impl PartialEq for Immediate {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Self::I8(x), Self::I8(y)) => x == y,
            (Self::U16(x), Self::U16(y)) => x == y,
            (Self::I16(x), Self::I16(y)) => x == y,
            (Self::U32(x), Self::U32(y)) => x == y,
            (Self::I32(x), Self::I32(y)) => x == y,
            (Self::U64(x), Self::U64(y)) => x == y,
            (Self::I64(x), Self::I64(y)) => x == y,
            (Self::I128(x), Self::I128(y)) => x == y,
            (Self::F64(x), Self::F64(y)) => x == y,
            (Self::Felt(x), Self::Felt(y)) => x == y,
            _ => false,
        }
    }
}
impl PartialOrd for Immediate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;

        match (self, other) {
            (Self::I128(x), Self::I128(y)) => x.partial_cmp(y),
            (Self::I128(x), Self::F64(y)) => {
                assert!(!y.is_finite());
                let y = unsafe { y.to_int_unchecked::<i128>() };
                x.partial_cmp(&y)
            }
            (Self::I128(x), y) => {
                let y = y.as_i64().unwrap() as i128;
                x.partial_cmp(&y)
            }
            (Self::F64(x), Self::I128(y)) => {
                assert!(!x.is_finite());
                let x = unsafe { x.to_int_unchecked::<i128>() };
                x.partial_cmp(y)
            }
            (Self::F64(x), Self::F64(y)) => x.partial_cmp(y),
            (Self::F64(x), y) => {
                let y = y.as_i64().unwrap() as f64;
                match x.total_cmp(&y) {
                    Ordering::Equal => Some(Ordering::Equal),
                    ord => Some(ord),
                }
            }
            (x, Self::F64(y)) => {
                let x = x.as_i64().unwrap() as f64;
                match x.total_cmp(y) {
                    Ordering::Equal => Some(Ordering::Equal),
                    ord => Some(ord),
                }
            }
            (x, Self::I128(y)) => {
                let x = x.as_i64().unwrap() as i128;
                x.partial_cmp(y)
            }
            (x, y) => {
                let x = x.as_i64().unwrap();
                let y = y.as_i64().unwrap();
                match x.cmp(&y) {
                    Ordering::Equal => Some(Ordering::Equal),
                    ord => Some(ord),
                }
            }
        }
    }
}
impl From<bool> for Immediate {
    #[inline(always)]
    fn from(value: bool) -> Self {
        Self::I1(value)
    }
}
impl From<i8> for Immediate {
    #[inline(always)]
    fn from(value: i8) -> Self {
        Self::I8(value)
    }
}
impl From<u8> for Immediate {
    #[inline(always)]
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}
impl From<i16> for Immediate {
    #[inline(always)]
    fn from(value: i16) -> Self {
        Self::I16(value)
    }
}
impl From<u16> for Immediate {
    #[inline(always)]
    fn from(value: u16) -> Self {
        Self::U16(value)
    }
}
impl From<i32> for Immediate {
    #[inline(always)]
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}
impl From<u32> for Immediate {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}
impl From<i64> for Immediate {
    #[inline(always)]
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}
impl From<u64> for Immediate {
    #[inline(always)]
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}
impl From<i128> for Immediate {
    #[inline(always)]
    fn from(value: i128) -> Self {
        Self::I128(value)
    }
}
impl From<f64> for Immediate {
    #[inline(always)]
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}
impl From<char> for Immediate {
    #[inline(always)]
    fn from(value: char) -> Self {
        Self::I32(value as u32 as i32)
    }
}
