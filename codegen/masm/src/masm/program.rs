use core::fmt;
use std::{collections::BTreeMap, path::Path, sync::Arc};

use miden_hir::{self as hir, DataSegmentTable, FunctionIdent, Ident};
use rustc_hash::FxHashMap;

use super::{module::Modules, *};

/// A [Program] represents a complete set of modules which are intended to
/// be shipped together as an artifact, either as an executable, or as a library
/// to be integrated into a larger executable.
#[derive(Default)]
pub struct Program {
    /// The set of modules which belong to this program
    modules: Modules,
    /// The data segment table for this program
    pub segments: DataSegmentTable,
    /// The top-level global initialization code for this program, if applicable
    pub body: Option<Begin>,
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
    pub fn unwrap_frozen_modules(&self) -> &FrozenModuleTree {
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
        self.body.is_some()
    }

    pub fn is_library(&self) -> bool {
        self.body.is_none()
    }

    /// Get a reference to a module in this program by name
    pub fn get<Q>(&self, name: &Q) -> Option<&Module>
    where
        Q: ?Sized + Ord,
        Ident: core::borrow::Borrow<Q>,
    {
        self.modules.get(name)
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

        let program = self.to_program_ast(codemap);
        program.write_to_file(path.join(masm::LibraryPath::EXEC_PATH))?;

        for module in self.modules.iter() {
            module.write_to_directory(codemap, path)?;
        }

        Ok(())
    }

    /// Convert this program to its [miden_assembly::ast::ProgramAst] representation
    pub fn to_program_ast(
        &self,
        codemap: &miden_diagnostics::CodeMap,
    ) -> miden_assembly::ast::ProgramAst {
        use miden_assembly::{
            self as masm,
            ast::{Instruction, ModuleImports, Node, ProgramAst},
        };

        if let Some(begin) = &self.body {
            // Create module import table
            let mut imported = BTreeMap::<String, masm::LibraryPath>::default();
            let mut invoked = BTreeMap::<masm::ProcedureId, _>::default();
            let mut proc_ids = FxHashMap::<FunctionIdent, masm::ProcedureId>::default();
            for import in begin.imports.iter() {
                let path =
                    masm::LibraryPath::new(import.name.as_str()).expect("invalid module name");
                imported.insert(import.alias.to_string(), path.clone());
                if let Some(imported_fns) = begin.imports.imported(&import.alias) {
                    for import_fn in imported_fns.iter().copied() {
                        let name = masm::ProcedureName::try_from(import_fn.function.as_str())
                            .expect("invalid function name");
                        let id = masm::ProcedureId::from_name(import_fn.function.as_str(), &path);
                        invoked.insert(id, (name, path.clone()));
                        proc_ids.insert(import_fn, id);
                    }
                }
            }
            let imports = ModuleImports::new(imported, invoked);
            let local_ids = Default::default();
            let (nodes, _) = begin
                .body
                .to_code_body(codemap, &begin.imports, &local_ids, &proc_ids)
                .into_parts();

            ProgramAst::new(nodes, vec![])
                .expect("invalid program")
                .with_import_info(imports)
        } else if let Some(entry) = self.modules.iter().find_map(|m| m.entrypoint()) {
            let entry_import = Import::try_from(entry.module).expect("invalid module name");
            let entry_module_path =
                masm::LibraryPath::new(entry_import.name.as_str()).expect("invalid module path");
            let entry_id =
                masm::ProcedureId::from_name(entry.function.as_str(), &entry_module_path);
            let entry_name = masm::ProcedureName::try_from(entry.function.as_str())
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

    /// Load a [Program] from a `.masl` file
    pub fn from_masl<P: AsRef<Path>>(
        path: P,
        codemap: &miden_diagnostics::CodeMap,
    ) -> Result<Self, miden_assembly::LibraryError> {
        use miden_assembly::MaslLibrary;

        MaslLibrary::read_from_file(path.as_ref()).map(|lib| Self::from_masl_library(&lib, codemap))
    }

    /// Load a [Program] from a MASL directory hierarchy, with the given root namespace.
    pub fn from_dir<P: AsRef<Path>, S: AsRef<str>>(
        path: P,
        root_ns: S,
        codemap: &miden_diagnostics::CodeMap,
    ) -> Result<Self, miden_assembly::LibraryError> {
        use miden_assembly::{LibraryError, LibraryNamespace, MaslLibrary};

        let root_ns = LibraryNamespace::new(root_ns.as_ref())?;
        let path = path.as_ref();
        let library = MaslLibrary::read_from_dir(
            path,
            root_ns,
            /* with_source_locations= */ true,
            Default::default(),
        )
        .map_err(|err| LibraryError::file_error(path.to_str().unwrap(), &format!("{err}")))?;

        Ok(Self::from_masl_library(&library, codemap))
    }

    pub fn to_masl_library<S: AsRef<str>>(
        &self,
        root_ns: S,
        codemap: &miden_diagnostics::CodeMap,
    ) -> Result<miden_assembly::MaslLibrary, miden_assembly::LibraryError> {
        use std::collections::BTreeSet;

        use miden_assembly::{LibraryNamespace, MaslLibrary, Version};

        let ns = LibraryNamespace::new(root_ns)?;
        let version = Version::default();
        let has_source_locations = false;
        let mut modules = Vec::with_capacity(self.modules.iter().count());
        let mut dependencies = BTreeSet::default();
        for module in self.modules() {
            for import in module.imports.iter() {
                if self.modules.get(&import.name).is_some() {
                    continue;
                }
                let root = match import.name.as_str().split_once("::") {
                    None => LibraryNamespace::new(import.name.as_str())?,
                    Some((root, _)) => LibraryNamespace::new(root)?,
                };
                dependencies.insert(root);
            }
            modules.push(module.to_module_ast(codemap));
        }
        MaslLibrary::new(
            ns,
            version,
            has_source_locations,
            modules,
            dependencies.into_iter().collect(),
        )
    }

    /// Convert a [miden_assembly::MaslLibrary] into a [Program]
    pub fn from_masl_library(
        library: &miden_assembly::MaslLibrary,
        codemap: &miden_diagnostics::CodeMap,
    ) -> Self {
        use miden_assembly::Library;
        use miden_diagnostics::SourceSpan;

        let mut modules = ModuleTree::default();
        for module in library.modules() {
            let module = Module::from_module(module, SourceSpan::UNKNOWN, codemap);
            modules.insert(Box::new(module));
        }

        Self {
            modules: Modules::Open(modules),
            segments: DataSegmentTable::default(),
            body: None,
        }
    }
}
impl From<&hir::Program> for Program {
    fn from(program: &hir::Program) -> Self {
        let segments = program.segments().clone();
        let body = if let Some(entry) = program.entrypoint() {
            let mut begin = Begin::default();
            begin.imports.add(entry);
            let entry_module = begin.imports.alias(&entry.module);
            begin.body.block_mut(begin.body.body).ops.push(Op::Exec(FunctionIdent {
                module: entry_module.unwrap_or(entry.module),
                function: entry.function,
            }));
            Some(begin)
        } else {
            None
        };
        Self {
            modules: Default::default(),
            segments,
            body,
        }
    }
}
impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for module in self.modules() {
            writeln!(f, "mod {}\n", &module.name)?;
            writeln!(f, "{}", module)?;
        }
        if let Some(entry) = self.body.as_ref() {
            writeln!(f, "program\n")?;
            for import in entry.imports.iter() {
                if import.is_aliased() {
                    writeln!(f, "use {}->{}", &import.name, &import.alias)?;
                } else {
                    writeln!(f, "use {}", &import.name)?;
                }
            }
            writeln!(f, "\n{entry}")?;
        }
        Ok(())
    }
}
