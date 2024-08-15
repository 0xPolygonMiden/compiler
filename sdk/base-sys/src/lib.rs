// Enable no_std for the bindings module
#![cfg_attr(feature = "bindings", no_std)]

#[cfg(feature = "bindings")]
pub mod bindings;
#[cfg(feature = "masl-lib")]
pub mod masl;
