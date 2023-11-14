# Miden Cargo Extension

This crate provides a cargo extension that allows you to compile Rust code to Miden VM MASM.

## Installation

To install the extension, run:

```bash
cargo install cargo-miden
```

## Requirements

Since Rust is first compiled to Wasm, you'll need to have the `wasm32-unknown-unknown` target installed:

```bash
rustup target add wasm32-unknown-unknown
```

## Usage

### Getting help
To get help with the extension, run:

```bash
cargo miden
```

Or for help with a specific command:

```bash
cargo miden <command> --help
```

### Creating a new project
To create a new Miden VM project, run:

```bash
cargo miden new <project-name>
```

### Compiling a project
To compile a Rust crate to Miden VM MASM, run:

```bash
cargo miden compile -o <output-file>
```

Without any additional arguments, this will compile the library target of the crate in the current directory.
