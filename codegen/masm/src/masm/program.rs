use std::{collections::BTreeMap, path::Path};

use miden_hir::{self as hir, DataSegmentTable, FunctionIdent, Ident};

use super::*;

/// A [Program] represents a complete set of modules which are intended to
/// be shipped together as an artifact, either as an executable, or as a library
/// to be integrated into a larger executable.
#[derive(Default)]
pub struct Program {
    /// The set of modules which belong to this program
    pub modules: Vec<Module>,
    /// The function identifier for the program entrypoint, if this is an executable module
    pub entrypoint: Option<FunctionIdent>,
    /// The data segment table for this program
    pub segments: DataSegmentTable,
}
impl Program {
    /// Create a new, empty [Program]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_executable(&self) -> bool {
        self.entrypoint.is_some()
    }

    pub fn is_library(&self) -> bool {
        self.entrypoint.is_none()
    }

    /// Write this [Program] to the given output directory.
    ///
    /// The provided [miden_diagnostics::CodeMap] is used for computing source locations.
    pub fn write_to_directory<P: AsRef<Path>>(
        &self,
        codemap: &miden_diagnostics::CodeMap,
        path: P,
    ) -> std::io::Result<()> {
        use miden_assembly as masm;

        let path = path.as_ref();
        assert!(path.is_dir());

        let program = self.to_program_ast();
        program.write_to_file(path.join(masm::LibraryPath::EXEC_PATH))?;

        for module in self.modules.iter() {
            module.write_to_directory(codemap, path)?;
        }

        Ok(())
    }

    /// Convert this program to its [miden_assembly::ast::ProgramAst] representation
    pub fn to_program_ast(&self) -> miden_assembly::ast::ProgramAst {
        use miden_assembly::{
            self as masm,
            ast::{Instruction, ModuleImports, Node, ProgramAst},
        };

        if let Some(entry) = self.entrypoint {
            let entry_import = Import::try_from(entry.module).expect("invalid module name");
            let entry_module_path =
                masm::LibraryPath::new(entry_import.name.as_str()).expect("invalid module path");
            let entry_id =
                masm::ProcedureId::from_name(entry.function.as_str(), &entry_module_path);
            let entry_name = masm::ProcedureName::try_from(
                FunctionIdent {
                    module: Ident::with_empty_span(entry_import.alias),
                    ..entry
                }
                .to_string(),
            )
            .expect("invalid entrypoint function name");
            let imported =
                BTreeMap::from([(entry_import.alias.to_string(), entry_module_path.clone())]);
            let invoked = BTreeMap::from([(entry_id, (entry_name, entry_module_path))]);
            let imports = ModuleImports::new(imported, invoked);

            // TODO: Write data segments, initialize function table
            let body = vec![Node::Instruction(Instruction::ExecImported(entry_id))];

            ProgramAst::new(body, vec![])
                .expect("invalid program")
                .with_import_info(imports)
        } else {
            todo!("0xPolygonMiden/miden-vm#1108")
        }
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

impl Program {}
