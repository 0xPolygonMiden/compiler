use std::{
    collections::BTreeMap,
    fmt,
    path::{Path, PathBuf},
    sync::Arc,
};

use intrusive_collections::{intrusive_adapter, RBTree, RBTreeAtomicLink};
use miden_assembly::ast::ModuleAst;
use miden_diagnostics::{CodeMap, SourceFile, SourceSpan};
use miden_hir::{FunctionIdent, Ident, Symbol};
use rustc_hash::FxHashMap;

use super::{function::Functions, FrozenFunctionList, Function, ModuleImportInfo};

#[derive(Debug, thiserror::Error)]
pub enum LoadModuleError {
    #[error("failed to load module from disk: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid path to module: '{}' is not a file", .0.display())]
    InvalidPath(PathBuf),
    #[error(transparent)]
    InvalidModulePath(#[from] miden_assembly::PathError),
    #[error(transparent)]
    ParseFailed(#[from] miden_assembly::ParsingError),
}

/// This represents a single compiled Miden Assembly module in a form that is
/// designed to integrate well with the rest of our IR. You can think of this
/// as an intermediate representation corresponding to the Miden Assembly AST,
/// i.e. [miden_assembly::ast::ModuleAst].
pub struct Module {
    link: RBTreeAtomicLink,
    pub span: SourceSpan,
    /// The name of this module, e.g. `std::math::u64`
    pub name: Ident,
    /// The module-scoped documentation for this module
    pub docs: Option<String>,
    /// The modules to import, along with their local aliases
    pub imports: ModuleImportInfo,
    /// The functions defined in this module
    functions: Functions,
}
impl Module {
    /// Create a new, empty [Module] with the given name.
    pub fn new(name: Ident) -> Self {
        Self {
            link: Default::default(),
            span: SourceSpan::UNKNOWN,
            name,
            docs: None,
            imports: Default::default(),
            functions: Default::default(),
        }
    }

    /// If this module contains a function marked with the `entrypoint` attribute,
    /// return the fully-qualified name of that function
    pub fn entrypoint(&self) -> Option<FunctionIdent> {
        self.functions.iter().find_map(|f| {
            if f.is_entrypoint() {
                Some(f.name)
            } else {
                None
            }
        })
    }

    /// Returns true if this module contains a [Function] `name`
    pub fn contains(&self, name: Ident) -> bool {
        self.functions.iter().any(|f| f.name.function == name)
    }

    /// Parse a [Module] from the given string
    pub fn parse_source_file<N: Into<Ident>>(
        source_file: Arc<SourceFile>,
        name: N,
        codemap: &CodeMap,
    ) -> Result<Self, LoadModuleError> {
        use miden_assembly::LibraryPath;

        let name = name.into();
        let module = miden_assembly::Module {
            path: LibraryPath::new(name.as_str())?,
            ast: ModuleAst::parse(source_file.source())?,
        };
        let span = source_file.source_span();
        Ok(Self::from_module(&module, span, codemap))
    }

    /// Parse a [Module] from the given file path
    pub fn parse_file<P: AsRef<Path>>(
        path: P,
        root_ns: Option<miden_assembly::LibraryNamespace>,
        codemap: &CodeMap,
    ) -> Result<Self, LoadModuleError> {
        let id = codemap.add_file(path)?;
        let source_file = codemap.get(id).unwrap();
        let basename = source_file
            .name()
            .as_ref()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let name = match root_ns {
            None => basename.to_string(),
            Some(root_ns) => format!("{}::{basename}", root_ns.as_str()),
        };
        Self::parse_source_file(source_file, name.as_str(), codemap)
    }

    pub fn from_module(
        module: &miden_assembly::Module,
        span: SourceSpan,
        codemap: &CodeMap,
    ) -> Self {
        let name = Ident::with_empty_span(Symbol::intern(module.path.as_str()));
        Self::from_module_ast_with_name(&module.ast, name, span, codemap)
    }

    /// Convert a [miden_assembly::ast::ModuleAst] to a [Module]
    pub fn from_module_ast_with_name<N: Into<Ident>>(
        ast: &ModuleAst,
        name: N,
        span: SourceSpan,
        _codemap: &CodeMap,
    ) -> Self {
        let module_name = name.into();
        let mut module = Self::new(module_name);
        module.span = span;
        module.docs = ast.docs().cloned();

        let imported = ast.import_info().clone();
        let locals = ast
            .procs()
            .iter()
            .map(|p| FunctionIdent {
                module: module_name,
                function: Ident::with_empty_span(Symbol::intern(p.name.as_ref())),
            })
            .collect::<Vec<_>>();

        for proc in ast.procs() {
            let function = Function::from_procedure_ast(module_name, proc, &locals, &imported);
            module.functions.push_back(function);
        }

        module
    }

    /// Freezes this program, preventing further modifications
    pub fn freeze(mut self: Box<Self>) -> Arc<Module> {
        self.functions.freeze();
        Arc::from(self)
    }

    /// Get an iterator over the functions in this module
    pub fn functions(&self) -> impl Iterator<Item = &Function> + '_ {
        self.functions.iter()
    }

    /// Access the frozen functions list of this module, and panic if not frozen
    pub fn unwrap_frozen_functions(&self) -> &FrozenFunctionList {
        match self.functions {
            Functions::Frozen(ref functions) => functions,
            Functions::Open(_) => panic!("expected module to be frozen"),
        }
    }

    /// Append a function to the end of this module
    ///
    /// NOTE: This function will panic if the module has been frozen
    pub fn push_back(&mut self, function: Box<Function>) {
        self.functions.push_back(function);
    }

    /// Convert this module into its [miden_assembly::Module] representation.
    pub fn to_module_ast(&self, codemap: &miden_diagnostics::CodeMap) -> miden_assembly::Module {
        use miden_assembly::{self as masm, ast::ModuleImports};

        // Create module import table
        let mut imported = BTreeMap::<String, masm::LibraryPath>::default();
        let mut invoked = BTreeMap::<masm::ProcedureId, _>::default();
        let mut proc_ids = FxHashMap::<FunctionIdent, masm::ProcedureId>::default();
        for import in self.imports.iter() {
            let path = masm::LibraryPath::new(import.name.as_str()).expect("invalid module name");
            imported.insert(import.alias.to_string(), path.clone());
            if let Some(imported_fns) = self.imports.imported(&import.alias) {
                for import_fn in imported_fns.iter().copied() {
                    let fname = import_fn.to_string();
                    let name = masm::ProcedureName::try_from(fname.as_str())
                        .expect("invalid function name");
                    let id = masm::ProcedureId::from_name(fname.as_str(), &path);
                    invoked.insert(id, (name, path.clone()));
                    proc_ids.insert(import_fn, id);
                }
            }
        }
        let imports = ModuleImports::new(imported, invoked);

        // Translate functions
        let mut local_ids = FxHashMap::default();
        for (id, function) in self.functions.iter().enumerate() {
            local_ids.insert(function.name, id as u16);
        }
        let mut procs = Vec::with_capacity(self.num_imported_functions());
        for function in self.functions.iter() {
            procs.push(function.to_function_ast(codemap, &self.imports, &local_ids, &proc_ids));
        }

        // Construct module
        let path = masm::LibraryPath::new(self.name.as_str()).expect("invalid module name");
        let ast = ModuleAst::new(procs, vec![], self.docs.clone())
            .expect("invalid module body")
            .with_import_info(imports);
        masm::Module { path, ast }
    }

    fn num_imported_functions(&self) -> usize {
        self.imports
            .iter()
            .map(|i| {
                self.imports
                    .imported(&i.alias)
                    .map(|imported| imported.len())
                    .unwrap_or(0)
            })
            .sum()
    }

    /// Write this module to a new file under `dir`, assuming `dir` is the root directory for a program.
    ///
    /// For example, if this module is named `std::math::u64`, then it will be written to `<dir>/std/math/u64.masm`
    pub fn write_to_directory<P: AsRef<Path>>(
        &self,
        codemap: &miden_diagnostics::CodeMap,
        dir: P,
    ) -> std::io::Result<()> {
        use std::fs::File;

        let mut path = dir.as_ref().to_path_buf();
        assert!(path.is_dir());
        for component in self.name.as_str().split("::") {
            path.push(component);
        }
        assert!(path.set_extension("masm"));

        let mut out = File::create(&path)?;
        self.emit(codemap, &mut out)
    }

    /// Write this module as Miden Assembly text to `out`
    pub fn emit(
        &self,
        codemap: &miden_diagnostics::CodeMap,
        out: &mut dyn std::io::Write,
    ) -> std::io::Result<()> {
        let ast = self.to_module_ast(codemap);
        out.write_fmt(format_args!("{}", &ast.ast))
    }
}
impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for import in self.imports.iter() {
            if import.is_aliased() {
                writeln!(f, "use.{}->{}", import.name, import.alias)?;
            } else {
                writeln!(f, "use.{}", import.name)?;
            }
        }

        if !self.imports.is_empty() {
            writeln!(f)?;
        }

        for (i, function) in self.functions.iter().enumerate() {
            if i > 0 {
                writeln!(f, "\n{}", function.display(&self.imports))?;
            } else {
                writeln!(f, "{}", function.display(&self.imports))?;
            }
        }

        Ok(())
    }
}
impl midenc_session::Emit for Module {
    fn name(&self) -> Option<Symbol> {
        Some(self.name.as_symbol())
    }
    fn output_type(&self) -> midenc_session::OutputType {
        midenc_session::OutputType::Masm
    }
    fn write_to<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_fmt(format_args!("{}", self))
    }
}

intrusive_adapter!(pub ModuleTreeAdapter = Box<Module>: Module { link: RBTreeAtomicLink });
intrusive_adapter!(pub FrozenModuleTreeAdapter = Arc<Module>: Module { link: RBTreeAtomicLink });
impl<'a> intrusive_collections::KeyAdapter<'a> for ModuleTreeAdapter {
    type Key = Ident;

    #[inline]
    fn get_key(&self, module: &'a Module) -> Ident {
        module.name
    }
}
impl<'a> intrusive_collections::KeyAdapter<'a> for FrozenModuleTreeAdapter {
    type Key = Ident;

    #[inline]
    fn get_key(&self, module: &'a Module) -> Ident {
        module.name
    }
}

pub type ModuleTree = RBTree<ModuleTreeAdapter>;
pub type ModuleTreeIter<'a> = intrusive_collections::rbtree::Iter<'a, ModuleTreeAdapter>;

pub type FrozenModuleTree = RBTree<FrozenModuleTreeAdapter>;
pub type FrozenModuleTreeIter<'a> =
    intrusive_collections::rbtree::Iter<'a, FrozenModuleTreeAdapter>;

pub(super) enum Modules {
    Open(ModuleTree),
    Frozen(FrozenModuleTree),
}
impl Default for Modules {
    fn default() -> Self {
        Self::Open(Default::default())
    }
}
impl Modules {
    pub fn iter(&self) -> impl Iterator<Item = &Module> + '_ {
        match self {
            Self::Open(ref tree) => ModulesIter::Open(tree.iter()),
            Self::Frozen(ref tree) => ModulesIter::Frozen(tree.iter()),
        }
    }

    pub fn get<Q>(&self, name: &Q) -> Option<&Module>
    where
        Q: ?Sized + Ord,
        Ident: core::borrow::Borrow<Q>,
    {
        match self {
            Self::Open(ref tree) => tree.find(name).get(),
            Self::Frozen(ref tree) => tree.find(name).get(),
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

    pub fn freeze(&mut self) {
        if let Self::Open(ref mut modules) = self {
            let mut frozen = FrozenModuleTree::default();

            let mut open = modules.front_mut();
            while let Some(module) = open.remove() {
                frozen.insert(module.freeze());
            }

            *self = Self::Frozen(frozen);
        }
    }
}

enum ModulesIter<'a> {
    Open(ModuleTreeIter<'a>),
    Frozen(FrozenModuleTreeIter<'a>),
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
