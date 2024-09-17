#[allow(dead_code)]
pub mod miden {
    #[allow(dead_code)]
    pub mod base {
        #[allow(dead_code, clippy::all)]
        pub mod core_types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            /// Represents base field element in the field using Montgomery representation.
            /// Internal values represent x * R mod M where R = 2^64 mod M and x in [0, M).
            /// The backing type is `f64` but the internal values are always integer in the range [0, M).
            /// Field modulus M = 2^64 - 2^32 + 1
            #[repr(C)]
            #[derive(Clone, Copy, PartialEq)]
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
            #[derive(Clone, Copy, PartialEq)]
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
            /// Recipient of the note, i.e., hash(hash(hash(serial_num, [0; 4]), note_script_hash), input_hash)
            #[repr(C)]
            #[derive(Clone, Copy, PartialEq)]
            pub struct Recipient {
                pub inner: Word,
            }
            impl ::core::fmt::Debug for Recipient {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Recipient").field("inner", &self.inner).finish()
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy, PartialEq)]
            pub struct Tag {
                pub inner: Felt,
            }
            impl ::core::fmt::Debug for Tag {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Tag").field("inner", &self.inner).finish()
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
            #[derive(Clone, Copy, PartialEq)]
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
            /// Account nonce
            #[repr(C)]
            #[derive(Clone, Copy, PartialEq)]
            pub struct Nonce {
                pub inner: Felt,
            }
            impl ::core::fmt::Debug for Nonce {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Nonce").field("inner", &self.inner).finish()
                }
            }
            /// Account hash
            #[repr(C)]
            #[derive(Clone, Copy, PartialEq)]
            pub struct AccountHash {
                pub inner: Word,
            }
            impl ::core::fmt::Debug for AccountHash {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("AccountHash").field("inner", &self.inner).finish()
                }
            }
            /// Block hash
            #[repr(C)]
            #[derive(Clone, Copy, PartialEq)]
            pub struct BlockHash {
                pub inner: Word,
            }
            impl ::core::fmt::Debug for BlockHash {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("BlockHash").field("inner", &self.inner).finish()
                }
            }
            /// Storage value
            #[repr(C)]
            #[derive(Clone, Copy, PartialEq)]
            pub struct StorageValue {
                pub inner: Word,
            }
            impl ::core::fmt::Debug for StorageValue {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("StorageValue").field("inner", &self.inner).finish()
                }
            }
            /// Account storage root
            #[repr(C)]
            #[derive(Clone, Copy, PartialEq)]
            pub struct StorageRoot {
                pub inner: Word,
            }
            impl ::core::fmt::Debug for StorageRoot {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("StorageRoot").field("inner", &self.inner).finish()
                }
            }
            /// Account code root
            #[repr(C)]
            #[derive(Clone, Copy, PartialEq)]
            pub struct AccountCodeRoot {
                pub inner: Word,
            }
            impl ::core::fmt::Debug for AccountCodeRoot {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("AccountCodeRoot")
                        .field("inner", &self.inner)
                        .finish()
                }
            }
            /// Commitment to the account vault
            #[repr(C)]
            #[derive(Clone, Copy, PartialEq)]
            pub struct VaultCommitment {
                pub inner: Word,
            }
            impl ::core::fmt::Debug for VaultCommitment {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("VaultCommitment")
                        .field("inner", &self.inner)
                        .finish()
                }
            }
            /// An id of the created note
            #[repr(C)]
            #[derive(Clone, Copy, PartialEq)]
            pub struct NoteId {
                pub inner: Felt,
            }
            impl ::core::fmt::Debug for NoteId {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("NoteId").field("inner", &self.inner).finish()
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Creates a new account ID from a field element.
            pub fn account_id_from_felt(felt: Felt) -> AccountId {
                unsafe {
                    let Felt { inner: inner0 } = felt;
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/core-types@1.0.0")]
                    extern "C" {
                        #[link_name = "account-id-from-felt"]
                        fn wit_import(_: i64) -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64) -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i64(inner0));
                    AccountId {
                        inner: Felt { inner: ret as u64 },
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod tx {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Felt = super::super::super::miden::base::core_types::Felt;
            pub type CoreAsset = super::super::super::miden::base::core_types::CoreAsset;
            pub type Tag = super::super::super::miden::base::core_types::Tag;
            pub type Recipient = super::super::super::miden::base::core_types::Recipient;
            pub type BlockHash = super::super::super::miden::base::core_types::BlockHash;
            pub type Word = super::super::super::miden::base::core_types::Word;
            pub type NoteId = super::super::super::miden::base::core_types::NoteId;
            #[allow(unused_unsafe, clippy::all)]
            /// Returns the block number of the last known block at the time of transaction execution.
            pub fn get_block_number() -> Felt {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/tx@1.0.0")]
                    extern "C" {
                        #[link_name = "get-block-number"]
                        fn wit_import() -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    super::super::super::miden::base::core_types::Felt {
                        inner: ret as u64,
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Returns the block hash of the last known block at the time of transaction execution.
            pub fn get_block_hash() -> BlockHash {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 32]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 32]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/tx@1.0.0")]
                    extern "C" {
                        #[link_name = "get-block-hash"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<i64>();
                    let l2 = *ptr0.add(8).cast::<i64>();
                    let l3 = *ptr0.add(16).cast::<i64>();
                    let l4 = *ptr0.add(24).cast::<i64>();
                    super::super::super::miden::base::core_types::BlockHash {
                        inner: (
                            super::super::super::miden::base::core_types::Felt {
                                inner: l1 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l2 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l3 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l4 as u64,
                            },
                        ),
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Returns the input notes hash. This is computed as a sequential hash of
            /// (nullifier, script_root) tuples over all input notes.
            pub fn get_input_notes_hash() -> Word {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 32]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 32]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/tx@1.0.0")]
                    extern "C" {
                        #[link_name = "get-input-notes-hash"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<i64>();
                    let l2 = *ptr0.add(8).cast::<i64>();
                    let l3 = *ptr0.add(16).cast::<i64>();
                    let l4 = *ptr0.add(24).cast::<i64>();
                    (
                        super::super::super::miden::base::core_types::Felt {
                            inner: l1 as u64,
                        },
                        super::super::super::miden::base::core_types::Felt {
                            inner: l2 as u64,
                        },
                        super::super::super::miden::base::core_types::Felt {
                            inner: l3 as u64,
                        },
                        super::super::super::miden::base::core_types::Felt {
                            inner: l4 as u64,
                        },
                    )
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Returns the output notes hash. This is computed as a sequential hash of
            /// (note_hash, note_metadata) tuples over all output notes.
            pub fn get_output_notes_hash() -> Word {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 32]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 32]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/tx@1.0.0")]
                    extern "C" {
                        #[link_name = "get-output-notes-hash"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<i64>();
                    let l2 = *ptr0.add(8).cast::<i64>();
                    let l3 = *ptr0.add(16).cast::<i64>();
                    let l4 = *ptr0.add(24).cast::<i64>();
                    (
                        super::super::super::miden::base::core_types::Felt {
                            inner: l1 as u64,
                        },
                        super::super::super::miden::base::core_types::Felt {
                            inner: l2 as u64,
                        },
                        super::super::super::miden::base::core_types::Felt {
                            inner: l3 as u64,
                        },
                        super::super::super::miden::base::core_types::Felt {
                            inner: l4 as u64,
                        },
                    )
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Creates a new note.
            /// asset is the asset to be included in the note.
            /// tag is the tag to be included in the note.
            /// recipient is the recipient of the note.
            /// Returns the id of the created note.
            pub fn create_note(
                asset: CoreAsset,
                tag: Tag,
                recipient: Recipient,
            ) -> NoteId {
                unsafe {
                    let super::super::super::miden::base::core_types::CoreAsset {
                        inner: inner0,
                    } = asset;
                    let (t1_0, t1_1, t1_2, t1_3) = inner0;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner2,
                    } = t1_0;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner3,
                    } = t1_1;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner4,
                    } = t1_2;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner5,
                    } = t1_3;
                    let super::super::super::miden::base::core_types::Tag {
                        inner: inner6,
                    } = tag;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner7,
                    } = inner6;
                    let super::super::super::miden::base::core_types::Recipient {
                        inner: inner8,
                    } = recipient;
                    let (t9_0, t9_1, t9_2, t9_3) = inner8;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner10,
                    } = t9_0;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner11,
                    } = t9_1;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner12,
                    } = t9_2;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner13,
                    } = t9_3;
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/tx@1.0.0")]
                    extern "C" {
                        #[link_name = "create-note"]
                        fn wit_import(
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                        ) -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                    ) -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import(
                        _rt::as_i64(inner2),
                        _rt::as_i64(inner3),
                        _rt::as_i64(inner4),
                        _rt::as_i64(inner5),
                        _rt::as_i64(inner7),
                        _rt::as_i64(inner10),
                        _rt::as_i64(inner11),
                        _rt::as_i64(inner12),
                        _rt::as_i64(inner13),
                    );
                    super::super::super::miden::base::core_types::NoteId {
                        inner: super::super::super::miden::base::core_types::Felt {
                            inner: ret as u64,
                        },
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod account {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Felt = super::super::super::miden::base::core_types::Felt;
            pub type CoreAsset = super::super::super::miden::base::core_types::CoreAsset;
            pub type AccountId = super::super::super::miden::base::core_types::AccountId;
            pub type Nonce = super::super::super::miden::base::core_types::Nonce;
            pub type AccountHash = super::super::super::miden::base::core_types::AccountHash;
            pub type StorageValue = super::super::super::miden::base::core_types::StorageValue;
            pub type StorageRoot = super::super::super::miden::base::core_types::StorageRoot;
            pub type AccountCodeRoot = super::super::super::miden::base::core_types::AccountCodeRoot;
            pub type VaultCommitment = super::super::super::miden::base::core_types::VaultCommitment;
            #[allow(unused_unsafe, clippy::all)]
            /// Get the id of the currently executing account
            pub fn get_id() -> AccountId {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/account@1.0.0")]
                    extern "C" {
                        #[link_name = "get-id"]
                        fn wit_import() -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    super::super::super::miden::base::core_types::AccountId {
                        inner: super::super::super::miden::base::core_types::Felt {
                            inner: ret as u64,
                        },
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Return the account nonce
            pub fn get_nonce() -> Nonce {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/account@1.0.0")]
                    extern "C" {
                        #[link_name = "get-nonce"]
                        fn wit_import() -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    super::super::super::miden::base::core_types::Nonce {
                        inner: super::super::super::miden::base::core_types::Felt {
                            inner: ret as u64,
                        },
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Get the initial hash of the currently executing account
            pub fn get_initial_hash() -> AccountHash {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 32]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 32]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/account@1.0.0")]
                    extern "C" {
                        #[link_name = "get-initial-hash"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<i64>();
                    let l2 = *ptr0.add(8).cast::<i64>();
                    let l3 = *ptr0.add(16).cast::<i64>();
                    let l4 = *ptr0.add(24).cast::<i64>();
                    super::super::super::miden::base::core_types::AccountHash {
                        inner: (
                            super::super::super::miden::base::core_types::Felt {
                                inner: l1 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l2 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l3 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l4 as u64,
                            },
                        ),
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Get the current hash of the account data stored in memory
            pub fn get_current_hash() -> AccountHash {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 32]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 32]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/account@1.0.0")]
                    extern "C" {
                        #[link_name = "get-current-hash"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<i64>();
                    let l2 = *ptr0.add(8).cast::<i64>();
                    let l3 = *ptr0.add(16).cast::<i64>();
                    let l4 = *ptr0.add(24).cast::<i64>();
                    super::super::super::miden::base::core_types::AccountHash {
                        inner: (
                            super::super::super::miden::base::core_types::Felt {
                                inner: l1 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l2 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l3 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l4 as u64,
                            },
                        ),
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Increment the account nonce by the specified value.
            /// value can be at most 2^32 - 1 otherwise this procedure panics
            pub fn incr_nonce(value: Felt) {
                unsafe {
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner0,
                    } = value;
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/account@1.0.0")]
                    extern "C" {
                        #[link_name = "incr-nonce"]
                        fn wit_import(_: i64);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64) {
                        unreachable!()
                    }
                    wit_import(_rt::as_i64(inner0));
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Get the value of the specified key in the account storage
            pub fn get_item(index: Felt) -> StorageValue {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 32]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 32]);
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner0,
                    } = index;
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/account@1.0.0")]
                    extern "C" {
                        #[link_name = "get-item"]
                        fn wit_import(_: i64, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(_rt::as_i64(inner0), ptr1);
                    let l2 = *ptr1.add(0).cast::<i64>();
                    let l3 = *ptr1.add(8).cast::<i64>();
                    let l4 = *ptr1.add(16).cast::<i64>();
                    let l5 = *ptr1.add(24).cast::<i64>();
                    super::super::super::miden::base::core_types::StorageValue {
                        inner: (
                            super::super::super::miden::base::core_types::Felt {
                                inner: l2 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l3 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l4 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l5 as u64,
                            },
                        ),
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Set the value of the specified key in the account storage
            /// Returns the old value of the key and the new storage root
            pub fn set_item(
                index: Felt,
                value: StorageValue,
            ) -> (StorageRoot, StorageValue) {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 64]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 64]);
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner0,
                    } = index;
                    let super::super::super::miden::base::core_types::StorageValue {
                        inner: inner1,
                    } = value;
                    let (t2_0, t2_1, t2_2, t2_3) = inner1;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner3,
                    } = t2_0;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner4,
                    } = t2_1;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner5,
                    } = t2_2;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner6,
                    } = t2_3;
                    let ptr7 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/account@1.0.0")]
                    extern "C" {
                        #[link_name = "set-item"]
                        fn wit_import(
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: *mut u8,
                        );
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64, _: i64, _: i64, _: i64, _: i64, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(
                        _rt::as_i64(inner0),
                        _rt::as_i64(inner3),
                        _rt::as_i64(inner4),
                        _rt::as_i64(inner5),
                        _rt::as_i64(inner6),
                        ptr7,
                    );
                    let l8 = *ptr7.add(0).cast::<i64>();
                    let l9 = *ptr7.add(8).cast::<i64>();
                    let l10 = *ptr7.add(16).cast::<i64>();
                    let l11 = *ptr7.add(24).cast::<i64>();
                    let l12 = *ptr7.add(32).cast::<i64>();
                    let l13 = *ptr7.add(40).cast::<i64>();
                    let l14 = *ptr7.add(48).cast::<i64>();
                    let l15 = *ptr7.add(56).cast::<i64>();
                    (
                        super::super::super::miden::base::core_types::StorageRoot {
                            inner: (
                                super::super::super::miden::base::core_types::Felt {
                                    inner: l8 as u64,
                                },
                                super::super::super::miden::base::core_types::Felt {
                                    inner: l9 as u64,
                                },
                                super::super::super::miden::base::core_types::Felt {
                                    inner: l10 as u64,
                                },
                                super::super::super::miden::base::core_types::Felt {
                                    inner: l11 as u64,
                                },
                            ),
                        },
                        super::super::super::miden::base::core_types::StorageValue {
                            inner: (
                                super::super::super::miden::base::core_types::Felt {
                                    inner: l12 as u64,
                                },
                                super::super::super::miden::base::core_types::Felt {
                                    inner: l13 as u64,
                                },
                                super::super::super::miden::base::core_types::Felt {
                                    inner: l14 as u64,
                                },
                                super::super::super::miden::base::core_types::Felt {
                                    inner: l15 as u64,
                                },
                            ),
                        },
                    )
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Sets the code of the account the transaction is being executed against.
            /// This procedure can only be executed on regular accounts with updatable
            /// code. Otherwise, this procedure fails. code is the hash of the code
            /// to set.
            pub fn set_code(code_root: AccountCodeRoot) {
                unsafe {
                    let super::super::super::miden::base::core_types::AccountCodeRoot {
                        inner: inner0,
                    } = code_root;
                    let (t1_0, t1_1, t1_2, t1_3) = inner0;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner2,
                    } = t1_0;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner3,
                    } = t1_1;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner4,
                    } = t1_2;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner5,
                    } = t1_3;
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/account@1.0.0")]
                    extern "C" {
                        #[link_name = "set-code"]
                        fn wit_import(_: i64, _: i64, _: i64, _: i64);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64, _: i64, _: i64, _: i64) {
                        unreachable!()
                    }
                    wit_import(
                        _rt::as_i64(inner2),
                        _rt::as_i64(inner3),
                        _rt::as_i64(inner4),
                        _rt::as_i64(inner5),
                    );
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Returns the balance of a fungible asset associated with a account_id.
            /// Panics if the asset is not a fungible asset. account_id is the faucet id
            /// of the fungible asset of interest. balance is the vault balance of the
            /// fungible asset.
            pub fn get_balance(account_id: AccountId) -> Felt {
                unsafe {
                    let super::super::super::miden::base::core_types::AccountId {
                        inner: inner0,
                    } = account_id;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner1,
                    } = inner0;
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/account@1.0.0")]
                    extern "C" {
                        #[link_name = "get-balance"]
                        fn wit_import(_: i64) -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64) -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i64(inner1));
                    super::super::super::miden::base::core_types::Felt {
                        inner: ret as u64,
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Returns a boolean indicating whether the non-fungible asset is present
            /// in the vault. Panics if the asset is a fungible asset. asset is the
            /// non-fungible asset of interest. has_asset is a boolean indicating
            /// whether the account vault has the asset of interest.
            pub fn has_non_fungible_asset(asset: CoreAsset) -> bool {
                unsafe {
                    let super::super::super::miden::base::core_types::CoreAsset {
                        inner: inner0,
                    } = asset;
                    let (t1_0, t1_1, t1_2, t1_3) = inner0;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner2,
                    } = t1_0;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner3,
                    } = t1_1;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner4,
                    } = t1_2;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner5,
                    } = t1_3;
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/account@1.0.0")]
                    extern "C" {
                        #[link_name = "has-non-fungible-asset"]
                        fn wit_import(_: i64, _: i64, _: i64, _: i64) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64, _: i64, _: i64, _: i64) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(
                        _rt::as_i64(inner2),
                        _rt::as_i64(inner3),
                        _rt::as_i64(inner4),
                        _rt::as_i64(inner5),
                    );
                    _rt::bool_lift(ret as u8)
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Add the specified asset to the vault. Panics under various conditions.
            /// Returns the final asset in the account vault defined as follows: If asset is
            /// a non-fungible asset, then returns the same as asset. If asset is a
            /// fungible asset, then returns the total fungible asset in the account
            /// vault after asset was added to it.
            pub fn add_asset(asset: CoreAsset) -> CoreAsset {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 32]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 32]);
                    let super::super::super::miden::base::core_types::CoreAsset {
                        inner: inner0,
                    } = asset;
                    let (t1_0, t1_1, t1_2, t1_3) = inner0;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner2,
                    } = t1_0;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner3,
                    } = t1_1;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner4,
                    } = t1_2;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner5,
                    } = t1_3;
                    let ptr6 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/account@1.0.0")]
                    extern "C" {
                        #[link_name = "add-asset"]
                        fn wit_import(_: i64, _: i64, _: i64, _: i64, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64, _: i64, _: i64, _: i64, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(
                        _rt::as_i64(inner2),
                        _rt::as_i64(inner3),
                        _rt::as_i64(inner4),
                        _rt::as_i64(inner5),
                        ptr6,
                    );
                    let l7 = *ptr6.add(0).cast::<i64>();
                    let l8 = *ptr6.add(8).cast::<i64>();
                    let l9 = *ptr6.add(16).cast::<i64>();
                    let l10 = *ptr6.add(24).cast::<i64>();
                    super::super::super::miden::base::core_types::CoreAsset {
                        inner: (
                            super::super::super::miden::base::core_types::Felt {
                                inner: l7 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l8 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l9 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l10 as u64,
                            },
                        ),
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Remove the specified asset from the vault
            pub fn remove_asset(asset: CoreAsset) -> CoreAsset {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 32]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 32]);
                    let super::super::super::miden::base::core_types::CoreAsset {
                        inner: inner0,
                    } = asset;
                    let (t1_0, t1_1, t1_2, t1_3) = inner0;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner2,
                    } = t1_0;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner3,
                    } = t1_1;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner4,
                    } = t1_2;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner5,
                    } = t1_3;
                    let ptr6 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/account@1.0.0")]
                    extern "C" {
                        #[link_name = "remove-asset"]
                        fn wit_import(_: i64, _: i64, _: i64, _: i64, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64, _: i64, _: i64, _: i64, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(
                        _rt::as_i64(inner2),
                        _rt::as_i64(inner3),
                        _rt::as_i64(inner4),
                        _rt::as_i64(inner5),
                        ptr6,
                    );
                    let l7 = *ptr6.add(0).cast::<i64>();
                    let l8 = *ptr6.add(8).cast::<i64>();
                    let l9 = *ptr6.add(16).cast::<i64>();
                    let l10 = *ptr6.add(24).cast::<i64>();
                    super::super::super::miden::base::core_types::CoreAsset {
                        inner: (
                            super::super::super::miden::base::core_types::Felt {
                                inner: l7 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l8 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l9 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l10 as u64,
                            },
                        ),
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Returns the commitment to the account vault.
            pub fn get_vault_commitment() -> VaultCommitment {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 32]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 32]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/account@1.0.0")]
                    extern "C" {
                        #[link_name = "get-vault-commitment"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<i64>();
                    let l2 = *ptr0.add(8).cast::<i64>();
                    let l3 = *ptr0.add(16).cast::<i64>();
                    let l4 = *ptr0.add(24).cast::<i64>();
                    super::super::super::miden::base::core_types::VaultCommitment {
                        inner: (
                            super::super::super::miden::base::core_types::Felt {
                                inner: l1 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l2 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l3 as u64,
                            },
                            super::super::super::miden::base::core_types::Felt {
                                inner: l4 as u64,
                            },
                        ),
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod note {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Felt = super::super::super::miden::base::core_types::Felt;
            pub type CoreAsset = super::super::super::miden::base::core_types::CoreAsset;
            pub type AccountId = super::super::super::miden::base::core_types::AccountId;
            #[allow(unused_unsafe, clippy::all)]
            /// Get the inputs of the currently executed note
            pub fn get_inputs() -> _rt::Vec<Felt> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/note@1.0.0")]
                    extern "C" {
                        #[link_name = "get-inputs"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let len3 = l2;
                    _rt::Vec::from_raw_parts(l1.cast(), len3, len3)
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Get the assets of the currently executing note
            pub fn get_assets() -> _rt::Vec<CoreAsset> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/note@1.0.0")]
                    extern "C" {
                        #[link_name = "get-assets"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let len3 = l2;
                    _rt::Vec::from_raw_parts(l1.cast(), len3, len3)
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Get the sender of the currently executing note
            pub fn get_sender() -> AccountId {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:base/note@1.0.0")]
                    extern "C" {
                        #[link_name = "get-sender"]
                        fn wit_import() -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    super::super::super::miden::base::core_types::AccountId {
                        inner: super::super::super::miden::base::core_types::Felt {
                            inner: ret as u64,
                        },
                    }
                }
            }
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
            pub type CoreAsset = super::super::super::miden::base::core_types::CoreAsset;
            pub type Tag = super::super::super::miden::base::core_types::Tag;
            pub type Recipient = super::super::super::miden::base::core_types::Recipient;
            #[allow(unused_unsafe, clippy::all)]
            pub fn receive_asset(core_asset: CoreAsset) {
                unsafe {
                    let super::super::super::miden::base::core_types::CoreAsset {
                        inner: inner0,
                    } = core_asset;
                    let (t1_0, t1_1, t1_2, t1_3) = inner0;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner2,
                    } = t1_0;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner3,
                    } = t1_1;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner4,
                    } = t1_2;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner5,
                    } = t1_3;
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:basic-wallet/basic-wallet@1.0.0")]
                    extern "C" {
                        #[link_name = "receive-asset"]
                        fn wit_import(_: i64, _: i64, _: i64, _: i64);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64, _: i64, _: i64, _: i64) {
                        unreachable!()
                    }
                    wit_import(
                        _rt::as_i64(inner2),
                        _rt::as_i64(inner3),
                        _rt::as_i64(inner4),
                        _rt::as_i64(inner5),
                    );
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn send_asset(core_asset: CoreAsset, tag: Tag, recipient: Recipient) {
                unsafe {
                    let super::super::super::miden::base::core_types::CoreAsset {
                        inner: inner0,
                    } = core_asset;
                    let (t1_0, t1_1, t1_2, t1_3) = inner0;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner2,
                    } = t1_0;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner3,
                    } = t1_1;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner4,
                    } = t1_2;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner5,
                    } = t1_3;
                    let super::super::super::miden::base::core_types::Tag {
                        inner: inner6,
                    } = tag;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner7,
                    } = inner6;
                    let super::super::super::miden::base::core_types::Recipient {
                        inner: inner8,
                    } = recipient;
                    let (t9_0, t9_1, t9_2, t9_3) = inner8;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner10,
                    } = t9_0;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner11,
                    } = t9_1;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner12,
                    } = t9_2;
                    let super::super::super::miden::base::core_types::Felt {
                        inner: inner13,
                    } = t9_3;
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:basic-wallet/basic-wallet@1.0.0")]
                    extern "C" {
                        #[link_name = "send-asset"]
                        fn wit_import(
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                            _: i64,
                        );
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                        _: i64,
                    ) {
                        unreachable!()
                    }
                    wit_import(
                        _rt::as_i64(inner2),
                        _rt::as_i64(inner3),
                        _rt::as_i64(inner4),
                        _rt::as_i64(inner5),
                        _rt::as_i64(inner7),
                        _rt::as_i64(inner10),
                        _rt::as_i64(inner11),
                        _rt::as_i64(inner12),
                        _rt::as_i64(inner13),
                    );
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
    pub use alloc_crate::vec::Vec;
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
macro_rules! __export_notes_world_impl {
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
pub(crate) use __export_notes_world_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.30.0:notes-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 2434] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x80\x12\x01A\x02\x01\
A\x1a\x01B\x1e\x01r\x01\x05innerw\x04\0\x04felt\x03\0\0\x01o\x04\x01\x01\x01\x01\
\x04\0\x04word\x03\0\x02\x01r\x01\x05inner\x01\x04\0\x0aaccount-id\x03\0\x04\x01\
r\x01\x05inner\x03\x04\0\x09recipient\x03\0\x06\x01r\x01\x05inner\x01\x04\0\x03t\
ag\x03\0\x08\x01r\x01\x05inner\x03\x04\0\x0acore-asset\x03\0\x0a\x01r\x01\x05inn\
er\x01\x04\0\x05nonce\x03\0\x0c\x01r\x01\x05inner\x03\x04\0\x0caccount-hash\x03\0\
\x0e\x01r\x01\x05inner\x03\x04\0\x0ablock-hash\x03\0\x10\x01r\x01\x05inner\x03\x04\
\0\x0dstorage-value\x03\0\x12\x01r\x01\x05inner\x03\x04\0\x0cstorage-root\x03\0\x14\
\x01r\x01\x05inner\x03\x04\0\x11account-code-root\x03\0\x16\x01r\x01\x05inner\x03\
\x04\0\x10vault-commitment\x03\0\x18\x01r\x01\x05inner\x01\x04\0\x07note-id\x03\0\
\x1a\x01@\x01\x04felt\x01\0\x05\x04\0\x14account-id-from-felt\x01\x1c\x03\x01\x1b\
miden:base/core-types@1.0.0\x05\0\x02\x03\0\0\x04felt\x02\x03\0\0\x0acore-asset\x02\
\x03\0\0\x03tag\x02\x03\0\0\x09recipient\x02\x03\0\0\x0aaccount-id\x02\x03\0\0\x05\
nonce\x02\x03\0\0\x0caccount-hash\x02\x03\0\0\x0dstorage-value\x02\x03\0\0\x0cst\
orage-root\x02\x03\0\0\x11account-code-root\x02\x03\0\0\x10vault-commitment\x02\x03\
\0\0\x0ablock-hash\x02\x03\0\0\x04word\x02\x03\0\0\x07note-id\x01B%\x02\x03\x02\x01\
\x01\x04\0\x04felt\x03\0\0\x02\x03\x02\x01\x02\x04\0\x0acore-asset\x03\0\x02\x02\
\x03\x02\x01\x03\x04\0\x03tag\x03\0\x04\x02\x03\x02\x01\x04\x04\0\x09recipient\x03\
\0\x06\x02\x03\x02\x01\x05\x04\0\x0aaccount-id\x03\0\x08\x02\x03\x02\x01\x06\x04\
\0\x05nonce\x03\0\x0a\x02\x03\x02\x01\x07\x04\0\x0caccount-hash\x03\0\x0c\x02\x03\
\x02\x01\x08\x04\0\x0dstorage-value\x03\0\x0e\x02\x03\x02\x01\x09\x04\0\x0cstora\
ge-root\x03\0\x10\x02\x03\x02\x01\x0a\x04\0\x11account-code-root\x03\0\x12\x02\x03\
\x02\x01\x0b\x04\0\x10vault-commitment\x03\0\x14\x02\x03\x02\x01\x0c\x04\0\x0abl\
ock-hash\x03\0\x16\x02\x03\x02\x01\x0d\x04\0\x04word\x03\0\x18\x02\x03\x02\x01\x0e\
\x04\0\x07note-id\x03\0\x1a\x01@\0\0\x01\x04\0\x10get-block-number\x01\x1c\x01@\0\
\0\x17\x04\0\x0eget-block-hash\x01\x1d\x01@\0\0\x19\x04\0\x14get-input-notes-has\
h\x01\x1e\x04\0\x15get-output-notes-hash\x01\x1e\x01@\x03\x05asset\x03\x03tag\x05\
\x09recipient\x07\0\x1b\x04\0\x0bcreate-note\x01\x1f\x03\x01\x13miden:base/tx@1.\
0.0\x05\x0f\x01B/\x02\x03\x02\x01\x01\x04\0\x04felt\x03\0\0\x02\x03\x02\x01\x02\x04\
\0\x0acore-asset\x03\0\x02\x02\x03\x02\x01\x03\x04\0\x03tag\x03\0\x04\x02\x03\x02\
\x01\x04\x04\0\x09recipient\x03\0\x06\x02\x03\x02\x01\x05\x04\0\x0aaccount-id\x03\
\0\x08\x02\x03\x02\x01\x06\x04\0\x05nonce\x03\0\x0a\x02\x03\x02\x01\x07\x04\0\x0c\
account-hash\x03\0\x0c\x02\x03\x02\x01\x08\x04\0\x0dstorage-value\x03\0\x0e\x02\x03\
\x02\x01\x09\x04\0\x0cstorage-root\x03\0\x10\x02\x03\x02\x01\x0a\x04\0\x11accoun\
t-code-root\x03\0\x12\x02\x03\x02\x01\x0b\x04\0\x10vault-commitment\x03\0\x14\x01\
@\0\0\x09\x04\0\x06get-id\x01\x16\x01@\0\0\x0b\x04\0\x09get-nonce\x01\x17\x01@\0\
\0\x0d\x04\0\x10get-initial-hash\x01\x18\x04\0\x10get-current-hash\x01\x18\x01@\x01\
\x05value\x01\x01\0\x04\0\x0aincr-nonce\x01\x19\x01@\x01\x05index\x01\0\x0f\x04\0\
\x08get-item\x01\x1a\x01o\x02\x11\x0f\x01@\x02\x05index\x01\x05value\x0f\0\x1b\x04\
\0\x08set-item\x01\x1c\x01@\x01\x09code-root\x13\x01\0\x04\0\x08set-code\x01\x1d\
\x01@\x01\x0aaccount-id\x09\0\x01\x04\0\x0bget-balance\x01\x1e\x01@\x01\x05asset\
\x03\0\x7f\x04\0\x16has-non-fungible-asset\x01\x1f\x01@\x01\x05asset\x03\0\x03\x04\
\0\x09add-asset\x01\x20\x04\0\x0cremove-asset\x01\x20\x01@\0\0\x15\x04\0\x14get-\
vault-commitment\x01!\x03\x01\x18miden:base/account@1.0.0\x05\x10\x01B\x1e\x02\x03\
\x02\x01\x01\x04\0\x04felt\x03\0\0\x02\x03\x02\x01\x02\x04\0\x0acore-asset\x03\0\
\x02\x02\x03\x02\x01\x03\x04\0\x03tag\x03\0\x04\x02\x03\x02\x01\x04\x04\0\x09rec\
ipient\x03\0\x06\x02\x03\x02\x01\x05\x04\0\x0aaccount-id\x03\0\x08\x02\x03\x02\x01\
\x06\x04\0\x05nonce\x03\0\x0a\x02\x03\x02\x01\x07\x04\0\x0caccount-hash\x03\0\x0c\
\x02\x03\x02\x01\x08\x04\0\x0dstorage-value\x03\0\x0e\x02\x03\x02\x01\x09\x04\0\x0c\
storage-root\x03\0\x10\x02\x03\x02\x01\x0a\x04\0\x11account-code-root\x03\0\x12\x02\
\x03\x02\x01\x0b\x04\0\x10vault-commitment\x03\0\x14\x01p\x01\x01@\0\0\x16\x04\0\
\x0aget-inputs\x01\x17\x01p\x03\x01@\0\0\x18\x04\0\x0aget-assets\x01\x19\x01@\0\0\
\x09\x04\0\x0aget-sender\x01\x1a\x03\x01\x15miden:base/note@1.0.0\x05\x11\x01B\x0a\
\x02\x03\x02\x01\x02\x04\0\x0acore-asset\x03\0\0\x02\x03\x02\x01\x03\x04\0\x03ta\
g\x03\0\x02\x02\x03\x02\x01\x04\x04\0\x09recipient\x03\0\x04\x01@\x01\x0acore-as\
set\x01\x01\0\x04\0\x0dreceive-asset\x01\x06\x01@\x03\x0acore-asset\x01\x03tag\x03\
\x09recipient\x05\x01\0\x04\0\x0asend-asset\x01\x07\x03\x01%miden:basic-wallet/b\
asic-wallet@1.0.0\x05\x12\x01B\x02\x01@\0\x01\0\x04\0\x0bnote-script\x01\0\x04\x01\
\x1cmiden:base/note-script@1.0.0\x05\x13\x04\x01\x1cmiden:p2id/notes-world@1.0.0\
\x04\0\x0b\x11\x01\0\x0bnotes-world\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\
\x0dwit-component\x070.215.0\x10wit-bindgen-rust\x060.30.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
