use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::ValueEnum;

/// This enum represents the type of outputs the compiler can produce
#[derive(Debug, Copy, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputType {
    /// The compiler will emit the abstract syntax tree of the input, if applicable
    Ast,
    /// The compiler will emit Miden IR
    Hir,
    /// The compiler will emit Miden Assembly
    Masm,
    /// The compiler will emit a Miden Assembly program or library
    #[default]
    Masl,
}
impl OutputType {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Ast => "ast",
            Self::Hir => "hir",
            Self::Masm => "masm",
            Self::Masl => "masl",
        }
    }

    pub fn shorthand_display() -> String {
        format!(
            "`{}`, `{}`, `{}`, `{}`",
            Self::Ast,
            Self::Hir,
            Self::Masm,
            Self::Masl,
        )
    }
}
impl fmt::Display for OutputType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Ast => f.write_str("ast"),
            Self::Hir => f.write_str("hir"),
            Self::Masm => f.write_str("masm"),
            Self::Masl => f.write_str("masl"),
        }
    }
}
impl FromStr for OutputType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ast" => Ok(Self::Ast),
            "hir" => Ok(Self::Hir),
            "masm" => Ok(Self::Masm),
            "masl" => Ok(Self::Masl),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum OutputFile {
    Real(PathBuf),
    Stdout,
}
impl OutputFile {
    pub fn parent(&self) -> Option<&Path> {
        match self {
            Self::Real(ref path) => path.parent(),
            Self::Stdout => None,
        }
    }

    pub fn filestem(&self) -> Option<&OsStr> {
        match self {
            Self::Real(ref path) => path.file_stem(),
            Self::Stdout => Some(OsStr::new("stdout")),
        }
    }

    pub fn is_stdout(&self) -> bool {
        matches!(self, Self::Stdout)
    }

    pub fn is_tty(&self) -> bool {
        match self {
            Self::Real(_) => false,
            Self::Stdout => atty::is(atty::Stream::Stdout),
        }
    }

    pub fn as_path(&self) -> &Path {
        match self {
            Self::Real(ref path) => path.as_ref(),
            Self::Stdout => &Path::new("stdout"),
        }
    }

    pub fn file_for_writing(
        &self,
        outputs: &OutputFiles,
        ty: OutputType,
        name: Option<&str>,
    ) -> PathBuf {
        match self {
            Self::Real(ref path) => path.clone(),
            Self::Stdout => outputs.temp_path(ty, name),
        }
    }
}

#[derive(Debug)]
pub struct OutputFiles {
    stem: String,
    pub out_dir: PathBuf,
    pub out_file: Option<OutputFile>,
    pub tmp_dir: Option<PathBuf>,
    pub outputs: OutputTypes,
}
impl OutputFiles {
    pub fn new(
        stem: String,
        out_dir: PathBuf,
        out_file: Option<OutputFile>,
        tmp_dir: Option<PathBuf>,
        outputs: OutputTypes,
    ) -> Self {
        Self {
            stem,
            out_dir,
            out_file,
            tmp_dir,
            outputs,
        }
    }

    pub fn path(&self, ty: OutputType) -> OutputFile {
        self.outputs
            .get(&ty)
            .and_then(|p| p.to_owned())
            .or_else(|| self.out_file.clone())
            .unwrap_or_else(|| OutputFile::Real(self.output_path(ty)))
    }

    pub fn output_path(&self, ty: OutputType) -> PathBuf {
        let extension = ty.extension();
        if let Some(output_file) = self.outputs.get(&ty) {
            match output_file {
                Some(OutputFile::Real(ref path)) if path.is_absolute() => {
                    path.with_extension(extension)
                }
                Some(OutputFile::Real(ref path)) => {
                    self.out_dir.join(path).with_extension(extension)
                }
                Some(OutputFile::Stdout) | None => {
                    self.with_directory_and_extension(&self.out_dir, extension)
                }
            }
        } else {
            self.with_directory_and_extension(&self.out_dir, extension)
        }
    }

    pub fn temp_path(&self, ty: OutputType, name: Option<&str>) -> PathBuf {
        let extension = ty.extension();
        self.temp_path_ext(extension, name)
    }

    fn temp_path_ext(&self, ext: &str, name: Option<&str>) -> PathBuf {
        let mut extension = String::new();

        if let Some(name) = name {
            extension.push_str(name);
        }

        if !ext.is_empty() {
            if !extension.is_empty() {
                extension.push('.');
            }
            extension.push_str(ext);
        }

        let tmp_dir = self.tmp_dir.as_ref().unwrap_or(&self.out_dir);
        self.with_directory_and_extension(tmp_dir, &extension)
    }

    pub fn with_extension(&self, extension: &str) -> PathBuf {
        self.with_directory_and_extension(&self.out_dir, extension)
    }

    fn with_directory_and_extension(&self, directory: &PathBuf, extension: &str) -> PathBuf {
        let mut path = directory.join(&self.stem);
        path.set_extension(extension);
        path
    }
}

#[derive(Debug, Clone, Default)]
pub struct OutputTypes(BTreeMap<OutputType, Option<OutputFile>>);
impl OutputTypes {
    pub fn new<I: IntoIterator<Item = OutputTypeSpec>>(entries: I) -> Self {
        Self(BTreeMap::from_iter(
            entries
                .into_iter()
                .map(|spec| (spec.output_type, spec.path)),
        ))
    }

    pub fn get(&self, key: &OutputType) -> Option<&Option<OutputFile>> {
        self.0.get(key)
    }

    pub fn insert(&mut self, key: OutputType, value: Option<OutputFile>) {
        self.0.insert(key, value);
    }

    pub fn contains_key(&self, key: &OutputType) -> bool {
        self.0.contains_key(key)
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, OutputType, Option<OutputFile>> {
        self.0.iter()
    }

    pub fn keys(&self) -> std::collections::btree_map::Keys<'_, OutputType, Option<OutputFile>> {
        self.0.keys()
    }

    pub fn values(
        &self,
    ) -> std::collections::btree_map::Values<'_, OutputType, Option<OutputFile>> {
        self.0.values()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn parse_only(&self) -> bool {
        self.0.keys().any(|k| !matches!(k, OutputType::Ast))
    }

    pub fn should_codegen(&self) -> bool {
        self.0
            .keys()
            .any(|k| matches!(k, OutputType::Masm | OutputType::Masl))
    }

    pub fn should_link(&self) -> bool {
        self.0.keys().any(|k| matches!(k, OutputType::Masl))
    }
}

/// This type describes an output type with optional path specification
#[derive(Clone)]
pub struct OutputTypeSpec {
    pub output_type: OutputType,
    pub path: Option<OutputFile>,
}
impl clap::builder::ValueParserFactory for OutputTypeSpec {
    type Parser = OutputTypeParser;

    fn value_parser() -> Self::Parser {
        OutputTypeParser
    }
}

#[doc(hidden)]
#[derive(Clone)]
pub struct OutputTypeParser;
impl clap::builder::TypedValueParser for OutputTypeParser {
    type Value = OutputTypeSpec;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, clap::error::Error> {
        use clap::error::{Error, ErrorKind};

        let output_type = value
            .to_str()
            .ok_or_else(|| Error::new(ErrorKind::InvalidUtf8))?;

        let (shorthand, path) = match output_type.split_once('=') {
            None => (output_type, None),
            Some((shorthand, "-")) => (shorthand, Some(OutputFile::Stdout)),
            Some((shorthand, path)) => (shorthand, Some(OutputFile::Real(PathBuf::from(path)))),
        };
        let output_type = shorthand.parse::<OutputType>().map_err(|_| {
            Error::raw(
                ErrorKind::InvalidValue,
                format!(
                    "invalid output type: `{shorthand}` - expected one of: {display}",
                    display = OutputType::shorthand_display()
                ),
            )
        })?;
        Ok(OutputTypeSpec { output_type, path })
    }
}
