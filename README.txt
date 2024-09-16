# Miden Compiler Docs

This directory contains the sources for the [Miden Compiler Documentation](https://docs.polygon.technology/miden/compiler/).).

All doc files are written in Markdown, and are converted into an online book using [mkdocs](https://squidfunk.github.io/mkdocs-material/).

## Building the docs

You can build and view the documentation using two different methods, depending on whether or not
you have `cargo` installed.

**NOTE:** Both methods described below are expected to be run from the root of the compiler project.

### Using `cargo`

To build the docs, use `cargo make docs`. This will place the build output in the `target/docs/site`
directory.

If you wish to build the docs _and_ run a server to view them, which will reload when changes are
made, use `cargo make serve-docs`. The hostname and port will be displayed in the terminal, but by
default the docs should be available at `http://127.0.0.1:8000`.

### Using `mkdocs` script

If you don't have `cargo` installed, you can use this method instead. Again, the commands below are
written with the current working directory being the root of the compiler project, _not_ this
directory.

To build the docs, use `docs/mkdocs build -d target/docs/site`. This matches the behavior of the
`cargo make docs` command.

To view the docs in your browser, with reload on change enabled, use `docs/mkdocs serve`. The
hostname and port where the docs can be viewed will be printed to the terminal, but by default this
is `http://127.0.0.1:8000`.

## License

MIT
