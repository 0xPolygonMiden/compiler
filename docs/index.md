# Getting Started

Welcome to the documentation for the Miden compiler toolchain!

!!! warning

    The compiler is currently in an experimental state, and has known bugs and limitations, it is
    not yet ready for production usage. However, we'd encourage you to start experimenting with it
    yourself, and give us feedback on any issues or sharp edges you encounter.

The documentation found here should provide a good starting point for the current capabilities of
the toolchain, however if you find something that is not covered, but is not listed as
unimplemented or a known limitation, please let us know by reporting an issue on the compiler
[issue tracker](https://github.com/0xpolygonmiden/compiler/issues).

## What is provided?

The compiler toolchain consists of the following primary components:

- An intermediate representation (IR), which can be lowered to by compiler backends wishing to
support Miden as a target. The Miden IR is an SSA IR, much like Cranelift or LLVM, providing a
much simpler path from any given source language (e.g. Rust), to Miden Assembly. It is used
internally by the rest of the Miden compiler suite.
- A WebAssembly (Wasm) frontend for Miden IR. It can handle lowering both core Wasm modules, as
well as basic components using the experimental WebAssembly Component Model. Currently, the Wasm
frontend is known to work with Wasm modules produced by `rustc`, which is largely just what LLVM
produces, but with the shadow stack placed at the start of linear memory rather than after
read-only data. In the future we intend to support more variety in the structure of Wasm modules
we accept, but for the time being we're primarily focused on using this as the path for lowering
Rust to Miden.
- The compiler driver, in the form of the `midenc` executable, and a Rust crate, `midenc-compiler`
to allow integrating the compiler into other tools. This plays the same role as `rustc` does in
the Rust ecosystem.
- A Cargo extension, `cargo-miden`, that provides a convenient developer experience for creating
and compiling Rust projects targeting Miden. It contains a project template for a basic Rust crate,
and handles orchestrating `rustc` and `midenc` to compile the crate to WebAssembly, and then to
Miden Assembly.
- A terminal-based interactive debugger, available via `midenc debug`, which provides a UI very
similar to `lldb` or `gdb` when using the TUI mode. You can use this to run a program, or step
through it cycle-by-cycle. You can set various types of breakpoints; see the source code, call
stack, and contents of the operand stack at the current program point; as well as interatively
read memory and format it in various ways for display.
- A Miden SDK for Rust, which provides types and bindings to functionality exported from the Miden
standard library, as well as the Miden transaction kernel API. You can use this to access native
Miden features which are not provided by Rust out-of-the-box. The project template generated by
`cargo miden new` automatically adds this as a dependency.

## What can I do with it?

That all sounds great, but what can you do with the compiler today? The answer depends a bit on what
aspect of the compiler you are interested in:

### Rust

The most practically useful, and interesting capability provided by the compiler currently, is the
ability to compile arbitrary Rust programs to Miden Assembly. See the guides for more information
on setting up and compiling a Rust crate for execution via Miden.

### WebAssembly

More generally, the compiler frontend is capable of compiling WebAssembly modules, with some
constraints, to Miden Assembly. As a result, it is possible to compile a wider variety of languages
to Miden Assembly than just Rust, so long as the language can compile to WebAssembly. However, we
do not currently provide any of the language-level support for languages other than Rust, and
have limited ability to provide engineering support for languages other than Rust at this time.

Our Wasm frontend does not support all of the extensions to the WebAssembly MVP, most notably the
reference types and GC proposals.

### Miden IR

If you are interested in compiling to Miden from your own compiler, you can target Miden IR, and
invoke the driver from your compiler to emit Miden artifacts. At this point in time, we don't have
the resources to provide much in the way of engineering support for this use case, but if you find
issues in your efforts to use the IR in your compiler, we would certainly like to know about them!

We do not currently perform any optimizations on the IR, since we are primarily working with the
output of compiler backends which have already applied optimizations, at this time. This may change
in the future, but for now it is expected that you implement your own optimization passes as needed.

## Known Bugs and Limitations

For the latest information on known bugs, see the [issue tracker](https://github.com/0xpolygonmiden/compiler/issues).

See [Known Limitations](appendix/known-limitations.md) for details on what functionality is
missing or only partially implemented.


## Where to start?

Provided here are a set of guides which are focused on documenting a couple of supported workflows
we expect will meet the needs of most users, within the constraints of the current feature set of
the compiler. If you find that there is something you wish to do that is not covered, and is not
one of our known limitations, please open an issue, and we will try to address the missing docs as
soon as possible.

## Installation

To get started, there are a few ways you might use the Miden compiler. Select the one that applies
to you, and the corresponding guide will walk you through getting up and running:

1. [Using the Cargo extension](usage/cargo-miden.md)
2. [Using the `midenc` executable](usage/midenc.md)