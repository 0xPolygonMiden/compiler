use std::{
    collections::BTreeSet,
    fmt,
    path::{Path, PathBuf},
    sync::Arc,
};

use intrusive_collections::{intrusive_adapter, RBTree, RBTreeAtomicLink};
use miden_assembly::{
    ast::{self, ModuleKind},
    diagnostics::{RelatedError, Report, SourceFile as MasmSourceFile},
    LibraryNamespace, LibraryPath,
};
use miden_diagnostics::{CodeMap, SourceFile, SourceIndex, SourceSpan};
use miden_hir::{formatter::PrettyPrint, FunctionIdent, Ident, Symbol};

use super::{function::Functions, FrozenFunctionList, Function, ModuleImportInfo};

#[derive(Debug, thiserror::Error)]
pub enum LoadModuleError {
    #[error("failed to load module from disk: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid path to module: '{}' is not a file", .0.display())]
    InvalidPath(PathBuf),
    #[error(transparent)]
    InvalidIdent(#[from] miden_assembly::ast::IdentError),
    #[error(transparent)]
    InvalidModulePath(#[from] miden_assembly::PathError),
    #[error(transparent)]
    InvalidNamespace(#[from] miden_assembly::library::LibraryNamespaceError),
    #[error(transparent)]
    Report(#[from] RelatedError),
}
impl From<Report> for LoadModuleError {
    fn from(report: Report) -> Self {
        Self::Report(RelatedError::new(report))
    }
}

/// This represents a single compiled Miden Assembly module in a form that is
/// designed to integrate well with the rest of our IR. You can think of this
/// as an intermediate representation corresponding to the Miden Assembly AST,
/// i.e. [miden_assembly::ast::Module].
pub struct Module {
    link: RBTreeAtomicLink,
    pub span: SourceSpan,
    /// The kind of this module, e.g. kernel or library
    pub kind: ModuleKind,
    /// The name of this module, e.g. `std::math::u64`
    pub id: Ident,
    pub name: LibraryPath,
    /// The module-scoped documentation for this module
    pub docs: Option<String>,
    /// The modules to import, along with their local aliases
    pub imports: ModuleImportInfo,
    /// The functions defined in this module
    functions: Functions,
    /// The set of re-exported functions declared in this module
    reexports: Vec<ast::ProcedureAlias>,
}
impl Module {
    /// Create a new, empty [Module] with the given name and kind.
    pub fn new(name: LibraryPath, kind: ModuleKind) -> Self {
        let id = Ident::with_empty_span(Symbol::intern(name.path()));
        Self {
            link: Default::default(),
            kind,
            span: SourceSpan::UNKNOWN,
            id,
            name,
            docs: None,
            imports: Default::default(),
            functions: Default::default(),
            reexports: Default::default(),
        }
    }

    /// Returns true if this module is a kernel module
    pub fn is_kernel(&self) -> bool {
        self.kind.is_kernel()
    }

    /// Returns true if this module is an executable module
    pub fn is_executable(&self) -> bool {
        self.kind.is_executable()
    }

    /// If this module contains a function marked with the `entrypoint` attribute,
    /// return the fully-qualified name of that function
    pub fn entrypoint(&self) -> Option<FunctionIdent> {
        if !self.is_executable() {
            return None;
        }

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
    pub fn parse_source_file(
        name: LibraryPath,
        kind: ModuleKind,
        source_file: Arc<SourceFile>,
        codemap: &CodeMap,
    ) -> Result<Self, LoadModuleError> {
        let filename = source_file.name().as_str().expect("invalid source file name");
        let module = ast::Module::parse(
            name,
            kind,
            Arc::new(MasmSourceFile::new(filename, source_file.source().to_string())),
        )?;
        let span = source_file.source_span();
        Ok(Self::from_ast(&module, span, codemap))
    }

    /// Parse a [Module] from the given file path
    pub fn parse_file<P: AsRef<Path>>(
        ns: Option<LibraryNamespace>,
        kind: ModuleKind,
        path: P,
        codemap: &CodeMap,
    ) -> Result<Self, LoadModuleError> {
        let path = path.as_ref();
        let id = codemap.add_file(path)?;
        let source_file = codemap.get(id).unwrap();
        let fallback_ns = match path.parent().and_then(|p| p.to_str()) {
            None => LibraryNamespace::Anon,
            Some(parent_dirname) => parent_dirname.parse::<LibraryNamespace>()?,
        };
        let ns = ns.unwrap_or(fallback_ns);
        let name = ast::Ident::new(path.file_stem().unwrap().to_str().unwrap())?;
        let module_path = LibraryPath::new_from_components(ns, [name]);
        let module = ast::Module::parse_file(module_path, kind, path)?;
        let span = source_file.source_span();
        Ok(Self::from_ast(&module, span, codemap))
    }

    pub fn from_ast(ast: &ast::Module, span: SourceSpan, _codemap: &CodeMap) -> Self {
        use miden_assembly::Spanned as MasmSpanned;

        let source_id = span.source_id();
        let mut module = Self::new(ast.path().clone(), ast.kind());
        module.span = span;
        module.docs = ast.docs().map(|s| s.to_string());

        let mut imports = ModuleImportInfo::default();
        for import in ast.imports() {
            let span = import.name.span();
            let start = SourceIndex::new(source_id, (span.start() as u32).into());
            let end = SourceIndex::new(source_id, (span.end() as u32).into());
            let span = SourceSpan::new(start, end);
            let alias = Symbol::intern(import.name.as_str());
            let name = if import.is_aliased() {
                Symbol::intern(import.path.last())
            } else {
                alias.clone()
            };
            imports.insert(miden_hir::MasmImport { span, name, alias });
        }

        for export in ast.procedures() {
            match export {
                ast::Export::Alias(ref alias) => {
                    module.reexports.push(alias.clone());
                }
                ast::Export::Procedure(ref proc) => {
                    let function = Function::from_ast(module.id, proc);
                    module.functions.push_back(function);
                }
            }
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

    /// Convert this module into its [miden_assembly::ast::Module] representation.
    pub fn to_ast(&self, codemap: &miden_diagnostics::CodeMap) -> Result<ast::Module, Report> {
        let source_id = self.span.source_id();
        let source_file = if let Ok(source_file) = codemap.get(source_id) {
            let file = miden_assembly::diagnostics::SourceFile::new(
                source_file.name().as_str().unwrap(),
                source_file.source().to_string(),
            );
            Some(Arc::new(file))
        } else {
            None
        };
        let span =
            miden_assembly::SourceSpan::new(self.span.start_index().0..self.span.end_index().0);
        let mut ast = ast::Module::new(self.kind, self.name.clone())
            .with_source_file(source_file)
            .with_span(span);
        ast.set_docs(self.docs.clone().map(miden_assembly::Span::unknown));

        // Create module import table
        for ir_import in self.imports.iter() {
            let ir_span = ir_import.span;
            let span =
                miden_assembly::SourceSpan::new(ir_span.start_index().0..ir_span.end_index().0);
            let name = ast::Ident::new_with_span(span, ir_import.alias.as_str())
                .map_err(|err| Report::msg(err))?;
            let path = LibraryPath::new(ir_import.name.as_str()).expect("invalid import path");
            let import = ast::Import {
                span,
                name,
                path,
                uses: 1,
            };
            let _ = ast.define_import(import);
        }

        // Translate functions
        let locals = BTreeSet::from_iter(self.functions.iter().map(|f| f.name));

        for reexport in self.reexports.iter() {
            ast.define_procedure(ast::Export::Alias(reexport.clone()))?;
        }

        for function in self.functions.iter() {
            ast.define_procedure(ast::Export::Procedure(function.to_ast(
                codemap,
                &self.imports,
                &locals,
            )))?;
        }

        Ok(ast)
    }

    /// Write this module to a new file under `dir`, assuming `dir` is the root directory for a
    /// program.
    ///
    /// For example, if this module is named `std::math::u64`, then it will be written to
    /// `<dir>/std/math/u64.masm`
    pub fn write_to_directory<P: AsRef<Path>>(
        &self,
        codemap: &miden_diagnostics::CodeMap,
        dir: P,
    ) -> std::io::Result<()> {
        use std::fs::File;

        let mut path = dir.as_ref().to_path_buf();
        assert!(path.is_dir());
        for component in self.name.components() {
            path.push(component.as_ref());
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
        let ast = self.to_ast(codemap).map_err(|err| std::io::Error::other(err))?;
        out.write_fmt(format_args!("{}", &ast))
    }
}
impl miden_hir::formatter::PrettyPrint for Module {
    fn render(&self) -> miden_hir::formatter::Document {
        use miden_hir::formatter::*;

        let mut doc = Document::Empty;
        if let Some(docs) = self.docs.as_ref() {
            let fragment =
                docs.lines().map(text).reduce(|acc, line| acc + nl() + text("#! ") + line);

            if let Some(fragment) = fragment {
                doc += fragment;
            }
        }

        for (i, import) in self.imports.iter().enumerate() {
            if i > 0 {
                doc += nl();
            }
            if import.is_aliased() {
                doc += flatten(
                    const_text("use")
                        + const_text(".")
                        + text(format!("{}", import.name))
                        + const_text("->")
                        + text(format!("{}", import.alias)),
                );
            } else {
                doc +=
                    flatten(const_text("use") + const_text(".") + text(format!("{}", import.name)));
            }
        }

        if !self.imports.is_empty() {
            doc += nl();
        }

        for (i, export) in self.reexports.iter().enumerate() {
            if i > 0 {
                doc += nl();
            }
            doc += export.render();
        }

        if !self.reexports.is_empty() {
            doc += nl();
        }

        for (i, func) in self.functions.iter().enumerate() {
            if i > 0 {
                doc += nl();
            }
            let func = func.display(&self.imports);
            doc += func.render();
        }

        doc
    }
}
impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pretty_print(f)
    }
}
impl midenc_session::Emit for Module {
    fn name(&self) -> Option<Symbol> {
        Some(self.id.as_symbol())
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
        module.id
    }
}
impl<'a> intrusive_collections::KeyAdapter<'a> for FrozenModuleTreeAdapter {
    type Key = Ident;

    #[inline]
    fn get_key(&self, module: &'a Module) -> Ident {
        module.id
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
