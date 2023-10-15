use winter_math::FieldElement;

use super::{
    testing::{self, TestContext},
    *,
};

/// Test that we can construct a basic module and function and validate it
#[test]
fn simple_builder_test() {
    let context = TestContext::default();

    let mut builder = ProgramBuilder::new(&context.diagnostics);
    {
        let mut mb = builder.module("test");
        testing::fib1(mb.as_mut(), &context);
        mb.build().expect("unexpected error building test module");
    }
    builder.link().expect("failed to link program");
}

/// Test that we can emit inline assembly within a function, and correctly validate it
#[test]
fn inline_asm_builders_test() {
    let context = TestContext::default();

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
        .function("sum", sig)
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
    let context = TestContext::default();

    let mut builder = ProgramBuilder::new(&context.diagnostics);
    testing::hello_world(&mut builder, &context)
        .expect("unexpected error constructing test modules");

    let _program = builder
        .with_entrypoint("test::main".parse().unwrap())
        .link()
        .expect("failed to link program");
}
