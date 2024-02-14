# Miden Compiler

> [!IMPORTANT] 
> This project is a work-in-progress, so if you encounter bugs or other
> things which are not covered in the issue tracker, there is a good chance we know
> about them, but please do report them anyway so we can ensure they are tracked
> publically as well.

This repository contains the Miden compiler, which can be used both as a compiler backend
for existing languages that wish to target Miden Assembly using a standard SSA-based IR;
or as means of compiling WebAssembly (Wasm) produced by another compiler to Miden Assembly.

This repo is broken into the following high-level components:

* Miden HIR (high-level intermediate representation) and it's supporting crates;
providing everything needed to build and compile IR for a program you want to
emit Miden Assembly for.
* The Wasm frontend; a library which can be used to convert a program compiled to `.wasm` to HIR
* The `midenc` executable, which provides a command-line tool that provides a convenient way
to compile Wasm or HIR modules/programs to Miden Assembly and test them.

> [!TIP] 
> We've published initial [documentation](https://0xpolygonmiden.github.io/compiler) 
> in mdBook format for easier reading, also accesible in the `docs` directory. This documentation 
> covers how to get started with the compiler, provides a couple guides for currently supported
> use cases, and contains appendices that go into detail about various design aspects of the 
> toolchain.

## Building

You'll need to have Rust installed (at time of writing, we're doing development against Rust 1.73).

Additionally, you'll want to have [`cargo-make`](https://github.com/sagiegurari/cargo-make) installed:

    $ cargo install cargo-make

From there, you can build all of the tooling used for the compiler, including the compiler itself with:

    $ cargo make

To build just the compiler:

    $ cargo make midenc

## Testing

To run the compiler test suite:

    $ cargo make test

This will run all of the unit tests in the workspace, as well as all of our `lit` tests.

## Packaging

TBD
