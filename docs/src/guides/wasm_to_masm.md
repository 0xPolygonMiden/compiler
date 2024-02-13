# Compiling WebAssembly to Miden Assembly

This chapter will walk you through compiling a WebAssembly (Wasm) module, in binary form 
(i.e. a `.wasm` file), to an corresponding Miden Assembly (Masm) module (i.e. a `.masm` file).

## Setup

We will be making use of the example crate we created in [Compiling Rust to WebAssembly](rust_to_wasm.md), 
which produces a small, lightweight Wasm module that is easy to examine in Wasm 
text format, and demonstrates a good set of default choices for a project compiling 
to Miden Assembly via WebAssembly.

In this chapter, we will be compiling Wasm to MASM using the `midenc` executable, so ensure that
you have followed the instructions in the [Getting Started (midenc)](../usage/midenc.md) guide 
and then return here.

## Compiling to Miden Assembly

In the last chapter, we compiled a Rust crate to WebAssembly that contains an implementation
of the Fibonacci function called `fib`, that was emitted to `target/wasm32-unknown-unknown/release/wasm_fib.wasm`.
All that remains is to tell `midenc` to compile this module to WebAssembly, as shown below:

> [!NOTE] 
> The compiler is still under heavy development, so there are some known bugs that
> may interfere with compilation depending on the flags you use - for the moment, the compiler
> invocation we have to use is quite verbose, but this is a short term situation while we 
> address various other higher-priority tasks. Ultimately, using `midenc` directly will be
> less common than other use cases (such as using `cargo miden`, or using the compiler as a
> library for your own language frontend).

    midenc compile -o wasm_fib.masm --emit=masm target/wasm32-unknown-unknown/release/wasm_fib.wasm
    
This will place the generated Miden Assembly code for our `wasm_fib` crate in the current directory.
If we dump the contents of this file, we'll see the following generated code:

```
export.fib
  push.0
  push.1
  movup.2
  swap.1
  dup.1
  neq.0
  push.1
  while.true
    if.true
      push.4294967295
      movup.2
      swap.1
      u32wrapping_add
      dup.1
      swap.1
      swap.3
      swap.1
      u32wrapping_add
      movup.2
      swap.1
      dup.1
      neq.0
      push.1
    else
      drop
      drop
      push.0
    end
  end
end
```

If you compare this to the WebAssembly text format, you can see that this is a fairly
faithful translation, but there may be areas where we generate sub-optimal Miden Assembly. 

At the moment the compiler does only minimal optimization, late in the pipeline during codegen,
and only in regards to operand stack management. In other words, if you see an instruction
sequence you think is bad, certainly bring it to our attention, but we can't guarantee that
the code we generate will match what you would write by hand.

## Testing with the Miden VM

> [!NOTE] 
> This example is more complicated than it needs to be at the moment, bear with us!

Assuming you have followed the instruction for installing the Miden VM locally,
we can test this program out as follows:

First, we need to define a program to link our `wasm_fib.masm` module into, since 
it is not a program, but a library module:

    cat <<EOF > main.masm
    use.wasm_fib::wasm_fib
    
    begin
        exec.wasm_fib::fib
    end
    
We will also need a `.inputs` file to pass arguments to the program:

    cat <<EOF > wasm_fib.inputs
    {
        "operand_stack": ["10"],
        "advice_stack": ["0"],
    }
    
Next, we need to build a MASL library (normally `midenc` would do this, but there is a bug
blocking it at the moment, this example will be updated accordingly soon):

    mkdir -p wasm_fib && mv wasm_fib.masm wasm_fib/
    miden bundle -n wasm_fib wasm_fib

With these in place, we can put it all together and run it:

    miden run -a main.masm -n 1 -i wasm_fib.inputs -l wasm_fib/wasm_fib.masl
    ============================================================
    Run program
    ============================================================
    Reading library file `wasm_fib/wasm_fib.masl`
    Reading program file `main.masm`
    Parsing program... done (0 ms)
    Compiling program... done (2 ms)
    Reading input file `wasm_fib.inputs`
    Executing program with hash 3d965e7c6cfbcfe9d9db67262cbbc31517931a0169257f385d447d497cf55778... done (1 ms)
    Output: [55]
    VM cycles: 263 extended to 512 steps (48% padding).
    ├── Stack rows: 263
    ├── Range checker rows: 67
    └── Chiplets rows: 201
        ├── Hash chiplet rows: 200
        ├── Bitwise chiplet rows: 0
        ├── Memory chiplet rows: 0
        └── Kernel ROM rows: 0


Success! We got the expected result of `55`.


## Next Steps

This guide is not comprehensive, as we have not yet examined in detail the differences between 
compiling libraries vs programs, linking together multiple libraries, emitting a `.masl` library,
or discussed some of the compiler options. We will be updating this documentation with those
details and more in the coming days, so bear with us while we flesh out our guides!
