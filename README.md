# Miden IR

This repository provides a compiler for the Miden VM, specifically for Miden Assembly. 
It does this by lowering from a higher-level intermediate representation intended as
a target for source language frontends. This repo defines not only the compiler and
associated tooling, but the IR and code generation libraries as well.

This project is a work-in-progress, stay tuned for updates as things develop.

While there is a standalone `midenc` compiler provided here, the compiler backend is 
intended and able to be used as a library by Rust-based compilers that wish to target
the Miden VM without having to do the code generation work themselves.

## Building

You'll need to have Rust installed (at time of writing, we're doing development against Rust 1.67).

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
