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
/// pub fn fib(i64) -> i64 {
/// entry(n: i64):
///   zero = const.i64 0
///   is_zero = eq n, zero
///   cond_br is_zero, result_block(zero), is_nonzero_block()
///
/// is_nonzero_block:
///   one = const.i64 1
///   is_one = eq n, one
///   cond_br is_one, result_block(one), calculate_block()
///
/// calculate_block:
///   n1 = sub n, one
///   fib1 = call fib(n1)
///   two = const.i64 2
///   n2 = sub n, two
///   fib2 = call fib(n2)
///   fibn = add fib1, fib2
///   ret fibn
///
/// result_block(result: i64):
///   ret result
/// }
/// ```
#[test]
fn integration_test_function_builder() {
    // Define the 'test' module
    let mut module = Module::new("test".to_string(), None);

    // Declare the `fib` function, with the appropriate type signature
    let sig = Signature {
        visibility: Visibility::PUBLIC,
        name: "fib".to_string(),
        ty: FunctionType::new(vec![Type::I64], vec![Type::I64]),
    };
    let id = module.declare_function(sig.clone());

    // Create the function for building - at this point the function is not yet attached to the module
    let mut function = Function::new(
        id,
        SourceSpan::default(),
        sig,
        module.signatures.clone(),
        module.names.clone(),
    );

    // We create a new lexical scope for the builder so that we can do more
    // with the function after we're done with the builder. You could also
    // explicitly call `drop` on the builder value, but using a block like this
    // gives us a nice visual separation as well.
    {
        // Instantiate a builder with the Function
        let mut builder = FunctionBuilder::new(&mut function);

        // The FunctionBuilder creates the entry block for us, matching the Signature the function was defined with
        let entry = builder.current_block();
        // We can get the entry block parameters corresponding to the function arguments like so
        // We're using a new lexical scope because `block_params` returns a reference to the underlying
        // DataFlowGraph, and we can't mutably borrow the builder until we release it.
        let n = {
            let args = builder.block_params(entry);
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
        let is_nonzero_block = builder.create_block();
        // Then, we create a block for when the input is not a base case (i.e. N > 1)
        let calculate_block = builder.create_block();
        // This block is used to return a value to the caller for the base cases
        let result_block = builder.create_block();
        // Since this block has multiple predecessors, we need a block argument to pass the result
        // value produced by each control flow path. NOTE: It is not necessary to use block arguments for
        // values which come from strictly dominating blocks.
        let result = builder.append_block_param(result_block, Type::I64, SourceSpan::default());

        // The result block simply redirects its argument to the caller, so lets flesh out its definition real quick
        builder.switch_to_block(result_block);
        builder.ins().ret(Some(result), SourceSpan::default());

        // Now, starting from the entry block, we build out the rest of the function in control flow order
        builder.switch_to_block(entry);
        let zero = builder.ins().i64(0, SourceSpan::default());
        let is_zero = builder.ins().eq(n, zero, SourceSpan::default());
        builder.ins().cond_br(
            is_zero,
            result_block,
            &[zero],
            is_nonzero_block,
            &[],
            SourceSpan::default(),
        );

        builder.switch_to_block(is_nonzero_block);
        let one = builder.ins().i64(1, SourceSpan::default());
        let is_one = builder.ins().eq(n, one, SourceSpan::default());
        builder.ins().cond_br(
            is_one,
            result_block,
            &[one],
            calculate_block,
            &[],
            SourceSpan::default(),
        );

        builder.switch_to_block(calculate_block);
        let n1 = builder.ins().sub(n, one, SourceSpan::default());
        // The call instruction may have multiple results, so the builder returns
        // the Inst corresponding to the call instruction, and expects you to request
        // the result Values explicitly as shown here. We use `first_result` because
        // the callee here returns only a single value.
        let fib1 = {
            let call = builder.ins().call(id, &[n1], SourceSpan::default());
            builder.first_result(call)
        };
        let two = builder.ins().i64(2, SourceSpan::default());
        let n2 = builder.ins().sub(n, two, SourceSpan::default());
        let fib2 = {
            let call = builder.ins().call(id, &[n2], SourceSpan::default());
            builder.first_result(call)
        };
        let fibn = builder.ins().add(fib1, fib2, SourceSpan::default());
        builder.ins().ret(Some(fibn), SourceSpan::default());
    }

    // Now, attach our built function to the module it was declared in
    module.define_function(function);
}
