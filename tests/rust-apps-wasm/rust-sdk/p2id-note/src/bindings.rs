#[allow(dead_code)]
pub mod miden {
    #[allow(dead_code)]
    pub mod base {
        #[allow(dead_code, clippy::all)]
        pub mod core_types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
        }
    }
    #[allow(dead_code)]
    pub mod basic_wallet {
        #[allow(dead_code, clippy::all)]
        pub mod basic_wallet {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn receive_asset(core_asset: miden::CoreAsset) {
                unsafe {
                    let miden::CoreAsset { inner: inner0 } = core_asset;
                    let miden::Word { inner: inner1 } = inner0;
                    let (t2_0, t2_1, t2_2, t2_3) = inner1;
                    let miden::Felt { inner: inner3 } = t2_0;
                    let miden::Felt { inner: inner4 } = t2_1;
                    let miden::Felt { inner: inner5 } = t2_2;
                    let miden::Felt { inner: inner6 } = t2_3;
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:basic-wallet/basic-wallet@1.0.0")]
                    extern "C" {
                        #[link_name = "receive-asset"]
                        fn wit_import(_: f32, _: f32, _: f32, _: f32);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: f32, _: f32, _: f32, _: f32) {
                        unreachable!()
                    }
                    wit_import(
                        _rt::as_f32(inner3),
                        _rt::as_f32(inner4),
                        _rt::as_f32(inner5),
                        _rt::as_f32(inner6),
                    );
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn send_asset(
                core_asset: miden::CoreAsset,
                tag: miden::Tag,
                note_type: miden::NoteType,
                recipient: miden::Recipient,
            ) {
                unsafe {
                    let miden::CoreAsset { inner: inner0 } = core_asset;
                    let miden::Word { inner: inner1 } = inner0;
                    let (t2_0, t2_1, t2_2, t2_3) = inner1;
                    let miden::Felt { inner: inner3 } = t2_0;
                    let miden::Felt { inner: inner4 } = t2_1;
                    let miden::Felt { inner: inner5 } = t2_2;
                    let miden::Felt { inner: inner6 } = t2_3;
                    let miden::Tag { inner: inner7 } = tag;
                    let miden::Felt { inner: inner8 } = inner7;
                    let miden::NoteType { inner: inner9 } = note_type;
                    let miden::Felt { inner: inner10 } = inner9;
                    let miden::Recipient { inner: inner11 } = recipient;
                    let miden::Word { inner: inner12 } = inner11;
                    let (t13_0, t13_1, t13_2, t13_3) = inner12;
                    let miden::Felt { inner: inner14 } = t13_0;
                    let miden::Felt { inner: inner15 } = t13_1;
                    let miden::Felt { inner: inner16 } = t13_2;
                    let miden::Felt { inner: inner17 } = t13_3;
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:basic-wallet/basic-wallet@1.0.0")]
                    extern "C" {
                        #[link_name = "send-asset"]
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
                        );
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
                    ) {
                        unreachable!()
                    }
                    wit_import(
                        _rt::as_f32(inner3),
                        _rt::as_f32(inner4),
                        _rt::as_f32(inner5),
                        _rt::as_f32(inner6),
                        _rt::as_f32(inner8),
                        _rt::as_f32(inner10),
                        _rt::as_f32(inner14),
                        _rt::as_f32(inner15),
                        _rt::as_f32(inner16),
                        _rt::as_f32(inner17),
                    );
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn test_felt_intrinsics(a: miden::Felt, b: miden::Felt) -> miden::Felt {
                unsafe {
                    let miden::Felt { inner: inner0 } = a;
                    let miden::Felt { inner: inner1 } = b;
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:basic-wallet/basic-wallet@1.0.0")]
                    extern "C" {
                        #[link_name = "test-felt-intrinsics"]
                        fn wit_import(_: f32, _: f32) -> f32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: f32, _: f32) -> f32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_f32(inner0), _rt::as_f32(inner1));
                    miden::Felt { inner: ret }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn test_stdlib(input: &[u8]) -> _rt::Vec<u8> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let vec0 = input;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:basic-wallet/basic-wallet@1.0.0")]
                    extern "C" {
                        #[link_name = "test-stdlib"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0.cast_mut(), len0, ptr1);
                    let l2 = *ptr1.add(0).cast::<*mut u8>();
                    let l3 = *ptr1.add(4).cast::<usize>();
                    let len4 = l3;
                    _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod core_import {
        #[allow(dead_code, clippy::all)]
        pub mod intrinsics_mem {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
        }
        #[allow(dead_code, clippy::all)]
        pub mod intrinsics_felt {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn eq(a: f32, b: f32) -> bool {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "miden:core-import/intrinsics-felt@1.0.0"
                    )]
                    extern "C" {
                        #[link_name = "eq"]
                        fn wit_import(_: f32, _: f32) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: f32, _: f32) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_f32(&a), _rt::as_f32(&b));
                    _rt::bool_lift(ret as u8)
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod stdlib_crypto_hashes_blake3 {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
        }
        #[allow(dead_code, clippy::all)]
        pub mod account {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            #[allow(unused_unsafe, clippy::all)]
            /// Get the id of the currently executing account
            pub fn get_id() -> f32 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:core-import/account@1.0.0")]
                    extern "C" {
                        #[link_name = "get-id"]
                        fn wit_import() -> f32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> f32 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod note {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            /// Get the inputs of the currently executed note
            pub fn get_inputs(ptr: i32) -> i32 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:core-import/note@1.0.0")]
                    extern "C" {
                        #[link_name = "get-inputs"]
                        fn wit_import(_: i32) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i32(&ptr));
                    ret
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Get the assets of the currently executing note
            pub fn get_assets(ptr: i32) -> i32 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:core-import/note@1.0.0")]
                    extern "C" {
                        #[link_name = "get-assets"]
                        fn wit_import(_: i32) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i32(&ptr));
                    ret
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod tx {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod miden {
        #[allow(dead_code)]
        pub mod base {
            #[allow(dead_code, clippy::all)]
            pub mod note_script {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_note_script_cabi<T: Guest>() {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::note_script();
                }
                pub trait Guest {
                    fn note_script();
                }
                #[doc(hidden)]
                macro_rules! __export_miden_base_note_script_1_0_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "miden:base/note-script@1.0.0#note-script"] unsafe extern "C" fn
                        export_note_script() { $($path_to_types)*::
                        _export_note_script_cabi::<$ty > () } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_miden_base_note_script_1_0_0_cabi;
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
    pub use alloc_crate::vec::Vec;
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
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
    extern crate alloc as alloc_crate;
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
macro_rules! __export_p2id_world_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::miden::base::note_script::__export_miden_base_note_script_1_0_0_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::miden::base::note_script);
    };
}
#[doc(inline)]
pub(crate) use __export_p2id_world_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:miden:p2id@1.0.0:p2id-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1642] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xe9\x0b\x01A\x02\x01\
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
\x1bmiden:base/core-types@1.0.0\x05\0\x02\x03\0\0\x0acore-asset\x02\x03\0\0\x03t\
ag\x02\x03\0\0\x09recipient\x02\x03\0\0\x09note-type\x02\x03\0\0\x04felt\x01B\x13\
\x02\x03\x02\x01\x01\x04\0\x0acore-asset\x03\0\0\x02\x03\x02\x01\x02\x04\0\x03ta\
g\x03\0\x02\x02\x03\x02\x01\x03\x04\0\x09recipient\x03\0\x04\x02\x03\x02\x01\x04\
\x04\0\x09note-type\x03\0\x06\x02\x03\x02\x01\x05\x04\0\x04felt\x03\0\x08\x01@\x01\
\x0acore-asset\x01\x01\0\x04\0\x0dreceive-asset\x01\x0a\x01@\x04\x0acore-asset\x01\
\x03tag\x03\x09note-type\x07\x09recipient\x05\x01\0\x04\0\x0asend-asset\x01\x0b\x01\
@\x02\x01a\x09\x01b\x09\0\x09\x04\0\x14test-felt-intrinsics\x01\x0c\x01p}\x01@\x01\
\x05input\x0d\0\x0d\x04\0\x0btest-stdlib\x01\x0e\x03\x01%miden:basic-wallet/basi\
c-wallet@1.0.0\x05\x06\x01B\x02\x01@\0\0z\x04\0\x09heap-base\x01\0\x03\x01&miden\
:core-import/intrinsics-mem@1.0.0\x05\x07\x01B\x04\x01@\x02\x01av\x01bv\0v\x04\0\
\x03add\x01\0\x01@\x02\x01av\x01bv\0\x7f\x04\0\x02eq\x01\x01\x03\x01'miden:core-\
import/intrinsics-felt@1.0.0\x05\x08\x01B\x02\x01@\x09\x02a0z\x02a1z\x02a2z\x02a\
3z\x02a4z\x02a5z\x02a6z\x02a7z\x0aresult-ptrz\x01\0\x04\0\x0fhash-one-to-one\x01\
\0\x03\x013miden:core-import/stdlib-crypto-hashes-blake3@1.0.0\x05\x09\x01B\x05\x01\
@\x05\x06asset0v\x06asset1v\x06asset2v\x06asset3v\x0aresult-ptrz\x01\0\x04\0\x09\
add-asset\x01\0\x04\0\x0cremove-asset\x01\0\x01@\0\0v\x04\0\x06get-id\x01\x01\x03\
\x01\x1fmiden:core-import/account@1.0.0\x05\x0a\x01B\x03\x01@\x01\x03ptrz\0z\x04\
\0\x0aget-inputs\x01\0\x04\0\x0aget-assets\x01\0\x03\x01\x1cmiden:core-import/no\
te@1.0.0\x05\x0b\x01B\x02\x01@\x0a\x06asset0v\x06asset1v\x06asset2v\x06asset3v\x03\
tagv\x09note-typev\x0arecipient0v\x0arecipient1v\x0arecipient2v\x0arecipient3v\0\
v\x04\0\x0bcreate-note\x01\0\x03\x01\x1amiden:core-import/tx@1.0.0\x05\x0c\x01B\x02\
\x01@\0\x01\0\x04\0\x0bnote-script\x01\0\x04\x01\x1cmiden:base/note-script@1.0.0\
\x05\x0d\x04\x01\x1bmiden:p2id/p2id-world@1.0.0\x04\0\x0b\x10\x01\0\x0ap2id-worl\
d\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.216.0\x10\
wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
