use miden_assembly::{library::Library as CompiledLibrary, utils::Deserializable};

/// Stubs for the Miden rollup tx kernel
pub struct MidenTxKernelLibrary(CompiledLibrary);

impl AsRef<CompiledLibrary> for MidenTxKernelLibrary {
    fn as_ref(&self) -> &CompiledLibrary {
        &self.0
    }
}

impl From<MidenTxKernelLibrary> for CompiledLibrary {
    fn from(lib: MidenTxKernelLibrary) -> Self {
        lib.0
    }
}

impl Default for MidenTxKernelLibrary {
    fn default() -> Self {
        let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/assets/tx.masl"));
        let contents = CompiledLibrary::read_from_bytes(bytes).expect("failed to read std masl!");
        Self(contents)
    }
}
