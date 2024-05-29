use std::{fs::File, io::Write, path::Path};

use midenc_hir_symbol::Symbol;

use crate::OutputType;

pub trait Emit {
    /// The name of this item, if applicable
    fn name(&self) -> Option<Symbol>;
    /// The output type associated with this item
    fn output_type(&self) -> OutputType;
    /// Write this item to standard output
    fn write_to_stdout(&self) -> std::io::Result<()> {
        let stdout = std::io::stdout().lock();
        self.write_to(stdout)
    }
    /// Write this item to the given file path
    fn write_to_file(&self, path: &Path) -> std::io::Result<()> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let file = File::create(path)?;
        self.write_to(file)
    }
    /// Write this item to the given [std::io::Write] handle
    fn write_to<W: Write>(&self, writer: W) -> std::io::Result<()>;
}

impl<T: Emit> Emit for Box<T> {
    #[inline]
    fn name(&self) -> Option<Symbol> {
        (**self).name()
    }

    #[inline]
    fn output_type(&self) -> OutputType {
        (**self).output_type()
    }

    #[inline]
    fn write_to_stdout(&self) -> std::io::Result<()> {
        (**self).write_to_stdout()
    }

    #[inline]
    fn write_to_file(&self, path: &Path) -> std::io::Result<()> {
        (**self).write_to_file(path)
    }

    #[inline]
    fn write_to<W: Write>(&self, writer: W) -> std::io::Result<()> {
        (**self).write_to(writer)
    }
}
