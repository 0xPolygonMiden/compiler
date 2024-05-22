// General runtime type-information about a component.
//
// Compared to the `Module` structure for core wasm this type is pretty
// significantly different. The core wasm `Module` corresponds roughly 1-to-1
// with the structure of the wasm module itself, but instead a `LinearComponent` is
// more of a "translated" representation where the original structure is thrown
// away in favor of a more optimized representation. The considerations for this
// are:
//
// * This representation of a `LinearComponent` avoids the need to create a `PrimaryMap` of some
//   form for each of the index spaces within a component. This is less so an issue about
//   allocations and moreso that this information generally just isn't needed any time after
//   instantiation. Avoiding creating these altogether helps components be lighter weight at runtime
//   and additionally accelerates instantiation.
//
// * Finally by performing this sort of dataflow analysis we are capable of identifying what
//   adapters need trampolines. For example this tracks when host functions are lowered which
//   enables us to enumerate what trampolines are required to enter into a component. Additionally
//   (eventually) this will track all of the "fused" adapter functions where a function from one
//   component instance is lifted and then lowered into another component instance.
//
// Note, however, that the current design of `LinearComponent` has fundamental
// limitations which it was not designed for. For example there is no feasible
// way to implement either importing or exporting a component itself from the
// root component. Currently we rely on the ability to have static knowledge of
// what's coming from the host which at this point can only be either functions
// or core wasm modules. Additionally one flat list of initializers for a
// component are produced instead of initializers-per-component which would
// otherwise be required to export a component from a component.
//
// For now this tradeoff is made as it aligns well with the intended use case
// for components in an embedding. This may need to be revisited though if the
// requirements of embeddings change over time.

// Based on wasmtime v16.0 Wasm component translation

use indexmap::IndexMap;
use miden_hir::cranelift_entity::PrimaryMap;

use crate::{
    component::*,
    module::types::{EntityIndex, MemoryIndex, WasmType},
};

/// Metadata as a result of translating a component.
pub struct LinearComponentTranslation {
    /// Serializable information that will be emitted into the final artifact.
    pub component: LinearComponent,

    /// Metadata about required trampolines and what they're supposed to do.
    pub trampolines: PrimaryMap<TrampolineIndex, Trampoline>,
}

/// Run-time-type-information about a `LinearComponent`, its structure, and how to
/// instantiate it.
///
/// This type is intended to mirror the `Module` type in this crate which
/// provides all the runtime information about the structure of a module and
/// how it works.
///
/// NB: Lots of the component model is not yet implemented in the runtime so
/// this is going to undergo a lot of churn.
#[derive(Default, Debug)]
pub struct LinearComponent {
    /// A list of typed values that this component imports.
    ///
    /// Note that each name is given an `ImportIndex` here for the next map to
    /// refer back to.
    pub import_types: PrimaryMap<ImportIndex, (String, TypeDef)>,

    /// A list of "flattened" imports that are used by this instance.
    ///
    /// This import map represents extracting imports, as necessary, from the
    /// general imported types by this component. The flattening here refers to
    /// extracting items from instances. Currently the flat imports are either a
    /// host function or a core wasm module.
    ///
    /// For example if `ImportIndex(0)` pointed to an instance then this import
    /// map represent extracting names from that map, for example extracting an
    /// exported module or an exported function.
    ///
    /// Each import item is keyed by a `RuntimeImportIndex` which is referred to
    /// by types below whenever something refers to an import. The value for
    /// each `RuntimeImportIndex` in this map is the `ImportIndex` for where
    /// this items comes from (which can be associated with a name above in the
    /// `import_types` array) as well as the list of export names if
    /// `ImportIndex` refers to an instance. The export names array represents
    /// recursively fetching names within an instance.
    pub imports: PrimaryMap<RuntimeImportIndex, (ImportIndex, Vec<String>)>,

    /// A list of this component's exports, indexed by either position or name.
    pub exports: IndexMap<String, Export>,

    /// Initializers that must be processed when instantiating this component.
    ///
    /// This list of initializers does not correspond directly to the component
    /// itself. The general goal with this is that the recursive nature of
    /// components is "flattened" with an array like this which is a linear
    /// sequence of instructions of how to instantiate a component.
    pub initializers: Vec<GlobalInitializer>,

    /// The number of runtime instances (maximum `RuntimeInstanceIndex`) created
    /// when instantiating this component.
    pub num_runtime_instances: u32,

    /// Same as `num_runtime_instances`, but for `RuntimeComponentInstanceIndex`
    /// instead.
    pub num_runtime_component_instances: u32,

    /// The number of runtime memories (maximum `RuntimeMemoryIndex`) needed to
    /// instantiate this component.
    pub num_runtime_memories: u32,

    /// The number of runtime reallocs (maximum `RuntimeReallocIndex`) needed to
    /// instantiate this component.
    pub num_runtime_reallocs: u32,

    /// Same as `num_runtime_reallocs`, but for post-return functions.
    pub num_runtime_post_returns: u32,

    /// WebAssembly type signature of all trampolines.
    pub trampolines: PrimaryMap<TrampolineIndex, SignatureIndex>,

    /// The number of lowered host functions (maximum `LoweredIndex`) needed to
    /// instantiate this component.
    pub num_lowerings: u32,

    /// Maximal number of tables that required at runtime for resource-related
    /// information in this component.
    pub num_resource_tables: usize,

    /// Total number of resources both imported and defined within this
    /// component.
    pub num_resources: u32,

    /// Metadata about imported resources and where they are within the runtime
    /// imports array.
    ///
    /// This map is only as large as the number of imported resources.
    pub imported_resources: PrimaryMap<ResourceIndex, RuntimeImportIndex>,

    /// Metadata about which component instances defined each resource within
    /// this component.
    ///
    /// This is used to determine which set of instance flags are inspected when
    /// testing reentrance.
    pub defined_resource_instances: PrimaryMap<DefinedResourceIndex, RuntimeComponentInstanceIndex>,
}

#[allow(dead_code)]
impl LinearComponent {
    /// Attempts to convert a resource index into a defined index.
    ///
    /// Returns `None` if `idx` is for an imported resource in this component or
    /// `Some` if it's a locally defined resource.
    pub fn defined_resource_index(&self, idx: ResourceIndex) -> Option<DefinedResourceIndex> {
        let idx = idx.as_u32().checked_sub(self.imported_resources.len() as u32)?;
        Some(DefinedResourceIndex::from_u32(idx))
    }

    /// Converts a defined resource index to a component-local resource index
    /// which includes all imports.
    pub fn resource_index(&self, idx: DefinedResourceIndex) -> ResourceIndex {
        ResourceIndex::from_u32(self.imported_resources.len() as u32 + idx.as_u32())
    }
}

/// GlobalInitializer instructions to get processed when instantiating a component
///
/// The variants of this enum are processed during the instantiation phase of
/// a component in-order from front-to-back. These are otherwise emitted as a
/// component is parsed and read and translated.
#[derive(Debug)]
pub enum GlobalInitializer {
    /// A core wasm module is being instantiated.
    ///
    /// This will result in a new core wasm instance being created, which may
    /// involve running the `start` function of the instance as well if it's
    /// specified. This largely delegates to the same standard instantiation
    /// process as the rest of the core wasm machinery already uses.
    InstantiateModule(InstantiateModule),

    /// A host function is being lowered, creating a core wasm function.
    LowerImport {
        /// The index of the lowered function that's being created.
        ///
        /// This is guaranteed to be the `n`th `LowerImport` instruction
        /// if the index is `n`.
        index: LoweredIndex,

        /// The index of the imported host function that is being lowered.
        ///
        /// It's guaranteed that this `RuntimeImportIndex` points to a function.
        import: RuntimeImportIndex,
    },

    /// A core wasm linear memory
    ///
    /// This instruction indicates that the `index`th core wasm linear memory
    /// needs to be extracted from the `export` specified, a pointer to a
    /// previously created module instance.  This lowering is then used in the
    /// future by pointers from `CanonicalOptions`.
    ExtractMemory(ExtractMemory),

    /// Same as `ExtractMemory`, except it's extracting a function pointer to be
    /// used as a `realloc` function.
    ExtractRealloc(ExtractRealloc),

    /// Same as `ExtractMemory`, except it's extracting a function pointer to be
    /// used as a `post-return` function.
    ExtractPostReturn(ExtractPostReturn),

    /// Declares a new defined resource within this component.
    ///
    /// Contains information about the destructor, for example.
    Resource(Resource),
}

/// Metadata for extraction of a memory of what's being extracted and where it's
/// going.
#[derive(Debug)]
pub struct ExtractMemory {
    /// The index of the memory being defined.
    pub index: RuntimeMemoryIndex,
    /// Where this memory is being extracted from.
    pub export: CoreExport<MemoryIndex>,
}

/// Same as `ExtractMemory` but for the `realloc` canonical option.
#[derive(Debug)]
pub struct ExtractRealloc {
    /// The index of the realloc being defined.
    pub index: RuntimeReallocIndex,
    /// Where this realloc is being extracted from.
    pub def: CoreDef,
}

/// Same as `ExtractMemory` but for the `post-return` canonical option.
#[derive(Debug)]
pub struct ExtractPostReturn {
    /// The index of the post-return being defined.
    pub index: RuntimePostReturnIndex,
    /// Where this post-return is being extracted from.
    pub def: CoreDef,
}

/// Different methods of instantiating a core wasm module.
#[derive(Debug)]
pub enum InstantiateModule {
    /// A module defined within this component is being instantiated.
    ///
    /// Note that this is distinct from the case of imported modules because the
    /// order of imports required is statically known and can be pre-calculated
    /// to avoid string lookups related to names at runtime, represented by the
    /// flat list of arguments here.
    Static(StaticModuleIndex, Box<[CoreDef]>),

    /// An imported module is being instantiated.
    ///
    /// This is similar to `Upvar` but notably the imports are provided as a
    /// two-level named map since import resolution order needs to happen at
    /// runtime.
    Import(RuntimeImportIndex, IndexMap<String, IndexMap<String, CoreDef>>),
}

/// Definition of a core wasm item and where it can come from within a
/// component.
///
/// Note that this is sort of a result of data-flow-like analysis on a component
/// during translation of the component itself. References to core wasm items
/// are "translated" to either referring to a previous instance or to some sort of
/// lowered host import.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum CoreDef {
    /// This item refers to an export of a previously instantiated core wasm
    /// instance.
    Export(CoreExport<EntityIndex>),
    /// This is a reference to a wasm global which represents the
    /// runtime-managed flags for a wasm instance.
    InstanceFlags(RuntimeComponentInstanceIndex),
    /// This is a reference to a trampoline which is described in the
    /// `trampolines` array.
    Trampoline(TrampolineIndex),
}

impl<T> From<CoreExport<T>> for CoreDef
where
    EntityIndex: From<T>,
{
    fn from(export: CoreExport<T>) -> CoreDef {
        CoreDef::Export(export.map_index(|i| i.into()))
    }
}

/// Identifier of an exported item from a core WebAssembly module instance.
///
/// Note that the `T` here is the index type for exports which can be
/// identified by index. The `T` is monomorphized with types like
/// [`EntityIndex`] or [`FuncIndex`].
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CoreExport<T> {
    /// The instance that this item is located within.
    ///
    /// Note that this is intended to index the `instances` map within a
    /// component. It's validated ahead of time that all instance pointers
    /// refer only to previously-created instances.
    pub instance: RuntimeInstanceIndex,

    /// The item that this export is referencing, either by name or by index.
    pub item: ExportItem<T>,
}

impl<T> CoreExport<T> {
    /// Maps the index type `T` to another type `U` if this export item indeed
    /// refers to an index `T`.
    pub fn map_index<U>(self, f: impl FnOnce(T) -> U) -> CoreExport<U> {
        CoreExport {
            instance: self.instance,
            item: match self.item {
                ExportItem::Index(i) => ExportItem::Index(f(i)),
                ExportItem::Name(s) => ExportItem::Name(s),
            },
        }
    }
}

/// An index at which to find an item within a runtime instance.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ExportItem<T> {
    /// An exact index that the target can be found at.
    ///
    /// This is used where possible to avoid name lookups at runtime during the
    /// instantiation process. This can only be used on instances where the
    /// module was statically known at translation time, however.
    Index(T),

    /// An item which is identified by a name, so at runtime we need to
    /// perform a name lookup to determine the index that the item is located
    /// at.
    ///
    /// This is used for instantiations of imported modules, for example, since
    /// the precise shape of the module is not known.
    Name(String),
}

/// Possible exports from a component.
#[derive(Debug, Clone)]
pub enum Export {
    /// A lifted function being exported which is an adaptation of a core wasm
    /// function.
    LiftedFunction {
        /// The component function type of the function being created.
        ty: TypeFuncIndex,
        /// Which core WebAssembly export is being lifted.
        func: CoreDef,
        /// Any options, if present, associated with this lifting.
        options: CanonicalOptions,
    },
    /// A module defined within this component is exported.
    ModuleStatic(StaticModuleIndex),
    /// A module imported into this component is exported.
    ModuleImport(RuntimeImportIndex),
    /// A nested instance is being exported which has recursively defined
    /// `Export` items.
    Instance(IndexMap<String, Export>),
    /// An exported type from a component or instance, currently only
    /// informational.
    Type(TypeDef),
}

/// Canonical ABI options associated with a lifted or lowered function.
#[derive(Debug, Clone)]
pub struct CanonicalOptions {
    /// The component instance that this bundle was associated with.
    pub instance: RuntimeComponentInstanceIndex,

    /// The encoding used for strings.
    pub string_encoding: StringEncoding,

    /// The memory used by these options, if specified.
    pub memory: Option<RuntimeMemoryIndex>,

    /// The realloc function used by these options, if specified.
    pub realloc: Option<RuntimeReallocIndex>,

    /// The post-return function used by these options, if specified.
    pub post_return: Option<RuntimePostReturnIndex>,
}

/// Possible encodings of strings within the component model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[repr(u8)]
pub enum StringEncoding {
    Utf8,
    Utf16,
    CompactUtf16,
}

/// Description of a new resource declared in a `GlobalInitializer::Resource`
/// variant.
///
/// This will have the effect of initializing runtime state for this resource,
/// namely the destructor is fetched and stored.
#[derive(Debug)]
pub struct Resource {
    /// The local index of the resource being defined.
    pub index: DefinedResourceIndex,
    /// Core wasm representation of this resource.
    pub rep: WasmType,
    /// Optionally-specified destructor and where it comes from.
    pub dtor: Option<CoreDef>,
    /// Which component instance this resource logically belongs to.
    pub instance: RuntimeComponentInstanceIndex,
}

/// A list of all possible trampolines that may be required to translate a
/// component completely.
///
/// All trampolines have a core wasm function signature associated with them
/// which is stored in the `Component::trampolines` array.
#[derive(Debug)]
pub enum Trampoline {
    /// Description of a lowered import used in conjunction with
    /// `GlobalInitializer::LowerImport`.
    LowerImport {
        /// The runtime lowering state that this trampoline will access.
        index: LoweredIndex,

        /// The type of the function that is being lowered, as perceived by the
        /// component doing the lowering.
        lower_ty: TypeFuncIndex,

        /// The canonical ABI options used when lowering this function specified
        /// in the original component.
        options: CanonicalOptions,
    },

    /// A small adapter which simply traps, used for degenerate lift/lower
    /// combinations.
    AlwaysTrap,

    /// A `resource.new` intrinsic which will inject a new resource into the
    /// table specified.
    ResourceNew(TypeResourceTableIndex),

    /// Same as `ResourceNew`, but for the `resource.rep` intrinsic.
    ResourceRep(TypeResourceTableIndex),

    /// Same as `ResourceNew`, but for the `resource.drop` intrinsic.
    ResourceDrop(TypeResourceTableIndex),

    /// An intrinsic used by FACT-generated modules which will transfer an owned
    /// resource from one table to another. Used in component-to-component
    /// adapter trampolines.
    ResourceTransferOwn,

    /// Same as `ResourceTransferOwn` but for borrows.
    ResourceTransferBorrow,

    /// An intrinsic used by FACT-generated modules which indicates that a call
    /// is being entered and resource-related metadata needs to be configured.
    ///
    /// Note that this is currently only invoked when borrowed resources are
    /// detected, otherwise this is "optimized out".
    ResourceEnterCall,

    /// Same as `ResourceEnterCall` except for when exiting a call.
    ResourceExitCall,
}
