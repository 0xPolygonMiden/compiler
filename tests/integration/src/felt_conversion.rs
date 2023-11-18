use miden_core::Felt;
use miden_core::StarkField;

/// Wrapper around `Felt` that implements `From` for a bunch of types that are want to support in tests
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TestFelt(pub Felt);

impl From<bool> for TestFelt {
    fn from(b: bool) -> Self {
        Self(Felt::from(b as u32))
    }
}

impl From<u8> for TestFelt {
    fn from(t: u8) -> Self {
        Self(t.into())
    }
}

impl From<i8> for TestFelt {
    fn from(t: i8) -> Self {
        Self((t as u8).into())
    }
}

impl From<i16> for TestFelt {
    fn from(t: i16) -> Self {
        Self((t as u16).into())
    }
}

impl From<u16> for TestFelt {
    fn from(t: u16) -> Self {
        Self(t.into())
    }
}

impl From<i32> for TestFelt {
    fn from(t: i32) -> Self {
        Self((t as u32).into())
    }
}

impl From<u32> for TestFelt {
    fn from(t: u32) -> Self {
        Self(t.into())
    }
}

impl From<u64> for TestFelt {
    fn from(t: u64) -> Self {
        Self(t.into())
    }
}

impl From<i64> for TestFelt {
    fn from(t: i64) -> Self {
        Self((t as u64).into())
    }
}

// Reverse TestFelt to Rust types conversion

impl From<TestFelt> for bool {
    fn from(f: TestFelt) -> Self {
        f.0.as_int() != 0
    }
}

impl From<TestFelt> for u8 {
    fn from(f: TestFelt) -> Self {
        f.0.as_int() as u8
    }
}

impl From<TestFelt> for i8 {
    fn from(f: TestFelt) -> Self {
        f.0.as_int() as i8
    }
}

impl From<TestFelt> for u16 {
    fn from(f: TestFelt) -> Self {
        f.0.as_int() as u16
    }
}

impl From<TestFelt> for i16 {
    fn from(f: TestFelt) -> Self {
        f.0.as_int() as i16
    }
}

impl From<TestFelt> for u32 {
    fn from(f: TestFelt) -> Self {
        f.0.as_int() as u32
    }
}

impl From<TestFelt> for i32 {
    fn from(f: TestFelt) -> Self {
        f.0.as_int() as i32
    }
}

impl From<TestFelt> for u64 {
    fn from(f: TestFelt) -> Self {
        f.0.as_int() as u64
    }
}

impl From<TestFelt> for i64 {
    fn from(f: TestFelt) -> Self {
        f.0.as_int() as i64
    }
}
