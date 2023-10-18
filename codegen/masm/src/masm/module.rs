use std::{collections::BTreeMap, fmt, path::Path};

use intrusive_collections::LinkedList;
use miden_diagnostics::Spanned;
use miden_hir::{FunctionIdent, Ident};
use rustc_hash::FxHashMap;

use super::{Function, FunctionListAdapter, Import, ModuleImportInfo};

const I32_INTRINSICS: &'static str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/intrinsics/i32.masm"));
const MEM_INTRINSICS: &'static str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/intrinsics/mem.masm"));

/// This represents a single compiled Miden Assembly module in a form that is
/// designed to integrate well with the rest of our IR. You can think of this
/// as an intermediate representation corresponding to the Miden Assembly AST,
/// i.e. [miden_assembly::ast::ModuleAst].
pub struct Module {
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
    pub functions: LinkedList<FunctionListAdapter>,
}
impl Module {
    /// Create a new, empty [Module] with the given name.
    pub fn new(name: Ident) -> Self {
        Self {
            name,
            docs: None,
            entry: None,
            imports: Default::default(),
            functions: Default::default(),
        }
    }

    pub fn parse_str<N: Into<Ident>>(
        source: &str,
        name: N,
    ) -> Result<Self, miden_assembly::ParsingError> {
        use miden_assembly::{
            ast::{ModuleAst, ModuleImports},
            ProcedureId,
        };
        use miden_hir::Symbol;

        let ast = ModuleAst::parse(source)?;
        let module_name = name.into();
        let mut module = Self::new(module_name);
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

        Ok(module)
    }

    /// Convert this module into its [miden_assembly::Module] representation.
    pub fn to_module_ast(&self, codemap: &miden_diagnostics::CodeMap) -> miden_assembly::Module {
        use miden_assembly::{
            self as masm,
            ast::{ModuleAst, ModuleImports},
        };

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

impl Module {
    /// This is a helper that parses and returns the predefined `intrinsics::mem` module
    pub fn mem_intrinsics() -> Self {
        Self::parse_str(MEM_INTRINSICS, "intrinsics::mem").expect("invalid module")
    }

    /// This is a helper that parses and returns the predefined `intrinsics::i32` module
    pub fn i32_intrinsics() -> Self {
        Self::parse_str(I32_INTRINSICS, "intrinsics::i32").expect("invalid module")
    }
}
