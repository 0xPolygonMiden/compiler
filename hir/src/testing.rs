use alloc::sync::Arc;
use core::{mem, slice};
use std::path::Path;

use midenc_session::{Options, Session};

use crate::{
    diagnostics::{
        DefaultSourceManager, Emitter, SourceFile, SourceId, SourceManagerExt, SourceSpan,
    },
    *,
};

const PAGE_SIZE: u32 = 64 * 1024;

fn setup_diagnostics() {
    use crate::diagnostics::reporting::{self, ReportHandlerOpts};

    let result = reporting::set_hook(Box::new(|_| Box::new(ReportHandlerOpts::new().build())));
    if result.is_ok() {
        reporting::set_panic_hook();
    }
}

/// The base context used by all IR tests
pub struct TestContext {
    pub session: Session,
}
impl Default for TestContext {
    fn default() -> Self {
        Self::default_with_emitter(None)
    }
}
impl TestContext {
    /// Create a new test context with the given [Session]
    pub fn new(session: Session) -> Self {
        setup_diagnostics();

        Self { session }
    }

    pub fn default_with_emitter(emitter: Option<Arc<dyn Emitter>>) -> Self {
        Self::default_with_opts_and_emitter(Default::default(), emitter)
    }

    pub fn default_with_opts_and_emitter(
        options: Options,
        emitter: Option<Arc<dyn Emitter>>,
    ) -> Self {
        use midenc_session::InputFile;

        setup_diagnostics();

        let source_manager = Arc::new(DefaultSourceManager::default());
        let session = Session::new(
            [InputFile::from_path("test.hir").unwrap()],
            None,
            None,
            std::env::temp_dir(),
            options,
            emitter,
            source_manager,
        );

        Self { session }
    }

    /// Add a source file to this context
    pub fn add<P: AsRef<Path>>(&self, path: P) -> Arc<SourceFile> {
        self.session
            .source_manager
            .load_file(path.as_ref())
            .expect("invalid source file")
    }

    /// Get a [SourceSpan] corresponding to the callsite of this function
    #[track_caller]
    #[inline(never)]
    pub fn current_span(&self) -> SourceSpan {
        let caller = core::panic::Location::caller();
        let caller_file =
            Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join(caller.file());
        let source_file = self.add(caller_file);
        source_file
            .line_column_to_span(caller.line(), caller.column())
            .expect("could not resolve source location")
    }

    /// Get a [SourceSpan] representing the location in the given source file (by id), line and
    /// column
    ///
    /// It is expected that line and column are 1-indexed, so they will be shifted to be 0-indexed,
    /// make sure to add 1 if you already have a 0-indexed line/column on hand
    pub fn span(&self, source_id: SourceId, line: u32, column: u32) -> SourceSpan {
        self.session
            .source_manager
            .get(source_id)
            .ok()
            .and_then(|file| file.line_column_to_span(line - 1, column - 1))
            .unwrap_or_default()
    }
}

#[macro_export]
macro_rules! current_file {
    () => {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join(file!())
    };
}

#[macro_export]
macro_rules! span {
    ($source_manager:ident, $src:ident) => {
        $source_manager
            .get($src)
            .ok()
            .and_then(|file| file.line_column_to_span(line!() - 1, column!() - 1))
            .unwrap()
    };
}

/// pub fn issue_56(i32, i32) -> i32 {
/// block0(v0: i32, v1: i32):
///     v3 = lt v0, v1 : i1;
///     v4 = cast v3 : i32;
///     v5 = neq 0, v4 : i1;
///     v6 = select v5, v0, v1 : i32;
///     v7 = const.i32 0
///     cond_br v7, block1(v6), block2(v6)
///
/// block1(v8: i32):
///     v9 = add.wrapping v7, v8
///     ret v9;
///
/// block2(v10: i32):
///     v11 = add.wrapping v7, v10
///     ret v11
///}
pub fn issue56(builder: &mut ModuleBuilder, context: &TestContext) -> FunctionIdent {
    // Declare the `fib` function, with the appropriate type signature
    let sig = Signature {
        params: vec![AbiParam::new(Type::I32), AbiParam::new(Type::I32)],
        results: vec![AbiParam::new(Type::I32)],
        cc: CallConv::SystemV,
        linkage: Linkage::External,
    };
    let mut fb = builder.function("entrypoint", sig).expect("unexpected symbol conflict");

    let entry = fb.current_block();
    // Get the value for `v0` and `v1`
    let (v0, v1) = {
        let args = fb.block_params(entry);
        (args[0], args[1])
    };

    let v3 = fb.ins().lt(v0, v1, context.current_span());
    let v4 = fb.ins().cast(v3, Type::I32, context.current_span());
    let v5 = fb.ins().neq_imm(v4, Immediate::I32(0), context.current_span());
    let v6 = fb.ins().select(v5, v0, v1, context.current_span());
    let v7 = fb.ins().i32(0, context.current_span());
    let cond = fb.ins().i1(true, context.current_span());

    let block1 = fb.create_block();
    let block2 = fb.create_block();
    let v8 = fb.append_block_param(block1, Type::I32, context.current_span());
    let v10 = fb.append_block_param(block2, Type::I32, context.current_span());

    fb.ins().cond_br(cond, block1, &[v6], block2, &[v6], context.current_span());

    fb.switch_to_block(block1);
    let v9 = fb.ins().add_wrapping(v7, v8, context.current_span());
    fb.ins().ret(Some(v9), context.current_span());

    fb.switch_to_block(block2);
    let v11 = fb.ins().add_wrapping(v7, v10, context.current_span());
    fb.ins().ret(Some(v11), context.current_span());

    // We're done
    fb.build(&context.session.diagnostics)
        .expect("unexpected validation error, see diagnostics output")
}

/// Construct an implementation of a function which computes the sum
/// of a Fibonnaci sequence of length `n`, using the provided builder.
///
/// This function is very simple, does not contain any memory operations,
/// any local variables, or function calls. It makes for a good sanity
/// check.
///
/// In simple pseudocode, this is the function we're building:
///
/// ```text,ignore
/// fn fib(n: u32) -> u32 {
///     let mut a = 0;
///     let mut b = 1;
///     for _ in 0..=n {
///         let c = a + b;
///         a = b;
///         b = c;
///     }
///     a
/// }
/// ```
///
/// Expressed as IR, we're looking for:
///
/// ```text,ignore
/// module test
///
/// pub fn fib(u32) -> u32 {
/// entry(n: u32):
///   a0 = const.u32 0 : u32;
///   b0 = const.u32 1 : u32;
///   n0 = const.u32 0 : u32;
///   br blk0(a0, b0, n0);
///
/// blk0(a1: u32, b1: u32, n1: u32):
///   continue = lt n1, n : i1;
///   cond_br continue, blk1, blk2(a1);
///
/// blk1:
///   b2 = add.checked a1, b1 : u32;
///   n2 = incr.wrapping n1 : u32;
///   br blk0(b1, b2, n2);
///
/// blk2(result: u32):
///   ret result;
/// }
/// ```
pub fn fib1(builder: &mut ModuleBuilder, context: &TestContext) -> FunctionIdent {
    // Declare the `fib` function, with the appropriate type signature
    let sig = Signature {
        params: vec![AbiParam::new(Type::U32)],
        results: vec![AbiParam::new(Type::U32)],
        cc: CallConv::SystemV,
        linkage: Linkage::External,
    };
    let mut fb = builder.function("fib", sig).expect("unexpected symbol conflict");

    let entry = fb.current_block();
    // Get the value for `n`
    let n = {
        let args = fb.block_params(entry);
        args[0]
    };

    // This block corresponds to `blk0` in the example
    let loop_header = fb.create_block();
    let a1 = fb.append_block_param(loop_header, Type::U32, context.current_span());
    let b1 = fb.append_block_param(loop_header, Type::U32, context.current_span());
    let n1 = fb.append_block_param(loop_header, Type::U32, context.current_span());

    // This block corresponds to `blk1` in the example
    let loop_body = fb.create_block();

    // This block corresponds to `blk2` in the example
    let loop_exit = fb.create_block();
    let result = fb.append_block_param(loop_exit, Type::U32, context.current_span());

    // Now, starting from the entry block, we build out the rest of the function in control flow
    // order
    fb.switch_to_block(entry);
    let a0 = fb.ins().u32(0, context.current_span());
    let b0 = fb.ins().u32(1, context.current_span());
    let i0 = fb.ins().u32(0, context.current_span());
    fb.ins().br(loop_header, &[a0, b0, i0], context.current_span());

    fb.switch_to_block(loop_header);
    let continue_flag = fb.ins().lt(n1, n, context.current_span());
    fb.ins()
        .cond_br(continue_flag, loop_body, &[], loop_exit, &[a1], context.current_span());

    fb.switch_to_block(loop_body);
    let b2 = fb.ins().add_checked(a1, b1, context.current_span());
    let n2 = fb.ins().incr_wrapping(n1, context.current_span());
    fb.ins().br(loop_header, &[b1, b2, n2], context.current_span());

    fb.switch_to_block(loop_exit);
    fb.ins().ret(Some(result), context.current_span());

    // We're done
    fb.build(&context.session.diagnostics)
        .expect("unexpected validation error, see diagnostics output")
}

/// Construct an implementation of a function which computes the sum
/// of a matrix of u32 values, with dimensions `rows` by `cols`.
///
/// This helper does not take a [ModuleBuilder] because it is used in some
/// simpler tests where we just want the function, you can simply add the
/// function to the [ModuleBuilder] directly.
///
/// This function is very simple, does not contain any memory operations,
/// any local variables, or function calls. It makes for a good sanity
/// check.
///
/// In simple pseudocode, this is the function we're building:
///
/// ```text,ignore
/// pub fn sum_matrix(ptr: *mut u32, rows: u32, cols: u32) -> u32 {
///     let mut sum = 0;
///     if ptr.is_null() { return sum; }
///     for r in 0..rows {
///        for col in 0..cols {
///            let value = *ptr[row][col];
///            sum += value;
///        }
///     }
///     sum
/// }
/// ```
///
/// This corresponds to the following IR:
///
/// ```text,ignore
/// pub fn test(*mut u32, u32, u32) -> *mut u32 {
/// entry(ptr0: *mut u32, rows: u32, cols: u32):
///     sum0 = const.u32 0  : u32
///     ptr1 = ptrtoint ptr0  : u32
///     not_null = neq ptr1, 0  : i1
///     condbr not_null, blk1, blk0(sum0)
///
/// blk0(result0: u32):
///     ret result0
///
/// blk1:
///     rows0 = const.u32 0  : u32
///     cols0 = const.u32 0  : u32
///     row_size = mul.checked cols, 4
///     br blk2(sum0, rows0, cols0)
///
/// blk2(sum1: u32, rows1: u32, cols1: u32):
///     has_more_rows = lt rows1, rows
///     row_skip = mul.checked rows1, row_size
///     condbr has_more_rows, blk3(sum1, rows1, cols1), blk0(sum1)
///
/// blk3(sum3: u32, rows3: u32, cols3: u32):
///     has_more_cols = lt cols3, cols
///     condbr has_more_cols, blk4, blk5
///
/// blk4:
///     col_skip = mul.checked cols3, 4
///     skip = add.checked row_skip, col_skip
///     ptr4i = add.checked ptr1, skip
///     ptr4 = inttoptr ptr4i : *mut u32
///     value = load ptr4 : u32
///     sum4 = add.checked sum3, value
///     cols4 = incr.wrapping cols3
///     br blk3(sum4, rows3, cols4)
///
/// blk5:
///     rows5 = incr.wrapping rows3
///     br blk2(sum3, rows5, cols3)
/// }
/// ```
pub fn sum_matrix(builder: &mut ModuleBuilder, context: &TestContext) -> FunctionIdent {
    let sig = Signature::new(
        [
            AbiParam::new(Type::Ptr(Box::new(Type::U32))),
            AbiParam::new(Type::U32),
            AbiParam::new(Type::U32),
        ],
        [AbiParam::new(Type::U32)],
    );
    let id = Ident::new(Symbol::intern("sum_matrix"), context.current_span());
    let mut fb = builder.function(id, sig).expect("unexpected symbol conflict");

    let entry = fb.current_block();

    let a = fb.create_block(); // blk0(result0: u32)
    let result0 = fb.append_block_param(a, Type::U32, context.current_span());
    let b = fb.create_block(); // blk1
    let c = fb.create_block(); // blk2(sum1: u32, rows1: u32, cols1: u32)
    let sum1 = fb.append_block_param(c, Type::U32, context.current_span());
    let rows1 = fb.append_block_param(c, Type::U32, context.current_span());
    let cols1 = fb.append_block_param(c, Type::U32, context.current_span());
    let d = fb.create_block(); // blk3(sum3: u32, rows3: u32, cols3: u32)
    let sum3 = fb.append_block_param(d, Type::U32, context.current_span());
    let rows3 = fb.append_block_param(d, Type::U32, context.current_span());
    let cols3 = fb.append_block_param(d, Type::U32, context.current_span());
    let e = fb.create_block(); // blk4
    let f = fb.create_block(); // blk5

    let (ptr0, rows, cols) = {
        let args = fb.block_params(entry);
        (args[0], args[1], args[2])
    };
    // entry
    let sum0 = fb.ins().u32(0, context.current_span());
    let ptr1 = fb.ins().ptrtoint(ptr0, Type::U32, context.current_span());
    let not_null = fb.ins().neq_imm(ptr1, Immediate::U32(0), context.current_span());
    fb.ins().cond_br(not_null, b, &[], a, &[sum0], context.current_span());

    // blk0
    fb.switch_to_block(a);
    fb.ins().ret(Some(result0), context.current_span());

    // blk1
    fb.switch_to_block(b);
    let rows0 = fb.ins().u32(0, context.current_span());
    let cols0 = fb.ins().u32(0, context.current_span());
    let row_size = fb.ins().mul_imm_checked(cols, Immediate::U32(4), context.current_span());
    fb.ins().br(c, &[sum0, rows0, cols0], context.current_span());

    // blk2(sum1, rows1, cols1)
    fb.switch_to_block(c);
    let has_more_rows = fb.ins().lt(rows1, rows, context.current_span());
    let row_skip = fb.ins().mul_checked(rows1, row_size, context.current_span());
    fb.ins()
        .cond_br(has_more_rows, d, &[sum1, rows1, cols1], a, &[sum1], context.current_span());

    // blk3(sum3, rows3, cols3)
    fb.switch_to_block(d);
    let has_more_cols = fb.ins().lt(cols3, cols, context.current_span());
    fb.ins().cond_br(has_more_cols, e, &[], f, &[], context.current_span());

    // blk4
    fb.switch_to_block(e);
    let col_skip = fb.ins().mul_imm_checked(cols3, Immediate::U32(4), context.current_span());
    let skip = fb.ins().add_checked(row_skip, col_skip, context.current_span());
    let ptr4i = fb.ins().add_checked(ptr1, skip, context.current_span());
    let ptr4 = fb.ins().inttoptr(ptr4i, Type::Ptr(Box::new(Type::U32)), context.current_span());
    let value = fb.ins().load(ptr4, context.current_span());
    let sum4 = fb.ins().add_checked(sum3, value, context.current_span());
    let cols4 = fb.ins().incr_wrapping(cols3, context.current_span());
    fb.ins().br(d, &[sum4, rows3, cols4], context.current_span());

    // blk5
    fb.switch_to_block(f);
    let rows5 = fb.ins().incr_wrapping(rows3, context.current_span());
    let cols5 = fb.ins().u32(0, context.current_span());
    fb.ins().br(c, &[sum3, rows5, cols5], context.current_span());

    // We're done
    fb.build(&context.session.diagnostics)
        .expect("unexpected validation error, see diagnostics output")
}

/// Add a predefined set of intrinsics to a given [ProgramBuilder], making
/// them available for use by other modules in that program.
///
/// This defines the following modules and functions:
///
/// * The `mem` module, containing memory-management intrinsics, see [mem_intrinsics] for details.
/// * The `str` module, containing string-related intrinsics, see [str_intrinsics] for details
pub fn intrinsics(builder: &mut ProgramBuilder, context: &TestContext) -> anyhow::Result<()> {
    mem_intrinsics(builder, context)?;
    str_intrinsics(builder, context)
}

/// Adds a `mem` module to the given [ProgramBuilder], containing a handful of
/// useful memory-management intrinsics:
///
/// * `malloc(u32) -> *mut u8`, allocates from the usable heap
/// * `memory_size() -> u32`, returns the amount of allocated memory, in pages
/// * `memory_grow(u32) -> u32`, grows memory by a given number of pages, returning the previous
///   memory size
///
/// Expressed as pseudocode, the `mem` module is as follows:
///
/// ```text,ignore
/// module mem
///
/// /// These three globals are provided by the linker, based on where the unreserved
/// /// heap memory segment begins and ends.
/// extern {
///   #[no_mangle]
///   static mut HEAP_BASE: *mut u8;
///   #[no_mangle]
///   static mut HEAP_END: *mut u8;
///   #[no_mangle]
///   static mut HEAP_TOP: *mut u8;
/// }
///
/// pub const PAGE_SIZE: usize = 64 * 1024;
///
/// /// Allocate `size` bytes from the memory reserved for heap allocations
/// pub fn malloc(size: usize) -> *mut u8 {
///   let top = HEAP_TOP as usize;
///   let available = (HEAP_END as usize - top);
///   if size > available {
///       let needed = size - available;
///       let pages = (needed / PAGE_SIZE) + ((needed % PAGE_SIZE) > 0) as usize;
///       assert_ne!(memory_grow(pages), usize::MAX);
///   }
///   let addr = top + size;
///   let mut ptr: *mut u8 = addr as *mut u8;
///   // Require a min alignment of 8 bytes
///   let align_offset = addr % 8;
///   if align_offset != 0 {
///       ptr = (addr + (8 - align_offset)) as *mut u8;
///   }
///   HEAP_TOP = ptr;
///   ptr
/// }
///
/// /// Get the size, in pages, of the current heap
/// pub fn memory_size() -> usize {
///     (HEAP_END as usize - HEAP_BASE as usize) / PAGE_SIZE
/// }
///
/// /// Grow the number of pages reserved for the heap by `num_pages`, returning the previous page count
/// ///
/// /// Returns `usize::MAX` if unable to allocate the requested number of pages
/// pub fn memory_grow(num_pages: usize) -> usize {
///     const HEAP_MAX: usize = 2u32.pow(30);
///     let end  = HEAP_END as usize;
///     let remaining = (HEAP_MAX - end) / PAGE_SIZE;
///     if num_pages > remaining {
///         usize::MAX
///     } else {
///         let prev = (end - HEAP_BASE as usize) / PAGE_SIZE;
///         HEAP_END = (end + (num_pages * PAGE_SIZE)) as *mut u8;
///         prev
///     }
/// }
/// ```
pub fn mem_intrinsics(builder: &mut ProgramBuilder, _context: &TestContext) -> anyhow::Result<()> {
    // Next up, the `mem` module
    let mut mb = builder.module("mem");

    // This module knows about the stack segment, but no others
    mb.declare_data_segment(0, PAGE_SIZE, vec![], false)
        .expect("unexpected data segment error");

    // pub const PAGE_SIZE: usize = 64 * 1024;
    mb.declare_global_variable(
        "PAGE_SIZE",
        Type::U32,
        Linkage::External,
        Some(PAGE_SIZE.to_le_bytes().into()),
        SourceSpan::UNKNOWN,
    )
    .expect("unexpected global variable error");

    mb.declare_global_variable(
        "HEAP_BASE",
        Type::Ptr(Box::new(Type::U8)),
        Linkage::External,
        Some((2 * 65536u32).to_le_bytes().into()),
        SourceSpan::UNKNOWN,
    )
    .expect("unexpected global variable error");

    mb.declare_global_variable(
        "HEAP_TOP",
        Type::Ptr(Box::new(Type::U8)),
        Linkage::External,
        Some((2 * 65536u32).to_le_bytes().into()),
        SourceSpan::UNKNOWN,
    )
    .expect("unexpected global variable error");

    mb.declare_global_variable(
        "HEAP_END",
        Type::Ptr(Box::new(Type::U8)),
        Linkage::External,
        Some((4096 * 65536u32).to_le_bytes().into()),
        SourceSpan::UNKNOWN,
    )
    .expect("unexpected global variable error");

    // Define the alloc function
    let mut fb = mb.function("alloc", malloc_signature()).expect("unexpected symbol conflict");

    let memory_grow_sig = Signature::new([AbiParam::new(Type::U32)], [AbiParam::new(Type::U32)]);
    let memory_grow = fb.import_function("mem", "memory_grow", memory_grow_sig.clone()).unwrap();

    // pub fn alloc(size: usize) -> *mut u8 {
    //   let top = HEAP_TOP as usize;
    //   let available = (HEAP_END as usize - top);
    //   if size > available {
    //       let needed = size - available;
    //       let pages = (needed / PAGE_SIZE) + ((needed % PAGE_SIZE) > 0) as usize;
    //       assert_ne!(memory_grow(pages), usize::MAX);
    //   }
    //   let addr = top + size;
    //   let mut ptr: *mut u8 = addr as *mut u8;
    //   // Require a min alignment of 8 bytes
    //   let align_offset = addr % 8;
    //   if align_offset != 0 {
    //       ptr = (addr + (8 - align_offset)) as *mut u8;
    //   }
    //   HEAP_TOP = ptr;
    //   ptr
    // }
    let raw_ptr_ty = Type::Ptr(Box::new(Type::U8));
    let size = {
        let args = fb.block_params(fb.current_block());
        args[0]
    };
    let heap_top = fb.ins().load_symbol("HEAP_TOP", Type::U32, SourceSpan::UNKNOWN);
    let heap_end = fb.ins().load_symbol("HEAP_END", Type::U32, SourceSpan::UNKNOWN);
    let available = fb.ins().sub_checked(heap_end, heap_top, SourceSpan::UNKNOWN);
    let requires_growth = fb.ins().gt(size, available, SourceSpan::UNKNOWN);
    let grow_mem_block = fb.create_block();
    let alloc_block = fb.create_block();
    fb.ins()
        .cond_br(requires_growth, grow_mem_block, &[], alloc_block, &[], SourceSpan::UNKNOWN);

    fb.switch_to_block(grow_mem_block);
    let needed = fb.ins().sub_checked(size, available, SourceSpan::UNKNOWN);
    let need_pages =
        fb.ins().div_imm_checked(needed, Immediate::U32(PAGE_SIZE), SourceSpan::UNKNOWN);
    let need_extra =
        fb.ins().mod_imm_checked(needed, Immediate::U32(PAGE_SIZE), SourceSpan::UNKNOWN);
    let extra_page = fb.ins().gt_imm(need_extra, Immediate::U32(0), SourceSpan::UNKNOWN);
    let extra_count = fb.ins().zext(extra_page, Type::U32, SourceSpan::UNKNOWN);
    let num_pages = fb.ins().add_checked(need_pages, extra_count, SourceSpan::UNKNOWN);
    let prev_pages = {
        let call = fb.ins().call(memory_grow, &[num_pages], SourceSpan::UNKNOWN);
        fb.first_result(call)
    };
    let usize_max = fb.ins().u32(u32::MAX, SourceSpan::UNKNOWN);
    fb.ins().assert_eq(prev_pages, usize_max, SourceSpan::UNKNOWN);
    fb.ins().br(alloc_block, &[], SourceSpan::UNKNOWN);

    fb.switch_to_block(alloc_block);
    let addr = fb.ins().add_checked(heap_top, size, SourceSpan::UNKNOWN);
    let align_offset = fb.ins().mod_imm_checked(addr, Immediate::U32(8), SourceSpan::UNKNOWN);
    let is_aligned = fb.ins().eq_imm(align_offset, Immediate::U32(0), SourceSpan::UNKNOWN);
    let align_block = fb.create_block();
    let aligned_block = fb.create_block();
    let new_heap_top_ptr =
        fb.append_block_param(aligned_block, raw_ptr_ty.clone(), SourceSpan::UNKNOWN);

    let ptr = fb.ins().inttoptr(addr, raw_ptr_ty.clone(), SourceSpan::UNKNOWN);
    fb.ins()
        .cond_br(is_aligned, aligned_block, &[ptr], align_block, &[], SourceSpan::UNKNOWN);

    fb.switch_to_block(align_block);
    let aligned_addr = fb.ins().add_imm_checked(addr, Immediate::U32(8), SourceSpan::UNKNOWN);
    let aligned_addr = fb.ins().sub_checked(aligned_addr, align_offset, SourceSpan::UNKNOWN);
    let aligned_ptr = fb.ins().inttoptr(aligned_addr, raw_ptr_ty.clone(), SourceSpan::UNKNOWN);
    fb.ins().br(aligned_block, &[aligned_ptr], SourceSpan::UNKNOWN);

    fb.switch_to_block(aligned_block);
    let heap_top_addr = fb.ins().symbol_addr(
        "HEAP_TOP",
        Type::Ptr(Box::new(raw_ptr_ty.clone())),
        SourceSpan::UNKNOWN,
    );
    fb.ins().store(heap_top_addr, new_heap_top_ptr, SourceSpan::UNKNOWN);
    fb.ins().ret(Some(new_heap_top_ptr), SourceSpan::UNKNOWN);

    let _alloc = fb.build().expect("unexpected validation error, see diagnostics output");

    // Define the memory_size function
    let memory_size_sig = Signature::new([], [AbiParam::new(Type::U32)]);
    let mut fb = mb.function("memory_size", memory_size_sig).expect("unexpected symbol conflict");

    // pub fn memory_size() -> usize {
    //     (HEAP_END as usize - HEAP_BASE as usize) / PAGE_SIZE
    // }
    let heap_base_addr = fb.ins().load_symbol("HEAP_BASE", Type::U32, SourceSpan::UNKNOWN);
    let heap_end_addr = fb.ins().load_symbol("HEAP_END", Type::U32, SourceSpan::UNKNOWN);
    let used = fb.ins().sub_checked(heap_end_addr, heap_base_addr, SourceSpan::UNKNOWN);
    let used_pages = fb.ins().div_imm_checked(used, Immediate::U32(PAGE_SIZE), SourceSpan::UNKNOWN);
    fb.ins().ret(Some(used_pages), SourceSpan::UNKNOWN);

    let _memory_size = fb.build().expect("unexpected validation error, see diagnostics output");

    // Define the memory_grow function
    let mut fb = mb.function("memory_grow", memory_grow_sig).expect("unexpected symbol conflict");

    // pub fn memory_grow(num_pages: usize) -> usize {
    //     const HEAP_MAX: usize = 2u32.pow(30);
    //     let end  = HEAP_END as usize;
    //     let remaining = (HEAP_MAX - end) / PAGE_SIZE;
    //     if num_pages > remaining {
    //         usize::MAX
    //     } else {
    //         let prev = (end - HEAP_BASE as usize) / PAGE_SIZE;
    //         HEAP_END = (end + (num_pages * PAGE_SIZE)) as *mut u8;
    //         prev
    //     }
    // }
    let num_pages = {
        let args = fb.block_params(fb.current_block());
        args[0]
    };
    let heap_end = fb.ins().load_symbol("HEAP_END", Type::U32, SourceSpan::UNKNOWN);
    let heap_max = fb.ins().u32(u32::MAX, SourceSpan::UNKNOWN);
    let remaining_bytes = fb.ins().sub_checked(heap_max, heap_end, SourceSpan::UNKNOWN);
    let remaining_pages =
        fb.ins()
            .div_imm_checked(remaining_bytes, Immediate::U32(PAGE_SIZE), SourceSpan::UNKNOWN);
    let out_of_memory = fb.ins().gt(num_pages, remaining_pages, SourceSpan::UNKNOWN);
    let out_of_memory_block = fb.create_block();
    let grow_memory_block = fb.create_block();
    fb.ins().cond_br(
        out_of_memory,
        out_of_memory_block,
        &[],
        grow_memory_block,
        &[],
        SourceSpan::UNKNOWN,
    );

    fb.switch_to_block(out_of_memory_block);
    fb.ins().ret_imm(Immediate::U32(u32::MAX), SourceSpan::UNKNOWN);

    fb.switch_to_block(grow_memory_block);
    let heap_base = fb.ins().load_symbol("HEAP_BASE", Type::U32, SourceSpan::UNKNOWN);
    let prev_bytes = fb.ins().sub_checked(heap_end, heap_base, SourceSpan::UNKNOWN);
    let prev_pages =
        fb.ins()
            .div_imm_checked(prev_bytes, Immediate::U32(PAGE_SIZE), SourceSpan::UNKNOWN);
    let num_bytes =
        fb.ins()
            .mul_imm_checked(num_pages, Immediate::U32(PAGE_SIZE), SourceSpan::UNKNOWN);
    let new_heap_end = fb.ins().add_checked(heap_end, num_bytes, SourceSpan::UNKNOWN);
    let heap_end_addr =
        fb.ins()
            .symbol_addr("HEAP_END", Type::Ptr(Box::new(Type::U32)), SourceSpan::UNKNOWN);
    fb.ins().store(heap_end_addr, new_heap_end, SourceSpan::UNKNOWN);
    fb.ins().ret(Some(prev_pages), SourceSpan::UNKNOWN);

    let _memory_grow = fb.build().expect("unexpected validation error, see diagnostics output");

    mb.build()?;

    Ok(())
}

/// Adds a `str` module to the given [ProgramBuilder], containing a handful of
/// useful string-related intrinsics:
///
/// * `from_raw_parts(*mut u8, u32)`, gets a &str from a pointer + length
/// * `compare(&str, &str) -> i8`, compares two strings, returning -1, 0, or 1
///
/// Expressed as pseudocode, the `str` module is as follows:
///
/// ```text,ignore
/// module str
///
/// // This is to illustrate the type layout of a string reference
/// type &str = { *mut u8, usize };
///
/// /// Convert a raw pointer and length to a `&str`, assert the pointer is non-null
/// pub fn from_raw_parts(ptr: *mut u8, len: usize) -> &str {
///     assert!(ptr as usize);
///     // This intrinsic represents construction of the fat pointer for the &str reference
///     let ptr = intrinsics::ptr::from_raw_parts(ptr as *mut (), len);
///     &*ptr
/// }
///
/// /// Compare two `&str`, returning one of the following:
/// ///
/// /// * 1 if `a` is greater than `b`
/// /// * 0 if `a` is equal to `b`
/// /// * -1 if `a` is less than `b`
/// pub fn compare(a: &str, b: &str) -> i8 {
///     let len = max(a.len, b.len);
///     let mut i = 0;
///     while i < len {
///          let a_char = a.as_bytes()[i];
///          let b_char = b.as_bytes()[i];
///          if a_char > b_char {
///              return 1;
///          } else if a_char < b_char {
///              return -1;
///          } else {
///              i += 1;
///          }
///     }
///
///     if a.len > b.len {
///         1
///     } else if a.len < b.len {
///         -1
///     } else {
///         0
///     }
/// }
/// ```
pub fn str_intrinsics(builder: &mut ProgramBuilder, _context: &TestContext) -> anyhow::Result<()> {
    // Next up, the `str` module
    let mut mb = builder.module("str");

    // Define the from_raw_parts function
    let mut fb = mb
        .function("from_raw_parts", str_from_raw_parts_signature())
        .expect("unexpected symbol conflict");

    // Unlike the high-level pseudocode, the actual implementation uses an sret parameter, i.e.:
    //
    // pub fn from_raw_parts(sret result: *mut str, ptr: *mut u8, len: usize) {
    //     assert!(ptr as usize);
    //     *result = { ptr, len };
    // }
    let (result, ptr, len) = {
        let args = fb.block_params(fb.current_block());
        (args[0], args[1], args[2])
    };
    let addr = fb.ins().ptrtoint(ptr, Type::U32, SourceSpan::UNKNOWN);
    let is_nonnull_addr = fb.ins().gt_imm(addr, Immediate::U32(0), SourceSpan::UNKNOWN);
    fb.ins().assert(is_nonnull_addr, SourceSpan::UNKNOWN);
    let ptr_ptr = fb.ins().getelementptr(result, &[0], SourceSpan::UNKNOWN);
    fb.ins().store(ptr_ptr, ptr, SourceSpan::UNKNOWN);
    let len_ptr = fb.ins().getelementptr(result, &[1], SourceSpan::UNKNOWN);
    fb.ins().store(len_ptr, len, SourceSpan::UNKNOWN);
    fb.ins().ret(None, SourceSpan::UNKNOWN);

    let _from_raw_parts = fb.build().expect("unexpected validation error, see diagnostics output");

    // Define the compare function
    let mut fb = mb
        .function("compare", str_compare_signature())
        .expect("unexpected symbol conflict");

    // pub fn compare(a: &str, b: &str) -> i8 {
    //     let len = max(a.len, b.len);
    //     let mut i = 0;
    //     while i < len {
    //          let a_char = a.as_bytes()[i];
    //          let b_char = b.as_bytes()[i];
    //          if a_char > b_char {
    //              return 1;
    //          } else if a_char < b_char {
    //              return -1;
    //          } else {
    //              i += 1;
    //          }
    //     }
    //
    //     if a.len > b.len {
    //         1
    //     } else if a.len < b.len {
    //         -1
    //     } else {
    //         0
    //     }
    // }
    let (a, b) = {
        let args = fb.block_params(fb.current_block());
        (args[0], args[1])
    };
    let a_ptr_ptr = fb.ins().getelementptr(a, &[0], SourceSpan::UNKNOWN);
    let a_ptr = fb.ins().load(a_ptr_ptr, SourceSpan::UNKNOWN);
    let a_addr = fb.ins().ptrtoint(a_ptr, Type::U32, SourceSpan::UNKNOWN);
    let b_ptr_ptr = fb.ins().getelementptr(b, &[0], SourceSpan::UNKNOWN);
    let b_ptr = fb.ins().load(b_ptr_ptr, SourceSpan::UNKNOWN);
    let b_addr = fb.ins().ptrtoint(b_ptr, Type::U32, SourceSpan::UNKNOWN);

    let a_len_ptr = fb.ins().getelementptr(a, &[1], SourceSpan::UNKNOWN);
    let a_len = fb.ins().load(a_len_ptr, SourceSpan::UNKNOWN);
    let b_len_ptr = fb.ins().getelementptr(b, &[1], SourceSpan::UNKNOWN);
    let b_len = fb.ins().load(b_len_ptr, SourceSpan::UNKNOWN);
    let len = fb.ins().max(a_len, b_len, SourceSpan::UNKNOWN);

    let loop_header = fb.create_block();
    let i = fb.append_block_param(loop_header, Type::U32, SourceSpan::UNKNOWN);
    let loop_body = fb.create_block();
    let loop_exit = fb.create_block();
    let exit_block = fb.create_block();
    let result = fb.append_block_param(exit_block, Type::I8, SourceSpan::UNKNOWN);
    let zero = fb.ins().u32(0, SourceSpan::UNKNOWN);
    fb.ins().br(loop_header, &[zero], SourceSpan::UNKNOWN);

    fb.switch_to_block(loop_header);
    let done = fb.ins().lt(i, len, SourceSpan::UNKNOWN);
    fb.ins().cond_br(done, loop_exit, &[], loop_body, &[], SourceSpan::UNKNOWN);

    fb.switch_to_block(loop_body);
    let a_char_addr = fb.ins().incr_wrapping(a_addr, SourceSpan::UNKNOWN);
    let a_char_ptr =
        fb.ins()
            .inttoptr(a_char_addr, Type::Ptr(Box::new(Type::U8)), SourceSpan::UNKNOWN);
    let a_char = fb.ins().load(a_char_ptr, SourceSpan::UNKNOWN);
    let b_char_addr = fb.ins().incr_wrapping(b_addr, SourceSpan::UNKNOWN);
    let b_char_ptr =
        fb.ins()
            .inttoptr(b_char_addr, Type::Ptr(Box::new(Type::U8)), SourceSpan::UNKNOWN);
    let b_char = fb.ins().load(b_char_ptr, SourceSpan::UNKNOWN);
    let is_eq = fb.ins().eq(a_char, b_char, SourceSpan::UNKNOWN);
    let is_gt = fb.ins().gt(a_char, b_char, SourceSpan::UNKNOWN);
    let zero = fb.ins().i8(0, SourceSpan::UNKNOWN);
    let one = fb.ins().i8(1, SourceSpan::UNKNOWN);
    let neg_one = fb.ins().i8(-1, SourceSpan::UNKNOWN);
    let is_ne_result = fb.ins().select(is_gt, one, neg_one, SourceSpan::UNKNOWN);
    let i_incr = fb.ins().incr_wrapping(i, SourceSpan::UNKNOWN);
    fb.ins().cond_br(
        is_eq,
        loop_header,
        &[i_incr],
        exit_block,
        &[is_ne_result],
        SourceSpan::UNKNOWN,
    );

    fb.switch_to_block(loop_exit);
    let is_len_eq = fb.ins().eq(a_len, b_len, SourceSpan::UNKNOWN);
    let is_len_gt = fb.ins().gt(a_len, b_len, SourceSpan::UNKNOWN);
    let len_gt_result = fb.ins().select(is_len_gt, one, neg_one, SourceSpan::UNKNOWN);
    let len_eq_result = fb.ins().select(is_len_eq, zero, len_gt_result, SourceSpan::UNKNOWN);
    fb.ins().br(exit_block, &[len_eq_result], SourceSpan::UNKNOWN);

    fb.switch_to_block(exit_block);
    fb.ins().ret(Some(result), SourceSpan::UNKNOWN);

    let _compare = fb.build().expect("unexpected validation error, see diagnostics output");

    mb.build()?;

    Ok(())
}

/// This function uses the provided [ProgramBuilder] to define a module which exercises
/// a variety of fundamental functionality in the IR, and in Miden Assembly:
///
/// * Memory access, management
/// * Global variables, data segments
/// * Function calls
/// * Assertions
///
/// NOTE: This module builds on the [intrinsics] helper.
///
/// The following is pseudocode representing the module we define and the program entrypoint.
///
/// ```text,ignore
/// module test
///
/// use mem;
/// use str;
///
/// /// This is here solely to ensure that globals are linked correctly
/// extern {
///     const PAGE_SIZE: usize;
/// }
///
/// pub fn main() -> i32 {
///   const HELLO: &str = "hello";
///
///   let len = HELLO.as_bytes().len();
///   let ptr: *mut u8 = mem::alloc(len);
///   memcpy(HELLO.as_bytes().as_ptr(), ptr, len);
///   let greeting = str::from_raw_parts(ptr, len);
///
///   assert_eq!(PAGE_SIZE, 64 * 1024);
///   assertz!(str::compare(HELLO, greeting));
///
///   0
/// }
/// ```
pub fn hello_world(builder: &mut ProgramBuilder, context: &TestContext) -> anyhow::Result<()> {
    let mut mb = builder.module("test");

    // Every module is going to have the same data segment for the shadow stack,
    // and this module will additionally have a data segment for read-only data,
    // i.e. constants
    mb.declare_data_segment(0, PAGE_SIZE, vec![], false)
        .expect("unexpected data segment error");
    mb.declare_data_segment(PAGE_SIZE, PAGE_SIZE, b"hello\0".to_vec(), true)
        .expect("unexpected data segment error");

    // Declare the `main` function, with the appropriate type signature
    let sig = Signature::new([], [AbiParam::new(Type::I32)]);

    let mut fb = mb.function("main", sig).expect("unexpected symbol conflict");

    let raw_ptr_ty = Type::Ptr(Box::new(Type::U8));
    let malloc = fb.import_function("mem", "alloc", malloc_signature()).unwrap();
    let str_from_raw_parts = fb
        .import_function("str", "from_raw_parts", str_from_raw_parts_signature())
        .unwrap();
    let str_compare = fb.import_function("str", "compare", str_compare_signature()).unwrap();

    //   const HELLO: &str = "hello";
    //
    //   let len = HELLO.as_bytes().len();
    //   let ptr: *mut u8 = mem::alloc(len);
    //   memcpy(HELLO.as_bytes().as_ptr(), ptr, len);
    //   let greeting = str::from_raw_parts(ptr, len);
    //
    //   assert_eq!(PAGE_SIZE, 64 * 1024);
    //   assertz!(str::compare(HELLO, greeting));
    //
    //   0
    let hello_data = [PAGE_SIZE, 5u32];
    let hello_data = unsafe {
        let slice = hello_data.as_slice();
        ConstantData::from(slice::from_raw_parts(
            slice.as_ptr() as *const u8,
            mem::size_of::<u32>() * 2,
        ))
    };
    fb.module()
        .declare_global_variable(
            "HELLO",
            str_type(),
            Linkage::Odr,
            Some(hello_data),
            SourceSpan::UNKNOWN,
        )
        .expect("unexpected global variable error");
    let len = fb.ins().load_symbol_relative(
        "HELLO",
        Type::U32,
        mem::size_of::<u32>() as i32,
        SourceSpan::UNKNOWN,
    );
    let ptr = {
        let call = fb.ins().call(malloc, &[len], SourceSpan::UNKNOWN);
        fb.first_result(call)
    };
    let hello_gv = fb.ins().symbol("HELLO", SourceSpan::UNKNOWN);
    let hello_data_ptr = fb.ins().load_global_relative(
        hello_gv,
        raw_ptr_ty.clone(),
        mem::size_of::<u32>() as i32,
        SourceSpan::UNKNOWN,
    );
    //let hello_data_ptr = fb.ins().load_symbol_relative("HELLO", raw_ptr_ty.clone(),
    // mem::size_of::<u32>(), SourceSpan::UNKNOWN);
    fb.ins().memcpy(hello_data_ptr, ptr, len, SourceSpan::UNKNOWN);
    let greeting_ptr = fb.ins().alloca(str_type(), SourceSpan::UNKNOWN);
    fb.ins()
        .call(str_from_raw_parts, &[greeting_ptr, ptr, len], SourceSpan::UNKNOWN);
    let page_size = fb.ins().load_symbol("PAGE_SIZE", Type::U32, SourceSpan::UNKNOWN);
    let expected_page_size = fb.ins().u32(PAGE_SIZE, SourceSpan::UNKNOWN);
    fb.ins().assert_eq(page_size, expected_page_size, SourceSpan::UNKNOWN);
    let compared = {
        let hello_ptr =
            fb.ins()
                .symbol_addr("HELLO", Type::Ptr(Box::new(str_type())), SourceSpan::UNKNOWN);
        let call = fb.ins().call(str_compare, &[hello_ptr, greeting_ptr], SourceSpan::UNKNOWN);
        fb.first_result(call)
    };
    let compared = fb.ins().trunc(compared, Type::I1, SourceSpan::UNKNOWN);
    fb.ins().assertz(compared, SourceSpan::UNKNOWN);
    fb.ins().ret_imm(Immediate::I32(0), SourceSpan::UNKNOWN);

    // Finalize 'test::main'
    fb.build().expect("unexpected validation error, see diagnostics output");
    mb.build()?;

    // Add intrinsics
    intrinsics(builder, context)
}

#[inline]
fn str_type() -> Type {
    Type::Struct(StructType::new([Type::Ptr(Box::new(Type::U8)), Type::U32]))
}

fn malloc_signature() -> Signature {
    Signature::new([AbiParam::new(Type::U32)], [AbiParam::new(Type::Ptr(Box::new(Type::U8)))])
}

fn str_from_raw_parts_signature() -> Signature {
    Signature::new(
        [
            AbiParam::sret(Type::Ptr(Box::new(str_type()))),
            AbiParam::new(Type::Ptr(Box::new(Type::U8))),
            AbiParam::new(Type::U32),
        ],
        [],
    )
}

fn str_compare_signature() -> Signature {
    let str_ptr_ty = Type::Ptr(Box::new(str_type()));
    let a = AbiParam::new(str_ptr_ty);
    let b = a.clone();
    Signature::new([a, b], [AbiParam::new(Type::I8)])
}
