use alloc::collections::BTreeMap;
use core::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use miden_core::crypto::hash::RpoDigest;

use self::formatter::PrettyPrint;
use super::*;

mod interface;

pub use interface::*;

/// Canonical ABI options associated with a lifted or lowered function.
#[derive(Debug, Clone)]
pub struct CanonicalOptions {
    /// The realloc function used by these options, if specified.
    pub realloc: Option<FunctionIdent>,
    /// The post-return function used by these options, if specified.
    pub post_return: Option<FunctionIdent>,
}

/// A component import translated from a Wasm component import that is following
/// the Wasm Component Model Canonical ABI.
#[derive(Debug, Clone)]
pub struct CanonAbiImport {
    /// The interfact function name that is being imported
    pub interface_function: InterfaceFunctionIdent,
    /// The component(lifted) type of the imported function
    pub function_ty: FunctionType,
    /// The MAST root hash of the function to be used in codegen
    pub digest: RpoDigest,
    /// Any options associated with this import
    pub options: CanonicalOptions,
}

/// A Miden (sdklib, tx kernel) function import that is following the Miden ABI.
#[derive(Debug, Clone)]
pub struct MidenAbiImport {
    /// The Miden function type as it is defined in the MASM
    pub function_ty: FunctionType,
    /// The MAST root hash of the function to be used in codegen
    pub digest: RpoDigest,
}

/// A component import
#[derive(Debug, Clone)]
pub enum ComponentImport {
    /// A Wasm import that is following the Wasm Component Model Canonical ABI
    CanonAbiImport(CanonAbiImport),
    /// A Miden import that is following the Miden ABI
    MidenAbiImport(MidenAbiImport),
}

impl ComponentImport {
    pub fn digest(&self) -> RpoDigest {
        match self {
            ComponentImport::CanonAbiImport(import) => import.digest,
            ComponentImport::MidenAbiImport(import) => import.digest,
        }
    }

    pub fn unwrap_canon_abi_import(&self) -> &CanonAbiImport {
        match self {
            ComponentImport::CanonAbiImport(import) => import,
            _ => panic!("Expected CanonAbiImport"),
        }
    }
}

impl fmt::Display for ComponentImport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pretty_print(f)
    }
}

impl formatter::PrettyPrint for ComponentImport {
    fn render(&self) -> formatter::Document {
        use crate::formatter::*;
        let function_ty_str = match self {
            ComponentImport::CanonAbiImport(import) => import.function_ty.to_string(),
            ComponentImport::MidenAbiImport(import) => import.function_ty.to_string(),
        };
        let name = match self {
            ComponentImport::CanonAbiImport(import) => {
                format!("{} ", import.interface_function)
            }
            ComponentImport::MidenAbiImport(_import) => "".to_string(),
        };
        const_text("(")
            + text(name)
            + const_text("(")
            + const_text("digest")
            + const_text(" ")
            + display(self.digest())
            + const_text(")")
            + const_text(" ")
            + const_text("(")
            + const_text("type")
            + const_text(" ")
            + text(function_ty_str)
            + const_text(")")
            + const_text(")")
    }
}

/// The name of a exported function
#[derive(
    Debug, Clone, Ord, PartialEq, PartialOrd, Eq, Hash, derive_more::From, derive_more::Into,
)]
pub struct FunctionExportName(Symbol);

/// A component export
#[derive(Debug)]
pub struct ComponentExport {
    /// The module function that is being exported
    pub function: FunctionIdent,
    /// The component(lifted) type of the exported function
    pub function_ty: FunctionType,
    /// Any options associated with this export
    pub options: CanonicalOptions,
}

/// A [Component] is a collection of [Module]s that are being compiled together as a package and
/// have exports/imports.
#[derive(Default)]
pub struct Component {
    /// This tree stores all of the modules.
    /// The modules should be stored in a topological order
    modules: IndexMap<Ident, Box<Module>>,

    /// A list of this component's imports, indexed by function identifier
    imports: BTreeMap<FunctionIdent, ComponentImport>,

    /// A list of this component's exports, indexed by export name
    exports: BTreeMap<FunctionExportName, ComponentExport>,
}

impl Component {
    /// Create a new, empty [Component].
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return a reference to the module table for this program
    pub fn modules(&self) -> &IndexMap<Ident, Box<Module>> {
        &self.modules
    }

    pub fn to_modules(mut self) -> Vec<(Ident, Box<Module>)> {
        self.modules.drain(..).collect()
    }

    /// Return a mutable reference to the module table for this program
    pub fn modules_mut(&mut self) -> &mut IndexMap<Ident, Box<Module>> {
        &mut self.modules
    }

    /// Returns true if `name` is defined in this program.
    pub fn contains(&self, name: Ident) -> bool {
        !self.modules.contains_key(&name)
    }

    /// Look up the signature of a function in this program by `id`
    pub fn signature(&self, id: &FunctionIdent) -> Option<&Signature> {
        let module = self.modules.get(&id.module)?;
        module.function(id.function).map(|f| &f.signature)
    }

    pub fn imports(&self) -> &BTreeMap<FunctionIdent, ComponentImport> {
        &self.imports
    }

    pub fn exports(&self) -> &BTreeMap<FunctionExportName, ComponentExport> {
        &self.exports
    }

    /// Get the first module in this component
    pub fn first_module(&self) -> &Module {
        self.modules
            .values()
            .next()
            .expect("Expected at least one module in the component")
    }

    /// Extracts the single module consuming this component, panicking if there is not exactly one.
    pub fn unwrap_one_module(self) -> Box<Module> {
        assert_eq!(self.modules.len(), 1, "Expected exactly one module in the component");
        self.to_modules().drain(..).next().unwrap().1
    }
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pretty_print(f)
    }
}

impl formatter::PrettyPrint for Component {
    fn render(&self) -> formatter::Document {
        use crate::formatter::*;

        let imports = self
            .imports
            .iter()
            .map(|(id, import)| {
                const_text("(")
                    + const_text("lower")
                    + const_text(" ")
                    + import.render()
                    + const_text(" ")
                    + id.render()
                    + const_text(")")
            })
            .reduce(|acc, doc| acc + nl() + doc)
            .map(|doc| const_text(";; Component Imports") + nl() + doc)
            .unwrap_or(Document::Empty);

        let modules = self
            .modules
            .values()
            .map(PrettyPrint::render)
            .reduce(|acc, doc| acc + nl() + doc)
            .map(|doc| const_text(";; Modules") + nl() + doc)
            .unwrap_or(Document::Empty);

        let body = vec![imports, modules].into_iter().filter(|section| !section.is_empty()).fold(
            nl(),
            |a, b| {
                if matches!(a, Document::Newline) {
                    indent(4, a + b)
                } else {
                    a + nl() + indent(4, nl() + b)
                }
            },
        );

        let header = const_text("(") + const_text("component") + const_text(" ");

        if body.is_empty() {
            header + const_text(")") + nl()
        } else {
            header + body + nl() + const_text(")") + nl()
        }
    }
}

/// This struct provides an ergonomic way to construct a [Component] in an imperative fashion.
///
/// Simply create the builder, add/build one or more modules, then call `link` to obtain a
/// [Component].
pub struct ComponentBuilder<'a> {
    modules: IndexMap<Ident, Box<Module>>,
    imports: BTreeMap<FunctionIdent, ComponentImport>,
    exports: BTreeMap<FunctionExportName, ComponentExport>,
    entry: Option<FunctionIdent>,
    diagnostics: &'a miden_diagnostics::DiagnosticsHandler,
}
impl<'a> ComponentBuilder<'a> {
    pub fn new(diagnostics: &'a miden_diagnostics::DiagnosticsHandler) -> Self {
        Self {
            modules: Default::default(),
            entry: None,
            diagnostics,
            exports: Default::default(),
            imports: Default::default(),
        }
    }

    /// Set the entrypoint for the [Component] being built.
    #[inline]
    pub fn with_entrypoint(mut self, id: FunctionIdent) -> Self {
        self.entry = Some(id);
        self
    }

    /// Add `module` to the set of modules to link into the final [Component]
    ///
    /// Unlike `add_module`, this function consumes the current builder state
    /// and returns a new one, to allow for chaining builder calls together.
    ///
    /// Returns `Err` if a module with the same name already exists
    pub fn with_module(mut self, module: Box<Module>) -> Result<Self, ModuleConflictError> {
        self.add_module(module).map(|_| self)
    }

    /// Add `module` to the set of modules to link into the final [Component]
    ///
    /// Returns `Err` if a module with the same name already exists
    pub fn add_module(&mut self, module: Box<Module>) -> Result<(), ModuleConflictError> {
        let module_name = module.name;
        if self.modules.contains_key(&module_name) {
            return Err(ModuleConflictError(module_name));
        }

        self.modules.insert(module_name, module);

        Ok(())
    }

    /// Start building a [Module] with the given name.
    ///
    /// When the builder is done, the resulting [Module] will be inserted
    /// into the set of modules to be linked into the final [Component].
    pub fn module<S: Into<Ident>>(&mut self, name: S) -> ComponentModuleBuilder<'_, 'a> {
        let name = name.into();
        let module = match self.modules.shift_remove(&name) {
            None => Box::new(Module::new(name)),
            Some(module) => module,
        };
        ComponentModuleBuilder {
            cb: self,
            mb: ModuleBuilder::from(module),
        }
    }

    pub fn add_import(&mut self, function_id: FunctionIdent, import: ComponentImport) {
        self.imports.insert(function_id, import);
    }

    pub fn add_export(&mut self, name: FunctionExportName, export: ComponentExport) {
        self.exports.insert(name, export);
    }

    pub fn build(self) -> Component {
        assert!(!self.modules.is_empty(), "Cannot build a component with no modules");
        Component {
            modules: self.modules,
            imports: self.imports,
            exports: self.exports,
        }
    }
}

/// This is used to build a [Module] from a [ComponentBuilder].
///
/// It is basically just a wrapper around [ModuleBuilder], but overrides two things:
///
/// * `build` will add the module to the [ComponentBuilder] directly, rather than returning it
/// * `function` will delegate to [ComponentFunctionBuilder] which plays a similar role to this
/// struct, but for [ModuleFunctionBuilder].
pub struct ComponentModuleBuilder<'a, 'b: 'a> {
    cb: &'a mut ComponentBuilder<'b>,
    mb: ModuleBuilder,
}
impl<'a, 'b: 'a> ComponentModuleBuilder<'a, 'b> {
    /// Start building a [Function] wwith the given name and signature.
    pub fn function<'c, 'd: 'c, S: Into<Ident>>(
        &'d mut self,
        name: S,
        signature: Signature,
    ) -> Result<ComponentFunctionBuilder<'c, 'd>, SymbolConflictError> {
        Ok(ComponentFunctionBuilder {
            diagnostics: self.cb.diagnostics,
            fb: self.mb.function(name, signature)?,
        })
    }

    /// Build the current [Module], adding it to the [ComponentBuilder].
    ///
    /// Returns `err` if a module with that name already exists.
    pub fn build(self) -> Result<(), ModuleConflictError> {
        let pb = self.cb;
        let mb = self.mb;

        pb.add_module(mb.build())?;
        Ok(())
    }
}
impl<'a, 'b: 'a> Deref for ComponentModuleBuilder<'a, 'b> {
    type Target = ModuleBuilder;

    fn deref(&self) -> &Self::Target {
        &self.mb
    }
}
impl<'a, 'b: 'a> DerefMut for ComponentModuleBuilder<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mb
    }
}
impl<'a, 'b: 'a> AsRef<ModuleBuilder> for ComponentModuleBuilder<'a, 'b> {
    fn as_ref(&self) -> &ModuleBuilder {
        &self.mb
    }
}
impl<'a, 'b: 'a> AsMut<ModuleBuilder> for ComponentModuleBuilder<'a, 'b> {
    fn as_mut(&mut self) -> &mut ModuleBuilder {
        &mut self.mb
    }
}

/// This is used to build a [Function] from a [ComponentModuleBuilder].
///
/// It is basically just a wrapper around [ModuleFunctionBuilder], but overrides
/// `build` to use the [miden_diagnostics::DiagnosticsHandler] of the parent
/// [ComponentBuilder].
pub struct ComponentFunctionBuilder<'a, 'b: 'a> {
    diagnostics: &'b miden_diagnostics::DiagnosticsHandler,
    fb: ModuleFunctionBuilder<'a>,
}
impl<'a, 'b: 'a> ComponentFunctionBuilder<'a, 'b> {
    /// Build the current function
    pub fn build(self) -> Result<FunctionIdent, InvalidFunctionError> {
        let diagnostics = self.diagnostics;
        self.fb.build(diagnostics)
    }
}
impl<'a, 'b: 'a> Deref for ComponentFunctionBuilder<'a, 'b> {
    type Target = ModuleFunctionBuilder<'a>;

    fn deref(&self) -> &Self::Target {
        &self.fb
    }
}
impl<'a, 'b: 'a> DerefMut for ComponentFunctionBuilder<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fb
    }
}
impl<'a, 'b: 'a> AsRef<ModuleFunctionBuilder<'a>> for ComponentFunctionBuilder<'a, 'b> {
    fn as_ref(&self) -> &ModuleFunctionBuilder<'a> {
        &self.fb
    }
}
impl<'a, 'b: 'a> AsMut<ModuleFunctionBuilder<'a>> for ComponentFunctionBuilder<'a, 'b> {
    fn as_mut(&mut self) -> &mut ModuleFunctionBuilder<'a> {
        &mut self.fb
    }
}
