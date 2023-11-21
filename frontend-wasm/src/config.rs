/// Configuration for the WASM translation.
#[derive(Debug, Default)]
pub struct WasmTranslationConfig {
    /// The module name to use if Wasm module doesn't have one.
    pub module_name_fallback: Option<String>,
}
