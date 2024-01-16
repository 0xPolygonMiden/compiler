# As an Executable

At the present time, we do not yet have prebuilt packages of the compiler toolchain
available, so it must be built from source, but the requirements for this are minimal,
as shown below:

## Installation

First, you'll need to have Rust installed (at time of writing, we're doing development 
against Rust 1.74).

Then, simply install `midenc` using Cargo, like so:

    # If you have cloned the git repo, and are in the project root:
    $ cargo install --path midenc midenc
    
    # If you have Rust installed, but have not cloned the git repo:
    $ cargo install --git https://github.com/0xpolygonmiden/compiler --branch develop midenc

NOTE: This installation method relies on Cargo-managed binaries being in your shell `PATH`,
which is almost always the case, but if you have disabled this functionality, you'll need
to add `midenc` to your `PATH` manually.

## Usage

Once built, you should be able to invoke the compiler now, for example:

    $ midenc help compile
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


## Next Steps

We currently have two frontends to the compiler, one that accepts the compiler's IR in textual 
form (as a `.hir` file), primarily used for testing; and one that accepts a WebAssembly module
in binary form (i.e. as a `.wasm` file).

For the vast majority of people, if not everyone, the `.wasm` form will be the one you are interested
in, so we have put together a [helpful guide](../guides/wasm_to_masm.md) that walks through how to
compile a WebAssembly module (in this case, produced by `rustc`) to Miden Assembly using `midenc`.

If you aren't sure how to produce a WebAssembly module, you may be interested in 
[another guide](../guides/rust_to_wasm.md) that demonstrates how to emit a WebAssembly module from
a Rust crate.
