use std::sync::Arc;

use miden_diagnostics::{
    term::termcolor::ColorChoice, CodeMap, DefaultEmitter, DiagnosticsHandler,
};

use super::*;

/// The following test constructs an IR module equivalent to the following pseudocode:
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
#[test]
fn integration_test_builders() {
    let codemap = Arc::new(CodeMap::new());
    let emitter = Arc::new(DefaultEmitter::new(ColorChoice::Auto));
    let diagnostics = DiagnosticsHandler::new(Default::default(), codemap.clone(), emitter);

    // Define the 'test' module
    let mut builder = ModuleBuilder::new("test");

    // Declare the `fib` function, with the appropriate type signature
    let sig = Signature {
        params: vec![AbiParam::new(Type::Isize)],
        results: vec![AbiParam::new(Type::Isize)],
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
    let result = fb.append_block_param(result_block, Type::Isize, SourceSpan::default());

    // The result block simply redirects its argument to the caller, so lets flesh out its definition real quick
    fb.switch_to_block(result_block);
    fb.ins().ret(Some(result), SourceSpan::default());

    // Now, starting from the entry block, we build out the rest of the function in control flow order
    fb.switch_to_block(entry);
    let zero = fb.ins().isize(0, SourceSpan::default());
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
    let one = fb.ins().isize(1, SourceSpan::default());
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
    let two = fb.ins().isize(2, SourceSpan::default());
    let n2 = fb.ins().sub(n, two, SourceSpan::default());
    let fib2 = {
        let call = fb.ins().call(fib, &[n2], SourceSpan::default());
        fb.first_result(call)
    };
    let fibn = fb.ins().add(fib1, fib2, SourceSpan::default());
    fb.ins().ret(Some(fibn), SourceSpan::default());

    // Finish building the function, getting back the function identifier
    let _fib = fb
        .build(&diagnostics)
        .expect("unexpected validation error, see diagnostics output");

    // Finalize the module
    builder.build();
}
