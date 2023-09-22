use std::{mem, slice, sync::Arc};

use miden_diagnostics::{
    term::termcolor::ColorChoice, CodeMap, DefaultEmitter, DiagnosticsHandler,
};
use smallvec::SmallVec;
use winter_math::FieldElement;

use super::*;

/// Test that we can construct a basic module and function and validate it
#[test]
fn simple_builder_test() {
    let context = Test::default();
    build_basic_module(&context.diagnostics);
}

/// Test that we can emit inline assembly within a function, and correctly validate it
#[test]
fn inline_asm_builders_test() {
    let context = Test::default();

    // Define the 'test' module
    let mut builder = ModuleBuilder::new("test");

    // Declare the `sum` function, with the appropriate type signature
    let sig = Signature {
        params: vec![
            AbiParam::new(Type::Ptr(Box::new(Type::Felt))),
            AbiParam::new(Type::U32),
        ],
        results: vec![AbiParam::new(Type::Felt)],
        cc: CallConv::SystemV,
        linkage: Linkage::External,
    };
    let mut fb = builder
        .build_function("sum", sig, SourceSpan::UNKNOWN)
        .expect("unexpected symbol conflict");

    let entry = fb.current_block();
    let (ptr, len) = {
        let args = fb.block_params(entry);
        (args[0], args[1])
    };

    let mut asm_builder = fb
        .ins()
        .inline_asm(&[ptr, len], [Type::Felt], SourceSpan::UNKNOWN);
    asm_builder.ins().push(Felt::ZERO); // [sum, ptr, len]
    asm_builder.ins().push_u32(0); // [i, sum, ptr, len]
    asm_builder.ins().dup(0); // [i, i, sum, ptr, len]
    asm_builder.ins().dup(4); // [len, i, i, sum, ptr, len]
    asm_builder.ins().lt_u32(); // [i < len, i, sum, ptr, len]

    // Now, build the loop body
    //
    // The state of the stack on entry is: [i, sum, ptr, len]
    let mut lb = asm_builder.ins().while_true();

    // Calculate `i / 4`
    lb.ins().dup(0); // [i, i, sum, ptr, len]
    lb.ins().div_imm_u32(4); // [word_offset, i, sum, ptr, len]

    // Calculate the address for `array[i / 4]`
    lb.ins().dup(3); // [ptr, word_offset, ..]
    lb.ins().swap(1);
    lb.ins().add_u32(Overflow::Checked); // [ptr + word_offset, i, sum, ptr, len]

    // Calculate the `i % 4`
    lb.ins().dup(1); // [i, ptr + word_offset, i, sum, ptr, len]
    lb.ins().mod_imm_u32(4); // [element_offset, ptr + word_offset, ..]

    // Precalculate what elements of the word to drop, so that
    // we are only left with the specific element we wanted
    lb.ins().push_u32(4); // [n, element_offset, ..]
    let mut rb = lb.ins().repeat(3);
    rb.ins().sub_imm_u32(1, Overflow::Checked); // [n = n - 1, element_offset]
    rb.ins().dup(1); // [element_offset, n, element_offset, ..]
    rb.ins().dup(1); // [n, element_offset, n, element_offset, ..]
    rb.ins().lt_u32(); // [element_offset < n, n, element_offset, ..]
    rb.ins().movdn(2); // [n, element_offset, element_offset < n]
    rb.build(); // [0, element_offset, element_offset < 1, element_offset < 2, ..]

    // Clean up the now unused operands we used to calculate which element we want
    lb.ins().drop(); // [element_offset, ..]
    lb.ins().drop(); // [element_offset < 1, ..]

    // Load the word
    lb.ins().movup(3); // [ptr + word_offset, element_offset < 1]
    lb.ins().loadw(); // [word[0], word[1], word[2], word[3], element_offset < 1]

    // Select the element, `E`, that we want by conditionally dropping
    // elements on the operand stack with a carefully chosen sequence
    // of conditionals: E < N forall N in 0..=3
    lb.ins().movup(4); // [element_offset < 1, word[0], ..]
    lb.ins().cdrop(); // [word[0 or 1], word[2], word[3], element_offset < 2]
    lb.ins().movup(3); // [element_offset < 2, word[0 or 1], ..]
    lb.ins().cdrop(); // [word[0 or 1 or 2], word[3], element_offset < 3]
    lb.ins().movup(2); // [element_offset < 3, ..]
    lb.ins().cdrop(); // [array[i], i, sum, ptr, len]
    lb.ins().movup(2); // [sum, array[i], i, ptr, len]
    lb.ins().add(); // [sum + array[i], i, ptr, len]
    lb.ins().swap(1); // [i, sum + array[i], ptr, len]

    // We've reached the end of the loop, but we need a copy of the
    // loop header here in order to use the expression `i < len` as
    // the condition for the loop
    lb.ins().dup(0); // [i, i, sum + array[i], ptr, len]
    lb.ins().dup(4); // [len, i, i, sum + array[i], ptr, len]
    lb.ins().lt_u32(); // [i < len, i, sum + array[i], ptr, len]

    // Finalize, it is at this point that validation will occur
    lb.build();

    // Clean up the operand stack and return the sum
    //
    // The stack here is: [i, sum, ptr, len]
    asm_builder.ins().swap(1); // [sum, i, ptr, len]
    asm_builder.ins().movdn(3); // [i, ptr, len, sum]
    let mut rb = asm_builder.ins().repeat(3);
    rb.ins().drop();
    rb.build(); // [sum]

    // Finish the inline assembly block
    let asm = asm_builder.build();
    // Extract the result from the inline assembly block
    let sum = fb.data_flow_graph().first_result(asm);
    fb.ins().ret(Some(sum), SourceSpan::default());

    // Finish building the function, getting back the function identifier
    let _sum = fb
        .build(&context.diagnostics)
        .expect("unexpected validation error, see diagnostics output");

    // Finalize the module
    builder.build();
}

/// Test that we can construct and link a set of modules correctly
#[test]
fn linker_test() {
    let context = Test::default();

    let modules = build_basic_module_set(&context.diagnostics);
    let mut linker = Linker::new();
    linker
        .with_entrypoint("test::main".parse().unwrap())
        .expect("unexpected issue when setting program entrypoint");

    for module in modules.into_iter() {
        linker
            .add(module)
            .expect("unexpected issue when preprocessing module at link-time");
    }

    linker.link().expect("failed to link program");
}

/// The context used by every test
struct Test {
    #[allow(unused)]
    pub codemap: Arc<CodeMap>,
    pub diagnostics: DiagnosticsHandler,
}
impl Default for Test {
    fn default() -> Self {
        let codemap = Arc::new(CodeMap::new());
        let emitter = Arc::new(DefaultEmitter::new(ColorChoice::Auto));
        let diagnostics = DiagnosticsHandler::new(Default::default(), codemap.clone(), emitter);

        Self {
            codemap,
            diagnostics,
        }
    }
}

/// Construct a basic module, called `test`, with a function `fib/1`:
///
/// ```ignore
/// fib(0) -> 0;
/// fib(1) -> 1;
/// fib(N) -> fib(N - 1) + fib(N - 2).
/// ```
///
/// In the textual form of the IR, it would look like so:
///
/// ```ignore
/// module test
///
/// pub fn fib(isize) -> isize {
/// entry(n: isize):
///   zero = const.isize 0
///   is_zero = eq n, zero
///   cond_br is_zero, result_block(zero), is_nonzero_block()
///
/// is_nonzero_block:
///   one = const.isize 1
///   is_one = eq n, one
///   cond_br is_one, result_block(one), calculate_block()
///
/// calculate_block:
///   n1 = sub n, one
///   fib1 = call fib(n1)
///   two = const.isize 2
///   n2 = sub n, two
///   fib2 = call fib(n2)
///   fibn = add fib1, fib2
///   ret fibn
///
/// result_block(result: isize):
///   ret result
/// }
/// ```
fn build_basic_module(diagnostics: &DiagnosticsHandler) -> Box<Module> {
    // Define the 'test' module
    let mut builder = ModuleBuilder::new("test");

    // Declare the `fib` function, with the appropriate type signature
    let sig = Signature {
        params: vec![AbiParam::new(Type::I32)],
        results: vec![AbiParam::new(Type::I32)],
        cc: CallConv::SystemV,
        linkage: Linkage::External,
    };
    let mut fb = builder
        .build_function("fib", sig, SourceSpan::UNKNOWN)
        .expect("unexpected symbol conflict");
    let fib = fb.id();

    // The the entry block is created for us, matching the Signature the function was defined with
    let entry = fb.current_block();
    // We can get the entry block parameters corresponding to the function arguments like so
    let n = {
        let args = fb.block_params(entry);
        args[0]
    };

    // Our function has three conditions:
    //
    // 1. If the input is zero, the output is zero
    // 2. If the input is one, the output is one
    // 3. For all other inputs 'N', the output is N-1 + N-2
    //
    // So we need an extra block for each conditional branch

    // First, we create a block when the input is non-zero
    let is_nonzero_block = fb.create_block();
    // Then, we create a block for when the input is not a base case (i.e. N > 1)
    let calculate_block = fb.create_block();
    // This block is used to return a value to the caller for the base cases
    let result_block = fb.create_block();
    // Since this block has multiple predecessors, we need a block argument to pass the result
    // value produced by each control flow path. NOTE: It is not necessary to use block arguments for
    // values which come from strictly dominating blocks.
    let result = fb.append_block_param(result_block, Type::I32, SourceSpan::default());

    // The result block simply redirects its argument to the caller, so lets flesh out its definition real quick
    fb.switch_to_block(result_block);
    fb.ins().ret(Some(result), SourceSpan::default());

    // Now, starting from the entry block, we build out the rest of the function in control flow order
    fb.switch_to_block(entry);
    let zero = fb.ins().i32(0, SourceSpan::default());
    let is_zero = fb.ins().eq(n, zero, SourceSpan::default());
    fb.ins().cond_br(
        is_zero,
        result_block,
        &[zero],
        is_nonzero_block,
        &[],
        SourceSpan::default(),
    );

    fb.switch_to_block(is_nonzero_block);
    let one = fb.ins().i32(1, SourceSpan::default());
    let is_one = fb.ins().eq(n, one, SourceSpan::default());
    fb.ins().cond_br(
        is_one,
        result_block,
        &[one],
        calculate_block,
        &[],
        SourceSpan::default(),
    );

    fb.switch_to_block(calculate_block);
    let n1 = fb.ins().sub(n, one, SourceSpan::default());
    // The call instruction may have multiple results, so the builder returns
    // the Inst corresponding to the call instruction, and expects you to request
    // the result Values explicitly as shown here. We use `first_result` because
    // the callee here returns only a single value.
    let fib1 = {
        let call = fb.ins().call(fib, &[n1], SourceSpan::default());
        fb.first_result(call)
    };
    let two = fb.ins().i32(2, SourceSpan::default());
    let n2 = fb.ins().sub(n, two, SourceSpan::default());
    let fib2 = {
        let call = fb.ins().call(fib, &[n2], SourceSpan::default());
        fb.first_result(call)
    };
    let fibn = fb.ins().add(fib1, fib2, SourceSpan::default());
    fb.ins().ret(Some(fibn), SourceSpan::default());

    // Finish building the function, getting back the function identifier
    let _fib = fb
        .build(diagnostics)
        .expect("unexpected validation error, see diagnostics output");

    // Finalize the module
    builder.build()
}

/// Construct a set of modules which constitute a program, with dependencies
/// between them to validate various aspects of the linker. We also make use
/// of constant data and global variables to test the linker across multiple
/// tasks.
///
/// The following is pseudocode representing the modules we define.
///
/// First, the module containing our entrypoint, `main`.
///
/// ```ignore
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
/// #[entrypoint]
/// pub fn main() -> isize {
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
///
/// Second, the module containing memory management intrinsics, `mem`:
///
/// ```ignore
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
///
/// Third, a module containing the string intrinsics, `str`:
///
/// ```ignore
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
fn build_basic_module_set(diagnostics: &DiagnosticsHandler) -> SmallVec<[Box<Module>; 3]> {
    const PAGE_SIZE: u32 = 64 * 1024;

    let mut modules = SmallVec::<[Box<Module>; 3]>::default();

    // Define the 'test' module
    let mut builder = ModuleBuilder::new("test");

    // Every module is going to have the same data segment for the shadow stack,
    // and this module will additionally have a data segment for read-only data,
    // i.e. constants
    builder
        .declare_data_segment(0, PAGE_SIZE, vec![], false)
        .expect("unexpected data segment error");
    builder
        .declare_data_segment(PAGE_SIZE, PAGE_SIZE, b"hello\0".to_vec(), true)
        .expect("unexpected data segment error");

    // Declare the `main` function, with the appropriate type signature
    let sig = Signature {
        params: vec![],
        results: vec![AbiParam::new(Type::I32)],
        cc: CallConv::SystemV,
        linkage: Linkage::External,
    };
    let mut fb = builder
        .build_function("main", sig, SourceSpan::UNKNOWN)
        .expect("unexpected symbol conflict");
    let raw_ptr_ty = Type::Ptr(Box::new(Type::U8));
    let str_ty = Type::Struct(vec![raw_ptr_ty.clone(), Type::U32]);
    let str_ptr_ty = Type::Ptr(Box::new(str_ty.clone()));
    let malloc_sig = Signature {
        params: vec![AbiParam::new(Type::U32)],
        results: vec![AbiParam::new(raw_ptr_ty.clone())],
        cc: CallConv::SystemV,
        linkage: Linkage::External,
    };
    let malloc = fb
        .import_function("mem", "alloc", malloc_sig.clone())
        .unwrap();
    let sret_str = {
        let mut param = AbiParam::new(str_ptr_ty.clone());
        param.purpose = ArgumentPurpose::StructReturn;
        param
    };
    let str_from_raw_parts_sig = Signature {
        params: vec![
            sret_str.clone(),
            AbiParam::new(raw_ptr_ty.clone()),
            AbiParam::new(Type::U32),
        ],
        results: vec![],
        cc: CallConv::SystemV,
        linkage: Linkage::External,
    };
    let str_from_raw_parts = fb
        .import_function("str", "from_raw_parts", str_from_raw_parts_sig.clone())
        .unwrap();
    let str_compare_sig = Signature {
        params: vec![
            AbiParam::new(str_ptr_ty.clone()),
            AbiParam::new(str_ptr_ty.clone()),
        ],
        results: vec![AbiParam::new(Type::I8)],
        cc: CallConv::SystemV,
        linkage: Linkage::External,
    };
    let str_compare = fb
        .import_function("str", "compare", str_compare_sig.clone())
        .unwrap();

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
            str_ty.clone(),
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
    //let hello_data_ptr = fb.ins().load_symbol_relative("HELLO", raw_ptr_ty.clone(), mem::size_of::<u32>(), SourceSpan::UNKNOWN);
    fb.ins()
        .memcpy(hello_data_ptr, ptr, len, Type::U8, SourceSpan::UNKNOWN);
    let greeting_ptr = fb.ins().alloca(str_ty.clone(), SourceSpan::UNKNOWN);
    fb.ins().call(
        str_from_raw_parts,
        &[greeting_ptr, ptr, len],
        SourceSpan::UNKNOWN,
    );
    let page_size = fb
        .ins()
        .load_symbol("PAGE_SIZE", Type::U32, SourceSpan::UNKNOWN);
    let expected_page_size = fb.ins().u32(PAGE_SIZE, SourceSpan::UNKNOWN);
    fb.ins()
        .assert_eq(page_size, expected_page_size, SourceSpan::UNKNOWN);
    let compared = {
        let hello_ptr = fb
            .ins()
            .symbol_addr("HELLO", str_ptr_ty.clone(), SourceSpan::UNKNOWN);
        let call = fb
            .ins()
            .call(str_compare, &[hello_ptr, greeting_ptr], SourceSpan::UNKNOWN);
        fb.first_result(call)
    };
    let compared = fb.ins().trunc(compared, Type::I1, SourceSpan::UNKNOWN);
    fb.ins().assertz(compared, SourceSpan::UNKNOWN);
    fb.ins().ret_imm(Immediate::I32(0), SourceSpan::UNKNOWN);

    // Finalize 'test::main'
    let _main = fb
        .build(diagnostics)
        .expect("unexpected validation error, see diagnostics output");

    // Finalize 'test'
    modules.push(builder.build());

    // Next up, the `mem` module
    let mut builder = ModuleBuilder::new("mem");

    // This module knows about the stack segment, but no others
    builder
        .declare_data_segment(0, PAGE_SIZE, vec![], false)
        .expect("unexpected data segment error");

    // pub const PAGE_SIZE: usize = 64 * 1024;
    builder
        .declare_global_variable(
            "PAGE_SIZE",
            Type::U32,
            Linkage::External,
            Some(PAGE_SIZE.to_le_bytes().into()),
            SourceSpan::UNKNOWN,
        )
        .expect("unexpected global variable error");

    // Define the alloc function
    let mut fb = builder
        .build_function("alloc", malloc_sig.clone(), SourceSpan::UNKNOWN)
        .expect("unexpected symbol conflict");

    let memory_grow_sig = Signature {
        params: vec![AbiParam::new(Type::U32)],
        results: vec![AbiParam::new(Type::U32)],
        cc: CallConv::SystemV,
        linkage: Linkage::External,
    };
    let memory_grow = fb
        .import_function("mem", "memory_grow", memory_grow_sig.clone())
        .unwrap();

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
    let size = {
        let args = fb.block_params(fb.current_block());
        args[0]
    };
    let heap_top = fb
        .ins()
        .load_symbol("HEAP_TOP", Type::U32, SourceSpan::UNKNOWN);
    let heap_end = fb
        .ins()
        .load_symbol("HEAP_END", Type::U32, SourceSpan::UNKNOWN);
    let available = fb.ins().sub(heap_end, heap_top, SourceSpan::UNKNOWN);
    let requires_growth = fb.ins().gt(size, available, SourceSpan::UNKNOWN);
    let grow_mem_block = fb.create_block();
    let alloc_block = fb.create_block();
    fb.ins().cond_br(
        requires_growth,
        grow_mem_block,
        &[],
        alloc_block,
        &[],
        SourceSpan::UNKNOWN,
    );

    fb.switch_to_block(grow_mem_block);
    let needed = fb.ins().sub(size, available, SourceSpan::UNKNOWN);
    let need_pages = fb
        .ins()
        .div_imm(needed, Immediate::U32(PAGE_SIZE), SourceSpan::UNKNOWN);
    let need_extra = fb
        .ins()
        .mod_imm(needed, Immediate::U32(PAGE_SIZE), SourceSpan::UNKNOWN);
    let extra_page = fb
        .ins()
        .gt_imm(need_extra, Immediate::U32(0), SourceSpan::UNKNOWN);
    let extra_count = fb.ins().zext(extra_page, Type::U32, SourceSpan::UNKNOWN);
    let num_pages = fb.ins().add(need_pages, extra_count, SourceSpan::UNKNOWN);
    let prev_pages = {
        let call = fb
            .ins()
            .call(memory_grow, &[num_pages], SourceSpan::UNKNOWN);
        fb.first_result(call)
    };
    let usize_max = fb.ins().u32(u32::MAX, SourceSpan::UNKNOWN);
    fb.ins()
        .assert_eq(prev_pages, usize_max, SourceSpan::UNKNOWN);
    fb.ins().br(alloc_block, &[], SourceSpan::UNKNOWN);

    fb.switch_to_block(alloc_block);
    let addr = fb.ins().add(heap_top, size, SourceSpan::UNKNOWN);
    let align_offset = fb
        .ins()
        .mod_imm(addr, Immediate::U32(8), SourceSpan::UNKNOWN);
    let is_aligned = fb
        .ins()
        .eq_imm(align_offset, Immediate::U32(0), SourceSpan::UNKNOWN);
    let align_block = fb.create_block();
    let aligned_block = fb.create_block();
    let new_heap_top_ptr =
        fb.append_block_param(aligned_block, raw_ptr_ty.clone(), SourceSpan::UNKNOWN);

    let ptr = fb
        .ins()
        .inttoptr(addr, raw_ptr_ty.clone(), SourceSpan::UNKNOWN);
    fb.ins().cond_br(
        is_aligned,
        aligned_block,
        &[ptr],
        align_block,
        &[],
        SourceSpan::UNKNOWN,
    );

    fb.switch_to_block(align_block);
    let aligned_addr = fb
        .ins()
        .add_imm(addr, Immediate::U32(8), SourceSpan::UNKNOWN);
    let aligned_addr = fb
        .ins()
        .sub(aligned_addr, align_offset, SourceSpan::UNKNOWN);
    let aligned_ptr = fb
        .ins()
        .inttoptr(aligned_addr, raw_ptr_ty.clone(), SourceSpan::UNKNOWN);
    fb.ins()
        .br(aligned_block, &[aligned_ptr], SourceSpan::UNKNOWN);

    fb.switch_to_block(aligned_block);
    let heap_top_addr = fb.ins().symbol_addr(
        "HEAP_TOP",
        Type::Ptr(Box::new(raw_ptr_ty.clone())),
        SourceSpan::UNKNOWN,
    );
    fb.ins()
        .store(heap_top_addr, new_heap_top_ptr, SourceSpan::UNKNOWN);
    fb.ins().ret(Some(new_heap_top_ptr), SourceSpan::UNKNOWN);

    let _alloc = fb
        .build(diagnostics)
        .expect("unexpected validation error, see diagnostics output");

    // Define the memory_size function
    let memory_size_sig = Signature {
        params: vec![],
        results: vec![AbiParam::new(Type::U32)],
        cc: CallConv::SystemV,
        linkage: Linkage::External,
    };
    let mut fb = builder
        .build_function("memory_size", memory_size_sig, SourceSpan::UNKNOWN)
        .expect("unexpected symbol conflict");

    // pub fn memory_size() -> usize {
    //     (HEAP_END as usize - HEAP_BASE as usize) / PAGE_SIZE
    // }
    let heap_base_addr = fb
        .ins()
        .load_symbol("HEAP_BASE", Type::U32, SourceSpan::UNKNOWN);
    let heap_end_addr = fb
        .ins()
        .load_symbol("HEAP_END", Type::U32, SourceSpan::UNKNOWN);
    let used = fb
        .ins()
        .sub(heap_end_addr, heap_base_addr, SourceSpan::UNKNOWN);
    let used_pages = fb
        .ins()
        .div_imm(used, Immediate::U32(PAGE_SIZE), SourceSpan::UNKNOWN);
    fb.ins().ret(Some(used_pages), SourceSpan::UNKNOWN);

    let _memory_size = fb
        .build(diagnostics)
        .expect("unexpected validation error, see diagnostics output");

    // Define the memory_grow function
    let mut fb = builder
        .build_function("memory_grow", memory_grow_sig, SourceSpan::UNKNOWN)
        .expect("unexpected symbol conflict");

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
    let heap_end = fb
        .ins()
        .load_symbol("HEAP_END", Type::U32, SourceSpan::UNKNOWN);
    let heap_max = fb.ins().u32(u32::MAX, SourceSpan::UNKNOWN);
    let remaining_bytes = fb.ins().sub(heap_max, heap_end, SourceSpan::UNKNOWN);
    let remaining_pages = fb.ins().div_imm(
        remaining_bytes,
        Immediate::U32(PAGE_SIZE),
        SourceSpan::UNKNOWN,
    );
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
    fb.ins()
        .ret_imm(Immediate::U32(u32::MAX), SourceSpan::UNKNOWN);

    fb.switch_to_block(grow_memory_block);
    let heap_base = fb
        .ins()
        .load_symbol("HEAP_BASE", Type::U32, SourceSpan::UNKNOWN);
    let prev_bytes = fb.ins().sub(heap_end, heap_base, SourceSpan::UNKNOWN);
    let prev_pages = fb
        .ins()
        .div_imm(prev_bytes, Immediate::U32(PAGE_SIZE), SourceSpan::UNKNOWN);
    let num_bytes = fb
        .ins()
        .mul_imm(num_pages, Immediate::U32(PAGE_SIZE), SourceSpan::UNKNOWN);
    let new_heap_end = fb.ins().add(heap_end, num_bytes, SourceSpan::UNKNOWN);
    let heap_end_addr = fb.ins().symbol_addr(
        "HEAP_END",
        Type::Ptr(Box::new(Type::U32)),
        SourceSpan::UNKNOWN,
    );
    fb.ins()
        .store(heap_end_addr, new_heap_end, SourceSpan::UNKNOWN);
    fb.ins().ret(Some(prev_pages), SourceSpan::UNKNOWN);

    let _memory_grow = fb
        .build(diagnostics)
        .expect("unexpected validation error, see diagnostics output");

    // Finalize 'mem'
    modules.push(builder.build());

    // Next up, the `str` module
    let mut builder = ModuleBuilder::new("str");

    // Define the from_raw_parts function
    let mut fb = builder
        .build_function(
            "from_raw_parts",
            str_from_raw_parts_sig.clone(),
            SourceSpan::UNKNOWN,
        )
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
    let is_nonnull_addr = fb
        .ins()
        .gt_imm(addr, Immediate::U32(0), SourceSpan::UNKNOWN);
    fb.ins().assert(is_nonnull_addr, SourceSpan::UNKNOWN);
    let ptr_ptr = fb.ins().getelementptr(result, &[0], SourceSpan::UNKNOWN);
    fb.ins().store(ptr_ptr, ptr, SourceSpan::UNKNOWN);
    let len_ptr = fb.ins().getelementptr(result, &[1], SourceSpan::UNKNOWN);
    fb.ins().store(len_ptr, len, SourceSpan::UNKNOWN);
    fb.ins().ret(None, SourceSpan::UNKNOWN);

    let _from_raw_parts = fb
        .build(diagnostics)
        .expect("unexpected validation error, see diagnostics output");

    // Define the compare function
    let mut fb = builder
        .build_function("compare", str_compare_sig.clone(), SourceSpan::UNKNOWN)
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
    fb.ins()
        .cond_br(done, loop_exit, &[], loop_body, &[], SourceSpan::UNKNOWN);

    fb.switch_to_block(loop_body);
    let a_char_addr = fb.ins().incr(a_addr, SourceSpan::UNKNOWN);
    let a_char_ptr = fb.ins().inttoptr(
        a_char_addr,
        Type::Ptr(Box::new(Type::U8)),
        SourceSpan::UNKNOWN,
    );
    let a_char = fb.ins().load(a_char_ptr, SourceSpan::UNKNOWN);
    let b_char_addr = fb.ins().incr(b_addr, SourceSpan::UNKNOWN);
    let b_char_ptr = fb.ins().inttoptr(
        b_char_addr,
        Type::Ptr(Box::new(Type::U8)),
        SourceSpan::UNKNOWN,
    );
    let b_char = fb.ins().load(b_char_ptr, SourceSpan::UNKNOWN);
    let is_eq = fb.ins().eq(a_char, b_char, SourceSpan::UNKNOWN);
    let is_gt = fb.ins().gt(a_char, b_char, SourceSpan::UNKNOWN);
    let zero = fb.ins().i8(0, SourceSpan::UNKNOWN);
    let one = fb.ins().i8(1, SourceSpan::UNKNOWN);
    let neg_one = fb.ins().i8(-1, SourceSpan::UNKNOWN);
    let is_ne_result = fb.ins().select(is_gt, one, neg_one, SourceSpan::UNKNOWN);
    let i_incr = fb.ins().incr(i, SourceSpan::UNKNOWN);
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
    let len_gt_result = fb
        .ins()
        .select(is_len_gt, one, neg_one, SourceSpan::UNKNOWN);
    let len_eq_result = fb
        .ins()
        .select(is_len_eq, zero, len_gt_result, SourceSpan::UNKNOWN);
    fb.ins()
        .br(exit_block, &[len_eq_result], SourceSpan::UNKNOWN);

    fb.switch_to_block(exit_block);
    fb.ins().ret(Some(result), SourceSpan::UNKNOWN);

    let _compare = fb
        .build(diagnostics)
        .expect("unexpected validation error, see diagnostics output");

    // Finalize 'str'
    modules.push(builder.build());

    // We're done
    modules
}
