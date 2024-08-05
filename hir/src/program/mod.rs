mod linker;

use alloc::collections::BTreeMap;
use core::ops::{Deref, DerefMut};

use intrusive_collections::RBTree;
use miden_assembly::library::CompiledLibrary;
use miden_core::crypto::hash::RpoDigest;

pub use self::linker::Linker;
use crate::{
    diagnostics::{DiagnosticsHandler, Report},
    *,
};

/// A [Program] is a collection of [Module]s that are being compiled together as a package.
///
/// This is primarily used for storing/querying data which must be shared across modules:
///
/// * The set of global variables which will be allocated on the global heap
/// * The set of modules and functions which have been defined
///
/// When translating to Miden Assembly, we need something like this to allow us to perform some
/// basic linker tasks prior to emitting the textual MASM which will be fed to the Miden VM.
///
/// This structure is intended to be allocated via [std::sync::Arc], so that it can be shared
/// across multiple threads which are emitting/compiling modules at the same time. It is designed
/// so that individual fields are locked, rather than the structure as a whole, to minimize
/// contention. The intuition is that, in general, changes at the [Program] level are relatively
/// infrequent, i.e. only when declaring a new [Module], or [GlobalVariable], do we actually need to
/// mutate the structure. In all other situations, changes are scoped at the [Module] level.
#[derive(Default)]
pub struct Program {
    /// This tree stores all of the modules being compiled as part of the current program.
    modules: RBTree<ModuleTreeAdapter>,
    /// The set of compiled libraries this program links against
    libraries: BTreeMap<RpoDigest, CompiledLibrary>,
    /// If set, this field is used to determine which function is the entrypoint for the program.
    ///
    /// When generating Miden Assembly, this will determine whether or not we're emitting
    /// a program or just a collection of modules; and in the case of the former, what code
    /// to emit in the root code block.
    ///
    /// If not present, but there is a function in the program with the `entrypoint` attribute,
    /// that function will be used instead. If there are multiple functions with the `entrypoint`
    /// attribute, and this field is `None`, the linker will raise an error.
    entrypoint: Option<FunctionIdent>,
    /// The data segments gathered from all modules in the program, and laid out in address order.
    segments: DataSegmentTable,
    /// The global variable table produced by linking the global variable tables of all
    /// modules in this program. The layout of this table corresponds to the layout of
    /// global variables in the linear memory heap at runtime.
    globals: GlobalVariableTable,
}

impl Program {
    /// Create a new, empty [Program].
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add to the set of libraries this [Program] will be assembled with
    pub fn add_library(&mut self, lib: CompiledLibrary) {
        self.libraries.insert(*lib.digest(), lib);
    }

    /// Returns true if this program has a defined entrypoint
    pub fn has_entrypoint(&self) -> bool {
        self.entrypoint().is_none()
    }

    /// Returns true if this program is executable.
    ///
    /// An executable program is one which has an entrypoint that will be called
    /// after the program is loaded.
    pub fn is_executable(&self) -> bool {
        self.has_entrypoint()
    }

    /// Returns the [FunctionIdent] corresponding to the program entrypoint
    pub fn entrypoint(&self) -> Option<FunctionIdent> {
        self.entrypoint.or_else(|| self.modules.iter().find_map(|m| m.entrypoint()))
    }

    /// Return a reference to the module table for this program
    pub fn modules(&self) -> &RBTree<ModuleTreeAdapter> {
        &self.modules
    }

    /// Return a mutable reference to the module table for this program
    pub fn modules_mut(&mut self) -> &mut RBTree<ModuleTreeAdapter> {
        &mut self.modules
    }

    /// Return the set of libraries this program links against
    pub fn libraries(&self) -> &BTreeMap<RpoDigest, CompiledLibrary> {
        &self.libraries
    }

    /// Return the set of libraries this program links against as a mutable reference
    pub fn libraries_mut(&mut self) -> &mut BTreeMap<RpoDigest, CompiledLibrary> {
        &mut self.libraries
    }

    /// Return a reference to the data segment table for this program
    pub fn segments(&self) -> &DataSegmentTable {
        &self.segments
    }

    /// Get a reference to the global variable table for this program
    pub fn globals(&self) -> &GlobalVariableTable {
        &self.globals
    }

    /// Get a mutable reference to the global variable table for this program
    pub fn globals_mut(&mut self) -> &mut GlobalVariableTable {
        &mut self.globals
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
}

#[doc(hidden)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ProgramAnalysisKey;
impl crate::pass::AnalysisKey for Program {
    type Key = ProgramAnalysisKey;

    fn key(&self) -> Self::Key {
        ProgramAnalysisKey
    }
}

/// This struct provides an ergonomic way to construct a [Program] in an imperative fashion.
///
/// Simply create the builder, add/build one or more modules, then call `link` to obtain a
/// [Program].
pub struct ProgramBuilder<'a> {
    /// The set of HIR modules to link into the program
    modules: BTreeMap<Ident, Box<Module>>,
    /// The set of modules defined externally, which will be linked during assembly
    extern_modules: BTreeMap<Ident, Vec<Ident>>,
    /// The set of libraries we're linking against
    libraries: BTreeMap<RpoDigest, CompiledLibrary>,
    entry: Option<FunctionIdent>,
    diagnostics: &'a DiagnosticsHandler,
}
impl<'a> ProgramBuilder<'a> {
    pub fn new(diagnostics: &'a DiagnosticsHandler) -> Self {
        Self {
            modules: Default::default(),
            extern_modules: Default::default(),
            libraries: Default::default(),
            entry: None,
            diagnostics,
        }
    }

    /// Set the entrypoint for the [Program] being built.
    #[inline]
    pub fn with_entrypoint(mut self, id: FunctionIdent) -> Self {
        self.entry = Some(id);
        self
    }

    /// Add `module` to the set of modules to link into the final [Program]
    ///
    /// Unlike `add_module`, this function consumes the current builder state
    /// and returns a new one, to allow for chaining builder calls together.
    ///
    /// Returns `Err` if a module with the same name already exists
    pub fn with_module(mut self, module: Box<Module>) -> Result<Self, ModuleConflictError> {
        self.add_module(module).map(|_| self)
    }

    /// Add `module` to the set of modules to link into the final [Program]
    ///
    /// Returns `Err` if a module with the same name already exists
    pub fn add_module(&mut self, module: Box<Module>) -> Result<(), ModuleConflictError> {
        let module_name = module.name;
        if self.modules.contains_key(&module_name) || self.extern_modules.contains_key(&module_name)
        {
            return Err(ModuleConflictError::new(module_name));
        }

        self.modules.insert(module_name, module);

        Ok(())
    }

    /// Make the linker aware that `module` (with the given set of exports), is available to be
    /// linked against, but is already compiled to Miden Assembly, so has no HIR representation.
    ///
    /// Returns `Err` if a module with the same name already exists
    pub fn add_extern_module<E>(
        &mut self,
        module: Ident,
        exports: E,
    ) -> Result<(), ModuleConflictError>
    where
        E: IntoIterator<Item = Ident>,
    {
        if self.modules.contains_key(&module) || self.extern_modules.contains_key(&module) {
            return Err(ModuleConflictError::new(module));
        }

        self.extern_modules.insert(module, exports.into_iter().collect());

        Ok(())
    }

    /// Make the linker aware of the objects contained in the given library.
    ///
    /// Duplicate libraries/objects are ignored.
    pub fn add_library(&mut self, library: CompiledLibrary) {
        self.libraries.insert(*library.digest(), library);
    }

    /// Start building a [Module] with the given name.
    ///
    /// When the builder is done, the resulting [Module] will be inserted
    /// into the set of modules to be linked into the final [Program].
    pub fn module<S: Into<Ident>>(&mut self, name: S) -> ProgramModuleBuilder<'_, 'a> {
        let name = name.into();
        let module = match self.modules.remove(&name) {
            None => Box::new(Module::new(name)),
            Some(module) => module,
        };
        ProgramModuleBuilder {
            pb: self,
            mb: ModuleBuilder::from(module),
        }
    }

    /// Link a [Program] from the current [ProgramBuilder] state
    pub fn link(self) -> Result<Box<Program>, Report> {
        let mut linker = Linker::new(self.diagnostics);

        let entrypoint = self.entry.or_else(|| self.modules.values().find_map(|m| m.entrypoint()));
        if let Some(entry) = entrypoint {
            linker.with_entrypoint(entry)?;
        }

        linker.add_libraries(self.libraries.into_values());

        self.extern_modules.into_iter().try_for_each(|obj| linker.add_object(obj))?;
        self.modules.into_values().try_for_each(|obj| linker.add_object(obj))?;

        linker.link()
    }
}

/// This is used to build a [Module] from a [ProgramBuilder].
///
/// It is basically just a wrapper around [ModuleBuilder], but overrides two things:
///
/// * `build` will add the module to the [ProgramBuilder] directly, rather than returning it
/// * `function` will delegate to [ProgramFunctionBuilder] which plays a similar role to this
/// struct, but for [ModuleFunctionBuilder].
pub struct ProgramModuleBuilder<'a, 'b: 'a> {
    pb: &'a mut ProgramBuilder<'b>,
    mb: ModuleBuilder,
}
impl<'a, 'b: 'a> ProgramModuleBuilder<'a, 'b> {
    /// Start building a [Function] wwith the given name and signature.
    pub fn function<'c, 'd: 'c, S: Into<Ident>>(
        &'d mut self,
        name: S,
        signature: Signature,
    ) -> Result<ProgramFunctionBuilder<'c, 'd>, SymbolConflictError> {
        Ok(ProgramFunctionBuilder {
            diagnostics: self.pb.diagnostics,
            fb: self.mb.function(name, signature)?,
        })
    }

    /// Build the current [Module], adding it to the [ProgramBuilder].
    ///
    /// Returns `err` if a module with that name already exists.
    pub fn build(self) -> Result<(), ModuleConflictError> {
        let pb = self.pb;
        let mb = self.mb;

        pb.add_module(mb.build())?;
        Ok(())
    }
}
impl<'a, 'b: 'a> Deref for ProgramModuleBuilder<'a, 'b> {
    type Target = ModuleBuilder;

    fn deref(&self) -> &Self::Target {
        &self.mb
    }
}
impl<'a, 'b: 'a> DerefMut for ProgramModuleBuilder<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mb
    }
}
impl<'a, 'b: 'a> AsRef<ModuleBuilder> for ProgramModuleBuilder<'a, 'b> {
    fn as_ref(&self) -> &ModuleBuilder {
        &self.mb
    }
}
impl<'a, 'b: 'a> AsMut<ModuleBuilder> for ProgramModuleBuilder<'a, 'b> {
    fn as_mut(&mut self) -> &mut ModuleBuilder {
        &mut self.mb
    }
}

/// This is used to build a [Function] from a [ProgramModuleBuilder].
///
/// It is basically just a wrapper around [ModuleFunctionBuilder], but overrides
/// `build` to use the [DiagnosticsHandler] of the parent
/// [ProgramBuilder].
pub struct ProgramFunctionBuilder<'a, 'b: 'a> {
    diagnostics: &'b DiagnosticsHandler,
    fb: ModuleFunctionBuilder<'a>,
}
impl<'a, 'b: 'a> ProgramFunctionBuilder<'a, 'b> {
    /// Build the current function
    pub fn build(self) -> Result<FunctionIdent, Report> {
        let diagnostics = self.diagnostics;
        self.fb.build(diagnostics)
    }
}
impl<'a, 'b: 'a> Deref for ProgramFunctionBuilder<'a, 'b> {
    type Target = ModuleFunctionBuilder<'a>;

    fn deref(&self) -> &Self::Target {
        &self.fb
    }
}
impl<'a, 'b: 'a> DerefMut for ProgramFunctionBuilder<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fb
    }
}
impl<'a, 'b: 'a> AsRef<ModuleFunctionBuilder<'a>> for ProgramFunctionBuilder<'a, 'b> {
    fn as_ref(&self) -> &ModuleFunctionBuilder<'a> {
        &self.fb
    }
}
impl<'a, 'b: 'a> AsMut<ModuleFunctionBuilder<'a>> for ProgramFunctionBuilder<'a, 'b> {
    fn as_mut(&mut self) -> &mut ModuleFunctionBuilder<'a> {
        &mut self.fb
    }
}
