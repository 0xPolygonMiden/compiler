use std::{
    collections::BTreeMap,
    fmt,
    path::{Path, PathBuf},
    sync::Arc,
};

use intrusive_collections::{intrusive_adapter, LinkedList, RBTreeAtomicLink};
use miden_assembly::ast::ModuleAst;
use miden_diagnostics::{CodeMap, SourceFile, SourceSpan, Spanned};
use miden_hir::{FunctionIdent, Ident, Symbol};
use rustc_hash::FxHashMap;

use super::{FrozenFunctionListAdapter, Function, FunctionListAdapter, Import, ModuleImportInfo};

const I32_INTRINSICS: &'static str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/intrinsics/i32.masm"));
const MEM_INTRINSICS: &'static str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/intrinsics/mem.masm"));

/// This is a mapping of intrinsics module name to the raw MASM source for that module
const INTRINSICS: [(&'static str, &'static str, &'static str); 2] = [
    ("intrinsics::i32", I32_INTRINSICS, "i32.masm"),
    ("intrinsics::mem", MEM_INTRINSICS, "mem.masm"),
];

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
    /// If this module contains a program entrypoint, this is the
    /// function identifier which should be used for that purpose.
    pub entry: Option<FunctionIdent>,
    /// The modules to import, along with their local aliases
    pub imports: ModuleImportInfo,
    /// The functions defined in this module
    functions: FunctionList,
}
impl Module {
    /// Create a new, empty [Module] with the given name.
    pub fn new(name: Ident) -> Self {
        Self {
            link: Default::default(),
            span: SourceSpan::UNKNOWN,
            name,
            docs: None,
            entry: None,
            imports: Default::default(),
            functions: Default::default(),
        }
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
            None => format!("{basename}"),
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
        use miden_assembly::{ast::ModuleImports, ProcedureId};

        let module_name = name.into();
        let mut module = Self::new(module_name);
        module.span = span;
        module.docs = ast.docs().cloned();

        // HACK: We're waiting on 0xPolygonMiden/miden-vm#1110
        let imported = {
            let mut imports = BTreeMap::<_, miden_assembly::LibraryPath>::default();
            let mut invoked = BTreeMap::<_, (_, miden_assembly::LibraryPath)>::default();
            let import_paths = ast.import_paths();
            for path in import_paths.iter() {
                let alias = Symbol::intern(path.last());
                let name = Symbol::intern(path.as_ref());
                module.imports.insert(Import {
                    span: module_name.span(),
                    alias,
                    name,
                });
                imports.insert(
                    path.last().to_string(),
                    miden_assembly::LibraryPath::clone(path),
                );
            }
            for (id, name) in ast.get_imported_procedures_map().into_iter() {
                let path = import_paths
                    .iter()
                    .find(|p| id == ProcedureId::from_name(name.as_ref(), p))
                    .expect("could not find module for imported procedure");
                invoked.insert(
                    id.clone(),
                    (name.clone(), miden_assembly::LibraryPath::clone(path)),
                );
            }
            ModuleImports::new(imports, invoked)
        };

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
    pub fn unwrap_frozen_functions(&self) -> &LinkedList<FrozenFunctionListAdapter> {
        match self.functions {
            FunctionList::Frozen(ref functions) => functions,
            FunctionList::Open(_) => panic!("expected module to be frozen"),
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

        if let Some(entry) = self.entry {
            f.write_str("begin\n")?;
            writeln!(f, "    exec.{}", entry.function)?;
            f.write_str("end\n")?;
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

enum FunctionList {
    Open(LinkedList<FunctionListAdapter>),
    Frozen(LinkedList<FrozenFunctionListAdapter>),
}
impl Default for FunctionList {
    fn default() -> Self {
        Self::Open(Default::default())
    }
}
impl FunctionList {
    pub fn iter(&self) -> FunctionListIter<'_> {
        match self {
            Self::Open(ref list) => FunctionListIter::Open(list.iter()),
            Self::Frozen(ref list) => FunctionListIter::Frozen(list.iter()),
        }
    }

    pub fn push_back(&mut self, function: Box<Function>) {
        match self {
            Self::Open(ref mut list) => {
                list.push_back(function);
            }
            Self::Frozen(_) => panic!("cannot insert function into frozen module"),
        }
    }

    fn freeze(&mut self) {
        if let Self::Open(ref mut functions) = self {
            let mut frozen = LinkedList::<FrozenFunctionListAdapter>::default();

            while let Some(function) = functions.pop_front() {
                frozen.push_back(Arc::from(function));
            }

            *self = Self::Frozen(frozen);
        }
    }
}

enum FunctionListIter<'a> {
    Open(intrusive_collections::linked_list::Iter<'a, FunctionListAdapter>),
    Frozen(intrusive_collections::linked_list::Iter<'a, FrozenFunctionListAdapter>),
}
impl<'a> Iterator for FunctionListIter<'a> {
    type Item = &'a Function;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Open(ref mut iter) => iter.next(),
            Self::Frozen(ref mut iter) => iter.next(),
        }
    }
}

impl Module {
    /// This helper loads the named module from the set of intrinsics modules defined in this crate.
    ///
    /// Expects the fully-qualified name to be given, e.g. `intrinsics::mem`
    pub fn load_intrinsic<N: AsRef<str>>(name: N, codemap: &CodeMap) -> Option<Self> {
        use miden_diagnostics::FileName;

        let name = name.as_ref();
        let (name, source, filename) = INTRINSICS.iter().copied().find(|(n, _, _)| *n == name)?;
        let id = codemap.add(FileName::Virtual(filename.into()), source.to_string());
        let source_file = codemap.get(id).unwrap();
        Some(Self::parse_source_file(source_file, name, codemap).expect("invalid module"))
    }
}
