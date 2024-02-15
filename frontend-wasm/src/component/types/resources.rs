//! Implementation of resource type information
//!
//! Resource types are one of the trickier parts of the component model. Types
//! such as `list`, `record`, and `string` are considered "structural" where two
//! types are considered equal if their components are equal. For example `(list
//! $a)` and `(list $b)` are the same if `$a` and `$b` are the same. Resources,
//! however, are not as simple.
//!
//! The type of a resource can "change" sort of depending on who you are and how
//! you view it. Some examples of resources are:
//!
//! * When a resource is imported into a component the internal component
//!   doesn't know the underlying resource type, but the outer component which
//!   performed an instantiation knows that. This means that if a component
//!   imports two unique resources but is instantiated with two copies of the
//!   same resource the internal component can't know they're the same but the
//!   outer component knows they're the same.
//!
//! * Each instantiation of a component produces new resource types. This means
//!   that if a component instantiates a subcomponent twice then the resources
//!   defined in that subcomponent are considered different between the two
//!   instances.
//!
//! All this is basically to say that resources require special care. The
//! purpose of resources are to provide isolation across component boundaries
//! and strict guarantees around ownership and borrowing. Getting the type
//! information wrong can compromise on all of these guarantees which is
//! something we would ideally avoid.
//!
//! ## Interaction with `wasmparser`
//!
//! The first line of translating a component, `wasmparser`, already has quite a
//! lot of support for handling all the various special cases of resources.
//! Namely `wasmparser` has a `ResourceId` type which can be used to test
//! whether two resources are the same or unique. For example in the above
//! scenario where a component imports two resources then within that component
//! they'll have unique ids. Externally though the outer component will be able
//! to see that the ids are the same.
//!
//! Given the subtlety here the goal is to lean on `wasmparser` as much as
//! possible for this information. The thinking is "well it got things right so
//! let's not duplicate". This is one of the motivations for plumbing
//! `wasmparser`'s type information throughout `LocalInitializer` structures
//! during translation of a component. During conversion to a
//! `GlobalInitializer` is where everything is boiled away.
//!
//! ## Converting
//!
//! The purpose of this module then is to convert `wasmparser`'s view of
//! resources into our view of resources. Our goal is to determine how many
//! tables are required for each resource within a component and then from then
//! on purely talk about table indices. Each component instance will require a
//! table per-resource and this figures that all out.
//!
//! The conversion process, however, is relatively tightly intertwined with type
//! conversion in general. The "leaves" of a type may be resources but there are
//! other structures in a type such as lists, records, variants, etc. This means
//! that the `ResourcesBuilder` below is embedded within a
//! `ComponentTypesBuilder`. This also means that it's unfortuantely not easy to
//! disentangle pieces and have one nice standalone file that handles everything
//! related to type information about resources. Instead this one file was
//! chosen as the place for this doc comment but the file itself is deceptively
//! small as much of the other handling happens elsewhere in component
//! translation.
//!
//! For more details on fiddly bits see the documentation on various fields and
//! methods below.

// Based on wasmtime v16.0 Wasm component translation

use crate::component::{
    ComponentTypes, ResourceIndex, RuntimeComponentInstanceIndex, TypeResourceTable,
    TypeResourceTableIndex,
};
use rustc_hash::FxHashMap;
use wasmparser::types;

/// Builder state used to translate wasmparser's `ResourceId` types to
/// `TypeResourceTableIndex` type.
///
/// This is contained in a `ComponentTypesBuilder` but is modified quite a bit
/// manually via the `inline` phase of component instantiation.
///
/// This type crucially implements the `Clone` trait which is used to "snapshot"
/// the current state of resource translation. The purpose of `Clone` here is to
/// record translation information just before a subcomponent is instantiated to
/// restore it after the subcomponent's instantiation has completed. This is
/// done to handle instantiations of the same component multiple times
/// correctly.
///
/// Wasmparser produces one set of type information for a component, and not a
/// unique set of type information about its internals for each instantiation.
/// Each instance which results from instantiation gets a new type, but when
/// we're translating the instantiation of a component we will re-run all
/// initializers. This means that if naively implemented the `ResourceId`
/// mapping from the first instantiation will be reused by the second
/// instantiation. The snapshotting behavior and restoration guarantees that
/// whenever a subcomponent is visited and instantiated it's guaranteed that
/// there's no registered information for its `ResourceId` definitions within
/// this builder.
///
/// Note that `ResourceId` references are guaranteed to be "local" in the sense
/// that if a resource is defined within a component then the ID it's assigned
/// internally within a component is different than the ID when it's
/// instantiated (since all instantiations produce new types). This means that
/// when throwing away state built-up from within a component that's also
/// reasonable because the information should never be used after a component is
/// left anyway.
#[derive(Default, Clone)]
pub struct ResourcesBuilder {
    /// A cache of previously visited `ResourceId` items and which table they
    /// correspond to. This is lazily populated as resources are visited and is
    /// exclusively used by the `convert` function below.
    resource_id_to_table_index: FxHashMap<types::ResourceId, TypeResourceTableIndex>,

    /// A cache of the origin resource type behind a `ResourceId`.
    ///
    /// Unlike `resource_id_to_table_index` this is required to be eagerly
    /// populated before translation of a type occurs. This is populated by
    /// `register_*` methods below and is manually done during the `inline`
    /// phase. This is used to record the actual underlying type of a resource
    /// and where it originally comes from. When a resource is later referred to
    /// then a table is injected to be referred to.
    resource_id_to_resource_index: FxHashMap<types::ResourceId, ResourceIndex>,

    /// The current instance index that's being visited. This is updated as
    /// inliner frames are processed and components are instantiated.
    current_instance: Option<RuntimeComponentInstanceIndex>,
}

impl ResourcesBuilder {
    /// Converts the `id` provided into a `TypeResourceTableIndex`.
    ///
    /// If `id` has previously been seen or converted, the prior value is
    /// returned. Otherwise the `resource_id_to_resource_index` table must have
    /// been previously populated and additionally `current_instance` must have
    /// been previously set. Using these a new `TypeResourceTable` value is
    /// allocated which produces a fresh `TypeResourceTableIndex` within the
    /// `types` provided.
    ///
    /// Due to `wasmparser`'s uniqueness of resource IDs combined with the
    /// snapshotting and restoration behavior of `ResourcesBuilder` itself this
    /// should have the net effect of the first time a resource is seen within
    /// any component it's assigned a new table, which is exactly what we want.
    pub fn convert(
        &mut self,
        id: types::ResourceId,
        types: &mut ComponentTypes,
    ) -> TypeResourceTableIndex {
        *self
            .resource_id_to_table_index
            .entry(id)
            .or_insert_with(|| {
                let ty = self.resource_id_to_resource_index[&id];
                let instance = self.current_instance.expect("current instance not set");
                types
                    .resource_tables
                    .push(TypeResourceTable { ty, instance })
            })
    }

    /// Walks over the `ty` provided, as defined within `types`, and registers
    /// all the defined resources found with the `register` function provided.
    ///
    /// This is used to register `ResourceIndex` entries within the
    /// `resource_id_to_resource_index` table of this type for situations such
    /// as when a resource is imported into a component. During the inlining
    /// phase of translation the actual underlying type of the resource is
    /// known due to tracking dataflow and this registers that relationship.
    ///
    /// The `path` provided is temporary storage to pass to the `register`
    /// function eventually.
    ///
    /// The `register` callback is invoked with `path` with a list of names
    /// which correspond to exports of instances to reach the "leaf" where a
    /// resource type is expected.
    pub fn register_component_entity_type<'a>(
        &mut self,
        types: &'a types::TypesRef<'_>,
        ty: types::ComponentEntityType,
        path: &mut Vec<&'a str>,
        register: &mut dyn FnMut(&[&'a str]) -> ResourceIndex,
    ) {
        match ty {
            // If `ty` is itself a type, and that's a resource type, then this
            // is where registration happens. The `register` callback is invoked
            // with the current path and that's inserted in to
            // `resource_id_to_resource_index` if the resource hasn't been seen
            // yet.
            types::ComponentEntityType::Type {
                created: types::ComponentAnyTypeId::Resource(id),
                ..
            } => {
                self.resource_id_to_resource_index
                    .entry(id.resource())
                    .or_insert_with(|| register(path));
            }

            // Resources can be imported/defined through exports of instances so
            // all instance exports are walked here. Note the management of
            // `path` which is used for the recursive invocation of this method.
            types::ComponentEntityType::Instance(id) => {
                let ty = &types[id];
                for (name, ty) in ty.exports.iter() {
                    path.push(name);
                    self.register_component_entity_type(types, *ty, path, register);
                    path.pop();
                }
            }

            // None of these items can introduce a new component type, so
            // there's no need to recurse over these.
            types::ComponentEntityType::Func(_)
            | types::ComponentEntityType::Type { .. }
            | types::ComponentEntityType::Module(_)
            | types::ComponentEntityType::Component(_)
            | types::ComponentEntityType::Value(_) => {}
        }
    }

    /// Declares that the wasmparser `id`, which must point to a resource, is
    /// defined by the `ty` provided.
    ///
    /// This is used when a local resource is defined within a component for example.
    pub fn register_resource(&mut self, id: types::ResourceId, ty: ResourceIndex) {
        let prev = self.resource_id_to_resource_index.insert(id, ty);
        assert!(prev.is_none());
    }

    /// Updates the `current_instance` field to assign instance fields of future
    /// `TypeResourceTableIndex` values produced via `convert`.
    pub fn set_current_instance(&mut self, instance: RuntimeComponentInstanceIndex) {
        self.current_instance = Some(instance);
    }
}
