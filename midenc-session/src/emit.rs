use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::OutputType;

pub trait Emit {
    fn output_type(&self) -> OutputType;
    fn write_to_stdout(&self) -> std::io::Result<()> {
        let stdout = std::io::stdout();
        self.write_to(stdout)
    }
    fn write_to_file(&self, path: &Path) -> std::io::Result<()> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let file = File::create(path)?;
        self.write_to(file)
    }
    fn write_to<W: Write>(&self, writer: W) -> std::io::Result<()>;
}
