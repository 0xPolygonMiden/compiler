use std::{collections::BTreeSet, fmt, path::Path, sync::Arc};

use intrusive_collections::{intrusive_adapter, RBTree, RBTreeAtomicLink};
use miden_assembly::{
    ast::{self, ModuleKind},
    LibraryPath,
};
use midenc_hir::{
    diagnostics::{Report, SourceFile, SourceSpan, Span, Spanned},
    formatter::PrettyPrint,
    FunctionIdent, Ident, Symbol,
};

use super::{function::Functions, FrozenFunctionList, Function, ModuleImportInfo};

/// This represents a single compiled Miden Assembly module in a form that is
/// designed to integrate well with the rest of our IR. You can think of this
/// as an intermediate representation corresponding to the Miden Assembly AST,
/// i.e. [miden_assembly::ast::Module].
///
/// Functions are stored in a [Module] in a linked list, so as to allow precise
/// ordering of functions in the module body. We typically access all of the
/// functions in a given module, so O(1) access to a specific function is not
/// of primary importance.
#[derive(Clone)]
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

    /// Parse a [Module] from `source` using the given [ModuleKind] and [LibraryPath]
    pub fn parse(
        kind: ModuleKind,
        path: LibraryPath,
        source: Arc<SourceFile>,
    ) -> Result<Self, Report> {
        let span = source.source_span();
        let mut parser = ast::Module::parser(kind);
        let ast = parser.parse(path, source)?;
        Ok(Self::from_ast(&ast, span))
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

    pub fn from_ast(ast: &ast::Module, span: SourceSpan) -> Self {
        let mut module = Self::new(ast.path().clone(), ast.kind());
        module.span = span;
        module.docs = ast.docs().map(|s| s.to_string());

        let mut imports = ModuleImportInfo::default();
        for import in ast.imports() {
            let span = import.name.span();
            let alias = Symbol::intern(import.name.as_str());
            let name = if import.is_aliased() {
                Symbol::intern(import.path.last())
            } else {
                alias
            };
            imports.insert(midenc_hir::MasmImport { span, name, alias });
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
    pub fn to_ast(&self, tracing_enabled: bool) -> Result<ast::Module, Report> {
        let mut ast = ast::Module::new(self.kind, self.name.clone()).with_span(self.span);
        ast.set_docs(self.docs.clone().map(Span::unknown));

        // Create module import table
        for ir_import in self.imports.iter() {
            let span = ir_import.span;
            let name =
                ast::Ident::new_with_span(span, ir_import.alias.as_str()).map_err(Report::msg)?;
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
                &self.imports,
                &locals,
                tracing_enabled,
            )))?;
        }

        Ok(ast)
    }

    /// Write this module to a new file under `dir`, assuming `dir` is the root directory for a
    /// program.
    ///
    /// For example, if this module is named `std::math::u64`, then it will be written to
    /// `<dir>/std/math/u64.masm`
    pub fn write_to_directory<P: AsRef<Path>>(&self, dir: P) -> std::io::Result<()> {
        let mut path = dir.as_ref().to_path_buf();
        assert!(path.is_dir());
        for component in self.name.components() {
            path.push(component.as_ref());
        }
        assert!(path.set_extension("masm"));

        let ast = self.to_ast(false).map_err(std::io::Error::other)?;
        ast.write_to_file(path)
    }
}
impl midenc_hir::formatter::PrettyPrint for Module {
    fn render(&self) -> midenc_hir::formatter::Document {
        use midenc_hir::formatter::*;

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
            doc += nl() + nl();
        }

        for (i, export) in self.reexports.iter().enumerate() {
            if i > 0 {
                doc += nl();
            }
            doc += export.render();
        }

        if !self.reexports.is_empty() {
            doc += nl() + nl();
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

    fn output_type(&self, _mode: midenc_session::OutputMode) -> midenc_session::OutputType {
        midenc_session::OutputType::Masm
    }

    fn write_to<W: std::io::Write>(
        &self,
        writer: W,
        mode: midenc_session::OutputMode,
        session: &midenc_session::Session,
    ) -> std::io::Result<()> {
        let ast = self.to_ast(false).map_err(std::io::Error::other)?;
        ast.write_to(writer, mode, session)
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
impl Clone for Modules {
    fn clone(&self) -> Self {
        let mut out = ModuleTree::default();
        for module in self.iter() {
            out.insert(Box::new(module.clone()));
        }
        Self::Open(out)
    }
}
impl Modules {
    pub fn len(&self) -> usize {
        match self {
            Self::Open(ref tree) => tree.iter().count(),
            Self::Frozen(ref tree) => tree.iter().count(),
        }
    }

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
