//! Support for the Wasm component model translation
//!
//! This module contains all of the internal type definitions to parse and
//! translate the component model.

mod info;
pub mod translate;
mod types;

pub use self::info::*;
pub use self::types::*;
