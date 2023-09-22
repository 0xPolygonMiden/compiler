use petgraph::{prelude::DiGraphMap, Direction};
use rustc_hash::FxHashMap;

use crate::*;

/// This represents the various types of errors which may be raised by a [Linker]
#[derive(Debug, thiserror::Error)]
pub enum LinkerError {
    /// The given module has already been declared
    #[error("duplicate module declaration for '{0}'")]
    ModuleConflict(Ident),
    /// The given identifier references a [Module] which is not present in the set of
    /// modules to link.
    #[error("encountered reference to undefined module '{0}'")]
    MissingModule(Ident),
    /// The given identifier references a [Function] which is not defined in any of the
    /// modules being linked, and is not a standard library function whose definition is
    /// expected to be provided by the Miden VM.
    #[error("encountered reference to undefined function '{0}'")]
    MissingFunction(FunctionIdent),
    /// The given identifier references [GlobalVariableData] which has not been defined
    /// in any of the modules being linked.
    #[error("encountered reference to undefined global '{0}'")]
    MissingGlobal(Ident),
    /// The given function is referenced by an external declaration, but the actual definition
    /// of that function has a different signature than was expected by the external reference.
    ///
    /// The types of mismatches that will cause this error are:
    ///
    /// * The calling convention is different
    /// * The number and/or types of the arguments and results are not the same
    /// * A special purpose parameter is declared in one signature but not the other
    /// * Argument extension conflicts, i.e. one signature says a parameter is zero-extended, the other sign-extended
    #[error(
        "signature mismatch for '{0}': external function declaration does not match definition"
    )]
    SignatureMismatch(FunctionIdent),
    /// An external declaration for the given function was found in a different module than the
    /// one in which the function is defined, and the actual definition does not have external
    /// linkage.
    ///
    /// This error is a variant of `SignatureMismatch`, but occurs when the signature is otherwise
    /// correct, but is ultimately an invalid declaration because the function should not be visible
    /// outside its containing module.
    #[error("invalid reference to '{0}': only functions with external linkage can be referenced from other modules")]
    LinkageMismatch(FunctionIdent),
    /// A cycle in the call graph was found starting at the given function.
    ///
    /// This occurs due to recursion (self or mutual), and is not supported by Miden.
    #[error("encountered an invalid cycle in the call graph caused by a call from '{caller}' to '{callee}'")]
    InvalidCycle {
        caller: FunctionIdent,
        callee: FunctionIdent,
    },
    /// Occurs when the declared entrypoint does not have external linkage
    #[error("invalid entrypoint '{0}': must have external linkage")]
    InvalidEntryLinkage(FunctionIdent),
    /// Occurs when attempting to set the program entrypoint when it has already been set
    #[error("conflicting entrypoints: '{current}' conflicts with previously declared entrypoint '{prev}'")]
    InvalidMultipleEntry {
        current: FunctionIdent,
        prev: FunctionIdent,
    },
    /// An error occurred when attempting to link segments declared by a module into the
    /// set of segments already declared in the program. A segment might be valid in the
    /// context of a single module, but invalid in the context of a whole program, either
    /// due to conflicts, or an inability to allocate all segments without running out of
    /// available heap memory.
    #[error(transparent)]
    SegmentError(#[from] DataSegmentError),
    /// A conflict between two global variables with the same symbol was detected.
    ///
    /// When this occurs, the definitions must have been in separate modules, with external linkage,
    /// and they disagree on the type of the value or its initializer.
    #[error(transparent)]
    GlobalVariableError(#[from] GlobalVariableError),
}

/// Represents a node in the global variable dependency graph
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Node {
    /// A global symbol that was defined or has been referenced
    Global(Ident),
    /// A function which refers to one or more global symbols
    Function(FunctionIdent),
}

/// The [Linker] performs a similar role in conjunction with the Miden compiler, as the system linker
/// does (e.g. `ld`) when used with compilers like `clang` or `rustc`.
///
/// As a (very) rough overview, a typical linker is given a set of object files containing machine
/// code and data, one for every translation unit participating in the link (e.g. for Rust the translation
/// unit is a single crate). The linker will also be informed when dependencies are expected to be provided
/// at runtime, i.e. dynamic libraries. With this, a linker does the following:
///
/// * Determines the final layout of all code and data in the executable or library being produced,
/// this allows the linker to know the absolute and/or relative address for every symbol in the program.
/// * Ensures that all referenced symbols (functions/globals) are defined, or that there are runtime
/// dependencies that will satisfy the missing symbols (in practice, what actually happens is the
/// static linker, i.e. `ld`, assumes missing symbols will be provided by the runtime dependencies,
/// and it is the runtime dynamic linker, i.e. `rtdyld`, which handles the case where those symbols
/// cannot be located when the program is starting up).
/// * Rewrites instructions with symbol references to use the absolute/relative addressing once the
/// layout of the program in memory is known.
/// * Emits the linked program in binary form, either as an executable or as a library
///
/// However, there a couple of things that make [Linker] somewhat different than your typical system linker:
///
/// * We do not emit assembly/run the assembler prior to linking. This is because Miden Assembly (MASM)
/// does not have a way to represent things like data segments or global variables natively. Instead, the
/// linker is responsible for laying those out in memory ahead of time, and then all operations involving
/// them are lowered to use absolute addresses.
/// * [Linker] does not emit the final binary form of the program. It still plans the layout of program data
/// in memory, and performs the same type of validations as a typical linker, but the output of the linker
/// is a [Program], which must be emitted as Miden Assembly in a separate step _after_ being linked.
/// * We cannot guarantee that the [Program] we emit constitutes a closed set of modules/functions, even
/// accounting for functions whose definitions will be provided at runtime. This is because the Miden VM
/// acts as the final assembler of the programs it runs, and if the [Program] we emit is used as a library,
/// we can't know what other modules might end up being linked into the final program run by the VM. As a
/// result, it is assumed that any code introduced separately is either:
///   1. memory-agnostic, i.e. it doesn't use the heap and/or make any assumptions about the heap layout.
///   2. compatible with the layout decided upon by the linker, i.e. it uses well-known allocator functions
///      like `malloc`; or it places its memory in the range 2^30-2^31 for user contexts, or 2^30-(2^32 - 2^30)
///      for root contexts (the latter allocates a separate region for syscall locals). The linker will always
///      reserve memory starting at address 2^30 for locals and "unmanaged" memory allocations, to support
///      scenarios whereby a linked library is used with a program that needs its own region of heap to manage.
/// * Miden has separate address spaces depending on the context in which a function is executed, i.e. the root
/// vs user context distinction. Currently, all programs are assumed to be executed in the root context, and
/// we do not provide instructions for executing calls in another context. However, we will eventually be linking
/// programs which have a potentially unbounded number of address spaces, which is an additional complication
/// that your typical linker doesn't have to deal with
pub struct Linker {
    /// This is the program being constructed by the linker
    program: Box<Program>,
    /// This is the set of modules which have yet to be linked
    pending: FxHashMap<Ident, Box<Module>>,
    /// This is the dependency graph for all functions in the program.
    ///
    /// This graph is used to obtain a topological ordering of the
    /// functions in the program, so that we may emit Miden Assembly
    /// such that all procedure definitions occur before their uses.
    ///
    /// It is allowed for there to be cyclical module dependencies, but
    /// we do not permit cyclical function dependencies (i.e. recursive
    /// function calls).
    ///
    /// The edge weight is unused.
    callgraph: DiGraphMap<FunctionIdent, ()>,
    /// This is a subset of `callgraph` for a single module.
    ///
    /// This is only used when preprocessing a module, and is reset on each call to `add`
    local_callgraph: DiGraphMap<FunctionIdent, ()>,
    /// This is the dependency graph for all globals in the program.
    ///
    /// This graph is used to identify what global symbols are used, from where,
    /// and whether or not those dependencies are satisfied by the set of modules
    /// being linked into the program.
    globals: DiGraphMap<Node, ()>,
    /// The set of renamed global symbols for a single module.
    ///
    /// This is only used when preprocessing a module, and is reset on each call to `add`
    renamed: FxHashMap<Ident, Ident>,
}
impl Default for Linker {
    fn default() -> Self {
        let mut program = Box::new(Program::new());

        // We reserve the first page of memory for the shadow stack
        program
            .segments
            .declare(0, 64 * 1024, vec![].into(), false)
            .expect("unexpected error declaring shadow stack segment");

        Self {
            program,
            pending: Default::default(),
            callgraph: DiGraphMap::new(),
            local_callgraph: DiGraphMap::new(),
            globals: DiGraphMap::new(),
            renamed: Default::default(),
        }
    }
}
impl Linker {
    /// Create a [Linker] for a new, empty [Program].
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the entrypoint for the linked program
    ///
    /// Returns a [LinkerError] if a different entrypoint was already declared.
    pub fn with_entrypoint(&mut self, id: FunctionIdent) -> Result<(), LinkerError> {
        if let Some(prev) = self.program.entrypoint {
            if prev != id {
                return Err(LinkerError::InvalidMultipleEntry { current: id, prev });
            }
        }

        self.program.entrypoint = Some(id);

        Ok(())
    }

    /// Add `module` to the set of modules to be linked
    ///
    /// This preprocesses the module for the linker, and will catch the following issues:
    ///
    /// * Multiple modules with the same name
    /// * Conflicting data segment declarations
    /// * Conflicting global variable declarations
    /// * Recursion in the local call graph of the module (global analysis comes later)
    ///
    /// If any of the above errors occurs, a [LinkerError] is returned.
    pub fn add(&mut self, mut module: Box<Module>) -> Result<(), LinkerError> {
        let id = module.name;

        // Reset the auxiliary data structures used for preprocessing
        self.local_callgraph.clear();
        self.renamed.clear();

        // Raise an error if we've already got a module by this name pending
        if self.pending.contains_key(&id) {
            return Err(LinkerError::ModuleConflict(id));
        }

        // Import all data segments
        while let Some(segment) = module.segments.pop_front() {
            self.program.segments.insert(segment)?;
        }

        // Import all globals, and in the process:
        //
        // * Record all global variable definitions in the dependency graph
        // * Track renamed symbols, if any, produced by the import
        for global in module.globals.iter() {
            let mut imported_global = global.clone();
            // If an initializer was set, we need to import the constant data too
            if let Some(init) = imported_global.init.take() {
                let data = module.globals.get_constant(init).clone();
                imported_global.init = Some(self.program.globals.insert_constant(data));
            }
            let (id, renamed) = self.program.globals.try_insert(imported_global)?;
            let name = self.program.globals.get(id).name;
            if renamed {
                self.renamed.insert(global.name, name);
            }
            self.globals.add_node(Node::Global(name));
        }

        // Compute a subset of the call graph just for this module
        for function in module.functions.iter() {
            // While we're here, update the global call graph as well
            let caller = self.callgraph.add_node(function.id);
            let caller = self.local_callgraph.add_node(caller);
            for import in function.imports() {
                let callee = self.callgraph.add_node(import.id);
                self.callgraph.add_edge(caller, callee, ());
                // For the toposort, we only care about functions in the same module
                if import.id.module == module.name {
                    let callee = self.local_callgraph.add_node(callee);
                    self.local_callgraph.add_edge(caller, callee, ());
                }
            }
        }

        // Compute the topographical ordering of functions in this module
        let topography = petgraph::algo::toposort(&self.local_callgraph, None).map_err(|_| {
            // The error here only gives us the caller, but we'd like to know
            // the callee for our error diagnostics. To get it, we call the
            // call graph validation routine which does a traversal specifically
            // designed to obtain that information.
            validate_callgraph(&self.local_callgraph)
                .expect_err("expected call graph to contain a cycle")
        })?;

        // Preprocess all functions in this module by:
        //
        // * Record dependencies on global symbols in the dependency graph
        // * Rewrite any references to renamed global symbols with internal/odr linkage
        // * Reorder the function list according to the topological order computed above
        //
        // We do this in a single pass, by draining the list of sorted function ids, and
        // then unlinking each function from the module in that order, processing it, and
        // then inserting it at the end of the function list of the module. After all the
        // ids have been visited, the function list of the module will be in topographical
        // order. This is about as efficient as we can make it, roughly `O(N * M)`, where
        // `N` is the number of functions in the module, and `M` is the number of items
        // we must skip before we find the next function to process. If the module is in
        // topographical order already, the `M` factor is 1.
        for id in topography.into_iter() {
            // Find and remove the function with the given name from the module
            let mut function = module.unlink(id.function);

            // Process the global values of this function, i.e. references to global symbols
            for gvalue in function.dfg.globals.values_mut() {
                match gvalue {
                    // This global value type is sensitive to renaming
                    GlobalValueData::Symbol {
                        name: ref mut global_name,
                        ..
                    } => {
                        if let Some(new_name) = self.renamed.get(global_name).copied() {
                            *global_name = new_name;
                        }
                        // Make sure the symbol is in the graph
                        let global_node = self.globals.add_node(Node::Global(*global_name));
                        let dependent_node = self.globals.add_node(Node::Function(id));
                        // Record a dependency from this function to the named global
                        self.globals.add_edge(dependent_node, global_node, ());
                    }
                    // These global values are stable even in the face of symbol renaming
                    GlobalValueData::Load { .. } | GlobalValueData::IAddImm { .. } => continue,
                }
            }

            // Insert the function back in the list
            module.functions.push_back(function);
        }

        // We're done preprocessing, so add the module to the pending set
        self.pending.insert(id, module);

        Ok(())
    }

    /// Links all of the modules which were added, producing a [Program] if no issues are found.
    ///
    /// Returns a [LinkerError] if the link fails for any reason.
    ///
    /// When called, all of the added modules have been preprocessed, and what remains are the
    /// following tasks:
    ///
    /// * Verify that all referenced modules exist, or are known to be provided at runtime
    /// * Verify that all referenced functions exist, or are known to be provided at runtime,
    ///   and that the signature known to the caller matches the actual definition.
    /// * Verifies that the entrypoint, if set, is valid
    /// * Verify that there are no cycles in the call graph, i.e. that there is no recursion present
    /// * Verify that all references to global symbols have corresponding definitions
    /// * Perform garbage collection of unreferenced globals
    /// * TODO: If linking an executable program, garbage collect unused modules/functions
    ///
    /// Once linked, a [Program] can be emitted to Miden Assembly using the code generation passes.
    pub fn link(mut self) -> Result<Box<Program>, LinkerError> {
        // Ensure linker-defined globals and intrinsics are present
        self.populate_builtins();

        // Look for cycles in the call graph
        validate_callgraph(&self.callgraph)?;

        // Verify the entrypoint, if declared
        if let Some(entry) = self.program.entrypoint {
            let is_linked = self.pending.contains_key(&entry.module);
            if !is_linked {
                return Err(LinkerError::MissingModule(entry.module));
            }

            let module = &self.pending[&entry.module];
            let function = module
                .function(entry.function)
                .ok_or(LinkerError::MissingFunction(entry))?;
            if !function.is_public() {
                return Err(LinkerError::InvalidEntryLinkage(entry));
            }
        }

        // Verify module/function references
        for node in self.callgraph.nodes() {
            // If the module is pending, it is being linked
            let is_linked = self.pending.contains_key(&node.module);
            let is_stdlib = node.module.as_str().starts_with("std::");

            // If a referenced module is not being linked, raise an error
            if !is_linked {
                // However we ignore standard library modules in this check,
                // as they are known to be provided at runtime.
                //
                // TODO: We need to validate that the given module/function
                // is actually in the standard library though, and that the
                // signature matches what is expected.
                if is_stdlib {
                    continue;
                }

                return Err(LinkerError::MissingModule(node.module));
            }

            // The module is present, so we must verify that the function is defined in that module
            let module = &self.pending[&node.module];
            let function = module
                .function(node.function)
                .ok_or(LinkerError::MissingFunction(node))?;
            let is_externally_linkable = function.is_public();

            // Next, visit all of the dependent functions, and ensure their signatures match
            for dependent_id in self.callgraph.neighbors_directed(node, Direction::Incoming) {
                // If the dependent is in another module, but the function has internal linkage, raise an error
                if dependent_id.module != node.module && !is_externally_linkable {
                    return Err(LinkerError::LinkageMismatch(node));
                }
                // Otherwise, make sure the signatures match
                let dependent_module = &self.pending[&dependent_id.module];
                let dependent_function = dependent_module
                    .function(dependent_id.function)
                    .expect("dependency graph is outdated");
                let external_ref = dependent_function
                    .dfg
                    .get_import(&node)
                    .expect("dependency graph is outdated");
                verify_matching_signature(
                    function.id,
                    &function.signature,
                    &external_ref.signature,
                )?;
            }
        }

        // Verify global symbol references, and garbage collect unused globals
        for node in self.globals.nodes() {
            // Skip nodes in the graph which aren't globals
            let Node::Global(name) = node else {
                continue;
            };

            // If this global has no incoming edges, it's dead, so garbage collect it
            let mut dependents = self.globals.neighbors_directed(node, Direction::Incoming);
            let is_dead = dependents.next().is_none();
            if is_dead {
                let id = self
                    .program
                    .globals
                    .find(name)
                    .expect("expected global to be in table when dead");
                self.program.globals.remove(id);
            }

            // If it has dependents, but isn't defined anywhere, raise an error
            if !self.program.globals.exists(name) {
                return Err(LinkerError::MissingGlobal(name));
            }
        }

        // Run the garbage collector
        self.garbage_collect();

        // We're finished processing all pending modules, so add them to the program
        for module in self.pending.into_values() {
            self.program.modules.insert(module);
        }

        Ok(self.program)
    }

    /// Programs we construct may depend on one or more predefined globals/intrinsics
    /// that are provided by the compiler in order to support common functionality, such
    /// as memory management primitives. This function handles defining these prior to
    /// linking the program.
    fn populate_builtins(&mut self) {
        // We provide three globals for managing the heap, based on the layout
        // of the data segments and these globals.
        let mut globals_offset = 0;
        if let Some(last_segment) = self.program.segments.last() {
            let next_offset = last_segment.offset() + last_segment.size();
            // Ensure the start of the globals segment is word-aligned
            globals_offset = next_offset.align_up(32);
        }
        // Compute the start of the heap by finding the end of the globals segment, aligned to the nearest word boundary
        let heap_base = globals_offset
            .checked_add(
                self.program
                    .globals
                    .size_in_bytes()
                    .try_into()
                    .expect("unable to allocate globals, unable to fit in linear memory"),
            )
            .expect("unable to allocate globals, not enough unreserved space available")
            .align_up(32);
        let hp = heap_base.to_le_bytes();
        // Initialize all 3 globals with the computed heap pointer
        let heap_ptr_ty = Type::Ptr(Box::new(Type::U8));
        self.program.globals.declare("HEAP_BASE".into(), heap_ptr_ty.clone(), Linkage::External, Some(hp.into())).expect("unable to declare HEAP_BASE, a conflicting global by that name was already defined");
        self.program
            .globals
            .declare(
                "HEAP_TOP".into(),
                heap_ptr_ty.clone(),
                Linkage::External,
                Some(hp.into()),
            )
            .expect(
                "unable to declare HEAP_TOP, a conflicting global by that name was already defined",
            );
        self.program
            .globals
            .declare(
                "HEAP_END".into(),
                heap_ptr_ty,
                Linkage::External,
                Some(hp.into()),
            )
            .expect(
                "unable to declare HEAP_END, a conflicting global by that name was already defined",
            );
    }

    /// If an executable is being linked, discover unused functions and garbage collect them.
    ///
    /// Once a function has been identified as dead and is collected, any transitive items it
    /// references may also be collected if they are orphaned as a result of the collection.
    ///
    /// If a library is being linked, this function is a no-op, as it is not known what will
    /// be needed at runtime once used in the context of a program.
    fn garbage_collect(&mut self) {
        // TODO:
    }
}

/// Verifies that the actual signature of the given function matches what was expected.
///
/// Here, `actual` is the defined signature of the function; while `expected` is the signature
/// associated with an [ExternalFunction], i.e. it is the signature expected by a prospective
/// caller.
fn verify_matching_signature(
    id: FunctionIdent,
    actual: &Signature,
    expected: &Signature,
) -> Result<(), LinkerError> {
    // If the number of parameters differs, raise an error
    if expected.arity() != actual.arity() {
        return Err(LinkerError::SignatureMismatch(id));
    }

    // If the type or specification of any parameters differs, raise an error
    for (ep, ap) in expected.params().iter().zip(actual.params().iter()) {
        if !is_matching_param(ep, ap) {
            return Err(LinkerError::SignatureMismatch(id));
        }
    }

    // If the number of results differs, raise an error
    let expected_results = expected.results();
    let actual_results = actual.results();
    if expected_results.len() != actual_results.len() {
        return Err(LinkerError::SignatureMismatch(id));
    }

    // If the type of results differs, raise an error
    for (er, ar) in expected_results.iter().zip(actual_results.iter()) {
        if !is_matching_param(er, ar) {
            return Err(LinkerError::SignatureMismatch(id));
        }
    }

    Ok(())
}

/// Determines if the actual ABI of a parameter matches the expected ABI.
fn is_matching_param(expected: &AbiParam, actual: &AbiParam) -> bool {
    if expected.purpose != actual.purpose {
        return false;
    }

    match actual.extension {
        // If the actual definition has no extension requirement,
        // it is ok if the caller is more strict, however we may
        // want to raise a warning in such cases
        ArgumentExtension::None => expected.ty == actual.ty,
        ArgumentExtension::Zext if expected.extension != ArgumentExtension::Zext => false,
        ArgumentExtension::Sext if expected.extension != ArgumentExtension::Sext => false,
        ArgumentExtension::Zext | ArgumentExtension::Sext => expected.ty == actual.ty,
    }
}

/// Validate the given call graph by looking for cycles caused by recursion.
///
/// Returns a [LinkerError] if a cycle is found.
fn validate_callgraph(callgraph: &DiGraphMap<FunctionIdent, ()>) -> Result<(), LinkerError> {
    use petgraph::visit::{depth_first_search, DfsEvent, IntoNodeIdentifiers};

    depth_first_search(
        callgraph,
        callgraph.node_identifiers(),
        |event| match event {
            DfsEvent::BackEdge(caller, callee) => Err(LinkerError::InvalidCycle { caller, callee }),
            _ => Ok(()),
        },
    )
}
