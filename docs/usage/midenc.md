# Getting Started with `midenc`

The `midenc` executable is the command-line interface for the compiler driver, as well as other
helpful tools, such as the interactive debugger.

While it is a lower-level tool compared to `cargo-miden`, just like the difference between `rustc`
and `cargo`, it provides a lot of functionality for emitting diagnostic information, controlling
the output of the compiler, and configuring the compilation pipeline. Most users will want to use
`cargo-miden`, but understanding `midenc` is helpful for those times where you need to get your
hands dirty.

## Installation

First, you'll need to have Rust installed, with the nightly toolchain (currently we're building
against the `nightly-2024-05-07` toolchain, but we regularly update this).

Then, simply install `midenc` using Cargo in one of the following ways:

    # From crates.io:
    cargo +nightly install midenc

    # If you have cloned the git repo, and are in the project root:
    cargo make install

    # If you have Rust installed, but have not cloned the git repo:
    cargo install --git https://github.com/0xpolygonmiden/compiler midenc


!!! advice

    This installation method relies on Cargo-managed binaries being in your shell `PATH`,
    which is almost always the case, but if you have disabled this functionality, you'll need
    to add `midenc` to your `PATH` manually.

## Usage

Once installed, you should be able to invoke the compiler, you should see output similar to this:

    midenc help compile
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


The actual help output covers quite a bit more than shown here, this is just for illustrative
purposes.

The `midenc` executable supports two primary functions at this time:

* `midenc compile` to compile one of our supported input formats to Miden Assembly
* `midenc debug` to run a Miden program attached to an interactive debugger

## Compilation

See the help output for `midenc compile` for detailed information on its options and their
behavior. However, the following is an example of how one might use `midenc compile` in practice:

```bash
midenc compile --target rollup \
    --entrypoint 'foo::main' \
    -lextra \
    -L ./masm \
    --emit=hir=-,masp \
    -o out.masp \
    target/wasm32-wasip1/release/foo.wasm
```

In this scenario, we are in the root of a Rust crate, named `foo`, which we have compiled for the
`wasm32-wasip1` target, which placed the resulting WebAssembly module in the
`target/wasm32-wasip1/release` directory. This crate exports a function named `main`, which we want
to use as the entrypoint of the program.

Additionally, our Rust code links against some hand-written Miden Assembly code, namespaced under
`extra`, which can be found in `./masm/extra`. We are telling `midenc` to link the `extra` library,
and to add the `./masm` directory to the library search path.

Lastly, we're configuring the output:

* We're using `--emit` to request `midenc` to dump Miden IR (`hir`) to stdout (specified via the `-`
shorthand), in addition to the Miden package artifact (`masp`).
* We're telling `midenc` to write the compiled output to `out.masp` in the current directory, rather
than the default path that would have been used (`target/miden/foo.masp`).

## Debugging

See [Debugging Programs](debugger.md) for details on using `midenc debug` to debug Miden programs.

## Next Steps

We have put together two useful guides to walk through more detail on compiling Rust to WebAssembly:

1. To learn how to compile Rust to WebAssembly so that you can invoke `midenc compile` on the
resulting Wasm module, see [this guide](../guides/rust_to_wasm.md).
2. If you already have a WebAssembly module, or know how to produce one, and want to learn how to
compile it to Miden Assembly, see [this guide](../guides/wasm_to_masm.md).

You may also be interested in our [basic account project template](https://github.com/0xpolygonmiden/rust-templates/tree/main/account/template),
as a starting point for your own Rust project.
