#![no_std]

use core::ops::Add;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
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
    const M: u64 = 0xFFFFFFFF00000001;

    pub fn new(value: u64) -> Result<Self, FeltError> {
        if value > Self::M {
            Err(FeltError::InvalidValue)
        } else {
            Ok(Felt(value as f64))
        }
    }

    pub fn as_u64(self) -> u64 {
        self.0 as u64
    }
}

impl From<Felt> for u64 {
    fn from(felt: Felt) -> u64 {
        felt.0 as u64
    }
}

// extern "C" {
//     #[link_name = "miden_compiler_intrinsics_felt_add"]
//     fn felt_add(a: Felt, b: Felt) -> Felt;
// }

impl Add for Felt {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        // unsafe { felt_add(self, other) }
        Felt(self.0 + other.0)
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Word(Felt, Felt, Felt, Felt);
