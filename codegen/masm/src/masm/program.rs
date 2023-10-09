use std::path::Path;

use miden_hir::{self as hir, DataSegmentTable, FunctionIdent};

use super::*;

#[derive(Default)]
pub struct Program {
    pub modules: Vec<Module>,
    pub entrypoint: Option<FunctionIdent>,
    pub segments: DataSegmentTable,
}
impl Program {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_executable(&self) -> bool {
        self.entrypoint.is_some()
    }

    pub fn is_library(&self) -> bool {
        self.entrypoint.is_none()
    }
}
impl From<&hir::Program> for Program {
    fn from(program: &hir::Program) -> Self {
        let entrypoint = program.entrypoint();
        let segments = program.segments().clone();
        Self {
            modules: vec![],
            entrypoint,
            segments,
        }
    }
}

impl Program {
    pub fn write_to_directory<P: AsRef<Path>>(&self, _path: P) -> std::io::Result<()> {
        todo!()
    }
}
