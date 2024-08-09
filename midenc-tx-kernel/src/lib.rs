use std::sync::Arc;

use miden_assembly::{
    ast::ModuleKind, library::Library as CompiledLibrary, Assembler, Compile, CompileOptions,
    DefaultSourceManager,
};

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
        // TODO: Load compiled MASL from file
        let source_manager = Arc::new(DefaultSourceManager::default());
        let assembler = Assembler::new(source_manager.clone());
        let tx_module = (include_str!("../masm/tx.masm"))
            .compile_with_options(
                source_manager.as_ref(),
                CompileOptions {
                    warnings_as_errors: assembler.warnings_as_errors(),
                    ..CompileOptions::new(ModuleKind::Library, "miden::tx").unwrap()
                },
            )
            .unwrap();
        let account_module = (include_str!("../masm/account.masm"))
            .compile_with_options(
                source_manager.as_ref(),
                CompileOptions {
                    warnings_as_errors: assembler.warnings_as_errors(),
                    ..CompileOptions::new(ModuleKind::Library, "miden::account").unwrap()
                },
            )
            .unwrap();
        let lib = assembler.assemble_library([tx_module, account_module]).unwrap();
        Self(lib)
    }
}
