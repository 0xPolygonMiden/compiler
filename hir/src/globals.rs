use std::alloc::Layout;
use std::collections::{hash_map::DefaultHasher, BTreeMap};
use std::fmt::{self, Write};
use std::hash::{Hash, Hasher};

use cranelift_entity::entity_impl;
use intrusive_collections::{intrusive_adapter, LinkedList, LinkedListLink, UnsafeRef};
use miden_diagnostics::Spanned;

use super::*;

/// The policy to apply to a global variable (or function) when linking
/// together a program during code generation.
///
/// Miden doesn't (currently) have a notion of a symbol table for things like global variables.
/// At runtime, there are not actually symbols at all in any familiar sense, instead functions,
/// being the only entities with a formal identity in MASM, are either inlined at all their call
/// sites, or are referenced by the hash of their MAST root, to be unhashed at runtime if the call
/// is executed.
///
/// Because of this, and because we cannot perform linking ourselves (we must emit separate modules,
/// and leave it up to the VM to link them into the MAST), there are limits to what we can do in
/// terms of linking function symbols. We essentially just validate that given a set of modules in
/// a [Program], that there are no invalid references across modules to symbols which either don't
/// exist, or which exist, but have internal linkage.
///
/// However, with global variables, we have a bit more freedom, as it is a concept that we are
/// completely inventing from whole cloth without explicit support from the VM or Miden Assembly.
/// In short, when we compile a [Program] to MASM, we first gather together all of the global variables
/// into a program-wide table, merging and garbage collecting as appropriate, and updating all
/// references to them in each module. This global variable table is then assumed to be laid out
/// in memory starting at the base of the linear memory address space in the same order, with appropriate
/// padding to ensure accesses are aligned. Then, when emitting MASM instructions which reference
/// global values, we use the layout information to derive the address where that global value
/// is allocated.
///
/// This has some downsides however, the biggest of which is that we can't prevent someone from
/// loading modules generated from a [Program] with either their own hand-written modules, or
/// even with modules from another [Program]. In such cases, assumptions about the allocation of
/// linear memory from different sets of modules will almost certainly lead to undefined behavior.
/// In the future, we hope to have a better solution to this problem, preferably one involving
/// native support from the Miden VM itself. For now though, we're working with what we've got.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum Linkage {
    /// This symbol is only visible in the containing module.
    ///
    /// Internal symbols may be renamed to avoid collisions
    ///
    /// Unreferenced internal symbols can be discarded at link time.
    Internal,
    /// This symbol will be linked using the "one definition rule", i.e. symbols with
    /// the same name, type, and linkage will be merged into a single definition.
    ///
    /// Unlike `internal` linkage, unreferenced `odr` symbols cannot be discarded.
    ///
    /// NOTE: `odr` symbols cannot satisfy external symbol references
    Odr,
    /// This symbol is visible externally, and can be used to resolve external symbol references.
    #[default]
    External,
}
impl fmt::Display for Linkage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Internal => f.write_str("internal"),
            Self::Odr => f.write_str("odr"),
            Self::External => f.write_str("external"),
        }
    }
}

intrusive_adapter!(pub GlobalVariableAdapter = UnsafeRef<GlobalVariableData>: GlobalVariableData { link: LinkedListLink });

/// This error is raised when attempting to declare [GlobalVariableData]
/// with a conflicting symbol name and/or linkage.
///
/// For example, two global variables with the same name, but differing
/// types will result in this error, as there is no way to resolve the
/// conflict.
pub struct GlobalVariableConflictError(GlobalVariable);

/// This error is raised when attempting to define the initializer value for a [GlobalVariable] fails.
pub enum InvalidInitializerError {
    /// The initializer data is too large for the type of the global
    OutOfBounds,
    /// The initializer data conflicts with a previous initializer definition
    Conflict,
}

/// Describes the way in which global variable conflicts will be handled
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum ConflictResolutionStrategy {
    /// Do not attempt to resolve conflicts
    ///
    /// NOTE: This does not change the behavior of "one definition rule" linkage,
    /// when the globals have identical definitions.
    None,
    /// Attempt to resolve conflicts by renaming symbols with `internal` linkage.
    #[default]
    Rename,
}

/// This table is used to lay out and link together global variables for a [Program].
///
/// See the docs for [Linkage], [GlobalVariableData], and [GlobalVariableTable::declare] for more details.
pub struct GlobalVariableTable {
    layout: LinkedList<GlobalVariableAdapter>,
    names: BTreeMap<Ident, GlobalVariable>,
    arena: ArenaMap<GlobalVariable, GlobalVariableData>,
    data: ConstantPool,
    next_unique_id: usize,
    conflict_strategy: ConflictResolutionStrategy,
}
impl Default for GlobalVariableTable {
    fn default() -> Self {
        Self::new(Default::default())
    }
}
impl GlobalVariableTable {
    pub fn new(conflict_strategy: ConflictResolutionStrategy) -> Self {
        Self {
            layout: Default::default(),
            names: Default::default(),
            arena: Default::default(),
            data: ConstantPool::default(),
            next_unique_id: 0,
            conflict_strategy,
        }
    }

    /// Returns true if the global variable table is empty
    pub fn is_empty(&self) -> bool {
        self.layout.is_empty()
    }

    /// Get a double-ended iterator over the current table layout
    pub fn iter<'a, 'b: 'a>(
        &'b self,
    ) -> intrusive_collections::linked_list::Iter<'a, GlobalVariableAdapter> {
        self.layout.iter()
    }

    /// Returns true if a global variable with `name` has been declared
    pub fn exists(&self, name: Ident) -> bool {
        self.names.contains_key(&name)
    }

    /// Looks up a [GlobalVariable] by name.
    pub fn find(&self, name: Ident) -> Option<GlobalVariable> {
        self.names.get(&name).copied()
    }

    /// Gets the data associated with the given [GlobalVariable]
    pub fn get(&self, id: GlobalVariable) -> &GlobalVariableData {
        &self.arena[id]
    }

    /// Computes the total size in bytes of the table, as it is currently laid out.
    pub fn size_in_bytes(&self) -> usize {
        // We mimic the allocation process here, by visiting each
        // global variable, padding the current heap pointer as necessary
        // to provide the necessary minimum alignment for the value, and
        // then bumping it by the size of the value itself.
        //
        // At the end, the effective address of the pointer is the total
        // size in bytes of the allocation
        let mut hp = 0 as *const u8;
        for gv in self.layout.iter() {
            let layout = gv.layout();
            // SAFETY: We aren't actually using these pointers,
            // we're just making use of the optimized alignment
            // intrinsics available for them
            let offset = hp.align_offset(layout.align());
            unsafe {
                hp = hp.add(offset).add(layout.size());
            }
        }
        hp as usize
    }

    /// Computes the offset, in bytes, of the given [GlobalVariable] from the
    /// start of the segment in which globals are allocated, assuming that the
    /// layout of the global variable table up to and including `id` remains
    /// unchanged.
    ///
    /// # SAFETY
    ///
    /// This should only be used once all data segments and global variables have
    /// been declared, and the layout of the table has been decided. It is technically
    /// safe to use offsets obtained before all global variables are declared, _IF_ the
    /// data segments and global variable layout up to and including those global variables
    /// remains unchanged after that point.
    ///
    /// If the offset for a given global variable is obtained, and the heap layout is
    /// subsequently changed in such a way that the original offset is no longer
    /// accurate, bad things will happen.
    pub unsafe fn offset_of(&self, id: GlobalVariable) -> usize {
        let mut hp = 0 as *const u8;
        for gv in self.layout.iter() {
            let layout = gv.layout();
            let offset = hp.align_offset(layout.align());
            hp = hp.add(offset);

            // If the current variable is the one we're after,
            // the aligned address is the offset to the start
            // of the allocation, so we're done
            if gv.id == id {
                break;
            }
            // Otherwise, continue by adding the size of the value
            hp = hp.add(layout.size());
        }
        hp as usize
    }

    /// Declares a new global variable with the given symbol name, type, and linkage.
    ///
    /// If successful, `Ok` is returned, with the [GlobalVariable] corresponding to the data for the symbol.
    ///
    /// If a symbol with `name` already exists:
    ///
    /// * If the linkage is internal, a new unique name will be derived from `name` in order
    /// to ensure there are no symbol name conflicts. It is assumed that global variables with internal
    /// linkage were checked for uniqueness at the module level, so renaming them is safe when linking.
    ///
    /// * If the linkage is external, `Err` is returned, as only one declaration is allowed globally
    ///
    /// * If the linkage follows the "one definition rule", then as long as the previous declaration
    /// has the same type and linkage, then the definitions are "merged" and the handle to the existing
    /// declaration is returned rather than allocating a new one.
    pub fn declare(
        &mut self,
        name: Ident,
        ty: Type,
        linkage: Linkage,
    ) -> Result<GlobalVariable, GlobalVariableConflictError> {
        self.try_insert(name, ty, linkage)
    }

    /// Get the constant data associated with `id`
    pub fn get_constant(&self, id: Constant) -> &ConstantData {
        self.data.get(id)
    }

    /// Inserts the given constant data into this table without allocating a global
    pub fn insert_constant(&mut self, data: ConstantData) -> Constant {
        self.data.insert(data)
    }

    /// Returns true if the given constant data is in the constant pool
    pub fn contains_constant(&self, data: &ConstantData) -> bool {
        self.data.contains(data)
    }

    /// This sets the initializer for the given [GlobalVariable] to `init`.
    ///
    /// This function will return `Err` if any of the following occur:
    ///
    /// * The global variable already has an initializer
    /// * The given data does not match the type of the global variable, i.e. more data than the type supports.
    ///
    /// If the data is smaller than the type of the global variable, the data will be zero-extended to fill it out.
    ///
    /// NOTE: The initializer data is expected to be in little-endian order.
    pub fn set_initializer(
        &mut self,
        gv: GlobalVariable,
        init: ConstantData,
    ) -> Result<(), InvalidInitializerError> {
        let global = &mut self.arena[gv];
        let layout = global.layout();
        if init.len() > layout.size() {
            return Err(InvalidInitializerError::OutOfBounds);
        }
        let init = self.data.insert(init);
        if let Some(prev_init) = global.initializer() {
            if prev_init != init {
                return Err(InvalidInitializerError::Conflict);
            }
        }
        global.init = Some(init);
        Ok(())
    }

    fn try_insert(
        &mut self,
        name: Ident,
        ty: Type,
        linkage: Linkage,
    ) -> Result<GlobalVariable, GlobalVariableConflictError> {
        assert_ne!(
            name.as_symbol(),
            symbols::Empty,
            "global variable declarations require a non-empty symbol name"
        );
        let mut data = GlobalVariableData {
            link: Default::default(),
            id: Default::default(),
            name,
            ty,
            linkage,
            init: None,
        };
        if let Some(gv) = self.names.get(&data.name).copied() {
            let rename_internal_symbols =
                matches!(self.conflict_strategy, ConflictResolutionStrategy::Rename);
            match linkage {
                Linkage::External => return Err(GlobalVariableConflictError(gv)),
                Linkage::Internal if rename_internal_symbols => {
                    let mut generated = String::from(data.name.as_str());
                    let original_len = generated.len();
                    loop {
                        // Allocate a new unique integer value to mix into the hash
                        let unique_id = self.next_unique_id;
                        self.next_unique_id += 1;
                        // Calculate the hash of the global variable data
                        let mut hasher = DefaultHasher::new();
                        data.hash(&mut hasher);
                        unique_id.hash(&mut hasher);
                        let hash = hasher.finish();
                        // Append `.<hash>` as a suffix to the original symbol name
                        write!(&mut generated, ".{:x}", hash)
                            .expect("failed to write unique suffix to global variable name");
                        // If by some stroke of bad luck we generate a symbol name that
                        // is in use, try again with a different unique id until we find
                        // an unused name
                        if !self.names.contains_key(generated.as_str()) {
                            data.name =
                                Ident::new(Symbol::intern(generated.as_str()), data.name.span());
                            break;
                        }
                        // Strip off the suffix we just added before we try again
                        generated.truncate(original_len);
                    }
                }
                Linkage::Internal => return Err(GlobalVariableConflictError(gv)),
                Linkage::Odr => {
                    let prev = &self.arena[gv];
                    if prev == &data {
                        return Ok(gv);
                    } else {
                        return Err(GlobalVariableConflictError(gv));
                    }
                }
            }
        }
        let name = data.name;
        // Allocate the data in the arena
        let gv = self.arena.alloc_key();
        data.id = gv;
        self.arena.append(gv, data);
        // Add the symbol name to the symbol map
        self.names.insert(name, gv);
        // Add the global variable to the layout
        let unsafe_ref = unsafe {
            let ptr = self.arena.get_raw(gv).unwrap();
            UnsafeRef::from_raw(ptr.as_ptr())
        };
        self.layout.push_back(unsafe_ref);
        Ok(gv)
    }
}

/// A handle to a global variable definition
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalVariable(u32);
entity_impl!(GlobalVariable, "gvar");
impl Default for GlobalVariable {
    #[inline]
    fn default() -> Self {
        use cranelift_entity::packed_option::ReservedValue;

        Self::reserved_value()
    }
}

/// A [GlobalVariable] represents a concrete definition for a symbolic value,
/// i.e. it corresponds to the actual allocated memory referenced by a [GlobalValueData::Symbol]
/// value.
#[derive(Debug, Clone)]
pub struct GlobalVariableData {
    /// The intrusive link used for storing this global variable in a list
    link: LinkedListLink,
    /// The unique identifier associated with this global variable
    id: GlobalVariable,
    /// The symbol name for this global variable
    pub name: Ident,
    /// The type of the value this variable is allocated for.
    ///
    /// Nothing prevents one from accessing the variable as if it is
    /// another type, but at a minimum this type is used to derive the
    /// size and alignment requirements for this global variable on
    /// the heap.
    pub ty: Type,
    /// The linkage for this global variable
    pub linkage: Linkage,
    /// The initializer for this global variable, if applicable
    pub init: Option<Constant>,
}
impl GlobalVariableData {
    /// Get the unique identifier assigned to this global variable
    pub fn id(&self) -> GlobalVariable {
        self.id
    }

    /// Return the [Layout] of this global variable in memory
    pub fn layout(&self) -> Layout {
        self.ty.layout()
    }

    /// Return a handle to the initializer for this global variable, if present
    pub fn initializer(&self) -> Option<Constant> {
        self.init
    }
}
impl Eq for GlobalVariableData {}
impl PartialEq for GlobalVariableData {
    fn eq(&self, other: &Self) -> bool {
        self.linkage == other.linkage
            && self.ty == other.ty
            && self.name == other.name
            && self.init == other.init
    }
}
impl Hash for GlobalVariableData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.ty.hash(state);
        self.linkage.hash(state);
        self.init.hash(state);
    }
}

/// A handle to a global variable definition
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalValue(u32);
entity_impl!(GlobalValue, "gv");
impl Default for GlobalValue {
    #[inline]
    fn default() -> Self {
        use cranelift_entity::packed_option::ReservedValue;

        Self::reserved_value()
    }
}

/// Data associated with a `GlobalValue`.
///
/// Globals are allocated statically, and live for the lifetime of the program.
/// In Miden, we allocate globals at the start of the heap. Since all globals are
/// known statically, we instructions which manipulate globals are converted to
/// loads/stores using constant addresses when translated to MASM.
///
/// Like other entities, globals may also have a [SourceSpan] associated with them.
#[derive(Debug, Clone)]
pub enum GlobalValueData {
    /// A symbolic reference to a global variable symbol
    ///
    /// The type of a symbolic global value is always a pointer, the address
    /// of the referenced global variable.
    Symbol {
        /// The name of the global variable that is referenced
        name: Ident,
        /// A constant offset, in bytes, from the address of the symbol
        offset: i32,
    },
    /// A global whose value is given by reading the value from the address
    /// derived from another global value and an offset.
    Load {
        /// The global value whose value is the base pointer
        base: GlobalValue,
        /// A constant offset, in bytes, from the base address
        offset: i32,
        /// The type of the value stored at `base + offset`
        ty: Type,
    },
    /// A global whose value is an address computed as the offset from another global
    ///
    /// This can be used for `getelementptr`-like situations, such as calculating the
    /// address of a field in a struct that is stored in a global variable.
    IAddImm {
        /// The global value whose value is the base pointer
        base: GlobalValue,
        /// A constant offset, in units of `ty`, from the base address
        offset: i32,
        /// The unit type of the offset
        ///
        /// This can be helpful when computing addresses to elements of an array
        /// stored in a global variable.
        ty: Type,
    },
}
impl GlobalValueData {
    /// Returns true if this global value is a symbolic or computed address
    /// which can be resolved at compile-time.
    ///
    /// Notably, global loads may produce an address, but the value of that
    /// address is not known until runtime.
    pub fn is_constant_addr(&self) -> bool {
        !matches!(self, Self::Load { .. })
    }
}
