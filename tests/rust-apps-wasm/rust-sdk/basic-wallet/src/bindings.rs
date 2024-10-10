#[allow(dead_code)]
pub mod miden {
    #[allow(dead_code)]
    pub mod base {
        #[allow(dead_code, clippy::all)]
        pub mod core_types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            /// Unique identifier of an account.
            ///
            /// Account ID consists of 1 field element (~64 bits). This field element uniquely identifies a
            /// single account and also specifies the type of the underlying account. Specifically:
            /// - The two most significant bits of the ID specify the type of the account:
            /// - 00 - regular account with updatable code.
            /// - 01 - regular account with immutable code.
            /// - 10 - fungible asset faucet with immutable code.
            /// - 11 - non-fungible asset faucet with immutable code.
            /// - The third most significant bit of the ID specifies whether the account data is stored on-chain:
            /// - 0 - full account data is stored on-chain.
            /// - 1 - only the account hash is stored on-chain which serves as a commitment to the account state.
            /// As such the three most significant bits fully describes the type of the account.
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct AccountId {
                pub inner: miden_sdk::Felt,
            }
            impl ::core::fmt::Debug for AccountId {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("AccountId").field("inner", &self.inner).finish()
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod core_import {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            /// Represents base field element in the field using Montgomery representation.
            /// Internal values represent x * R mod M where R = 2^64 mod M and x in [0, M).
            /// The backing type is `f64` but the internal values are always integer in the range [0, M).
            /// Field modulus M = 2^64 - 2^32 + 1
            pub type Felt = f32;
            pub type Ptr = i32;
        }
        #[allow(dead_code, clippy::all)]
        pub mod intrinsics_mem {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            pub type Ptr = super::super::super::miden::core_import::types::Ptr;
            #[allow(unused_unsafe, clippy::all)]
            pub fn heap_base() -> Ptr {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "miden:core-import/intrinsics-mem@1.0.0"
                    )]
                    extern "C" {
                        #[link_name = "heap-base"]
                        fn wit_import() -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod intrinsics_felt {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Felt = super::super::super::miden::core_import::types::Felt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn add(a: Felt, b: Felt) -> Felt {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "miden:core-import/intrinsics-felt@1.0.0"
                    )]
                    extern "C" {
                        #[link_name = "add"]
                        fn wit_import(_: f32, _: f32) -> f32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: f32, _: f32) -> f32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_f32(a), _rt::as_f32(b));
                    ret
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod stdlib_crypto_hashes_blake3 {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Ptr = super::super::super::miden::core_import::types::Ptr;
            #[allow(unused_unsafe, clippy::all)]
            pub fn hash_one_to_one(
                a0: i32,
                a1: i32,
                a2: i32,
                a3: i32,
                a4: i32,
                a5: i32,
                a6: i32,
                a7: i32,
                result_ptr: Ptr,
            ) {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "miden:core-import/stdlib-crypto-hashes-blake3@1.0.0"
                    )]
                    extern "C" {
                        #[link_name = "hash-one-to-one"]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                        );
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                        _: i32,
                    ) {
                        unreachable!()
                    }
                    wit_import(
                        _rt::as_i32(&a0),
                        _rt::as_i32(&a1),
                        _rt::as_i32(&a2),
                        _rt::as_i32(&a3),
                        _rt::as_i32(&a4),
                        _rt::as_i32(&a5),
                        _rt::as_i32(&a6),
                        _rt::as_i32(&a7),
                        _rt::as_i32(result_ptr),
                    );
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod account {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Felt = super::super::super::miden::core_import::types::Felt;
            pub type Ptr = super::super::super::miden::core_import::types::Ptr;
            #[allow(unused_unsafe, clippy::all)]
            /// Add the specified asset to the vault. Panics under various conditions.
            /// Returns the final asset in the account vault defined as follows: If asset is
            /// a non-fungible asset, then returns the same as asset. If asset is a
            /// fungible asset, then returns the total fungible asset in the account
            /// vault after asset was added to it.
            pub fn add_asset(
                asset0: Felt,
                asset1: Felt,
                asset2: Felt,
                asset3: Felt,
                result_ptr: Ptr,
            ) {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:core-import/account@1.0.0")]
                    extern "C" {
                        #[link_name = "add-asset"]
                        fn wit_import(_: f32, _: f32, _: f32, _: f32, _: i32);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: f32, _: f32, _: f32, _: f32, _: i32) {
                        unreachable!()
                    }
                    wit_import(
                        _rt::as_f32(asset0),
                        _rt::as_f32(asset1),
                        _rt::as_f32(asset2),
                        _rt::as_f32(asset3),
                        _rt::as_i32(result_ptr),
                    );
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Remove the specified asset from the vault
            pub fn remove_asset(
                asset0: Felt,
                asset1: Felt,
                asset2: Felt,
                asset3: Felt,
                result_ptr: Ptr,
            ) {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:core-import/account@1.0.0")]
                    extern "C" {
                        #[link_name = "remove-asset"]
                        fn wit_import(_: f32, _: f32, _: f32, _: f32, _: i32);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: f32, _: f32, _: f32, _: f32, _: i32) {
                        unreachable!()
                    }
                    wit_import(
                        _rt::as_f32(asset0),
                        _rt::as_f32(asset1),
                        _rt::as_f32(asset2),
                        _rt::as_f32(asset3),
                        _rt::as_i32(result_ptr),
                    );
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod tx {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Felt = super::super::super::miden::core_import::types::Felt;
            #[allow(unused_unsafe, clippy::all)]
            /// Creates a new note.
            /// asset is the asset to be included in the note.
            /// tag is the tag to be included in the note.
            /// recipient is the recipient of the note.
            /// Returns the id of the created note.
            pub fn create_note(
                asset0: Felt,
                asset1: Felt,
                asset2: Felt,
                asset3: Felt,
                tag: Felt,
                note_type: Felt,
                recipient0: Felt,
                recipient1: Felt,
                recipient2: Felt,
                recipient3: Felt,
            ) -> Felt {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:core-import/tx@1.0.0")]
                    extern "C" {
                        #[link_name = "create-note"]
                        fn wit_import(
                            _: f32,
                            _: f32,
                            _: f32,
                            _: f32,
                            _: f32,
                            _: f32,
                            _: f32,
                            _: f32,
                            _: f32,
                            _: f32,
                        ) -> f32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(
                        _: f32,
                        _: f32,
                        _: f32,
                        _: f32,
                        _: f32,
                        _: f32,
                        _: f32,
                        _: f32,
                        _: f32,
                        _: f32,
                    ) -> f32 {
                        unreachable!()
                    }
                    let ret = wit_import(
                        _rt::as_f32(asset0),
                        _rt::as_f32(asset1),
                        _rt::as_f32(asset2),
                        _rt::as_f32(asset3),
                        _rt::as_f32(tag),
                        _rt::as_f32(note_type),
                        _rt::as_f32(recipient0),
                        _rt::as_f32(recipient1),
                        _rt::as_f32(recipient2),
                        _rt::as_f32(recipient3),
                    );
                    ret
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod miden {
        #[allow(dead_code)]
        pub mod basic_wallet {
            #[allow(dead_code, clippy::all)]
            pub mod basic_wallet {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_receive_asset_cabi<T: Guest>(
                    arg0: f32,
                    arg1: f32,
                    arg2: f32,
                    arg3: f32,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::receive_asset(miden_sdk::CoreAsset {
                        inner: miden_sdk::Word {
                            inner: (
                                miden_sdk::Felt { inner: arg0 },
                                miden_sdk::Felt { inner: arg1 },
                                miden_sdk::Felt { inner: arg2 },
                                miden_sdk::Felt { inner: arg3 },
                            ),
                        },
                    });
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_send_asset_cabi<T: Guest>(
                    arg0: f32,
                    arg1: f32,
                    arg2: f32,
                    arg3: f32,
                    arg4: f32,
                    arg5: f32,
                    arg6: f32,
                    arg7: f32,
                    arg8: f32,
                    arg9: f32,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::send_asset(
                        miden_sdk::CoreAsset {
                            inner: miden_sdk::Word {
                                inner: (
                                    miden_sdk::Felt { inner: arg0 },
                                    miden_sdk::Felt { inner: arg1 },
                                    miden_sdk::Felt { inner: arg2 },
                                    miden_sdk::Felt { inner: arg3 },
                                ),
                            },
                        },
                        miden_sdk::Tag {
                            inner: miden_sdk::Felt { inner: arg4 },
                        },
                        miden_sdk::NoteType {
                            inner: miden_sdk::Felt { inner: arg5 },
                        },
                        miden_sdk::Recipient {
                            inner: miden_sdk::Word {
                                inner: (
                                    miden_sdk::Felt { inner: arg6 },
                                    miden_sdk::Felt { inner: arg7 },
                                    miden_sdk::Felt { inner: arg8 },
                                    miden_sdk::Felt { inner: arg9 },
                                ),
                            },
                        },
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_test_felt_intrinsics_cabi<T: Guest>(
                    arg0: f32,
                    arg1: f32,
                ) -> f32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::test_felt_intrinsics(
                        miden_sdk::Felt { inner: arg0 },
                        miden_sdk::Felt { inner: arg1 },
                    );
                    let miden_sdk::Felt { inner: inner1 } = result0;
                    _rt::as_f32(inner1)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_test_stdlib_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let result1 = T::test_stdlib(
                        _rt::Vec::from_raw_parts(arg0.cast(), len0, len0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec3 = (result1).into_boxed_slice();
                    let ptr3 = vec3.as_ptr().cast::<u8>();
                    let len3 = vec3.len();
                    ::core::mem::forget(vec3);
                    *ptr2.add(4).cast::<usize>() = len3;
                    *ptr2.add(0).cast::<*mut u8>() = ptr3.cast_mut();
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_test_stdlib<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 1, 1);
                }
                pub trait Guest {
                    fn receive_asset(core_asset: miden_sdk::CoreAsset);
                    fn send_asset(
                        core_asset: miden_sdk::CoreAsset,
                        tag: miden_sdk::Tag,
                        note_type: miden_sdk::NoteType,
                        recipient: miden_sdk::Recipient,
                    );
                    fn test_felt_intrinsics(
                        a: miden_sdk::Felt,
                        b: miden_sdk::Felt,
                    ) -> miden_sdk::Felt;
                    fn test_stdlib(input: _rt::Vec<u8>) -> _rt::Vec<u8>;
                }
                #[doc(hidden)]
                macro_rules! __export_miden_basic_wallet_basic_wallet_1_0_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "miden:basic-wallet/basic-wallet@1.0.0#receive-asset"] unsafe
                        extern "C" fn export_receive_asset(arg0 : f32, arg1 : f32, arg2 :
                        f32, arg3 : f32,) { $($path_to_types)*::
                        _export_receive_asset_cabi::<$ty > (arg0, arg1, arg2, arg3) }
                        #[export_name =
                        "miden:basic-wallet/basic-wallet@1.0.0#send-asset"] unsafe extern
                        "C" fn export_send_asset(arg0 : f32, arg1 : f32, arg2 : f32, arg3
                        : f32, arg4 : f32, arg5 : f32, arg6 : f32, arg7 : f32, arg8 :
                        f32, arg9 : f32,) { $($path_to_types)*::
                        _export_send_asset_cabi::<$ty > (arg0, arg1, arg2, arg3, arg4,
                        arg5, arg6, arg7, arg8, arg9) } #[export_name =
                        "miden:basic-wallet/basic-wallet@1.0.0#test-felt-intrinsics"]
                        unsafe extern "C" fn export_test_felt_intrinsics(arg0 : f32, arg1
                        : f32,) -> f32 { $($path_to_types)*::
                        _export_test_felt_intrinsics_cabi::<$ty > (arg0, arg1) }
                        #[export_name =
                        "miden:basic-wallet/basic-wallet@1.0.0#test-stdlib"] unsafe
                        extern "C" fn export_test_stdlib(arg0 : * mut u8, arg1 : usize,)
                        -> * mut u8 { $($path_to_types)*:: _export_test_stdlib_cabi::<$ty
                        > (arg0, arg1) } #[export_name =
                        "cabi_post_miden:basic-wallet/basic-wallet@1.0.0#test-stdlib"]
                        unsafe extern "C" fn _post_return_test_stdlib(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_test_stdlib::<$ty > (arg0) }
                        };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_miden_basic_wallet_basic_wallet_1_0_0_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 8],
                );
            }
        }
    }
}
mod _rt {
    pub fn as_f32<T: AsF32>(t: T) -> f32 {
        t.as_f32()
    }
    pub trait AsF32 {
        fn as_f32(self) -> f32;
    }
    impl<'a, T: Copy + AsF32> AsF32 for &'a T {
        fn as_f32(self) -> f32 {
            (*self).as_f32()
        }
    }
    impl AsF32 for f32 {
        #[inline]
        fn as_f32(self) -> f32 {
            self as f32
        }
    }
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_basic_wallet_world_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::miden::basic_wallet::basic_wallet::__export_miden_basic_wallet_basic_wallet_1_0_0_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::miden::basic_wallet::basic_wallet);
    };
}
#[doc(inline)]
pub(crate) use __export_basic_wallet_world_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:miden:basic-wallet@1.0.0:basic-wallet-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1707] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xa2\x0c\x01A\x02\x01\
A\x17\x01B\x1f\x01r\x01\x05innerv\x04\0\x04felt\x03\0\0\x01o\x04\x01\x01\x01\x01\
\x01r\x01\x05inner\x02\x04\0\x04word\x03\0\x03\x01r\x01\x05inner\x01\x04\0\x0aac\
count-id\x03\0\x05\x01r\x01\x05inner\x04\x04\0\x09recipient\x03\0\x07\x01r\x01\x05\
inner\x01\x04\0\x03tag\x03\0\x09\x01r\x01\x05inner\x04\x04\0\x0acore-asset\x03\0\
\x0b\x01r\x01\x05inner\x01\x04\0\x05nonce\x03\0\x0d\x01r\x01\x05inner\x04\x04\0\x0c\
account-hash\x03\0\x0f\x01r\x01\x05inner\x04\x04\0\x0ablock-hash\x03\0\x11\x01r\x01\
\x05inner\x04\x04\0\x0dstorage-value\x03\0\x13\x01r\x01\x05inner\x04\x04\0\x0cst\
orage-root\x03\0\x15\x01r\x01\x05inner\x04\x04\0\x11account-code-root\x03\0\x17\x01\
r\x01\x05inner\x04\x04\0\x10vault-commitment\x03\0\x19\x01r\x01\x05inner\x01\x04\
\0\x07note-id\x03\0\x1b\x01r\x01\x05inner\x01\x04\0\x09note-type\x03\0\x1d\x03\x01\
\x1bmiden:base/core-types@1.0.0\x05\0\x01B\x04\x01v\x04\0\x04felt\x03\0\0\x01z\x04\
\0\x03ptr\x03\0\x02\x03\x01\x1dmiden:core-import/types@1.0.0\x05\x01\x02\x03\0\x01\
\x04felt\x02\x03\0\x01\x03ptr\x01B\x06\x02\x03\x02\x01\x02\x04\0\x04felt\x03\0\0\
\x02\x03\x02\x01\x03\x04\0\x03ptr\x03\0\x02\x01@\0\0\x03\x04\0\x09heap-base\x01\x04\
\x03\x01&miden:core-import/intrinsics-mem@1.0.0\x05\x04\x01B\x06\x02\x03\x02\x01\
\x02\x04\0\x04felt\x03\0\0\x02\x03\x02\x01\x03\x04\0\x03ptr\x03\0\x02\x01@\x02\x01\
a\x01\x01b\x01\0\x01\x04\0\x03add\x01\x04\x03\x01'miden:core-import/intrinsics-f\
elt@1.0.0\x05\x05\x01B\x06\x02\x03\x02\x01\x02\x04\0\x04felt\x03\0\0\x02\x03\x02\
\x01\x03\x04\0\x03ptr\x03\0\x02\x01@\x09\x02a0z\x02a1z\x02a2z\x02a3z\x02a4z\x02a\
5z\x02a6z\x02a7z\x0aresult-ptr\x03\x01\0\x04\0\x0fhash-one-to-one\x01\x04\x03\x01\
3miden:core-import/stdlib-crypto-hashes-blake3@1.0.0\x05\x06\x01B\x07\x02\x03\x02\
\x01\x02\x04\0\x04felt\x03\0\0\x02\x03\x02\x01\x03\x04\0\x03ptr\x03\0\x02\x01@\x05\
\x06asset0\x01\x06asset1\x01\x06asset2\x01\x06asset3\x01\x0aresult-ptr\x03\x01\0\
\x04\0\x09add-asset\x01\x04\x04\0\x0cremove-asset\x01\x04\x03\x01\x1fmiden:core-\
import/account@1.0.0\x05\x07\x01B\x04\x02\x03\x02\x01\x02\x04\0\x04felt\x03\0\0\x01\
@\x0a\x06asset0\x01\x06asset1\x01\x06asset2\x01\x06asset3\x01\x03tag\x01\x09note\
-type\x01\x0arecipient0\x01\x0arecipient1\x01\x0arecipient2\x01\x0arecipient3\x01\
\0\x01\x04\0\x0bcreate-note\x01\x02\x03\x01\x1amiden:core-import/tx@1.0.0\x05\x08\
\x02\x03\0\0\x0acore-asset\x02\x03\0\0\x03tag\x02\x03\0\0\x09recipient\x02\x03\0\
\0\x09note-type\x02\x03\0\0\x04felt\x01B\x13\x02\x03\x02\x01\x09\x04\0\x0acore-a\
sset\x03\0\0\x02\x03\x02\x01\x0a\x04\0\x03tag\x03\0\x02\x02\x03\x02\x01\x0b\x04\0\
\x09recipient\x03\0\x04\x02\x03\x02\x01\x0c\x04\0\x09note-type\x03\0\x06\x02\x03\
\x02\x01\x0d\x04\0\x04felt\x03\0\x08\x01@\x01\x0acore-asset\x01\x01\0\x04\0\x0dr\
eceive-asset\x01\x0a\x01@\x04\x0acore-asset\x01\x03tag\x03\x09note-type\x07\x09r\
ecipient\x05\x01\0\x04\0\x0asend-asset\x01\x0b\x01@\x02\x01a\x09\x01b\x09\0\x09\x04\
\0\x14test-felt-intrinsics\x01\x0c\x01p}\x01@\x01\x05input\x0d\0\x0d\x04\0\x0bte\
st-stdlib\x01\x0e\x04\x01%miden:basic-wallet/basic-wallet@1.0.0\x05\x0e\x04\x01+\
miden:basic-wallet/basic-wallet-world@1.0.0\x04\0\x0b\x18\x01\0\x12basic-wallet-\
world\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.216.\
0\x10wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
