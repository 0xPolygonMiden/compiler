/// Configuration for the WASM translation.
#[derive(Debug)]
pub struct WasmTranslationConfig {
    /// The module name to use if Wasm module doesn't have one.
    pub module_name_fallback: String,
}

impl Default for WasmTranslationConfig {
    fn default() -> Self {
        Self {
            module_name_fallback: "noname".to_string(),
        }
    }
}
