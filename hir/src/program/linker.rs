use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
};

use miden_assembly::library::Library as CompiledLibrary;
use petgraph::{prelude::DiGraphMap, Direction};

use crate::{
    diagnostics::{DiagnosticsHandler, Report, Severity, Spanned},
    *,
};

/// Represents a node in the global variable dependency graph
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Node {
    /// A global symbol that was defined or has been referenced
    Global(Ident),
    /// A function which refers to one or more global symbols
    Function(FunctionIdent),
}

/// Represents an object input to the [Linker]
pub enum Object {
    /// The object is an HIR module
    Hir(Box<Module>),
    /// The object is a compiled Miden Assembly module
    Masm { name: Ident, exports: Vec<Ident> },
}
impl Object {
    /// Return the identifier associated with this object
    pub fn id(&self) -> Ident {
        match self {
            Self::Hir(module) => module.name,
            Self::Masm { name, .. } => *name,
        }
    }

    /// Return the set of exported functions/procedures from this object
    pub fn exports(&self) -> Box<(dyn Iterator<Item = FunctionIdent> + '_)> {
        match self {
            Self::Hir(module) => Box::new(module.functions().map(|f| f.id)),
            Self::Masm { name, ref exports } => {
                let name = *name;
                Box::new(exports.iter().copied().map(move |function| FunctionIdent {
                    module: name,
                    function,
                }))
            }
        }
    }
}
impl From<Box<Module>> for Object {
    fn from(module: Box<Module>) -> Self {
        Self::Hir(module)
    }
}
impl From<(Ident, Vec<Ident>)> for Object {
    fn from(module: (Ident, Vec<Ident>)) -> Self {
        Self::Masm {
            name: module.0,
            exports: module.1,
        }
    }
}

/// The [Linker] performs a similar role in conjunction with the Miden compiler, as the system
/// linker does (e.g. `ld`) when used with compilers like `clang` or `rustc`.
///
/// As a (very) rough overview, a typical linker is given a set of object files containing machine
/// code and data, one for every translation unit participating in the link (e.g. for Rust the
/// translation unit is a single crate). The linker will also be informed when dependencies are
/// expected to be provided at runtime, i.e. dynamic libraries. With this, a linker does the
/// following:
///
/// * Determines the final layout of all code and data in the executable or library being produced,
/// this allows the linker to know the absolute and/or relative address for every symbol in the
/// program.
/// * Ensures that all referenced symbols (functions/globals) are defined, or that there are runtime
/// dependencies that will satisfy the missing symbols (in practice, what actually happens is the
/// static linker, i.e. `ld`, assumes missing symbols will be provided by the runtime dependencies,
/// and it is the runtime dynamic linker, i.e. `rtdyld`, which handles the case where those symbols
/// cannot be located when the program is starting up).
/// * Rewrites instructions with symbol references to use the absolute/relative addressing once the
/// layout of the program in memory is known.
/// * Emits the linked program in binary form, either as an executable or as a library
///
/// However, there a couple of things that make [Linker] somewhat different than your typical system
/// linker:
///
/// * We do not emit assembly/run the assembler prior to linking. This is because Miden Assembly
///   (MASM)
/// does not have a way to represent things like data segments or global variables natively.
/// Instead, the linker is responsible for laying those out in memory ahead of time, and then all
/// operations involving them are lowered to use absolute addresses.
/// * [Linker] does not emit the final binary form of the program. It still plans the layout of
///   program data
/// in memory, and performs the same type of validations as a typical linker, but the output of the
/// linker is a [Program], which must be emitted as Miden Assembly in a separate step _after_ being
/// linked.
/// * We cannot guarantee that the [Program] we emit constitutes a closed set of modules/functions,
///   even
/// accounting for functions whose definitions will be provided at runtime. This is because the
/// Miden VM acts as the final assembler of the programs it runs, and if the [Program] we emit is
/// used as a library, we can't know what other modules might end up being linked into the final
/// program run by the VM. As a result, it is assumed that any code introduced separately is either:
///   1. memory-agnostic, i.e. it doesn't use the heap and/or make any assumptions about the heap
///      layout.
///   2. compatible with the layout decided upon by the linker, i.e. it uses well-known allocator
///      functions like `malloc`; or it places its memory in the range 2^30-2^31 for user contexts,
///      or 2^30-(2^32 - 2^30) for root contexts (the latter allocates a separate region for syscall
///      locals). The linker will always reserve memory starting at address 2^30 for locals and
///      "unmanaged" memory allocations, to support scenarios whereby a linked library is used with
///      a program that needs its own region of heap to manage.
/// * Miden has separate address spaces depending on the context in which a function is executed,
///   i.e. the root
/// vs user context distinction. Currently, all programs are assumed to be executed in the root
/// context, and we do not provide instructions for executing calls in another context. However, we
/// will eventually be linking programs which have a potentially unbounded number of address spaces,
/// which is an additional complication that your typical linker doesn't have to deal with
pub struct Linker<'a> {
    diagnostics: &'a DiagnosticsHandler,
    /// This is the program being constructed by the linker
    program: Box<Program>,
    /// This is the set of named objects which have yet to be linked
    pending: BTreeMap<Ident, Object>,
    /// This is the set of patterns that symbol names will be matched against when determining
    /// whether or not to raise an error when a reference to any symbol whose name starts with
    /// that pattern cannot be found.
    ///
    /// In practice, this is used to allow certain library modules to be referenced without
    /// requiring them to be loaded into the linker.
    allow_missing: BTreeSet<Cow<'static, str>>,
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
    renamed: BTreeMap<Ident, Ident>,
}
impl<'a> Linker<'a> {
    /// Create a [Linker] for a new, empty [Program].
    pub fn new(diagnostics: &'a DiagnosticsHandler) -> Self {
        let mut program = Box::new(Program::new());

        // We reserve the first page of memory for the shadow stack
        program
            .segments
            .declare(0, 64 * 1024, vec![].into(), false)
            .expect("unexpected error declaring shadow stack segment");

        Self {
            diagnostics,
            program,
            pending: Default::default(),
            allow_missing: BTreeSet::from_iter([
                "std::".into(),
                "intrinsics::".into(),
                "miden::account".into(),
                "miden::tx".into(),
                "miden::note".into(),
            ]),
            callgraph: DiGraphMap::new(),
            local_callgraph: DiGraphMap::new(),
            globals: DiGraphMap::new(),
            renamed: Default::default(),
        }
    }

    /// Set the entrypoint for the linked program
    ///
    /// Returns a [Report] if a different entrypoint was already declared.
    pub fn with_entrypoint(&mut self, id: FunctionIdent) -> Result<(), Report> {
        if let Some(prev) = self.program.entrypoint() {
            if prev != id {
                return Err(self
                    .diagnostics
                    .diagnostic(Severity::Error)
                    .with_message("linker error")
                    .with_primary_label(
                        id.function.span,
                        "this entrypoint conflicts with a previously declared entrypoint",
                    )
                    .with_secondary_label(prev.function.span, "previous entrypoint declared here")
                    .into_report());
            }
        }

        self.program.entrypoint = Some(id);

        Ok(())
    }

    /// Specify a pattern that will be matched against undefined symbols that determines whether or
    /// or not it should be treated as an error. It is assumed that the referenced symbol will be
    /// resolved during assembly to MAST.
    pub fn allow_missing(&mut self, name: impl Into<Cow<'static, str>>) {
        self.allow_missing.insert(name.into());
    }

    /// Add a compiled library to the set of libraries to link against
    pub fn add_library(&mut self, lib: CompiledLibrary) {
        // Add all of the exported objects to the callgraph
        for export in lib.exports() {
            let module = Ident::with_empty_span(Symbol::intern(export.module.path()));
            let name: &str = export.name.as_ref();
            let function = Ident::with_empty_span(Symbol::intern(name));
            self.callgraph.add_node(FunctionIdent { module, function });
        }
        self.program.add_library(lib);
    }

    /// Add multiple libraries to the set of libraries to link against
    pub fn add_libraries<I>(&mut self, libs: I)
    where
        I: IntoIterator<Item = CompiledLibrary>,
    {
        for lib in libs {
            self.add_library(lib);
        }
    }

    /// Add an object to link as part of the resulting [Program].
    ///
    /// There are different types of objects, see [Object] for details.
    ///
    /// # Errors
    ///
    /// The following conditions can cause an error to be raised, if applicable to the object given:
    ///
    /// * The object is invalid
    /// * The object introduces recursion into the call graph
    /// * Two or more objects export a module with the same name
    /// * Two or more objects contain conflicting data segment declarations
    /// * Two or more objects contain conflicting global variable declarations
    pub fn add_object(&mut self, object: impl Into<Object>) -> Result<(), Report> {
        let object = object.into();
        let id = object.id();

        // Raise an error if we've already got a module by this name pending
        if self.pending.contains_key(&id) {
            return Err(self
                .diagnostics
                .diagnostic(Severity::Error)
                .with_message("linker error")
                .with_primary_label(
                    id.span,
                    "this module conflicts with a previous module of the same name",
                )
                .into_report());
        }

        // Register functions in the callgraph
        for export in object.exports() {
            self.callgraph.add_node(export);
        }

        match object {
            Object::Hir(module) => self.add_hir_object(module),
            object @ Object::Masm { .. } => {
                // We're done preprocessing, so add the module to the pending set
                self.pending.insert(object.id(), object);

                Ok(())
            }
        }
    }

    /// Add `module` to the set of objects to be linked
    ///
    /// This preprocesses the module for the linker, and will catch the following issues:
    ///
    /// * Multiple modules with the same name
    /// * Conflicting data segment declarations
    /// * Conflicting global variable declarations
    /// * Recursion in the local call graph of the module (global analysis comes later)
    ///
    /// If any of the above errors occurs, a [Report] is returned.
    fn add_hir_object(&mut self, mut module: Box<Module>) -> Result<(), Report> {
        let id = module.name;

        // Reset the auxiliary data structures used for preprocessing
        self.local_callgraph.clear();
        self.renamed.clear();

        // Raise an error if we've already got a module by this name pending
        if self.pending.contains_key(&id) {
            return Err(self
                .diagnostics
                .diagnostic(Severity::Error)
                .with_message("linker error")
                .with_primary_label(
                    id.span,
                    "this module conflicts with a previous module of the same name",
                )
                .into_report());
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
            validate_callgraph(&self.local_callgraph, self.diagnostics)
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
        self.pending.insert(id, Object::Hir(module));

        Ok(())
    }

    /// Links all of the modules which were added, producing a [Program] if no issues are found.
    ///
    /// Returns a [Report] if the link fails for any reason.
    ///
    /// When called, all of the added modules have been preprocessed, and what remains are the
    /// following tasks:
    ///
    /// * Verify that all referenced modules exist, or are known to be provided at runtime
    /// * Verify that all referenced functions exist, or are known to be provided at runtime, and
    ///   that the signature known to the caller matches the actual definition.
    /// * Verifies that the entrypoint, if set, is valid
    /// * Verify that there are no cycles in the call graph, i.e. that there is no recursion present
    /// * Verify that all references to global symbols have corresponding definitions
    /// * Perform garbage collection of unreferenced globals
    /// * TODO: If linking an executable program, garbage collect unused modules/functions
    ///
    /// Once linked, a [Program] can be emitted to Miden Assembly using the code generation passes.
    pub fn link(mut self) -> Result<Box<Program>, Report> {
        // Look for cycles in the call graph
        validate_callgraph(&self.callgraph, self.diagnostics)?;

        // Verify the entrypoint, if declared
        if let Some(entry) = self.program.entrypoint() {
            // NOTE(pauls): Currently, we always raise an error here, but since we do allow
            // missing symbols in other situations, perhaps we should allow it here as well.
            // For now though, we assume this is a mistake, since presumably you are compiling
            // the code that contains the entrypoint.
            let object = self.pending.get(&entry.module).ok_or_else(|| {
                self.diagnostics
                    .diagnostic(Severity::Error)
                    .with_message(format!("linker error: undefined module '{}'", &entry.module))
                    .into_report()
            })?;
            match object {
                Object::Hir(module) => {
                    let function = module.function(entry.function).ok_or_else(|| {
                        self.diagnostics
                            .diagnostic(Severity::Error)
                            .with_message(format!("linker error: undefined function '{}'", &entry))
                            .into_report()
                    })?;
                    if !function.is_public() {
                        return Err(self
                            .diagnostics
                            .diagnostic(Severity::Error)
                            .with_message("linker error")
                            .with_primary_label(
                                entry.function.span,
                                "entrypoint must have external linkage",
                            )
                            .into_report());
                    }
                }
                Object::Masm { ref exports, .. } => {
                    if !exports.contains(&entry.function) {
                        return Err(self
                            .diagnostics
                            .diagnostic(Severity::Error)
                            .with_message(format!("linker error: undefined function '{}'", &entry))
                            .into_report());
                    }
                }
            }
        }

        // Verify module/function references
        for node in self.callgraph.nodes() {
            // If the module is pending, it is being linked
            let object = self.pending.get(&node.module);
            let is_allowed_missing = self
                .allow_missing
                .iter()
                .any(|pattern| node.module.as_str().starts_with(pattern.as_ref()));

            // If a referenced module is not present for the link, raise an error, unless it is
            // specifically allowed to be missing at this point.
            if object.is_none() {
                if is_allowed_missing {
                    continue;
                }

                return Err(self
                    .diagnostics
                    .diagnostic(Severity::Error)
                    .with_message(format!("linker error: undefined module '{}'", &node.module))
                    .into_report());
            }

            // The module is present, so we must verify that the function is defined in that module
            let object = unsafe { object.unwrap_unchecked() };
            let (is_externally_linkable, signature) = match object {
                Object::Hir(ref module) => match module.function(node.function) {
                    Some(function) => (function.is_public(), Some(&function.signature)),
                    None if is_allowed_missing => (true, None),
                    None => {
                        return Err(self
                            .diagnostics
                            .diagnostic(Severity::Error)
                            .with_message(format!("linker error: undefined function '{}'", &node))
                            .into_report())
                    }
                },
                Object::Masm { ref exports, .. } => {
                    if !exports.contains(&node.function) && !is_allowed_missing {
                        return Err(self
                            .diagnostics
                            .diagnostic(Severity::Error)
                            .with_message(format!("linker error: undefined function '{}'", &node))
                            .into_report());
                    }
                    (true, None)
                }
            };

            // Next, visit all of the dependent functions, and ensure their signatures match
            for dependent_id in self.callgraph.neighbors_directed(node, Direction::Incoming) {
                // If the dependent is in another module, but the function has internal linkage,
                // raise an error
                if dependent_id.module != node.module && !is_externally_linkable {
                    return Err(self
                        .diagnostics
                        .diagnostic(Severity::Error)
                        .with_message("linker error")
                        .with_primary_label(
                            dependent_id.function.span,
                            format!(
                                "this function contains an invalid reference to '{}'",
                                &node.function
                            ),
                        )
                        .with_help(
                            "Only functions with external linkage can be referenced across modules",
                        )
                        .into_report());
                }
                // Otherwise, make sure the signatures match (if we have signatures available)
                let dependent_object = &self.pending[&dependent_id.module];
                match (signature, dependent_object) {
                    (Some(signature), Object::Hir(ref dependent_module)) => {
                        let dependent_function = dependent_module
                            .function(dependent_id.function)
                            .expect("dependency graph is outdated");
                        let external_ref = dependent_function
                            .dfg
                            .get_import(&node)
                            .expect("dependency graph is outdated");
                        let external_span = external_ref.id.span();
                        verify_matching_signature(
                            node,
                            external_span,
                            signature,
                            &external_ref.signature,
                            self.diagnostics,
                        )?;
                    }
                    // If we don't have a signature for the dependency, we presume it matches the
                    // dependent
                    (None, Object::Hir(_)) => (),
                    // If the dependent is MASM, we don't know what signature it used, so we
                    // presume it is correct
                    (_, Object::Masm { .. }) => (),
                }
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
                return Err(self
                    .diagnostics
                    .diagnostic(Severity::Error)
                    .with_message(format!("linker error: undefined global variable '{name}'"))
                    .into_report());
            }
        }

        // Run the garbage collector
        self.garbage_collect();

        // We're finished processing all pending modules, so add them to the program
        for object in self.pending.into_values() {
            match object {
                Object::Hir(module) => {
                    self.program.modules.insert(module);
                }
                Object::Masm { .. } => {
                    // These objects are provided to the assembler directly
                    continue;
                }
            }
        }

        Ok(self.program)
    }

    /// If an executable is being linked, discover unused functions and garbage collect them.
    ///
    /// Once a function has been identified as dead and is collected, any transitive items it
    /// references may also be collected if they are orphaned as a result of the collection.
    ///
    /// If a library is being linked, this function is a no-op, as it is not known what will
    /// be needed at runtime once used in the context of a program.
    fn garbage_collect(&mut self) {
        // TODO: See https://github.com/0xPolygonMiden/miden-ir/issues/26
    }
}

/// Verifies that the actual signature of the given function matches what was expected.
///
/// Here, `actual` is the defined signature of the function; while `expected` is the signature
/// associated with an [ExternalFunction], i.e. it is the signature expected by a prospective
/// caller.
fn verify_matching_signature(
    id: FunctionIdent,
    expected_span: SourceSpan,
    actual: &Signature,
    expected: &Signature,
    diagnostics: &DiagnosticsHandler,
) -> Result<(), Report> {
    // If the number of parameters differs, raise an error
    if expected.arity() != actual.arity() {
        return Err(diagnostics
            .diagnostic(Severity::Error)
            .with_message("linker error")
            .with_primary_label(id.span(), "the arity of this function declaration is incorrect")
            .with_secondary_label(
                expected_span,
                format!("the actual arity of the definition is {}", expected.arity()),
            )
            .into_report());
    }

    // If the type or specification of any parameters differs, raise an error
    for (i, (ep, ap)) in expected.params().iter().zip(actual.params().iter()).enumerate() {
        if !is_matching_param(ep, ap) {
            return Err(diagnostics
                .diagnostic(Severity::Error)
                .with_message("linker error")
                .with_primary_label(
                    id.span(),
                    "the type signature of this function declaration is incorrect",
                )
                .with_secondary_label(expected_span, "it does not match the signature defined here")
                .with_help(format!("The parameter at index {i} is defined as {}", &ep.ty))
                .into_report());
        }
    }

    // If the number of results differs, raise an error
    let expected_results = expected.results();
    let actual_results = actual.results();
    if expected_results.len() != actual_results.len() {
        return Err(diagnostics
            .diagnostic(Severity::Error)
            .with_message("linker error")
            .with_primary_label(
                id.span(),
                "the return arity of this function declaration is incorrect",
            )
            .with_secondary_label(
                expected_span,
                format!("the actual number of return values is {}", expected_results.len()),
            )
            .into_report());
    }

    // If the type of results differs, raise an error
    for (i, (er, ar)) in expected_results.iter().zip(actual_results.iter()).enumerate() {
        if !is_matching_param(er, ar) {
            return Err(diagnostics
                .diagnostic(Severity::Error)
                .with_message("linker error")
                .with_primary_label(
                    id.span(),
                    "the type signature of this function declaration is incorrect",
                )
                .with_secondary_label(expected_span, "it does not match the signature defined here")
                .with_help(format!("The result at index {i} is defined as {}", &er.ty))
                .into_report());
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
/// Returns a [Report] if a cycle is found.
fn validate_callgraph(
    callgraph: &DiGraphMap<FunctionIdent, ()>,
    diagnostics: &DiagnosticsHandler,
) -> Result<(), Report> {
    use petgraph::visit::{depth_first_search, DfsEvent, IntoNodeIdentifiers};

    depth_first_search(callgraph, callgraph.node_identifiers(), |event| match event {
        DfsEvent::BackEdge(caller, callee) => Err(diagnostics
            .diagnostic(Severity::Error)
            .with_message("linker error")
            .with_primary_label(caller.span(), "this function contains recursion")
            .with_secondary_label(callee.span(), "due to one or more calls to this function")
            .with_help(
                "If you need to make the call recursive, you may need to use indirect calls to \
                 acheive this",
            )
            .into_report()),
        _ => Ok(()),
    })
}
