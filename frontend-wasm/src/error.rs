use miden_diagnostics::Diagnostic;
use miden_diagnostics::ToDiagnostic;
use miden_hir::SymbolConflictError;
use thiserror::Error;

/// A WebAssembly translation error.
///
/// When a WebAssembly function can't be translated, one of these error codes will be returned
/// to describe the failure.
#[derive(Error, Debug)]
pub enum WasmError {
    /// The input WebAssembly code is invalid.
    ///
    /// This error code is used by a WebAssembly translator when it encounters invalid WebAssembly
    /// code. This should never happen for validated WebAssembly code.
    #[error("Invalid input WebAssembly code at offset {offset}: {message}")]
    InvalidWebAssembly {
        /// A string describing the validation error.
        message: String,
        /// The bytecode offset where the error occurred.
        offset: usize,
    },

    /// A feature used by the WebAssembly code is not supported by the Miden IR.
    #[error("Unsupported Wasm: {0}")]
    Unsupported(String),

    #[error("Too many declared functions in the module")]
    FuncNumLimitExceeded,

    #[error("{0}")]
    SymbolConflictError(#[from] SymbolConflictError),
}

impl From<wasmparser::BinaryReaderError> for WasmError {
    fn from(e: wasmparser::BinaryReaderError) -> Self {
        Self::InvalidWebAssembly {
            message: e.message().into(),
            offset: e.offset(),
        }
    }
}

impl ToDiagnostic for WasmError {
    fn to_diagnostic(self) -> Diagnostic {
        Diagnostic::error().with_message(self.to_string())
    }
}

/// A convenient alias for a `Result` that uses `WasmError` as the error type.
pub type WasmResult<T> = Result<T, WasmError>;

/// Emit diagnostics and return an `Err(WasmError::Unsupported(msg))` where `msg` the string built by calling `format!`
/// on the arguments to this macro.
#[macro_export]
macro_rules! unsupported_diag {
    ($diagnostics:expr, $($arg:tt)*) => {
        let message = format!($($arg)*);
        $diagnostics
            .diagnostic(miden_diagnostics::Severity::Error)
            .with_message(message.clone())
            .emit();
        return Err($crate::error::WasmError::Unsupported(message));
    }
}
