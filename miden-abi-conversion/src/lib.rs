//! Function types and lowering for Miden ABI functions (tx kernel, stdlib,
//! etc.) that do not conform to the Wasm Canonical ABI.

// Coding conventions
#![deny(warnings)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod adapter;
pub mod tx_kernel;
