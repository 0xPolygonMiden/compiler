/// Configuration for the WASM translation.
#[derive(Debug)]
pub struct WasmTranslationConfig {
    /// The module name to use if Wasm module doesn't have one.
    pub module_name_fallback: String,

    /// Whether or not to generate native DWARF debug information.
    pub generate_native_debuginfo: bool,

    /// Whether or not to retain DWARF sections in compiled modules.
    pub parse_wasm_debuginfo: bool,
}

impl Default for WasmTranslationConfig {
    fn default() -> Self {
        Self {
            module_name_fallback: "noname".to_string(),
            generate_native_debuginfo: false,
            parse_wasm_debuginfo: false,
        }
    }
}
