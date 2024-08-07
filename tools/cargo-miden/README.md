# Miden Cargo Extension

This crate provides a cargo extension that allows you to compile Rust code to Miden VM MASM.

## Installation

In order to install(build) the extension, you need to have the nightly Rust toolchain installed:

```bash
rustup toolchain install nightly-2024-05-07
```

To install the extension, run:

```bash
cargo +nightly-2024-05-07 install cargo-miden
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
cargo miden build 
```

Without any additional arguments, this will compile the library target in the target directory in the `miden` folder.
