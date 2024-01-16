use std::hash::Hash;
use wasmparser::types;

macro_rules! indices {
    ($(
        $(#[$a:meta])*
        pub struct $name:ident(u32);
    )*) => ($(
        $(#[$a])*
        #[derive(
            Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug,
        )]
        #[repr(transparent)]
        pub struct $name(u32);
        miden_hir::cranelift_entity::entity_impl!($name);
    )*);
}

indices! {
    // ========================================================================
    // Like Core WebAssembly, the Component Model places each definition into
    // one of a fixed set of index spaces, allowing the definition to be
    // referred to by subsequent definitions (in the text and binary format) via
    // a nonnegative integral index. When defining, validating and executing a
    // component, there are 5 component-level index spaces:

    // (component) functions
    // (component) values
    // (component) types
    // component instances
    // components

    // and 2 additional core index spaces that contain core definition
    // introduced by the Component Model that are not in WebAssembly 1.0 (yet:
    // the module-linking proposal would add them):

    // module instances
    // modules

    // for a total of 12 index spaces that need to be maintained by an implementation when, e.g., validating a component.

    // These indices are used during compile time only when we're translating a
    // component at this time. The actual indices are not persisted beyond the
    // compile phase to when we're actually working with the component at
    // runtime.

    /// Index within a component's component type index space.
    pub struct ComponentTypeIndex(u32);

    /// Index within a component's module index space.
    pub struct ModuleIndex(u32);

    /// Index within a component's component index space.
    pub struct ComponentIndex(u32);

    /// Index within a component's module instance index space.
    pub struct ModuleInstanceIndex(u32);

    /// Index within a component's component instance index space.
    pub struct ComponentInstanceIndex(u32);

    /// Index within a component's component function index space.
    pub struct ComponentFuncIndex(u32);


    /// Index into the global list of modules found within an entire component.
    ///
    /// Module translations are saved on the side to get fully compiled after
    /// the original component has finished being translated.
    pub struct StaticModuleIndex(u32);

    // ========================================================================
    // Index types used to identify modules and components during compilation.

    /// Index into a "closed over variables" list for components used to
    /// implement outer aliases. For more information on this see the
    /// documentation for the `LexicalScope` structure.
    pub struct ModuleUpvarIndex(u32);

    /// Same as `ModuleUpvarIndex` but for components.
    pub struct ComponentUpvarIndex(u32);

    /// Same as `StaticModuleIndex` but for components.
    pub struct StaticComponentIndex(u32);

}

/// Equivalent of `EntityIndex` but for the component model instead of core
/// wasm.
#[derive(Debug, Clone, Copy)]
pub enum ComponentItem {
    Func(ComponentFuncIndex),
    Module(ModuleIndex),
    Component(ComponentIndex),
    ComponentInstance(ComponentInstanceIndex),
    Type(types::ComponentAnyTypeId),
}
