use std::{
    ffi::OsStr,
    fmt,
    path::{Path, PathBuf},
};

use miden_diagnostics::FileName;

/// An error that occurs when detecting the file type of an input
#[derive(Debug, thiserror::Error)]
pub enum InvalidInputError {
    /// Occurs if an unsupported file type is given as an input
    #[error("invalid input file '{}': unsupported file type", .0.display())]
    UnsupportedFileType(std::path::PathBuf),
    /// We attempted to detecth the file type from the raw bytes, but failed
    #[error("could not detect file type of input")]
    UnrecognizedFileType,
    /// Unable to read input file
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputType {
    Real(PathBuf),
    Stdin { name: FileName, input: Vec<u8> },
}

/// This enum represents the types of raw inputs provided to the compiler
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputFile {
    pub file: InputType,
    file_type: FileType,
}
impl InputFile {
    /// Get an [InputFile] representing the contents of `path`.
    ///
    /// This function returns an error if the contents are not a valid supported file type.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, InvalidInputError> {
        let path = path.as_ref();
        let file_type = FileType::try_from(path)?;
        Ok(Self {
            file: InputType::Real(path.to_path_buf()),
            file_type,
        })
    }

    /// Get an [InputFile] representing the contents received from standard input.
    ///
    /// This function returns an error if the contents are not a valid supported file type.
    pub fn from_stdin(name: FileName) -> Result<Self, InvalidInputError> {
        use std::io::Read;

        let mut input = Vec::with_capacity(1024);
        std::io::stdin().read_to_end(&mut input)?;
        Self::from_bytes(input, name)
    }

    pub fn from_bytes(bytes: Vec<u8>, name: FileName) -> Result<Self, InvalidInputError> {
        let file_type = FileType::detect(&bytes)?;
        match file_type {
            FileType::Hir | FileType::Wasm | FileType::Wat => Ok(Self {
                file: InputType::Stdin { name, input: bytes },
                file_type,
            }),
            // We do not yet have frontends for these file types
            FileType::Masm => Err(InvalidInputError::UnsupportedFileType(PathBuf::from("stdin"))),
        }
    }

    pub fn file_type(&self) -> FileType {
        self.file_type
    }

    pub fn as_path(&self) -> Option<&Path> {
        match &self.file {
            InputType::Real(ref path) => Some(path),
            _ => None,
        }
    }

    pub fn is_real(&self) -> bool {
        matches!(self.file, InputType::Real(_))
    }

    pub fn filestem(&self) -> &str {
        match &self.file {
            InputType::Real(ref path) => path.file_stem().unwrap().to_str().unwrap(),
            InputType::Stdin { .. } => "noname",
        }
    }
}
impl clap::builder::ValueParserFactory for InputFile {
    type Parser = InputFileParser;

    fn value_parser() -> Self::Parser {
        InputFileParser
    }
}

#[doc(hidden)]
#[derive(Clone)]
pub struct InputFileParser;
impl clap::builder::TypedValueParser for InputFileParser {
    type Value = InputFile;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, clap::error::Error> {
        use clap::error::{Error, ErrorKind};

        let input_file = match value.to_str() {
            Some("-") => InputFile::from_stdin(FileName::Virtual("stdin".into())).map_err(
                |err| match err {
                    InvalidInputError::Io(err) => Error::raw(ErrorKind::Io, err),
                    err => Error::raw(ErrorKind::ValueValidation, err),
                },
            )?,
            Some(_) | None => {
                InputFile::from_path(PathBuf::from(value)).map_err(|err| match err {
                    InvalidInputError::Io(err) => Error::raw(ErrorKind::Io, err),
                    err => Error::raw(ErrorKind::ValueValidation, err),
                })?
            }
        };

        match &input_file.file {
            InputType::Real(path) => {
                if path.exists() {
                    if path.is_file() {
                        Ok(input_file)
                    } else {
                        Err(Error::raw(
                            ErrorKind::ValueValidation,
                            format!("invalid input '{}': not a file", path.display()),
                        ))
                    }
                } else {
                    Err(Error::raw(
                        ErrorKind::ValueValidation,
                        format!("invalid input '{}': file does not exist", path.display()),
                    ))
                }
            }
            InputType::Stdin { .. } => Ok(input_file),
        }
    }
}

/// This represents the file types recognized by the compiler
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FileType {
    Hir,
    Masm,
    Wasm,
    Wat,
}
impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Hir => f.write_str("hir"),
            Self::Masm => f.write_str("masm"),
            Self::Wasm => f.write_str("wasm"),
            Self::Wat => f.write_str("wat"),
        }
    }
}
impl FileType {
    pub fn detect(bytes: &[u8]) -> Result<Self, InvalidInputError> {
        if bytes.starts_with(b"\0asm") {
            return Ok(FileType::Wasm);
        }

        fn is_masm_top_level_item(line: &str) -> bool {
            line.starts_with("const.") || line.starts_with("export.") || line.starts_with("proc.")
        }

        if let Ok(content) = core::str::from_utf8(bytes) {
            // Skip comment lines and empty lines
            let first_line = content
                .lines()
                .find(|line| !line.starts_with(['#', ';']) && !line.trim().is_empty());
            if let Some(first_line) = first_line {
                if first_line.starts_with("(module #") {
                    return Ok(FileType::Hir);
                }
                if first_line.starts_with("(module") {
                    return Ok(FileType::Wat);
                }
                if is_masm_top_level_item(first_line) {
                    return Ok(FileType::Masm);
                }
            }
        }

        Err(InvalidInputError::UnrecognizedFileType)
    }
}
impl TryFrom<&Path> for FileType {
    type Error = InvalidInputError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("hir") => Ok(FileType::Hir),
            Some("masm") => Ok(FileType::Masm),
            Some("wasm") => Ok(FileType::Wasm),
            Some("wat") => Ok(FileType::Wat),
            _ => Err(InvalidInputError::UnsupportedFileType(path.to_path_buf())),
        }
    }
}
