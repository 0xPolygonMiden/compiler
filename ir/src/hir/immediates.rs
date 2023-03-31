use core::fmt;
use core::hash::{Hash, Hasher};

use crate::types::Type;

#[derive(Debug, Copy, Clone)]
pub enum Immediate {
    I1(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    F64(f64),
    Felt(u64),
}
impl Immediate {
    pub fn ty(&self) -> Type {
        match self {
            Self::I1(_) => Type::I1,
            Self::I8(_) => Type::I8,
            Self::I16(_) => Type::I16,
            Self::I32(_) => Type::I32,
            Self::I64(_) => Type::I64,
            Self::I128(_) => Type::I128,
            Self::Isize(_) => Type::Isize,
            Self::F64(_) => Type::F64,
            Self::Felt(_) => Type::Felt,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::I1(b) => Some(*b as i64),
            Self::I8(i) => Some(*i as i64),
            Self::I16(i) => Some(*i as i64),
            Self::I32(i) => Some(*i as i64),
            Self::I64(i) => Some(*i),
            Self::I128(i) => (*i).try_into().ok(),
            Self::Isize(i) => Some(*i as i64),
            Self::F64(_) => None,
            Self::Felt(_) => None,
        }
    }
}
impl fmt::Display for Immediate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::I1(i) => write!(f, "{}", i),
            Self::I8(i) => write!(f, "{}", i),
            Self::I16(i) => write!(f, "{}", i),
            Self::I32(i) => write!(f, "{}", i),
            Self::I64(i) => write!(f, "{}", i),
            Self::I128(i) => write!(f, "{}", i),
            Self::Isize(i) => write!(f, "{}", i),
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
            Self::I8(i) => i.hash(state),
            Self::I16(i) => i.hash(state),
            Self::I32(i) => i.hash(state),
            Self::I64(i) => i.hash(state),
            Self::I128(i) => i.hash(state),
            Self::Isize(i) => i.hash(state),
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
            (Self::I16(x), Self::I16(y)) => x == y,
            (Self::I32(x), Self::I32(y)) => x == y,
            (Self::I64(x), Self::I64(y)) => x == y,
            (Self::I128(x), Self::I128(y)) => x == y,
            (Self::Isize(x), Self::Isize(y)) => x == y,
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
                match x.total_cmp(&y) {
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
impl From<i16> for Immediate {
    #[inline(always)]
    fn from(value: i16) -> Self {
        Self::I16(value)
    }
}
impl From<i32> for Immediate {
    #[inline(always)]
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}
impl From<i64> for Immediate {
    #[inline(always)]
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}
impl From<i128> for Immediate {
    #[inline(always)]
    fn from(value: i128) -> Self {
        Self::I128(value)
    }
}
impl From<isize> for Immediate {
    #[inline(always)]
    fn from(value: isize) -> Self {
        Self::Isize(value)
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
