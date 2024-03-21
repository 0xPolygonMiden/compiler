use core::ops::{Add, Div, Mul, Neg, Sub};

#[link(wasm_import_module = "miden:prelude/intrinsics_felt")]
extern "C" {
    #[link_name = "from_u64_unchecked"]
    fn extern_from_u64_unchecked(value: u64) -> Felt;

    #[link_name = "as_u64"]
    fn extern_as_u64(felt: Felt) -> u64;

    #[link_name = "add"]
    fn extern_add(a: Felt, b: Felt) -> Felt;

    #[link_name = "sub"]
    fn extern_sub(a: Felt, b: Felt) -> Felt;

    #[link_name = "mul"]
    fn extern_mul(a: Felt, b: Felt) -> Felt;

    #[link_name = "div"]
    fn extern_div(a: Felt, b: Felt) -> Felt;

    #[link_name = "neg"]
    fn extern_neg(a: Felt) -> Felt;

    #[link_name = "inv"]
    fn extern_inv(a: Felt) -> Felt;

    #[link_name = "pow2"]
    fn extern_pow2(a: Felt) -> Felt;

    #[link_name = "exp"]
    fn extern_exp(a: Felt, b: Felt) -> Felt;

    #[link_name = "eq"]
    fn extern_eq(a: Felt, b: Felt) -> i32;

    #[link_name = "gt"]
    fn extern_gt(a: Felt, b: Felt) -> i32;

    #[link_name = "lt"]
    fn extern_lt(a: Felt, b: Felt) -> i32;

    #[link_name = "ge"]
    fn extern_ge(a: Felt, b: Felt) -> i32;

    #[link_name = "le"]
    fn extern_le(a: Felt, b: Felt) -> i32;

    #[link_name = "is_odd"]
    fn extern_is_odd(a: Felt) -> i32;

    #[link_name = "assert"]
    fn extern_assert(a: Felt);

    #[link_name = "assertz"]
    fn extern_assertz(a: Felt);

    #[link_name = "assert_eq"]
    fn extern_assert_eq(a: Felt, b: Felt);
}

#[derive(Debug)]
pub enum FeltError {
    InvalidValue,
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Felt(f64);

impl Felt {
    /// Field modulus = 2^64 - 2^32 + 1
    const M: u64 = 0xffffffff00000001;

    #[inline(always)]
    pub fn from_u64_unchecked(value: u64) -> Self {
        unsafe { extern_from_u64_unchecked(value) }
    }

    #[inline(always)]
    pub fn new(value: u64) -> Result<Self, FeltError> {
        if value > Self::M {
            Err(FeltError::InvalidValue)
        } else {
            Ok(Self::from_u64_unchecked(value))
        }
    }

    #[inline(always)]
    pub fn as_u64(self) -> u64 {
        unsafe { extern_as_u64(self) }
    }

    /// Returns true if x is odd and false if x is even
    #[inline(always)]
    pub fn is_odd(self) -> bool {
        unsafe { extern_is_odd(self) != 0 }
    }

    /// Returns x^-1
    /// Fails if a=0
    #[inline(always)]
    pub fn inv(self) -> Felt {
        unsafe { extern_inv(self) }
    }

    /// Returns 2^x
    /// Fails if x > 63
    #[inline(always)]
    pub fn pow2(self) -> Felt {
        unsafe { extern_pow2(self) }
    }

    /// Returns a^b
    #[inline(always)]
    pub fn exp(self, other: Felt) -> Felt {
        unsafe { extern_exp(self, other) }
    }
}

impl From<Felt> for u64 {
    fn from(felt: Felt) -> u64 {
        felt.0 as u64
    }
}

impl Add for Felt {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        unsafe { extern_add(self, other) }
    }
}

impl Sub for Felt {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        unsafe { extern_sub(self, other) }
    }
}

impl Mul for Felt {
    type Output = Self;

    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        unsafe { extern_mul(self, other) }
    }
}

impl Div for Felt {
    type Output = Self;

    #[inline(always)]
    fn div(self, other: Self) -> Self {
        unsafe { extern_div(self, other) }
    }
}

impl Neg for Felt {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self {
        unsafe { extern_neg(self) }
    }
}

impl PartialEq for Felt {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        unsafe { extern_eq(*self, *other) == 1 }
    }
}

impl Eq for Felt {}

impl PartialOrd for Felt {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }

    #[inline(always)]
    fn gt(&self, other: &Self) -> bool {
        unsafe { extern_gt(*self, *other) != 0 }
    }

    #[inline(always)]
    fn ge(&self, other: &Self) -> bool {
        unsafe { extern_ge(*self, *other) != 0 }
    }

    #[inline(always)]
    fn lt(&self, other: &Self) -> bool {
        unsafe { extern_lt(*other, *self) != 0 }
    }

    #[inline(always)]
    fn le(&self, other: &Self) -> bool {
        unsafe { extern_le(*other, *self) != 0 }
    }
}

impl Ord for Felt {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if self.lt(other) {
            core::cmp::Ordering::Less
        } else if self.gt(other) {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }
}

/// If `a` == 1, removes it from the stack.  Fails if `a` != 1
#[inline(always)]
pub fn assert(a: Felt) {
    unsafe {
        extern_assert(a);
    }
}

/// If `a` == 0, removes it from the stack.  Fails if `a` != 0
#[inline(always)]
pub fn assertz(a: Felt) {
    unsafe {
        extern_assertz(a);
    }
}

/// If `a` == `b`, removes them from the stack.  Fails if `a` != `b`
#[inline(always)]
pub fn assert_eq(a: Felt, b: Felt) {
    unsafe {
        extern_assert_eq(a, b);
    }
}
