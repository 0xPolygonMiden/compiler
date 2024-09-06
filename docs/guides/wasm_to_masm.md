# Compiling WebAssembly to Miden Assembly

This guide will walk you through compiling a WebAssembly (Wasm) module, in binary form
(i.e. a `.wasm` file), to Miden Assembly (Masm), both in its binary package form (a `.masp` file),
and in textual Miden Assembly syntax form (i.e. a `.masm` file).

## Setup

We will be making use of the example crate we created in [Compiling Rust to WebAssembly](rust_to_wasm.md),
which produces a small Wasm module that is easy to examine in Wasm text format, and demonstrates a
good set of default choices for a project compiling to Miden Assembly from Rust.

In this chapter, we will be compiling Wasm to Masm using the `midenc` executable, so ensure that
you have followed the instructions in the [Getting Started with `midenc`](../usage/midenc.md) guide
and then return here.

!!! note

    While we are using `midenc` for this guide, the more common use case will be to use the
    `cargo-miden` Cargo extension to handle the gritty details of compiling from Rust to Wasm
    for you. However, the purpose of this guide is to show you what `cargo-miden` is handling
    for you, and to give you a foundation for using `midenc` yourself if needed.

## Compiling to Miden Assembly

In the last chapter, we compiled a Rust crate to WebAssembly that contains an implementation
of the Fibonacci function called `fib`, that was emitted to
`target/wasm32-wasip1/release/wasm_fib.wasm`. All that remains is to tell `midenc` to compile this
module to Miden Assembly.

Currently, by default, the compiler will emit an experimental package format that the Miden VM does
not yet support. To demonstrate what using compiled code with the VM will look like, we're going to
tell the compiler to emit a Miden Assembly library (a `.masl` file), as well as Miden Assembly text
format, so that we can take a look at what the actual Masm looks like:

```bash
midenc compile --emit masm=wasm_fib.masm,masl  target/wasm32-wasip1/release/wasm_fib.wasm
```

This will compile our Wasm module to a Miden Assembly library with the `.masl` extension, and also
emit the textual Masm to `wasm_fib.masm` so we can review it. The `wasm_fib.masl` file will be
emitted in the current directory by default.

If we dump the contents of `wasm_fib.masm`, we'll see the following generated code:

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

!!! note

    At the moment the compiler does only minimal optimization, late in the pipeline during codegen,
    and only in an effort to minimize operand stack management code. So if you see an instruction
    sequence you think is bad, bring it to our attention, and if it is something that we can solve
    as part of our overall optimization efforts, we will be sure to do so. There _are_ limits to
    what we can generate compared to what one can write by hand, particularly because Rust's
    memory model requires us to emulate byte-addressable memory on top of Miden's word-addressable
    memory, however our goal is to keep this overhead within an acceptable bound in the general case,
    and easily-recognized patterns that can be simplified using peephole optimization are precisely
    the kind of thing we'd like to know about, as those kinds of optimizations are likely to produce
    the most significant wins.

## Testing with the Miden VM

!!! note

    For the moment, the `miden run` command does not support running a compiled MAST program
    directly, so we are compiling to a library, and then providing a thin executable module
    which will execute the `fib` function. This is expected to change in an upcoming release.

Assuming you have followed the instructions for installing the Miden VM locally, we can test our
compiled program out as follows:

First, we need to define an executable module which will invoke the `fib` procedure from our
compiled `wasm_fib.masl` library:

```bash
cat <<EOF > main.masm
begin
    exec.::wasm_fib::fib
end
EOF
```

We will also need a `.inputs` file to pass arguments to the program:

```bash
cat <<EOF > wasm_fib.inputs
{
    "operand_stack": ["10"],
    "advice_stack": []
}
EOF
```

With these in place, we can put it all together and run it:

    miden run -a main.masm -n 1 -i wasm_fib.inputs -l wasm_fib/wasm_fib.masl
    ============================================================
    Run program
    ============================================================
    Reading library file `wasm_fib.masl`
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
compiling libraries vs programs, linking together multiple libraries, packages, or discussed some of
the more esoteric compiler options. We will be updating this documentation with those details and
more in the coming weeks and months, so bear with us while we flesh out our guides!
