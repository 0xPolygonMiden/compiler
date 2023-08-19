#![deny(warnings)]
pub mod analysis;
pub mod hir;
pub mod pass;
pub mod types;

// Re-export cranelift_entity so that users don't have to hunt for the same version
pub use cranelift_entity;
