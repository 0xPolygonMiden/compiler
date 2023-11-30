# Miden Compiler

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

See the `docs` directory for detailed documentation covering the design and implementation
of HIR and the various passes comprising the compilation pipeline.

This project is a work-in-progress, stay tuned for updates as things develop.

## Building

You'll need to have Rust installed (at time of writing, we're doing development against Rust 1.73).

Additionally, you'll want to have [`cargo-make`](https://github.com/sagiegurari/cargo-make) installed:

    $ cargo install cargo-make

From there, you can build all of the tooling used for the compiler, including the compiler itself with:

    $ cargo make

To build just the compiler:

    $ cargo make midenc

This will build the compiler frontend and place it under the `bin` folder in the project root.

    $ bin/midenc help compile
    Usage: midenc compile [OPTIONS] [-- <INPUTS>...]

    Arguments:
      [INPUTS]...
              Path(s) to the source file(s) to compile.

              You may also use `-` as a file name to read a file from stdin.

    Options:
          --output-dir <DIR>
              Write all compiler artifacts to DIR

      -W <LEVEL>
              Modify how warnings are treated by the compiler

              [default: auto]

              Possible values:
              - none:  Disable all warnings
              - auto:  Enable all warnings
              - error: Promotes warnings to errors

      -v, --verbose
              When set, produces more verbose output during compilation

      -h, --help
              Print help (see a summary with '-h')


## Testing

To run the compiler test suite:

    $ cargo make test

This will run all of the unit tests in the workspace, as well as all of our literate tests,
which are executed by the `filecheck` helper found in the `tools` folder.

## Packaging

TBD
