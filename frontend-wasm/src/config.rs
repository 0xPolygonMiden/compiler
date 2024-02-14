use miden_core::crypto::hash::RpoDigest;
use miden_hir::{FunctionExportName, FunctionInvocationMethod, InterfaceFunctionIdent};
use rustc_hash::FxHashMap;

/// Represents Miden VM codegen metadata for a function import.
/// This struct will have more fields in the future e.g. where the function
/// for this MAST hash is located (to be loaded by the VM)
#[derive(Debug, Clone)]
pub struct ImportMetadata {
    /// The MAST hash of the function to be used in codegen
    pub function_mast_root_hash: RpoDigest,
    /// The method of calling the function
    pub invoke_method: FunctionInvocationMethod,
}

/// Represents function export metadata
#[derive(Debug, Clone)]
pub struct ExportMetadata {
    /// The method of calling the function
    pub invoke_method: FunctionInvocationMethod,
}

/// Configuration for the WASM translation.
#[derive(Debug)]
pub struct WasmTranslationConfig {
    /// The source file name.
    /// This is used as a fallback for module/component name if it's not parsed from the Wasm binary.
    pub source_name: String,

    /// Whether or not to generate native DWARF debug information.
    pub generate_native_debuginfo: bool,

    /// Whether or not to retain DWARF sections in compiled modules.
    pub parse_wasm_debuginfo: bool,

    /// Import metadata for MAST hashes, calling convention, of
    /// each imported function. Having it here might be a temporary solution,
    /// later we might want to move it to Wasm custom section.
    pub import_metadata: FxHashMap<InterfaceFunctionIdent, ImportMetadata>,

    /// Export metadata for calling convention, etc.
    pub export_metadata: FxHashMap<FunctionExportName, ExportMetadata>,
}

impl Default for WasmTranslationConfig {
    fn default() -> Self {
        Self {
            source_name: "noname".to_string(),
            generate_native_debuginfo: false,
            parse_wasm_debuginfo: false,
            import_metadata: Default::default(),
            export_metadata: Default::default(),
        }
    }
}
