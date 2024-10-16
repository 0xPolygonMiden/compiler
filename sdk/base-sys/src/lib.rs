// Enable no_std for the bindings module
#![no_std]

#[cfg(feature = "bindings")]
pub mod bindings;

#[cfg(feature = "masl-lib")]
pub mod masl;
