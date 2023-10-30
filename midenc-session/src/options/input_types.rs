use std::path::Path;

use miden_diagnostics::FileName;

/// This enum represents the types of raw inputs provided to the compiler
#[derive(Debug, Clone)]
pub enum Input {
    /// This is a path to a single file on disk
    File(FileName),
    /// This is content read from standard input
    Stdin(FileName, Vec<u8>),
}

/// An error that occurs when detecting the file type of an input
#[derive(Debug, thiserror::Error)]
pub enum InvalidFileTypeError {
    /// Occurs if an unsupported file type is given as an input
    #[error("invalid input file '{}': unsupported file type", .0.display())]
    Unsupported(std::path::PathBuf),
    #[error("could not detect file type of input")]
    Unrecognized,
}

/// This represents the file types recognized by the compiler
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FileType {
    Hir,
    Masm,
    Masl,
    Wasm,
    Wat,
}
impl FileType {
    pub fn detect(bytes: &[u8]) -> Result<Self, InvalidFileTypeError> {
        if bytes.starts_with(b"\0asm") {
            return Ok(FileType::Wasm);
        }

        fn is_masm_top_level_item(line: &str) -> bool {
            line.starts_with("const.") || line.starts_with("export.") || line.starts_with("proc.")
        }

        if let Ok(content) = core::str::from_utf8(bytes) {
            if content.starts_with("(module ") {
                return Ok(FileType::Wat);
            }
            if content.starts_with("module ") {
                return Ok(FileType::Hir);
            }
            if content.lines().any(is_masm_top_level_item) {
                return Ok(FileType::Masm);
            }
        }

        Err(InvalidFileTypeError::Unrecognized)
    }
}
impl TryFrom<&Path> for FileType {
    type Error = InvalidFileTypeError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("hir") => Ok(FileType::Hir),
            Some("masm") => Ok(FileType::Masm),
            Some("masl") => Ok(FileType::Masl),
            Some("wasm") => Ok(FileType::Wasm),
            Some("wat") => Ok(FileType::Wat),
            _ => Err(InvalidFileTypeError::Unsupported(path.to_path_buf())),
        }
    }
}
