use std::{fmt, path::Path, sync::Arc};

use hir::{Signature, Symbol};
use miden_assembly::{
    ast::{ModuleKind, ProcedureName},
    KernelLibrary, Library as CompiledLibrary, LibraryNamespace,
};
use miden_core::crypto::hash::Rpo256;
use midenc_hir::{
    self as hir, diagnostics::Report, DataSegmentTable, Felt, FieldElement, FunctionIdent,
    GlobalVariableTable, Ident, SourceSpan,
};
use midenc_hir_analysis::GlobalVariableAnalysis;
use midenc_session::{Emit, Session};

use super::{module::Modules, *};
use crate::packaging::Rodata;

inventory::submit! {
    midenc_session::CompileFlag::new("test_harness")
        .long("test-harness")
        .action(midenc_session::FlagAction::SetTrue)
        .help("If present, causes the code generator to emit extra code for the VM test harness")
        .help_heading("Testing")
}

/// A [Program] represents a complete set of modules which are intended to be shipped and executed
/// together.
#[derive(Clone)]
pub struct Program {
    /// The code for this program
    library: Library,
    /// The function identifier for the program entrypoint, if applicable
    entrypoint: FunctionIdent,
    /// The base address of the dynamic heap, as computed by the codegen backend
    ///
    /// Defaults to an offset which is two 64k pages from the start of linear memory,
    /// or, if available, the next byte following the both the reserved linear memory region as
    /// declared in HIR, and the global variables of the program.
    heap_base: u32,
}
impl Program {
    /// Create a new [Program] initialized from an [hir::Program].
    ///
    /// The resulting [Program] will have the following:
    ///
    /// * Data segments described by the original [hir::Program]
    /// * The entrypoint function which will be invoked after the initialization phase of startup
    /// * If an entrypoint is set, an executable [Module] which performs initialization and then
    ///   invokes the entrypoint
    ///
    /// None of the HIR modules will have been added yet
    pub fn from_hir(
        program: &hir::Program,
        globals: &GlobalVariableAnalysis<hir::Program>,
    ) -> Result<Self, Report> {
        let Some(entrypoint) = program.entrypoint() else {
            return Err(Report::msg("invalid program: no entrypoint"));
        };
        let library = Library::from_hir(program, globals);

        // Compute the first page boundary after the end of the globals table to use as the start
        // of the dynamic heap when the program is executed
        let heap_base = program.reserved_memory_bytes()
            + u32::try_from(
                program.globals().size_in_bytes().next_multiple_of(program.page_size() as usize),
            )
            .expect("unable to allocate dynamic heap: global table too large");
        Ok(Self {
            library,
            entrypoint,
            heap_base,
        })
    }

    /// Get the raw [Rodata] segments for this program
    pub fn rodatas(&self) -> &[Rodata] {
        self.library.rodata.as_slice()
    }

    /// Link this [Program] against the given kernel during assembly
    pub fn link_kernel(&mut self, kernel: KernelLibrary) {
        self.library.link_kernel(kernel);
    }

    /// Link this [Program] against the given library during assembly
    pub fn link_library(&mut self, library: CompiledLibrary) {
        self.library.link_library(library);
    }

    /// Get the set of [CompiledLibrary] this program links against
    pub fn link_libraries(&self) -> &[CompiledLibrary] {
        self.library.link_libraries()
    }

    /// Generate an executable module which when run expects the raw data segment data to be
    /// provided on the advice stack in the same order as initialization, and the operands of
    /// the entrypoint function on the operand stack.
    fn generate_main(&self, entrypoint: FunctionIdent, emit_test_harness: bool) -> Box<Module> {
        let mut exe = Box::new(Module::new(LibraryNamespace::Exec.into(), ModuleKind::Executable));
        let start_id = FunctionIdent {
            module: Ident::with_empty_span(Symbol::intern(LibraryNamespace::EXEC_PATH)),
            function: Ident::with_empty_span(Symbol::intern(ProcedureName::MAIN_PROC_NAME)),
        };
        let start_sig = Signature::new([], []);
        let mut start = Box::new(Function::new(start_id, start_sig));
        {
            let body = start.body_mut();
            // Initialize dynamic heap
            body.push(Op::PushU32(self.heap_base), SourceSpan::default());
            body.push(
                Op::Exec("intrinsics::mem::heap_init".parse().unwrap()),
                SourceSpan::default(),
            );
            // Initialize data segments from advice stack
            self.emit_data_segment_initialization(body);
            // Possibly initialize test harness
            if emit_test_harness {
                self.emit_test_harness(body);
            }
            // Invoke the program entrypoint
            body.push(Op::Exec(entrypoint), SourceSpan::default());
        }
        exe.push_back(start);
        exe
    }

    fn emit_test_harness(&self, block: &mut Block) {
        let span = SourceSpan::default();

        // Advice Stack: [dest_ptr, num_words, ...]
        block.push(Op::AdvPush(2), span); // => [num_words, dest_ptr] on operand stack
        block.push(Op::Exec("std::mem::pipe_words_to_memory".parse().unwrap()), span);
        // Drop the commitment
        block.push(Op::Drop, span);
        // If we know the stack pointer address, update it to the value of `'write_ptr`, but cast
        // into the Rust address space (multiplying it by 16). So a word address of 1, is equal to
        // a byte address of 16, because each field element holds 4 bytes, and there are 4 elements
        // in a word.
        //
        // If we don't know the stack pointer, just drop the `'write_ptr` value
        if let Some(sp) = self.stack_pointer() {
            block.push(Op::U32OverflowingMulImm(16), span);
            block.push(Op::Assertz, span);
            // Align the stack pointer to a word boundary
            let elem_addr = (sp / 4) + (sp % 4 > 0) as u32;
            let word_addr = (elem_addr / 4) + (elem_addr % 4 > 0) as u32;
            block.push(Op::MemStoreImm(word_addr), span);
        } else {
            block.push(Op::Drop, span);
        }
    }

    /// Emit the sequence of instructions necessary to consume rodata from the advice stack and
    /// populate the global heap with the data segments of this program, verifying that the
    /// commitments match.
    fn emit_data_segment_initialization(&self, block: &mut Block) {
        // Emit data segment initialization code
        //
        // NOTE: This depends on the program being executed with the data for all data
        // segments having been placed in the advice map with the same commitment and
        // encoding used here. The program will fail to execute if this is not set up
        // correctly.
        //
        // TODO(pauls): To facilitate automation of this, we should emit an inputs file to
        // disk that maps each segment to a commitment and its data encoded as binary. This
        // can then be loaded into the advice provider during VM init.
        let pipe_preimage_to_memory = "std::mem::pipe_preimage_to_memory".parse().unwrap();
        for rodata in self.library.rodata.iter() {
            let span = SourceSpan::default();

            // Move rodata from advice map to advice stack
            block.push(Op::Pushw(rodata.digest.into()), span); // COM
            block.push(Op::AdvInjectPushMapVal, span);
            // write_ptr
            block.push(Op::PushU32(rodata.start.waddr), span);
            // num_words
            block.push(Op::PushU32(rodata.size_in_words() as u32), span);
            // [num_words, write_ptr, COM, ..] -> [write_ptr']
            block.push(Op::Exec(pipe_preimage_to_memory), span);
            // drop write_ptr'
            block.push(Op::Drop, span);
        }
    }

    #[inline(always)]
    pub fn entrypoint(&self) -> FunctionIdent {
        self.entrypoint
    }

    #[inline(always)]
    pub fn stack_pointer(&self) -> Option<u32> {
        self.library.stack_pointer
    }

    /// Freezes this program, preventing further modifications
    pub fn freeze(mut self: Box<Self>) -> Arc<Program> {
        self.library.modules.freeze();
        Arc::from(self)
    }

    /// Get an iterator over the modules in this program
    pub fn modules(&self) -> impl Iterator<Item = &Module> + '_ {
        self.library.modules.iter()
    }

    /// Access the frozen module tree of this program, and panic if not frozen
    pub fn unwrap_frozen_modules(&self) -> &FrozenModuleTree {
        self.library.unwrap_frozen_modules()
    }

    /// Insert a module into this program.
    ///
    /// The insertion order is not preserved - modules are ordered by name.
    ///
    /// NOTE: This function will panic if the program has been frozen
    pub fn insert(&mut self, module: Box<Module>) {
        self.library.insert(module)
    }

    /// Get a reference to a module in this program by name
    pub fn get<Q>(&self, name: &Q) -> Option<&Module>
    where
        Q: ?Sized + Ord,
        Ident: core::borrow::Borrow<Q>,
    {
        self.library.get(name)
    }

    /// Returns true if this program contains a [Module] named `name`
    pub fn contains<N>(&self, name: N) -> bool
    where
        Ident: PartialEq<N>,
    {
        self.library.contains(name)
    }

    /// Write this [Program] to the given output directory.
    pub fn write_to_directory<P: AsRef<Path>>(
        &self,
        path: P,
        session: &Session,
    ) -> std::io::Result<()> {
        let path = path.as_ref();
        assert!(path.is_dir());

        self.library.write_to_directory(path, session)?;

        let main = self.generate_main(self.entrypoint, /* test_harness= */ false);
        main.write_to_directory(path, session)?;

        Ok(())
    }

    // Assemble this program to MAST
    pub fn assemble(&self, session: &Session) -> Result<Arc<miden_core::Program>, Report> {
        use miden_assembly::{Assembler, CompileOptions};

        let debug_mode = session.options.emit_debug_decorators();

        log::debug!(
            "assembling executable with entrypoint '{}' (debug_mode={})",
            self.entrypoint,
            debug_mode
        );
        let mut assembler =
            Assembler::new(session.source_manager.clone()).with_debug_mode(debug_mode);

        // Link extra libraries
        for library in self.library.libraries.iter() {
            if log::log_enabled!(log::Level::Debug) {
                for module in library.module_infos() {
                    log::debug!("registering '{}' with assembler", module.path());
                }
            }
            assembler.add_library(library)?;
        }

        // Assemble library
        for module in self.library.modules.iter() {
            log::debug!("adding '{}' to assembler", module.id.as_str());
            let kind = module.kind;
            let module = module.to_ast(debug_mode).map(Box::new)?;
            assembler.add_module_with_options(
                module,
                CompileOptions {
                    kind,
                    warnings_as_errors: false,
                    path: None,
                },
            )?;
        }

        let emit_test_harness = session.get_flag("test_harness");
        let main = self.generate_main(self.entrypoint, emit_test_harness);
        let main = main.to_ast(debug_mode).map(Box::new)?;
        assembler.assemble_program(main).map(Arc::new)
    }

    pub(crate) fn library(&self) -> &Library {
        &self.library
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.library, f)
    }
}

impl Emit for Program {
    fn name(&self) -> Option<Symbol> {
        None
    }

    fn output_type(&self, _mode: midenc_session::OutputMode) -> midenc_session::OutputType {
        midenc_session::OutputType::Masm
    }

    fn write_to<W: std::io::Write>(
        &self,
        mut writer: W,
        mode: midenc_session::OutputMode,
        _session: &Session,
    ) -> std::io::Result<()> {
        assert_eq!(
            mode,
            midenc_session::OutputMode::Text,
            "binary mode is not supported for masm ir programs"
        );
        writer.write_fmt(format_args!("{}\n", self))
    }
}

/// A [Library] represents a set of modules and its dependencies, which are compiled/assembled
/// together into a single artifact, and then linked into a [Program] for execution at a later
/// time.
///
/// Modules are stored in a [Library] in a B-tree map, keyed by the module name. This is done to
/// make accessing modules by name efficient, and to ensure a stable ordering for compiled programs
/// when emitted as text.
#[derive(Default, Clone)]
pub struct Library {
    /// The set of modules which belong to this program
    modules: Modules,
    /// The set of libraries to link this program against
    libraries: Vec<CompiledLibrary>,
    /// The kernel library to link against
    kernel: Option<KernelLibrary>,
    /// The rodata segments of this program keyed by the offset of the segment
    rodata: Vec<Rodata>,
    /// The address of the `__stack_pointer` global, if such a global has been defined
    stack_pointer: Option<u32>,
}
impl Library {
    /// Create a new, empty [Library]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Create a new [Library] initialized from an [hir::Program].
    ///
    /// The resulting [Library] will have the following:
    ///
    /// * Data segments described by the original [hir::Program]
    ///
    /// None of the HIR modules will have been added yet
    pub fn from_hir(
        program: &hir::Program,
        globals: &GlobalVariableAnalysis<hir::Program>,
    ) -> Self {
        let stack_pointer = program.globals().find("__stack_pointer".parse().unwrap());
        let stack_pointer = if let Some(stack_pointer) = stack_pointer {
            let global_table_offset = globals.layout().global_table_offset();
            Some(global_table_offset + unsafe { program.globals().offset_of(stack_pointer) })
        } else {
            None
        };
        let rodata = compute_rodata(
            globals.layout().global_table_offset(),
            program.globals(),
            program.segments(),
        );
        Self {
            modules: Modules::default(),
            libraries: vec![],
            kernel: None,
            rodata,
            stack_pointer,
        }
    }

    pub fn rodatas(&self) -> &[Rodata] {
        self.rodata.as_slice()
    }

    /// Link this [Library] against the given kernel during assembly
    pub fn link_kernel(&mut self, kernel: KernelLibrary) {
        self.kernel = Some(kernel);
    }

    /// Link this [Library] against the given library during assembly
    pub fn link_library(&mut self, library: CompiledLibrary) {
        self.libraries.push(library);
    }

    /// Get the set of [CompiledLibrary] this library links against
    pub fn link_libraries(&self) -> &[CompiledLibrary] {
        self.libraries.as_slice()
    }

    /// Freezes this library, preventing further modifications
    pub fn freeze(mut self: Box<Self>) -> Arc<Library> {
        self.modules.freeze();
        Arc::from(self)
    }

    /// Get an iterator over the modules in this library
    pub fn modules(&self) -> impl Iterator<Item = &Module> + '_ {
        self.modules.iter()
    }

    /// Access the frozen module tree of this library, and panic if not frozen
    pub fn unwrap_frozen_modules(&self) -> &FrozenModuleTree {
        match self.modules {
            Modules::Frozen(ref modules) => modules,
            Modules::Open(_) => panic!("expected program to be frozen"),
        }
    }

    /// Insert a module into this library.
    ///
    /// The insertion order is not preserved - modules are ordered by name.
    ///
    /// NOTE: This function will panic if the program has been frozen
    pub fn insert(&mut self, module: Box<Module>) {
        self.modules.insert(module);
    }

    /// Get a reference to a module in this library by name
    pub fn get<Q>(&self, name: &Q) -> Option<&Module>
    where
        Q: ?Sized + Ord,
        Ident: core::borrow::Borrow<Q>,
    {
        self.modules.get(name)
    }

    /// Returns true if this library contains a [Module] named `name`
    pub fn contains<N>(&self, name: N) -> bool
    where
        Ident: PartialEq<N>,
    {
        self.modules.iter().any(|m| m.id == name)
    }

    /// Write this [Library] to the given output directory.
    pub fn write_to_directory<P: AsRef<Path>>(
        &self,
        path: P,
        session: &Session,
    ) -> std::io::Result<()> {
        let path = path.as_ref();
        assert!(path.is_dir());

        for module in self.modules.iter() {
            module.write_to_directory(path, session)?;
        }

        Ok(())
    }

    // Assemble this library to MAST
    pub fn assemble(&self, session: &Session) -> Result<Arc<CompiledLibrary>, Report> {
        use miden_assembly::Assembler;

        let debug_mode = session.options.emit_debug_decorators();
        log::debug!(
            "assembling library of {} modules (debug_mode={})",
            self.modules().count(),
            debug_mode
        );

        let mut assembler =
            Assembler::new(session.source_manager.clone()).with_debug_mode(debug_mode);

        // Link extra libraries
        for library in self.libraries.iter() {
            if log::log_enabled!(log::Level::Debug) {
                for module in library.module_infos() {
                    log::debug!("registering '{}' with assembler", module.path());
                }
            }
            assembler.add_library(library)?;
        }

        // Assemble library
        let mut modules = Vec::with_capacity(self.modules.len());
        for module in self.modules.iter() {
            log::debug!("adding '{}' to assembler", module.id.as_str());
            let module = module.to_ast(debug_mode).map(Box::new)?;
            modules.push(module);
        }
        assembler.assemble_library(modules).map(Arc::new)
    }
}

impl fmt::Display for Library {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for module in self.modules.iter() {
            // Don't print intrinsic modules
            if module.id.as_str().starts_with("intrinsics::") {
                continue;
            }
            if ["intrinsics", "std"].contains(&module.name.namespace().as_str()) {
                // Skip printing the standard library modules and intrinsics
                // modules to focus on the user-defined modules and avoid the
                // stack overflow error when printing large programs
                // https://github.com/0xPolygonMiden/miden-formatting/issues/4
                continue;
            } else {
                writeln!(f, "# mod {}\n", &module.name)?;
                writeln!(f, "{}", module)?;
            }
        }
        Ok(())
    }
}

impl Emit for Library {
    fn name(&self) -> Option<Symbol> {
        None
    }

    fn output_type(&self, _mode: midenc_session::OutputMode) -> midenc_session::OutputType {
        midenc_session::OutputType::Masm
    }

    fn write_to<W: std::io::Write>(
        &self,
        mut writer: W,
        mode: midenc_session::OutputMode,
        _session: &Session,
    ) -> std::io::Result<()> {
        assert_eq!(
            mode,
            midenc_session::OutputMode::Text,
            "binary mode is not supported for masm ir libraries"
        );
        writer.write_fmt(format_args!("{}\n", self))
    }
}

/// Compute the metadata for each non-empty rodata segment in the program.
///
/// This consists of the data itself, as well as a content digest, which will be used to place
/// that data in the advice map when the program starts.
fn compute_rodata(
    global_table_offset: u32,
    globals: &GlobalVariableTable,
    segments: &DataSegmentTable,
) -> Vec<Rodata> {
    let mut rodatas = Vec::with_capacity(segments.iter().count() + 1);

    // Convert global variable initializers to a data segment, and place it at the computed
    // global table offset in linear memory.
    let extra = if !globals.is_empty() {
        let size = globals.size_in_bytes();
        let offset = global_table_offset;
        let mut data = vec![0; size];
        for gv in globals.iter() {
            if let Some(init) = gv.initializer() {
                let offset = unsafe { globals.offset_of(gv.id()) } as usize;
                let init = globals.get_constant(init);
                let init_bytes = init.as_slice();
                assert!(offset + init_bytes.len() <= data.len());
                let dst = &mut data[offset..(offset + init_bytes.len())];
                dst.copy_from_slice(init_bytes);
            }
        }
        // Don't bother emitting anything for zeroed segments
        if data.iter().any(|&b| b != 0) {
            Some((size as u32, offset, Arc::new(midenc_hir::ConstantData::from(data))))
        } else {
            None
        }
    } else {
        None
    };

    // Process all segments, ignoring zeroed segments (as Miden's memory is always zeroed)
    for (size, offset, segment_data) in segments
        .iter()
        .filter_map(|segment| {
            if segment.is_zeroed() {
                None
            } else {
                Some((segment.size(), segment.offset(), segment.init()))
            }
        })
        .chain(extra)
    {
        let base = NativePtr::from_ptr(offset);

        // TODO(pauls): Do we ever have a need for data segments which are not aligned
        // to an word boundary? If so, we need to implement that
        // support when emitting the entry for a program
        assert_eq!(
            base.offset,
            0,
            "unsupported data segment alignment {}: must be aligned to a 32 byte boundary",
            base.alignment()
        );
        assert_eq!(
            base.index,
            0,
            "unsupported data segment alignment {}: must be aligned to a 32 byte boundary",
            base.alignment()
        );

        // Compute the commitment for the data
        let num_elements = (size.next_multiple_of(4) / 4) as usize;
        let num_words = num_elements.next_multiple_of(4) / 4;
        let padding = (num_words * 4).abs_diff(num_elements);
        let mut elements = Vec::with_capacity(num_elements + padding);
        // TODO(pauls): If the word containing the first element overlaps with the
        // previous segment, then ensure the overlapping elements
        // are mixed together, so that the data is preserved, and
        // the commitment is correct
        let mut iter = segment_data.as_slice().iter().copied().array_chunks::<4>();
        elements.extend(iter.by_ref().map(|bytes| Felt::new(u32::from_le_bytes(bytes) as u64)));
        if let Some(remainder) = iter.into_remainder() {
            let mut chunk = [0u8; 4];
            for (i, byte) in remainder.into_iter().enumerate() {
                chunk[i] = byte;
            }
            elements.push(Felt::new(u32::from_le_bytes(chunk) as u64));
        }
        elements.resize(num_elements + padding, Felt::ZERO);
        let digest = Rpo256::hash_elements(&elements);

        log::debug!(
            "computed commitment for data segment at offset {offset} ({size} bytes, \
             {num_elements} elements): '{digest}'"
        );

        rodatas.push(Rodata {
            digest,
            start: base,
            data: segment_data,
        });
    }

    rodatas
}
