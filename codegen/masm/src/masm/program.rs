use std::{fmt, path::Path, sync::Arc};

use hir::{Signature, Symbol};
use miden_assembly::{
    ast::{ModuleKind, ProcedureName},
    LibraryNamespace,
};
use miden_hir::{self as hir, DataSegmentTable, FunctionIdent, Ident};

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
    /// The function identifier for the program entrypoint, if applicable
    pub entrypoint: Option<FunctionIdent>,
}
impl Program {
    /// Create a new, empty [Program]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Create a new [Program] initialized from an [hir::Program].
    ///
    /// The resulting [Program] will have the following:
    ///
    /// * Data segments described by the original [hir::Program]
    /// * The entrypoint function which will be invoked after the initialization phase of startup
    /// * If an entrypoint is set, an executable [Module] which performs initialization and then
    ///   invokes the entrypoint
    ///
    /// None of the HIR modules will have been added yet
    pub fn from_hir(program: &hir::Program) -> Self {
        let mut modules = Modules::default();

        // Create executable module if we have an entrypoint
        let entrypoint = program.entrypoint();
        if let Some(entry) = entrypoint {
            let mut exe =
                Box::new(Module::new(LibraryNamespace::Exec.into(), ModuleKind::Executable));
            exe.imports.add(entry);
            let entry_module = exe
                .imports
                .alias(&entry.module)
                .expect("something went wrong when adding entrypoint import");
            let start_id = FunctionIdent {
                module: Ident::with_empty_span(Symbol::intern(LibraryNamespace::EXEC_PATH)),
                function: Ident::with_empty_span(Symbol::intern(ProcedureName::MAIN_PROC_NAME)),
            };
            let start_sig = Signature::new([], []);
            let mut start = Box::new(Function::new(start_id, start_sig));
            {
                let body = start.body_mut();
                body.push(Op::Exec(FunctionIdent {
                    module: entry_module,
                    function: entry.function,
                }));
            }
            exe.push_back(start);
            modules.insert(exe);
        }

        let segments = program.segments().clone();

        Self {
            modules,
            segments,
            entrypoint,
        }
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
        self.entrypoint.is_some()
    }

    pub fn is_library(&self) -> bool {
        self.entrypoint.is_none()
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
        self.modules.iter().any(|m| m.id == name)
    }

    /// Write this [Program] to the given output directory.
    ///
    /// The provided [miden_diagnostics::CodeMap] is used for computing source locations.
    pub fn write_to_directory<P: AsRef<Path>>(
        &self,
        codemap: &miden_diagnostics::CodeMap,
        path: P,
    ) -> std::io::Result<()> {
        let path = path.as_ref();
        assert!(path.is_dir());

        for module in self.modules.iter() {
            module.write_to_directory(codemap, path)?;
        }

        Ok(())
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for module in self.modules.iter() {
            if module.name.is_exec_path() {
                continue;
            }
            writeln!(f, "mod {}\n", &module.name)?;
            writeln!(f, "{}", module)?;
        }

        if let Some(module) = self.modules.iter().find(|m| m.name.is_exec_path()) {
            writeln!(f, "program\n")?;
            writeln!(f, "{}", module)?;
        }

        Ok(())
    }
}
