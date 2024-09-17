#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod miden {
        #[allow(dead_code)]
        pub mod base {
            #[allow(dead_code, clippy::all)]
            pub mod core_types {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                /// Represents base field element in the field using Montgomery representation.
                /// Internal values represent x * R mod M where R = 2^64 mod M and x in [0, M).
                /// The backing type is `f64` but the internal values are always integer in the range [0, M).
                /// Field modulus M = 2^64 - 2^32 + 1
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct Felt {
                    /// We plan to use f64 as the backing type for the field element. It has the size that we need and
                    /// we don't plan to support floating point arithmetic in programs for Miden VM.
                    ///
                    /// For now its u64
                    pub inner: u64,
                }
                impl ::core::fmt::Debug for Felt {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Felt").field("inner", &self.inner).finish()
                    }
                }
                /// A group of four field elements in the Miden base field.
                pub type Word = (Felt, Felt, Felt, Felt);
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
                    pub inner: Felt,
                }
                impl ::core::fmt::Debug for AccountId {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("AccountId").field("inner", &self.inner).finish()
                    }
                }
                /// A fungible or a non-fungible asset.
                ///
                /// All assets are encoded using a single word (4 elements) such that it is easy to determine the
                /// type of an asset both inside and outside Miden VM. Specifically:
                /// Element 1 will be:
                /// - ZERO for a fungible asset
                /// - non-ZERO for a non-fungible asset
                /// The most significant bit will be:
                /// - ONE for a fungible asset
                /// - ZERO for a non-fungible asset
                ///
                /// The above properties guarantee that there can never be a collision between a fungible and a
                /// non-fungible asset.
                ///
                /// The methodology for constructing fungible and non-fungible assets is described below.
                ///
                /// # Fungible assets
                /// The most significant element of a fungible asset is set to the ID of the faucet which issued
                /// the asset. This guarantees the properties described above (the first bit is ONE).
                ///
                /// The least significant element is set to the amount of the asset. This amount cannot be greater
                /// than 2^63 - 1 and thus requires 63-bits to store.
                ///
                /// Elements 1 and 2 are set to ZERO.
                ///
                /// It is impossible to find a collision between two fungible assets issued by different faucets as
                /// the faucet_id is included in the description of the asset and this is guaranteed to be different
                /// for each faucet as per the faucet creation logic.
                ///
                /// # Non-fungible assets
                /// The 4 elements of non-fungible assets are computed as follows:
                /// - First the asset data is hashed. This compresses an asset of an arbitrary length to 4 field
                /// elements: [d0, d1, d2, d3].
                /// - d1 is then replaced with the faucet_id which issues the asset: [d0, faucet_id, d2, d3].
                /// - Lastly, the most significant bit of d3 is set to ZERO.
                ///
                /// It is impossible to find a collision between two non-fungible assets issued by different faucets
                /// as the faucet_id is included in the description of the non-fungible asset and this is guaranteed
                /// to be different as per the faucet creation logic. Collision resistance for non-fungible assets
                /// issued by the same faucet is ~2^95.
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct CoreAsset {
                    pub inner: Word,
                }
                impl ::core::fmt::Debug for CoreAsset {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("CoreAsset").field("inner", &self.inner).finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_account_id_from_felt_cabi<T: Guest>(
                    arg0: i64,
                ) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::account_id_from_felt(Felt { inner: arg0 as u64 });
                    let AccountId { inner: inner1 } = result0;
                    let Felt { inner: inner2 } = inner1;
                    _rt::as_i64(inner2)
                }
                pub trait Guest {
                    /// Creates a new account ID from a field element.
                    fn account_id_from_felt(felt: Felt) -> AccountId;
                }
                #[doc(hidden)]
                macro_rules! __export_miden_base_core_types_1_0_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "miden:base/core-types@1.0.0#account-id-from-felt"] unsafe extern
                        "C" fn export_account_id_from_felt(arg0 : i64,) -> i64 {
                        $($path_to_types)*:: _export_account_id_from_felt_cabi::<$ty >
                        (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_miden_base_core_types_1_0_0_cabi;
            }
            #[allow(dead_code, clippy::all)]
            pub mod types {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type AccountId = super::super::super::super::exports::miden::base::core_types::AccountId;
                pub type Word = super::super::super::super::exports::miden::base::core_types::Word;
                pub type CoreAsset = super::super::super::super::exports::miden::base::core_types::CoreAsset;
                /// A fungible asset
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct FungibleAsset {
                    /// Faucet ID of the faucet which issued the asset as well as the asset amount.
                    pub asset: AccountId,
                    /// Asset amount is guaranteed to be 2^63 - 1 or smaller.
                    pub amount: u64,
                }
                impl ::core::fmt::Debug for FungibleAsset {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("FungibleAsset")
                            .field("asset", &self.asset)
                            .field("amount", &self.amount)
                            .finish()
                    }
                }
                /// A commitment to a non-fungible asset.
                ///
                /// A non-fungible asset consists of 4 field elements which are computed by hashing asset data
                /// (which can be of arbitrary length) to produce: [d0, d1, d2, d3].  We then replace d1 with the
                /// faucet_id that issued the asset: [d0, faucet_id, d2, d3]. We then set the most significant bit
                /// of the most significant element to ZERO.
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct NonFungibleAsset {
                    pub inner: Word,
                }
                impl ::core::fmt::Debug for NonFungibleAsset {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("NonFungibleAsset")
                            .field("inner", &self.inner)
                            .finish()
                    }
                }
                /// A fungible or a non-fungible asset.
                #[derive(Clone, Copy)]
                pub enum Asset {
                    Fungible(FungibleAsset),
                    NonFungible(NonFungibleAsset),
                }
                impl ::core::fmt::Debug for Asset {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            Asset::Fungible(e) => {
                                f.debug_tuple("Asset::Fungible").field(e).finish()
                            }
                            Asset::NonFungible(e) => {
                                f.debug_tuple("Asset::NonFungible").field(e).finish()
                            }
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_from_core_asset_cabi<T: Guest>(
                    arg0: i64,
                    arg1: i64,
                    arg2: i64,
                    arg3: i64,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::from_core_asset(super::super::super::super::exports::miden::base::core_types::CoreAsset {
                        inner: (
                            super::super::super::super::exports::miden::base::core_types::Felt {
                                inner: arg0 as u64,
                            },
                            super::super::super::super::exports::miden::base::core_types::Felt {
                                inner: arg1 as u64,
                            },
                            super::super::super::super::exports::miden::base::core_types::Felt {
                                inner: arg2 as u64,
                            },
                            super::super::super::super::exports::miden::base::core_types::Felt {
                                inner: arg3 as u64,
                            },
                        ),
                    });
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Asset::Fungible(e) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                            let FungibleAsset { asset: asset2, amount: amount2 } = e;
                            let super::super::super::super::exports::miden::base::core_types::AccountId {
                                inner: inner3,
                            } = asset2;
                            let super::super::super::super::exports::miden::base::core_types::Felt {
                                inner: inner4,
                            } = inner3;
                            *ptr1.add(8).cast::<i64>() = _rt::as_i64(inner4);
                            *ptr1.add(16).cast::<i64>() = _rt::as_i64(amount2);
                        }
                        Asset::NonFungible(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let NonFungibleAsset { inner: inner5 } = e;
                            let (t6_0, t6_1, t6_2, t6_3) = inner5;
                            let super::super::super::super::exports::miden::base::core_types::Felt {
                                inner: inner7,
                            } = t6_0;
                            *ptr1.add(8).cast::<i64>() = _rt::as_i64(inner7);
                            let super::super::super::super::exports::miden::base::core_types::Felt {
                                inner: inner8,
                            } = t6_1;
                            *ptr1.add(16).cast::<i64>() = _rt::as_i64(inner8);
                            let super::super::super::super::exports::miden::base::core_types::Felt {
                                inner: inner9,
                            } = t6_2;
                            *ptr1.add(24).cast::<i64>() = _rt::as_i64(inner9);
                            let super::super::super::super::exports::miden::base::core_types::Felt {
                                inner: inner10,
                            } = t6_3;
                            *ptr1.add(32).cast::<i64>() = _rt::as_i64(inner10);
                        }
                    }
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_to_core_asset_cabi<T: Guest>(
                    arg0: i32,
                    arg1: i64,
                    arg2: i64,
                    arg3: i64,
                    arg4: i64,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let v0 = match arg0 {
                        0 => {
                            let e0 = FungibleAsset {
                                asset: super::super::super::super::exports::miden::base::core_types::AccountId {
                                    inner: super::super::super::super::exports::miden::base::core_types::Felt {
                                        inner: arg1 as u64,
                                    },
                                },
                                amount: arg2 as u64,
                            };
                            Asset::Fungible(e0)
                        }
                        n => {
                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                            let e0 = NonFungibleAsset {
                                inner: (
                                    super::super::super::super::exports::miden::base::core_types::Felt {
                                        inner: arg1 as u64,
                                    },
                                    super::super::super::super::exports::miden::base::core_types::Felt {
                                        inner: arg2 as u64,
                                    },
                                    super::super::super::super::exports::miden::base::core_types::Felt {
                                        inner: arg3 as u64,
                                    },
                                    super::super::super::super::exports::miden::base::core_types::Felt {
                                        inner: arg4 as u64,
                                    },
                                ),
                            };
                            Asset::NonFungible(e0)
                        }
                    };
                    let result1 = T::to_core_asset(v0);
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let super::super::super::super::exports::miden::base::core_types::CoreAsset {
                        inner: inner3,
                    } = result1;
                    let (t4_0, t4_1, t4_2, t4_3) = inner3;
                    let super::super::super::super::exports::miden::base::core_types::Felt {
                        inner: inner5,
                    } = t4_0;
                    *ptr2.add(0).cast::<i64>() = _rt::as_i64(inner5);
                    let super::super::super::super::exports::miden::base::core_types::Felt {
                        inner: inner6,
                    } = t4_1;
                    *ptr2.add(8).cast::<i64>() = _rt::as_i64(inner6);
                    let super::super::super::super::exports::miden::base::core_types::Felt {
                        inner: inner7,
                    } = t4_2;
                    *ptr2.add(16).cast::<i64>() = _rt::as_i64(inner7);
                    let super::super::super::super::exports::miden::base::core_types::Felt {
                        inner: inner8,
                    } = t4_3;
                    *ptr2.add(24).cast::<i64>() = _rt::as_i64(inner8);
                    ptr2
                }
                pub trait Guest {
                    /// Converts a core asset to a an asset representation.
                    fn from_core_asset(core_asset: CoreAsset) -> Asset;
                    /// Converts an asset to a core asset representation.
                    fn to_core_asset(asset: Asset) -> CoreAsset;
                }
                #[doc(hidden)]
                macro_rules! __export_miden_base_types_1_0_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "miden:base/types@1.0.0#from-core-asset"] unsafe extern "C" fn
                        export_from_core_asset(arg0 : i64, arg1 : i64, arg2 : i64, arg3 :
                        i64,) -> * mut u8 { $($path_to_types)*::
                        _export_from_core_asset_cabi::<$ty > (arg0, arg1, arg2, arg3) }
                        #[export_name = "miden:base/types@1.0.0#to-core-asset"] unsafe
                        extern "C" fn export_to_core_asset(arg0 : i32, arg1 : i64, arg2 :
                        i64, arg3 : i64, arg4 : i64,) -> * mut u8 { $($path_to_types)*::
                        _export_to_core_asset_cabi::<$ty > (arg0, arg1, arg2, arg3, arg4)
                        } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_miden_base_types_1_0_0_cabi;
                #[repr(align(8))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 40]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 40],
                );
            }
        }
    }
}
mod _rt {
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub fn as_i64<T: AsI64>(t: T) -> i64 {
        t.as_i64()
    }
    pub trait AsI64 {
        fn as_i64(self) -> i64;
    }
    impl<'a, T: Copy + AsI64> AsI64 for &'a T {
        fn as_i64(self) -> i64 {
            (*self).as_i64()
        }
    }
    impl AsI64 for i64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    impl AsI64 for u64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
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
macro_rules! __export_base_world_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::miden::base::core_types::__export_miden_base_core_types_1_0_0_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::miden::base::core_types);
        $($path_to_types_root)*::
        exports::miden::base::types::__export_miden_base_types_1_0_0_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::miden::base::types);
    };
}
#[doc(inline)]
pub(crate) use __export_base_world_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.30.0:base-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 922] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x99\x06\x01A\x02\x01\
A\x08\x01B\x1e\x01r\x01\x05innerw\x04\0\x04felt\x03\0\0\x01o\x04\x01\x01\x01\x01\
\x04\0\x04word\x03\0\x02\x01r\x01\x05inner\x01\x04\0\x0aaccount-id\x03\0\x04\x01\
r\x01\x05inner\x03\x04\0\x09recipient\x03\0\x06\x01r\x01\x05inner\x01\x04\0\x03t\
ag\x03\0\x08\x01r\x01\x05inner\x03\x04\0\x0acore-asset\x03\0\x0a\x01r\x01\x05inn\
er\x01\x04\0\x05nonce\x03\0\x0c\x01r\x01\x05inner\x03\x04\0\x0caccount-hash\x03\0\
\x0e\x01r\x01\x05inner\x03\x04\0\x0ablock-hash\x03\0\x10\x01r\x01\x05inner\x03\x04\
\0\x0dstorage-value\x03\0\x12\x01r\x01\x05inner\x03\x04\0\x0cstorage-root\x03\0\x14\
\x01r\x01\x05inner\x03\x04\0\x11account-code-root\x03\0\x16\x01r\x01\x05inner\x03\
\x04\0\x10vault-commitment\x03\0\x18\x01r\x01\x05inner\x01\x04\0\x07note-id\x03\0\
\x1a\x01@\x01\x04felt\x01\0\x05\x04\0\x14account-id-from-felt\x01\x1c\x04\x01\x1b\
miden:base/core-types@1.0.0\x05\0\x02\x03\0\0\x04felt\x02\x03\0\0\x0aaccount-id\x02\
\x03\0\0\x04word\x02\x03\0\0\x0acore-asset\x01B\x12\x02\x03\x02\x01\x01\x04\0\x04\
felt\x03\0\0\x02\x03\x02\x01\x02\x04\0\x0aaccount-id\x03\0\x02\x02\x03\x02\x01\x03\
\x04\0\x04word\x03\0\x04\x02\x03\x02\x01\x04\x04\0\x0acore-asset\x03\0\x06\x01r\x02\
\x05asset\x03\x06amountw\x04\0\x0efungible-asset\x03\0\x08\x01r\x01\x05inner\x05\
\x04\0\x12non-fungible-asset\x03\0\x0a\x01q\x02\x08fungible\x01\x09\0\x0cnon-fun\
gible\x01\x0b\0\x04\0\x05asset\x03\0\x0c\x01@\x01\x0acore-asset\x07\0\x0d\x04\0\x0f\
from-core-asset\x01\x0e\x01@\x01\x05asset\x0d\0\x07\x04\0\x0dto-core-asset\x01\x0f\
\x04\x01\x16miden:base/types@1.0.0\x05\x05\x04\x01\x1bmiden:base/base-world@1.0.\
0\x04\0\x0b\x10\x01\0\x0abase-world\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\
\x0dwit-component\x070.215.0\x10wit-bindgen-rust\x060.30.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
