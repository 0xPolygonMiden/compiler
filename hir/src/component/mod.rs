use core::{
    convert::{AsMut, AsRef},
    ops::{Deref, DerefMut},
};
use intrusive_collections::RBTree;
use std::collections::BTreeMap;

use super::*;

mod interface;

pub use interface::*;

/// Represents the method by which a component function should be invoked in Miden VM
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FunctionInvocationMethod {
    /// A function should be invoked by a `call` Miden VM instruction
    Call,
    /// A function should be invoked by a `exec` Miden VM instruction
    #[default]
    Exec,
}

/// A component import
#[derive(Debug)]
pub struct ComponentImport {
    /// The interfact function name that is being imported
    pub interface_function: InterfaceFunctionIdent,
    /// The component(lifted) type of the imported function
    pub function_ty: LiftedFunctionType,
    /// The method of calling the function
    pub invoke_method: FunctionInvocationMethod,
    /// The MAST root hash of the function to be used in codegen
    pub function_mast_root_hash: MastRootHash,
}

/// The name of a exported function
#[derive(Debug, Ord, PartialEq, PartialOrd, Eq, Hash, derive_more::From, derive_more::Into)]
pub struct FunctionExportName(Symbol);

/// A component export
#[derive(Debug)]
pub struct ComponentExport {
    /// The module function that is being exported
    pub function: FunctionIdent,
    /// The component(lifted) type of the exported function
    pub function_ty: LiftedFunctionType,
    /// The method of calling the function
    pub invoke_method: FunctionInvocationMethod,
}

/// A [Component] is a collection of [Module]s that are being compiled together as a package and have exports/imports.
#[derive(Default)]
pub struct Component {
    /// This tree stores all of the modules
    modules: RBTree<ModuleTreeAdapter>,

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
    pub fn modules(&self) -> &RBTree<ModuleTreeAdapter> {
        &self.modules
    }

    /// Return a mutable reference to the module table for this program
    pub fn modules_mut(&mut self) -> &mut RBTree<ModuleTreeAdapter> {
        &mut self.modules
    }

    /// Returns true if `name` is defined in this program.
    pub fn contains(&self, name: Ident) -> bool {
        !self.modules.find(&name).is_null()
    }

    /// Look up the signature of a function in this program by `id`
    pub fn signature(&self, id: &FunctionIdent) -> Option<&Signature> {
        let module = self.modules.find(&id.module).get()?;
        module.function(id.function).map(|f| &f.signature)
    }

    pub fn imports(&self) -> &BTreeMap<FunctionIdent, ComponentImport> {
        &self.imports
    }

    pub fn exports(&self) -> &BTreeMap<FunctionExportName, ComponentExport> {
        &self.exports
    }
}

/// This struct provides an ergonomic way to construct a [Component] in an imperative fashion.
///
/// Simply create the builder, add/build one or more modules, then call `link` to obtain a [Component].
pub struct ComponentBuilder<'a> {
    modules: BTreeMap<Ident, Box<Module>>,
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
        let module = match self.modules.remove(&name) {
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
        let mut c = Component::default();
        for module in self.modules.into_values() {
            c.modules.insert(module);
        }
        c.exports = self.exports;
        c.imports = self.imports;
        c
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
