use std::{fs::File, io::prelude::*, path::Path};
use miden_assembly::ast::{self as masm};

/// The emitter for MASM output.

pub enum MASMAst {
    Program(masm::ProgramAst),
    Module(masm::ModuleAst),
}

impl fmt::Display for MASMAst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Program(p) => write!("{}", p),
            Module(m) => write!("{}", m),
        }
    }
}

fn write_ast_to_file<P: AsRef<Path>>(ast: &MASMAst, path: P) -> io::Result<()> {
    let mut file = File::create(path);
    file.write_fmt(format_args!("{}", ast))
}
