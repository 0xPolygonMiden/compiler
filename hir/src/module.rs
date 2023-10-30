use std::{collections::BTreeMap, fmt};

use intrusive_collections::{
    intrusive_adapter,
    linked_list::{Cursor, CursorMut},
    LinkedList, RBTreeLink,
};
use miden_diagnostics::{DiagnosticsHandler, Severity, Spanned};
use rustc_hash::FxHashSet;

use super::*;

/// This error is raised when two modules conflict with the same symbol name
#[derive(Debug, thiserror::Error)]
#[error("module {} has already been declared", .0)]
pub struct ModuleConflictError(pub Ident);

intrusive_adapter!(pub ModuleTreeAdapter = Box<Module>: Module { link: RBTreeLink });
impl<'a> intrusive_collections::KeyAdapter<'a> for ModuleTreeAdapter {
    type Key = Ident;

    #[inline]
    fn get_key(&self, module: &'a Module) -> Ident {
        module.name
    }
}

/// Represents a SSA IR module
///
/// These correspond to MASM modules
/// This module is largely a container for functions, but it also provides
/// as the owner for pooled resources available to functions:
///
/// * Mapping from Signature to FuncRef
/// * Mapping from FunctionName to FuncRef
#[derive(Spanned)]
pub struct Module {
    /// The link used to attach this module to a [Program]
    link: RBTreeLink,
    /// The name of this module
    #[span]
    pub name: Ident,
    /// Documentation attached to this module, to be passed through to
    /// Miden Assembly during code generation.
    pub docs: Option<String>,
    /// The set of data segments allocated in this module
    pub(crate) segments: DataSegmentTable,
    /// The set of global variables declared in this module
    pub(crate) globals: GlobalVariableTable,
    /// The set of functions which belong to this module, in the order
    /// in which they were defined.
    pub(crate) functions: LinkedList<FunctionListAdapter>,
    /// This flag indicates whether this module is a kernel module
    ///
    /// Kernel modules have additional constraints imposed on them that regular
    /// modules do not, in exchange for some useful functionality:
    ///
    /// * Functions with external linkage are required to use the `Kernel` calling convention.
    /// * A kernel module executes in the root context of the Miden VM, allowing one to expose functionality
    /// that is protected from tampering by other non-kernel functions in the program.
    /// * Due to the above, you may not reference globals outside the kernel module, from within
    /// kernel functions, as they are not available in the root context.
    is_kernel: bool,
}
impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::write::DisplayIdent;
        use std::fmt::Write;

        if self.is_kernel {
            writeln!(f, "kernel {}\n", DisplayIdent(&self.name))?;
        } else {
            writeln!(f, "module {}\n", DisplayIdent(&self.name))?;
        }

        let has_segments = !self.segments.is_empty();
        let has_globals = !self.globals.is_empty();
        let has_constants = self.globals.has_constants();

        if has_segments {
            f.write_str("memory {\n")?;
            for segment in self.segments.iter() {
                writeln!(
                    f,
                    "    segment @{:#x} x {} = {};",
                    segment.offset(),
                    segment.size(),
                    segment.init(),
                )?;
            }
            f.write_str("}\n")?;
        }

        if has_globals {
            if has_constants {
                if has_segments {
                    f.write_char('\n')?;
                }
                for (constant, constant_data) in self.globals.constants() {
                    let id = constant.as_u32();
                    writeln!(f, "const ${id} = {constant_data};")?;
                }

                f.write_char('\n')?;
            }

            for global in self.globals.iter() {
                write!(
                    f,
                    "global {} @{} : {}",
                    global.linkage,
                    DisplayIdent(&global.name),
                    global.ty
                )?;
                match global.init {
                    Some(init) => {
                        writeln!(
                            f,
                            " = ${} {{ id = {} }};",
                            init.as_u32(),
                            global.id().as_u32()
                        )?;
                    }
                    None => {
                        writeln!(f, " {{ id = {} }};", global.id().as_u32())?;
                    }
                }
            }
            f.write_char('\n')?;
        }

        let mut external_functions = BTreeMap::<FunctionIdent, Signature>::default();
        for (i, function) in self.functions.iter().enumerate() {
            for import in function.dfg.imports() {
                // Don't print declarations for functions in this module
                if import.id.module == self.name {
                    continue;
                }
                external_functions
                    .entry(import.id)
                    .or_insert_with(|| import.signature.clone());
            }
            if i > 0 {
                writeln!(f)?;
            }
            write_function(f, function)?;
        }

        if !external_functions.is_empty() {
            writeln!(f)?;

            for (id, sig) in external_functions.iter() {
                writeln!(f)?;
                write_external_function(f, id, sig)?;
            }
        }

        Ok(())
    }
}
impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Module")
            .field("name", &self.name)
            .field("is_kernel", &self.is_kernel)
            .field("docs", &self.docs)
            .field("segments", &self.segments)
            .field("globals", &self.globals)
            .field("functions", &self.functions)
            .finish()
    }
}
impl midenc_session::Emit for Module {
    fn output_type(&self) -> midenc_session::OutputType {
        midenc_session::OutputType::Hir
    }
    fn write_to<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_fmt(format_args!("{}", self))
    }
}
impl Eq for Module {}
impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        let is_eq = self.name == other.name
            && self.is_kernel == other.is_kernel
            && self.docs == other.docs
            && self.segments().eq(other.segments())
            && self.globals.len() == other.globals.len()
            && self.functions.iter().count() == other.functions.iter().count();
        if !is_eq {
            return false;
        }

        for global in self.globals() {
            let id = global.id();
            if !other.globals.contains_key(id) {
                return false;
            }
            let other_global = other.globals.get(id);
            if global != other_global {
                return false;
            }
        }

        for function in self.functions.iter() {
            if !other.contains(function.id.function) {
                return false;
            }
            if let Some(other_function) = other.function(function.id.function) {
                if function != other_function {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

/// This macro asserts that a function is valid for insertion into a given module.
macro_rules! assert_valid_function {
    ($module:ident, $function:ident) => {
        assert_eq!($module.name, $function.id.module, "mismatched module identifiers");
        assert!(
            $function.is_detached(),
            "cannot attach a function to a module that is already attached to a module"
        );
        // Validate the kernel rules
        if $function.is_kernel() {
            assert!($module.is_kernel, "cannot add kernel functions to a non-kernel module");
        } else if $module.is_kernel && $function.is_public() {
            panic!("functions with external linkage in kernel modules must use the kernel calling convention");
        }
    }
}

impl Module {
    /// Create a new, empty [Module]
    pub fn new<S: Into<Ident>>(name: S) -> Self {
        Self::make(name.into(), /*is_kernel=*/ false)
    }

    /// Create a new, empty [Module] with the given source location
    pub fn new_with_span<S: AsRef<str>>(name: S, span: SourceSpan) -> Self {
        let name = Ident::new(Symbol::intern(name.as_ref()), span);
        Self::make(name, /*is_kernel=*/ false)
    }

    /// Create a new, empty kernel [Module]
    pub fn new_kernel<S: Into<Ident>>(name: S) -> Self {
        Self::make(name.into(), /*is_kernel=*/ true)
    }

    /// Create a new, empty kernel [Module] with the given source location
    pub fn new_kernel_with_span<S: AsRef<str>>(name: S, span: SourceSpan) -> Self {
        let name = Ident::new(Symbol::intern(name.as_ref()), span);
        Self::make(name, /*is_kernel=*/ true)
    }

    fn make(name: Ident, is_kernel: bool) -> Self {
        Self {
            link: Default::default(),
            name,
            docs: None,
            segments: Default::default(),
            globals: GlobalVariableTable::new(ConflictResolutionStrategy::None),
            functions: Default::default(),
            is_kernel,
        }
    }

    /// Returns true if this module is a kernel module
    #[inline]
    pub const fn is_kernel(&self) -> bool {
        self.is_kernel
    }

    /// Returns true if this module has yet to be attached to a [Program]
    pub fn is_detached(&self) -> bool {
        !self.link.is_linked()
    }

    /// Return an iterator over the data segments allocated in this module
    ///
    /// The iterator is double-ended, so can be used to traverse the segments in either direction.
    ///
    /// Data segments are ordered by the address at which are are allocated, in ascending order.
    pub fn segments<'a, 'b: 'a>(
        &'b self,
    ) -> intrusive_collections::linked_list::Iter<'a, DataSegmentAdapter> {
        self.segments.iter()
    }

    /// Declare a new [DataSegment] in this module, with the given offset, size, and data.
    ///
    /// Returns `Err` if the segment declaration is invalid, or conflicts with an existing segment
    ///
    /// Data segments are ordered by the address at which they are allocated, at link-time, all
    /// segments from all modules are linked together, and they must either be disjoint, or exactly
    /// identical in order to overlap - it is not permitted to have partially overlapping segments
    /// with different views of the memory represented by that segment.
    pub fn declare_data_segment(
        &mut self,
        offset: Offset,
        size: u32,
        init: ConstantData,
        readonly: bool,
    ) -> Result<(), DataSegmentError> {
        self.segments.declare(offset, size, init, readonly)
    }

    /// Return an iterator over the global variables declared in this module
    ///
    /// The iterator is double-ended, so can be used to traverse the globals table in either direction
    pub fn globals<'a, 'b: 'a>(
        &'b self,
    ) -> intrusive_collections::linked_list::Iter<'a, GlobalVariableAdapter> {
        self.globals.iter()
    }

    /// Declare a new [GlobalVariable] in this module, with the given name, type, linkage, and optional initializer.
    ///
    /// Returns `Err` if a symbol with the same name but conflicting declaration already exists,
    /// or if the specification of the global variable is invalid in any way.
    ///
    /// NOTE: The [GlobalVariable] returned here is scoped to this module only, it cannot be used to
    /// index into the global variable table of a [Program], which is constructed at link-time.
    pub fn declare_global_variable(
        &mut self,
        name: Ident,
        ty: Type,
        linkage: Linkage,
        init: Option<ConstantData>,
    ) -> Result<GlobalVariable, GlobalVariableError> {
        self.globals.declare(name, ty, linkage, init)
    }

    /// Set the initializer for a [GlobalVariable] to `init`.
    ///
    /// Returns `Err` if the initializer conflicts with the current definition of the global in any way.
    pub fn set_global_initializer(
        &mut self,
        gv: GlobalVariable,
        init: ConstantData,
    ) -> Result<(), GlobalVariableError> {
        self.globals.set_initializer(gv, init)
    }

    /// Get the data associated with the given [GlobalVariable]
    #[inline]
    pub fn global(&self, id: GlobalVariable) -> &GlobalVariableData {
        self.globals.get(id)
    }

    /// Look up a global by `name`.
    pub fn find_global(&self, name: Ident) -> Option<&GlobalVariableData> {
        self.globals.find(name).map(|gv| self.globals.get(gv))
    }

    /// Return an iterator over the functions in this module
    ///
    /// The iterator is double-ended, so can be used to traverse the module body in either direction
    pub fn functions<'a, 'b: 'a>(
        &'b self,
    ) -> intrusive_collections::linked_list::Iter<'a, FunctionListAdapter> {
        self.functions.iter()
    }

    /// Get a [Function] in this module by name, if available
    pub fn function<'a, 'b: 'a>(&'b self, id: Ident) -> Option<&'a Function> {
        self.cursor_at(id).get()
    }

    /// Compute the set of imports for this module, automatically aliasing modules when there
    /// are namespace conflicts
    pub fn imports(&self) -> ModuleImportInfo {
        let mut imports = ModuleImportInfo::default();
        let locals = self
            .functions
            .iter()
            .map(|f| f.id)
            .collect::<FxHashSet<FunctionIdent>>();

        for function in self.functions.iter() {
            for import in function.imports() {
                if !locals.contains(&import.id) {
                    imports.add(import.id);
                }
            }
        }
        imports
    }

    /// Returns true if this module contains the function `name`
    pub fn contains(&self, name: Ident) -> bool {
        self.function(name).is_some()
    }

    /// Unlinks the given function from this module
    pub fn unlink(&mut self, id: Ident) -> Box<Function> {
        let mut cursor = self.cursor_mut_at(id);
        cursor.remove().expect("invalid function id")
    }

    /// Append `function` to the end of this module's body, returning the [FuncId]
    /// assigned to it within this module.
    ///
    /// NOTE: This function will panic if either of the following rules are violated:
    ///
    /// * If this module is a kernel module, public functions must use the kernel calling convention,
    /// however private functions can use any convention.
    /// * If this module is not a kernel module, functions may not use the kernel calling convention
    pub fn push(&mut self, function: Box<Function>) -> Result<(), SymbolConflictError> {
        assert_valid_function!(self, function);
        if let Some(prev) = self.function(function.id.function) {
            return Err(SymbolConflictError(prev.id));
        }
        self.functions.push_back(function);
        Ok(())
    }

    /// Insert `function` in the module body before the function with id `before`
    ///
    /// If `before` is no longer attached to this module, `function` is added to
    /// the end of the module body.
    pub fn insert_before(
        &mut self,
        function: Box<Function>,
        before: Ident,
    ) -> Result<(), SymbolConflictError> {
        assert_valid_function!(self, function);
        if let Some(prev) = self.function(function.id.function) {
            return Err(SymbolConflictError(prev.id));
        }

        let mut cursor = self.cursor_mut_at(before);
        cursor.insert_before(function);

        Ok(())
    }

    /// Insert `function` in the module body after the function with id `after`
    ///
    /// If `after` is no longer attached to this module, `function` is added to
    /// the end of the module body.
    pub fn insert_after(
        &mut self,
        function: Box<Function>,
        after: Ident,
    ) -> Result<(), SymbolConflictError> {
        assert_valid_function!(self, function);
        if let Some(prev) = self.function(function.id.function) {
            return Err(SymbolConflictError(prev.id));
        }

        let mut cursor = self.cursor_mut_at(after);
        if cursor.is_null() {
            cursor.insert_before(function);
        } else {
            cursor.insert_after(function);
        }

        Ok(())
    }

    /// Returns a mutable cursor to the module body, starting at the first function.
    ///
    /// If the module body is empty, the returned cursor will point to the null object.
    ///
    /// NOTE: If one uses this cursor to insert a function that is invalid
    #[inline]
    pub fn cursor_mut<'a, 'b: 'a>(&'b mut self) -> ModuleCursor<'a> {
        ModuleCursor {
            cursor: self.functions.front_mut(),
            name: self.name,
            is_kernel: self.is_kernel,
        }
    }

    /// Returns a cursor to the module body, located at the function indicated by `id`.
    ///
    /// If no function with `id` is in the list, the returned cursor will point to the null object.
    pub fn cursor_at<'a, 'b: 'a>(&'b self, id: Ident) -> Cursor<'a, FunctionListAdapter> {
        let mut cursor = self.functions.front();
        while let Some(function) = cursor.get() {
            if function.id.function == id {
                break;
            }
            cursor.move_next();
        }
        cursor
    }

    /// Returns a mutable cursor to the module body, located at the function indicated by `id`.
    ///
    /// If no function with `id` is in the list, the returned cursor will point to the null object.
    pub fn cursor_mut_at<'a, 'b: 'a>(&'b mut self, id: Ident) -> ModuleCursor<'a> {
        let mut cursor = self.functions.front_mut();
        while let Some(function) = cursor.get() {
            if function.id.function == id {
                break;
            }
            cursor.move_next();
        }
        ModuleCursor {
            cursor,
            name: self.name,
            is_kernel: self.is_kernel,
        }
    }
}

pub struct ModuleCursor<'a> {
    cursor: CursorMut<'a, FunctionListAdapter>,
    name: Ident,
    is_kernel: bool,
}
impl<'a> ModuleCursor<'a> {
    /// Returns true if this cursor is pointing to the null object
    #[inline(always)]
    pub fn is_null(&self) -> bool {
        self.cursor.is_null()
    }

    /// Return a reference to the function pointed to by this cursor
    ///
    /// If the cursor is pointing to the null object, `None` is returned
    #[inline(always)]
    pub fn get(&self) -> Option<&Function> {
        self.cursor.get()
    }

    /// Insert a new function into the module after the cursor.
    ///
    /// If the cursor is pointing to the null object, the insert happens at the front of the list.
    ///
    /// NOTE: This function will panic if the function violates the validation rules for
    /// the module, i.e. must not be attached, follows kernel module rules when applicable.
    pub fn insert_after(&mut self, function: Box<Function>) {
        assert_valid_function!(self, function);
        self.cursor.insert_after(function);
    }

    /// Insert a new function into the module before the cursor.
    ///
    /// If the cursor is pointing to the null object, the insert happens at the end of the list.
    ///
    /// NOTE: This function will panic if the function violates the validation rules for
    /// the module, i.e. must not be attached, follows kernel module rules when applicable.
    pub fn insert_before(&mut self, function: Box<Function>) {
        assert_valid_function!(self, function);
        self.cursor.insert_before(function);
    }

    /// Moves this cursor to the next function in the module.
    ///
    /// If the cursor is pointing to the null object, then this moves the cursor to the front
    /// of the list. If at the end of the list, it moves to the null object.
    #[inline(always)]
    pub fn move_next(&mut self) {
        self.cursor.move_next();
    }

    /// Moves this cursor to the previous function in the module.
    ///
    /// If the cursor is pointing to the null object, then this moves the cursor to the end
    /// of the list. If at the front of the list, it moves to the null object.
    #[inline(always)]
    pub fn move_prev(&mut self) {
        self.cursor.move_prev();
    }

    /// Return a cursor pointing to the next function in the module.
    ///
    /// If this cursor is on the null object, then the returned cursor will be on the
    /// front of the list. If at the last element, then the returned cursor will on the
    /// null object.
    #[inline(always)]
    pub fn peek_next(&self) -> Cursor<'_, FunctionListAdapter> {
        self.cursor.peek_next()
    }

    /// Return a cursor pointing to the previous function in the module.
    ///
    /// If this cursor is on the null object, then the returned cursor will be on the
    /// end of the list. If at the first element, then the returned cursor will on the
    /// null object.
    #[inline(always)]
    pub fn peek_prev(&self) -> Cursor<'_, FunctionListAdapter> {
        self.cursor.peek_prev()
    }

    /// Removes the current function from the module.
    ///
    /// The cursor will be moved to the next function in the module, or the null object
    /// if we're at the end of the module.
    #[inline(always)]
    pub fn remove(&mut self) -> Option<Box<Function>> {
        self.cursor.remove()
    }
}

pub struct ModuleBuilder {
    module: Box<Module>,
}
impl From<Box<Module>> for ModuleBuilder {
    fn from(module: Box<Module>) -> Self {
        Self { module }
    }
}
impl ModuleBuilder {
    pub fn new<S: Into<Ident>>(name: S) -> Self {
        Self {
            module: Box::new(Module::new(name)),
        }
    }

    pub fn new_kernel<S: Into<Ident>>(name: S) -> Self {
        Self {
            module: Box::new(Module::new_kernel(name)),
        }
    }

    pub fn with_span(&mut self, span: SourceSpan) -> &mut Self {
        self.module.name = Ident::new(self.module.name.as_symbol(), span);
        self
    }

    pub fn with_docs<S: Into<String>>(&mut self, docs: S) -> &mut Self {
        self.module.docs = Some(docs.into());
        self
    }

    pub fn name(&self) -> Ident {
        self.module.name
    }

    pub fn declare_global_variable<S: AsRef<str>>(
        &mut self,
        name: S,
        ty: Type,
        linkage: Linkage,
        init: Option<ConstantData>,
        span: SourceSpan,
    ) -> Result<GlobalVariable, GlobalVariableError> {
        let name = Ident::new(Symbol::intern(name.as_ref()), span);
        self.module.declare_global_variable(name, ty, linkage, init)
    }

    pub fn set_global_initializer(
        &mut self,
        gv: GlobalVariable,
        init: ConstantData,
    ) -> Result<(), GlobalVariableError> {
        self.module.set_global_initializer(gv, init)
    }

    pub fn declare_data_segment<I: Into<ConstantData>>(
        &mut self,
        offset: Offset,
        size: u32,
        init: I,
        readonly: bool,
    ) -> Result<(), DataSegmentError> {
        self.module
            .declare_data_segment(offset, size, init.into(), readonly)
    }

    /// Start building a new function in this module
    pub fn function<'a, 'b: 'a, S: Into<Ident>>(
        &'b mut self,
        name: S,
        signature: Signature,
    ) -> Result<ModuleFunctionBuilder<'a>, SymbolConflictError> {
        let name = name.into();
        if let Some(prev) = self.module.function(name) {
            return Err(SymbolConflictError(prev.id));
        }

        let id = FunctionIdent {
            module: self.module.name,
            function: name,
        };
        let function = Box::new(Function::new(id, signature));
        let entry = function.dfg.entry;

        Ok(ModuleFunctionBuilder {
            builder: self,
            function,
            position: entry,
        })
    }

    pub fn build(self) -> Box<Module> {
        self.module
    }
}

pub struct ModuleFunctionBuilder<'m> {
    builder: &'m mut ModuleBuilder,
    function: Box<Function>,
    position: Block,
}
impl<'m> ModuleFunctionBuilder<'m> {
    pub fn with_span(&mut self, span: SourceSpan) -> &mut Self {
        self.function.id.function = Ident::new(self.function.id.function.as_symbol(), span);
        self
    }

    /// Get the fully-qualified name of the underlying function
    pub fn id(&self) -> FunctionIdent {
        self.function.id
    }

    /// Get the signature of the underlying function
    pub fn signature(&self) -> &Signature {
        &self.function.signature
    }

    pub fn module<'a, 'b: 'a>(&'b mut self) -> &'a mut ModuleBuilder {
        self.builder
    }

    #[inline(always)]
    pub fn data_flow_graph(&self) -> &DataFlowGraph {
        &self.function.dfg
    }

    #[inline(always)]
    pub fn data_flow_graph_mut(&mut self) -> &mut DataFlowGraph {
        &mut self.function.dfg
    }

    #[inline]
    pub fn entry_block(&self) -> Block {
        self.function.dfg.entry
    }

    #[inline]
    pub fn current_block(&self) -> Block {
        self.position
    }

    #[inline]
    pub fn switch_to_block(&mut self, block: Block) {
        self.position = block;
    }

    pub fn create_block(&mut self) -> Block {
        self.data_flow_graph_mut().create_block()
    }

    pub fn block_params(&self, block: Block) -> &[Value] {
        self.data_flow_graph().block_params(block)
    }

    pub fn append_block_param(&mut self, block: Block, ty: Type, span: SourceSpan) -> Value {
        self.data_flow_graph_mut()
            .append_block_param(block, ty, span)
    }

    pub fn inst_results(&self, inst: Inst) -> &[Value] {
        self.data_flow_graph().inst_results(inst)
    }

    pub fn first_result(&self, inst: Inst) -> Value {
        self.data_flow_graph().first_result(inst)
    }

    pub fn import_function<M, F>(
        &mut self,
        module: M,
        function: F,
        signature: Signature,
    ) -> Result<FunctionIdent, SymbolConflictError>
    where
        M: Into<Ident>,
        F: Into<Ident>,
    {
        self.function
            .dfg
            .import_function(module.into(), function.into(), signature)
    }

    pub fn ins<'a, 'b: 'a>(&'b mut self) -> DefaultInstBuilder<'a> {
        DefaultInstBuilder::new(&mut self.function.dfg, self.position)
    }

    pub fn build(
        self,
        diagnostics: &DiagnosticsHandler,
    ) -> Result<FunctionIdent, InvalidFunctionError> {
        let sig = self.function.signature();
        match sig.linkage {
            Linkage::External | Linkage::Internal => (),
            linkage => {
                diagnostics
                    .diagnostic(Severity::Error)
                    .with_message(format!(
                        "invalid linkage ('{}') for function '{}'",
                        linkage, &self.function.id
                    ))
                    .with_note("Only 'external' and 'internal' linkage are valid for functions")
                    .emit();
                return Err(InvalidFunctionError);
            }
        }

        let is_kernel_module = self.builder.module.is_kernel;
        let is_public = sig.is_public();

        match sig.cc {
            CallConv::Kernel if is_kernel_module => {
                if !is_public {
                    diagnostics.diagnostic(Severity::Error)
                               .with_message(format!("expected external linkage for kernel function '{}'", &self.function.id))
                        .with_note("This function is private, but uses the 'kernel' calling convention. It must either be made public, or use a different convention")
                        .emit();
                    return Err(InvalidFunctionError);
                }
            }
            CallConv::Kernel => {
                diagnostics.diagnostic(Severity::Error)
                    .with_message(format!("invalid calling convention for function '{}'", &self.function.id))
                    .with_note("The 'kernel' calling convention is only allowed in kernel modules, on functions with external linkage")
                    .emit();
                return Err(InvalidFunctionError);
            }
            _ if is_kernel_module && is_public => {
                diagnostics.diagnostic(Severity::Error)
                    .with_message(format!("invalid calling convention for function '{}'", &self.function.id))
                    .with_note("Functions with external linkage, must use the 'kernel' calling convention when defined in a kernel module")
                    .emit();
                return Err(InvalidFunctionError);
            }
            _ => (),
        }

        let id = self.function.id;
        self.builder.module.functions.push_back(self.function);

        Ok(id)
    }
}

#[derive(Debug)]
pub struct InvalidFunctionError;
