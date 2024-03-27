use core::ops::{Add, Div, Mul, Neg, Not, Sub};

#[link(wasm_import_module = "miden:types/felt")]
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

    #[link_name = "not"]
    fn extern_not(a: Felt) -> Felt;

    #[link_name = "eq"]
    fn extern_eq(a: Felt, b: Felt) -> bool;
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

    pub fn as_u64(self) -> u64 {
        unsafe { extern_as_u64(self) }
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

/// Returns x^-1
/// Fails if a=0
#[inline(always)]
pub fn inv(x: Felt) -> Felt {
    unsafe { extern_inv(x) }
}

/// Returns 2^x
/// Fails if x > 63
#[inline(always)]
pub fn pow2(x: Felt) -> Felt {
    unsafe { extern_pow2(x) }
}

/// Returns a^b
#[inline(always)]
pub fn exp(a: Felt, b: Felt) -> Felt {
    unsafe { extern_exp(a, b) }
}

impl Not for Felt {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self {
        unsafe { extern_not(self) }
    }
}

impl PartialEq for Felt {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        unsafe { extern_eq(*self, *other) }
    }
}

impl Eq for Felt {}
