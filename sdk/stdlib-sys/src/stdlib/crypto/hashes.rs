//! Contains procedures for computing hashes using BLAKE3 and SHA256 hash
//! functions. The input and output elements are assumed to contain one 32-bit
//! value per element.

#[link(wasm_import_module = "miden:core-import/stdlib-crypto-hashes-blake3@1.0.0")]
extern "C" {
    /// Computes BLAKE3 1-to-1 hash.
    ///
    /// Input: 32-bytes stored in the first 8 elements of the stack (32 bits per element).
    /// Output: A 32-byte digest stored in the first 8 elements of stack (32 bits per element).
    /// The output is passed back to the caller via a pointer.
    #[link_name = "hash-one-to-one"]
    fn extern_blake3_hash_1to1(
        e1: u32,
        e2: u32,
        e3: u32,
        e4: u32,
        e5: u32,
        e6: u32,
        e7: u32,
        e8: u32,
        ptr: *mut u8,
    );

    /// Computes BLAKE3 2-to-1 hash.
    ///
    /// Input: 64-bytes stored in the first 16 elements of the stack (32 bits per element).
    /// Output: A 32-byte digest stored in the first 8 elements of stack (32 bits per element)
    /// The output is passed back to the caller via a pointer.
    #[link_name = "hash-two-to-one"]
    fn extern_blake3_hash_2to1(
        e1: u32,
        e2: u32,
        e3: u32,
        e4: u32,
        e5: u32,
        e6: u32,
        e7: u32,
        e8: u32,
        e9: u32,
        e10: u32,
        e11: u32,
        e12: u32,
        e13: u32,
        e14: u32,
        e15: u32,
        e16: u32,
        ptr: *mut u8,
    );
}

#[link(wasm_import_module = "miden:core-import/stdlib-crypto-hashes-sha256@1.0.0")]
extern "C" {
    /// Computes SHA256 1-to-1 hash.
    ///
    /// Input: 32-bytes stored in the first 8 elements of the stack (32 bits per element).
    /// Output: A 32-byte digest stored in the first 8 elements of stack (32 bits per element).
    /// The output is passed back to the caller via a pointer.
    #[link_name = "sha256-hash-one-to-one"]
    fn extern_sha256_hash_1to1(
        e1: u32,
        e2: u32,
        e3: u32,
        e4: u32,
        e5: u32,
        e6: u32,
        e7: u32,
        e8: u32,
        ptr: *mut u8,
    );

    /// Computes SHA256 2-to-1 hash.
    ///
    /// Input: 64-bytes stored in the first 16 elements of the stack (32 bits per element).
    /// Output: A 32-byte digest stored in the first 8 elements of stack (32 bits per element).
    /// The output is passed back to the caller via a pointer.
    #[link_name = "sha256-hash-two-to-one"]
    fn extern_sha256_hash_2to1(
        e1: u32,
        e2: u32,
        e3: u32,
        e4: u32,
        e5: u32,
        e6: u32,
        e7: u32,
        e8: u32,
        e9: u32,
        e10: u32,
        e11: u32,
        e12: u32,
        e13: u32,
        e14: u32,
        e15: u32,
        e16: u32,
        ptr: *mut u8,
    );
}

/// Hashes a 32-byte input to a 32-byte output using the given hash function.
#[inline(always)]
fn hash_1to1(
    input: [u8; 32],
    extern_hash_1to1: unsafe extern "C" fn(u32, u32, u32, u32, u32, u32, u32, u32, *mut u8),
) -> [u8; 32] {
    use crate::WordAligned;

    let input = unsafe { core::mem::transmute::<[u8; 32], [u32; 8]>(input) };
    unsafe {
        let mut ret_area = ::core::mem::MaybeUninit::<WordAligned<[u8; 32]>>::uninit();
        let ptr = ret_area.as_mut_ptr() as *mut u8;
        extern_hash_1to1(
            input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7], ptr,
        );
        ret_area.assume_init().into_inner()
    }
}

/// Hashes a 64-byte input to a 32-byte output using the given hash function.
#[inline(always)]
fn hash_2to1(
    input: [u8; 64],
    extern_hash_2to1: unsafe extern "C" fn(
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        *mut u8,
    ),
) -> [u8; 32] {
    let input = unsafe { core::mem::transmute::<[u8; 64], [u32; 16]>(input) };
    unsafe {
        let mut ret_area = ::core::mem::MaybeUninit::<[u8; 32]>::uninit();
        let ptr = ret_area.as_mut_ptr() as *mut u8;
        extern_hash_2to1(
            input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7],
            input[8], input[9], input[10], input[11], input[12], input[13], input[14], input[15],
            ptr,
        );
        ret_area.assume_init()
    }
}

/// Hashes a 32-byte input to a 32-byte output using the BLAKE3 hash function.
#[inline]
pub fn blake3_hash_1to1(input: [u8; 32]) -> [u8; 32] {
    hash_1to1(input, extern_blake3_hash_1to1)
}

/// Hashes a 64-byte input to a 32-byte output using the BLAKE3 hash function.
#[inline]
pub fn blake3_hash_2to1(input: [u8; 64]) -> [u8; 32] {
    hash_2to1(input, extern_blake3_hash_2to1)
}

/// Hashes a 32-byte input to a 32-byte output using the SHA256 hash function.
#[inline]
pub fn sha256_hash_1to1(input: [u8; 32]) -> [u8; 32] {
    hash_1to1(input, extern_sha256_hash_1to1)
}

/// Hashes a 64-byte input to a 32-byte output using the SHA256 hash function.
#[inline]
pub fn sha256_hash_2to1(input: [u8; 64]) -> [u8; 32] {
    hash_2to1(input, extern_sha256_hash_2to1)
}
