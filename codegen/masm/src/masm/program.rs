use std::{collections::BTreeMap, path::Path, sync::Arc};

use intrusive_collections::RBTree;
use miden_hir::{self as hir, DataSegmentTable, FunctionIdent, Ident};

use super::*;

/// A [Program] represents a complete set of modules which are intended to
/// be shipped together as an artifact, either as an executable, or as a library
/// to be integrated into a larger executable.
#[derive(Default)]
pub struct Program {
    /// The set of modules which belong to this program
    modules: Modules,
    /// The function identifier for the program entrypoint, if this is an executable module
    pub entrypoint: Option<FunctionIdent>,
    /// The data segment table for this program
    pub segments: DataSegmentTable,
}

enum Modules {
    Open(RBTree<ModuleTreeAdapter>),
    Frozen(RBTree<FrozenModuleTreeAdapter>),
}
impl Default for Modules {
    fn default() -> Self {
        Self::Open(Default::default())
    }
}
impl Modules {
    pub fn iter(&self) -> ModulesIter<'_> {
        match self {
            Self::Open(ref tree) => ModulesIter::Open(tree.iter()),
            Self::Frozen(ref tree) => ModulesIter::Frozen(tree.iter()),
        }
    }

    pub fn insert(&mut self, module: Box<Module>) {
        match self {
            Self::Open(ref mut tree) => {
                tree.insert(module);
            }
            Self::Frozen(_) => panic!("cannot insert module into frozen program"),
        }
    }

    fn freeze(&mut self) {
        if let Self::Open(ref mut modules) = self {
            let mut frozen = RBTree::<FrozenModuleTreeAdapter>::default();

            let mut open = modules.front_mut();
            while let Some(module) = open.remove() {
                frozen.insert(module.freeze());
            }

            *self = Self::Frozen(frozen);
        }
    }
}

impl Program {
    /// Create a new, empty [Program]
    pub fn new() -> Self {
        Self::default()
    }

    /// Freezes this program, preventing further modifications
    pub fn freeze(mut self: Box<Self>) -> Arc<Program> {
        self.modules.freeze();
        Arc::from(self)
    }

    /// Get an iterator over the modules in this program
    pub fn modules(&self) -> impl Iterator<Item = &Module> + '_ {
        self.modules.iter()
    }

    /// Access the frozen module tree of this program, and panic if not frozen
    pub fn unwrap_frozen_modules(&self) -> &RBTree<FrozenModuleTreeAdapter> {
        match self.modules {
            Modules::Frozen(ref modules) => modules,
            Modules::Open(_) => panic!("expected program to be frozen"),
        }
    }

    /// Insert a module into this program
    ///
    /// NOTE: This function will panic if the program has been frozen
    pub fn insert(&mut self, module: Box<Module>) {
        self.modules.insert(module);
    }

    pub fn is_executable(&self) -> bool {
        self.entrypoint.is_some()
    }

    pub fn is_library(&self) -> bool {
        self.entrypoint.is_none()
    }

    /// Returns true if this program contains a [Module] named `name`
    pub fn contains<N>(&self, name: N) -> bool
    where
        Ident: PartialEq<N>,
    {
        self.modules.iter().any(|m| m.name == name)
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
            modules: Default::default(),
            entrypoint,
            segments,
        }
    }
}

enum ModulesIter<'a> {
    Open(intrusive_collections::rbtree::Iter<'a, ModuleTreeAdapter>),
    Frozen(intrusive_collections::rbtree::Iter<'a, FrozenModuleTreeAdapter>),
}
impl<'a> Iterator for ModulesIter<'a> {
    type Item = &'a Module;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Open(ref mut iter) => iter.next(),
            Self::Frozen(ref mut iter) => iter.next(),
        }
    }
}
