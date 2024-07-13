use std::collections::VecDeque;

use miden_core::Felt;
use proptest::{
    arbitrary::Arbitrary,
    strategy::{BoxedStrategy, Strategy},
};

pub trait PushToStack: Sized {
    fn try_push(&self, stack: &mut Vec<Felt>) {
        let mut ptr = self as *const Self as *const u8;
        let mut num_bytes = core::mem::size_of::<Self>();
        let mut buf = Vec::with_capacity(num_bytes / core::mem::size_of::<u32>());
        while num_bytes > 0 {
            let mut next = [0u8; 4];
            let consume = core::cmp::min(4, num_bytes);
            unsafe {
                ptr.copy_to_nonoverlapping(next.as_mut_ptr(), consume);
                ptr = ptr.byte_add(consume);
            }
            num_bytes -= consume;
            buf.push(Felt::new(u32::from_be_bytes(next) as u64));
        }

        for item in buf.into_iter().rev() {
            stack.push(item);
        }
    }
}

pub trait PopFromStack: Sized {
    fn try_pop(stack: &mut VecDeque<TestFelt>) -> Result<Self, ()> {
        use core::mem::MaybeUninit;

        let mut num_bytes = core::mem::size_of::<Self>();
        let mut result = MaybeUninit::<Self>::uninit();
        let mut ptr = result.as_mut_ptr() as *mut u8;
        while num_bytes > 0 {
            let next = stack.pop_front().expect("expected more operand stack elements");
            let next_bytes = (next.0.as_int() as u32).to_be_bytes();
            let consume = core::cmp::min(4, num_bytes);
            unsafe {
                next_bytes.as_ptr().copy_to_nonoverlapping(ptr, consume);
                ptr = ptr.byte_add(consume);
            }
            num_bytes -= consume;
        }
        Ok(unsafe { result.assume_init() })
    }
}

impl PushToStack for bool {
    fn try_push(&self, stack: &mut Vec<Felt>) {
        stack.push(Felt::new(*self as u64))
    }
}
impl PopFromStack for bool {
    fn try_pop(stack: &mut VecDeque<TestFelt>) -> Result<Self, ()> {
        Ok(stack.pop_front().unwrap().0.as_int() != 0)
    }
}

impl PushToStack for u8 {
    fn try_push(&self, stack: &mut Vec<Felt>) {
        stack.push(Felt::new(*self as u64))
    }
}
impl PopFromStack for u8 {
    fn try_pop(stack: &mut VecDeque<TestFelt>) -> Result<Self, ()> {
        Ok(stack.pop_front().unwrap().0.as_int() as u8)
    }
}

impl PushToStack for i8 {
    fn try_push(&self, stack: &mut Vec<Felt>) {
        stack.push(Felt::new(*self as u8 as u64))
    }
}
impl PopFromStack for i8 {
    fn try_pop(stack: &mut VecDeque<TestFelt>) -> Result<Self, ()> {
        Ok(stack.pop_front().unwrap().0.as_int() as i8)
    }
}

impl PushToStack for u16 {
    fn try_push(&self, stack: &mut Vec<Felt>) {
        stack.push(Felt::new(*self as u64))
    }
}
impl PopFromStack for u16 {
    fn try_pop(stack: &mut VecDeque<TestFelt>) -> Result<Self, ()> {
        Ok(stack.pop_front().unwrap().0.as_int() as u16)
    }
}

impl PushToStack for i16 {
    fn try_push(&self, stack: &mut Vec<Felt>) {
        stack.push(Felt::new(*self as u16 as u64))
    }
}
impl PopFromStack for i16 {
    fn try_pop(stack: &mut VecDeque<TestFelt>) -> Result<Self, ()> {
        Ok(stack.pop_front().unwrap().0.as_int() as i16)
    }
}

impl PushToStack for u32 {
    fn try_push(&self, stack: &mut Vec<Felt>) {
        stack.push(Felt::new(*self as u64))
    }
}
impl PopFromStack for u32 {
    fn try_pop(stack: &mut VecDeque<TestFelt>) -> Result<Self, ()> {
        Ok(stack.pop_front().unwrap().0.as_int() as u32)
    }
}

impl PushToStack for i32 {
    fn try_push(&self, stack: &mut Vec<Felt>) {
        stack.push(Felt::new(*self as u32 as u64))
    }
}
impl PopFromStack for i32 {
    fn try_pop(stack: &mut VecDeque<TestFelt>) -> Result<Self, ()> {
        Ok(stack.pop_front().unwrap().0.as_int() as i32)
    }
}

impl PushToStack for u64 {
    fn try_push(&self, stack: &mut Vec<Felt>) {
        let lo = self.rem_euclid(2u64.pow(32));
        let hi = self.div_euclid(2u64.pow(32));
        dbg!(hi, lo);
        stack.push(Felt::new(lo));
        stack.push(Felt::new(hi));
    }
}
impl PopFromStack for u64 {
    fn try_pop(stack: &mut VecDeque<TestFelt>) -> Result<Self, ()> {
        dbg!(&stack);
        let hi = stack.pop_front().unwrap().0.as_int() * 2u64.pow(32);
        let lo = stack.pop_front().unwrap().0.as_int();
        dbg!(hi, lo);
        Ok(hi + lo)
    }
}

impl PushToStack for i64 {
    fn try_push(&self, stack: &mut Vec<Felt>) {
        (*self as u64).try_push(stack)
    }
}
impl PopFromStack for i64 {
    fn try_pop(stack: &mut VecDeque<TestFelt>) -> Result<Self, ()> {
        u64::try_pop(stack).map(|value| value as i64)
    }
}

impl PushToStack for u128 {
    fn try_push(&self, stack: &mut Vec<Felt>) {
        let lo = self.rem_euclid(2u128.pow(64));
        let hi = self.div_euclid(2u128.pow(64));
        (lo as u64).try_push(stack);
        (hi as u64).try_push(stack);
    }
}
impl PopFromStack for u128 {
    fn try_pop(stack: &mut VecDeque<TestFelt>) -> Result<Self, ()> {
        let hi = (u64::try_pop(stack).unwrap() as u128) * 2u128.pow(64);
        let lo = u64::try_pop(stack).unwrap() as u128;
        Ok(hi + lo)
    }
}

impl PushToStack for i128 {
    fn try_push(&self, stack: &mut Vec<Felt>) {
        (*self as u128).try_push(stack)
    }
}
impl PopFromStack for i128 {
    fn try_pop(stack: &mut VecDeque<TestFelt>) -> Result<Self, ()> {
        u128::try_pop(stack).map(|value| value as i128)
    }
}

impl PushToStack for Felt {
    #[inline(always)]
    fn try_push(&self, stack: &mut Vec<Felt>) {
        stack.push(*self);
    }
}
impl PopFromStack for Felt {
    #[inline(always)]
    fn try_pop(stack: &mut VecDeque<TestFelt>) -> Result<Self, ()> {
        Ok(stack.pop_front().ok_or(())?.0)
    }
}

impl PushToStack for TestFelt {
    #[inline(always)]
    fn try_push(&self, stack: &mut Vec<Felt>) {
        stack.push(self.0);
    }
}
impl PopFromStack for TestFelt {
    #[inline(always)]
    fn try_pop(stack: &mut VecDeque<TestFelt>) -> Result<Self, ()> {
        stack.pop_front().ok_or(())
    }
}

/// Wrapper around `Felt` that implements `From` for a bunch of types that are want to support in
/// tests
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TestFelt(pub Felt);

impl From<TestFelt> for Felt {
    fn from(f: TestFelt) -> Self {
        f.0
    }
}

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
        Self(Felt::new(t))
    }
}

impl From<i64> for TestFelt {
    fn from(t: i64) -> Self {
        Self(Felt::new(t as u64))
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
        f.0.as_int()
    }
}

impl From<TestFelt> for i64 {
    fn from(f: TestFelt) -> Self {
        f.0.as_int() as i64
    }
}

impl Arbitrary for TestFelt {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        (0u64..u64::MAX).prop_map(|v| TestFelt(Felt::new(v))).boxed()
    }
}
