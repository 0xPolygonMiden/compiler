# Packaging

This appendix describes the package format consumed and produced by the Miden toolchain.

## Overview

A Miden package represents a compiled _program_ or _library_, with sufficient metadata to
allow any Miden VM instance to execute the code it contains. Packages may reference other
packages, and all packages in the dependency graph of a program must be available to the
VM in order for the code to be executed. More about package dependencies will be covered
later in the [Dependencies](#dependencies) section.

A package consists of the following:

* A manifest that provides top-level metadata about the package
* The Merkelized Abstract Syntax Tree (MAST) which was produced by compiling the original
source code. The MAST is what will be directly executed by the Miden VM at runtime.
* A WebAssembly Interface Types (WIT) component definition, which is used to describe
the interfaces exported from the package, their type signatures, and other useful metadata
about the structure of the package. See the [Type Descriptor](#type-descriptor)
section for more details.
* Zero or more read-only data segments. See the [Read-Only Data Segments](#read-only-data-segments)
* One or more optional items that can be used with the package for development
and debugging, the structure of which depends on the specific item:
  * Debug info
  * Documentation

A package has a canonical directory layout, used both for packing and unpacking the contents
of the package, as needed, see [Directory Layout](#directory-layout). However, the contents
of the package can be directly extracted from the package file without extracting all of the
contents. See [Binary Format](#binary-format) for details.

## Top-Level Metadata

* Schema Version
  * The version of the package format itself, currently `1`.
* Package Name (fully-qualified, e.g. `namespace/package`)
* Package Version (using semver2 semantics)
* Minimum required Miden VM version
* Package Type (Executable vs Library)
* Start function/entrypoint (optional for libraries)
  * For program packages, specifies the entrypoint, i.e. `main`
  * For library packages, if given, specifies a function that must be called to initialize the package before any of its exports may be called within a given context
* Features (optional extras)
  * Documentation
  * Debug information
* Dependencies (name, version, digest)

## Type Descriptor

The type descriptor file is the canonical, binary form of a WebAssembly Component Model [type definition](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Explainer.md#type-definitions).
It is derived from a [WebAssembly Interface Types](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md)
file, maintained by hand by the package author.

This descriptor is used for two purposes:

1. To generate language-specific bindings to the interface(s) described. WIT is an Interface Description Language (IDL),
describing interfaces in a language-agnostic way, but defined in terms of a [Canonical ABI](https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md),
allowing efficient cross-language interop. We follow the semantics of the Wasm canonical ABI, but layered on top of
our canonical Miden ABI, which describes how the Wasm abstract machine is translated to the Miden abstract machine.

2. To provide the compiler with the necessary type information needed to emit lifting/lowering code at call boundaries,
as well as mapping imports (from the packages' dependencies) to their corresponding MAST roots, so that calls use those
roots rather than the fully-qualified names. This ensures that the code that was compiled and published is always the
exact code that gets executed, in perpetuity.

Because the type descriptor is in binary form, to view it in the more human-friendly WIT format, you
will need to use [wasm-tools](https://github.com/bytecodealliance/wasm-tools), like so:

    wasm-tools component wit extracted-package/component.wasm

The contents of `component.wasm` here is not an actual Wasm module, but simply the binary representation
of the WIT package.

## Directory Layout

The canonical layout of a package when extracted as a directory, is as follows:

    name@version/
    |- manifest.toml
    |- component.wasm
    |- lib/
       |- 0x00000000.mast
       |- 0x00000001.mast
    |- rodata/
       |- 0x0.bin
    |- etc/
       |- component.sourcemap
       |- docs/
          |- index.html
       |- vendor/
          |- foo@0.1.0.mpkg

### Manifest

The `manifest.toml` file contains all of the top-level metadata about the package. When produced as a result
of extracting a package, it's contents may differ from the `manifest.toml` used to create the package. This
is because we provide some helpful functionality in the manifest for creating packages by hand, that are
translated into canonical form when the package is created. These "canonicalizations" are covered below
in each relevant section.

> [!NOTE]
> TOML was chosen as the format for the manifest due to it being a nicer format for
> writing by hand, while being strongly typed, unambiguous, and supported by good
> tooling in most languages.

The manifest begins with basic metadata about the package:

```toml
[package]
name = "my-package"
version = "0.1.0"
miden-version = "0.8.0"  # Optional
```

The `miden-version` field lets you specify a minimum Miden VM version for the package. Any attempt to load
a package with a lower-versioned VM will fail. This field is optional, and when not present, is equivalent
to no minimum required version.

#### Package Type

There are two primary types of packages: libraries and programs (executables). These correspond to the
`[lib]` and `[bin]` sections of the manifest, respectively. Only one section is permitted.

For libraries:

```toml
[lib]
start = "0x..." # Optional
```

The `start` field specifies the MAST root of a function that must be called to initialize the library before
any of its other exports are called. This function must be called for each new context in which the library
is used, and must only be called once.

For programs:

```toml
[bin]
entrypoint = "0x..."
```

The `entrypoint` field specifies the MAST root of a function that will be called as the entrypoint of the
program, e.g. `_start`. An entrypoint is required, and the root specified _must_ be contained within the
primary MAST of the package (i.e. not in a dependency).

#### Read-Only Data/Data Segments

If the package has static data it expects to be present in linear memory when any of the code it contains starts
executing, it should be specified using the `[rodata]` section:

```toml
[rodata]
# Write the contents of `assets/foo.json` into linear memory at address 0x0,
0x0 = { path = "assets/foo.json", encoding = "bytes" }
0xDEADBEEF = { content = "0x...", encoding = "words" }
```

In this example, two segments will be written, with slightly different semantics:

* The contents of `assets/foo.json` will be written into linear memory starting at address `0x0`, treating the
raw binary data as if it is laid out in byte-addressable memory, and encoding it as data in word-addressable memory
by taking each 4 byte chunk and encoding it as a single field element.

* The data represented by the hex-encoded string in the `content` field will be written into linear memory
starting at address `0xDEADBEEF`, treating the decoded data as if it is already in the canonical word-addressable memory
representation. This means that every 8 bytes will be interpreted as the canonical little-endian byte representation
of a single field element.

In both cases, the data written into linear memory will be padded with zeros to the nearest word boundary. For example,
if you have 13 elements worth of data (3 words + 1 element), then the actual segment size will be 16 elements, with
the last 3 all zeroes.

Segment overlap is not permitted, except when the overlap only occurs due to padding. Segments are written in increasing
address order, so padding bytes will never overwrite non-padding bytes.

> [!IMPORTANT]
> The `[rodata]` section is only used for creating a package from a directory.
> It will never be present in a manifest extracted from a package, because the
> rodata will have been emitted in canonical on-disk form in the `rodata` directory

#### Extras

Certain items that a package can contain are considered optional extras. These items can be
controlled from the `[extras]` section:

```toml
[extras]
docs = true
debug = true
```

The entire `[extras]` section is optional, in which all optional features are assumed to be disabled.
If the section is present, any items that are not specified, are assumed to be disabled.

The structure and full configuration of extras is not yet fully defined, but the following are some
initial thoughts:

* The `docs` field indicates whether or not to include documentation in the package. If `true`, then
the docs are expected to be in the form of a static HTML site, placed under the `etc/docs` directory,
with an `index.html` file as the main page of the docs. The docs will be compressed as a zip file,
and stored within the package. If `false`, then whether or not `etc/docs` is present doesn't matter,
it will not be included in the package binary.

* The `debug` field indicates whether or not to include source-level debugging information in the package.
If `true`, then the debug info/source maps/etc., will be expected under `etc/debug`. Currently, there is
no format for this data, so this setting has no effect.

#### Dependencies

The final section of the manifest, is `[dependencies]`. This section specifies what dependencies this
package has, and optionally, how to fetch them.

The most basic way to express a dependency is as follows:

```toml
[dependencies]
foo = "0.1.0"
```

This expresses a dependency on the `foo` package at version `0.1.0`. When expressed this way, any suitable
match for this version string can satisfy this dependency. As such, the package must not contain direct
references to specific MAST roots in `foo`, as the roots are not guaranteed to be stable across builds.
For certain things, this flexibility may be convenient. However, if you want to be more precise, you
can specify the content digest to match:

```toml
[dependencies]
foo = { version = "0.1.0", digest = "..." }
```

This will look for a package that matches the given version string, and hashes to the given digest.

It is also supported to create a package which vendors its dependencies. Such packages cannot be published
to a registry, but are useful all-in-one artifacts that can be run in a VM that has no way to access
external registries. Vendored packages are placed under `etc/vendor`, e.g. `etc/vendor/foo@0.1.0.mpkg`.
You indicate that a dependency is vendored like so:

```toml
[dependencies]
foo = { version = "0.1.0", vendored = true }
```

You can also specify the `digest` key, and the hash of the vendored package will be checked against the
digest when loading the dependency.

Currently, there are no other ways to specify where to fetch a package from, e.g. `git` or `url` dependencies,
but those may come once we have more package infrastructure in place.

> [!IMPORTANT] If there are dependencies specified in the WIT type definition file, then they
> must agree with those specified in the `[dependencies]` section. This will be checked when
> a package is created.

### MAST

The MAST for this package can be found in the `lib` directory, in one or more `.mast` files, each of which contain
some MAST, in binary form. Each file must be named with the digest of the root of the MAST contained in that file.

In the canonical form, each MAST root is in its own file, but it is permitted to build a package from a single `.mast`
file containing the entire MAST forest.

### WIT

The canonical representation of the WIT type definition is the binary form, in a file named `component.wasm`.

To allow for more ergonomic creation of packages by hand, it is also supported to use two alternative formats:
WebAssembly text format (as `component.wat`), or WIT text format (as `component.wit`). The former is simply
the textual form of the canonical binary representation; but the latter is suitable for writing by hand.

Extracting a package will always extract to the canonical form.

### Read-Only Data Segments

We've already discussed the semantics of the `[rodata]` section in the manifest, however the `rodata` directory
represents the canonical form of that data. Specifically, each file in the `rodata` directory corresponds to a
single contiguous data segment, with the name of the file specifying the address (offset in words) at which the
segment will start. The data in the file is expected to be interpreted as a slice of field elements in their
canonical representation in little-endian byte order. This slice will then be padded out to the nearest word-sized
chunk.

> [!WARNING] Unlike the `[rodata]` section, no overlap checking is performed on the contents of the `rodata` directory.
> It is presumed that the tooling which produced those files has already done so, and any overlap is intentional.

If the `rodata` directory exists _and_ the `[rodata]` section is present in the manifest, the `[rodata]` section
takes precedence, and the contents of the `rodata` directory are ignored. This allows creating packages by hand
with an `rodata` directory in which you place the files that you reference in the `[rodata]` section.

A few notes:

## Binary Format

Now that we've laid out the contents of a package when extracted to a directory, let's look at how that gets
encoded in binary form, as a file with the `.mpkg` extension (short for Miden Package).

### Encoding

Encoding a package from the directory structure works like so:

1. Initialize two buffers:
  a. One for the package header
  b. One for the package content
2. Write the package schema version (1 byte), i.e. `0x1`, to the header
3. Write the package type (1 byte), i.e. `0x0` for library, `0x1` for executable, to the header
4. Write the `features` bitflags to the header, e.g. `0x0` for a package which contains no extra features
5. For each string in the `name`, `version`, and `miden_version` fields, in that order, do the following:
  a. Compute the size in bytes of the field
  b. Write the size in bytes as an unsigned [LEB128](https://en.wikipedia.org/wiki/LEB128)-encoded integer to the header
  c. Write the bytes of the string to the content buffer
6. Write the `start`/`entrypoint` MAST root digest to the header. A digest of all zeroes indicates that the value is `None`
7. Write the number of rodata segments (LEB128-encoded) to the header
8. Write the rodata segment metadata (segment data offset, base address and size) as follows:
  a. Capture the current offset in the content buffer as a u32 value called `segments_offset`
  b. For each rodata segment:
    aa. Write the current value of `segments_offset` to the header, recording the position where the segment data will start in the content buffer
    bb. Write the base address where the segment should be placed in linear memory as a u32 value to the header
    cc. Write the size in bytes of the segment as a u32 value to the header
    dd. Write the segment data to the content buffer
    ee. `segments_offset += segment.size`
9. Write the number of dependencies (LEB128-encoded) to the header
10. Write the dependency metadata (dependency offset, flags, name_len, version_len) as follows:
  a. Capture the current offset in the content buffer as a u32 value called `dependencies_offset`
  b. For each dependency:
    aa. Write the current value of `dependencies_offset` to the header
    bb. Write the flags byte to the header
    cc. Write the size in bytes of the dependency name to the header
    dd. Write the dependency name to the content buffer
    ee. Write the size in bytes of the dependency version to the header
    ff. Write the dependency version to the content buffer
    gg. Write the size in bytes of the dependency digest to the header
    hh. Write the dependency digest to the content buffer
11. Write the WIT definition (encoded in its binary form) to the content buffer
12. Write the MAST (merged into a forest, and encoded in its binary form) to the content buffer
13. If there are any extra files, i.e. files that should be extracted to `etc/`, then compress them as a zip file (either by zipping up the contents of the `etc/` directory, or by creating a zip that is structured as if you had done so), and write it to the end of the content buffer
14. Allocate an output buffer, with capacity for `8 + 64 + header.len() + content.len()` bytes
15. Write the `b"MIDENPF\0"` magic to the output buffer
16. Write 32 zeroes to the output buffer
17. Append the header bytes
18. Compute the HMAC-SHA256 signature of the content buffer, using the package
name and version concatenated together and hex-encoded as the signing key, and append the signature to the output buffer
19. Append the content bytes
20. Compute the HMAC-SHA256 signature of the output buffer, sans the first 40 bytes, using the header magic, schema version, and content signature concatenated together (hex-encoded) as the signing key.
21. Write the signature we just computed to the 32-byte region of step 16

The content of the output buffer can be described using the following structure (pseudo-Rust):

```rust
#[repr(C)]
pub struct Package {
    magic: [u8; 8],            // b"MIDENPF\0"
    digest: [u8; 32],
    header: Header,
    content_digest: [u8; 32],
    content: [u8], // dynamically-sized trailing bytes
}

#[repr(C)]
pub struct Header {
    schema_version: u8,        // 0x1
    ty: PackageType,           // 1 byte
    features: Features,        // 1 byte
    name_len: usize,           // LEB128-encoded
    version_len: usize,        // LEB128-encoded
    miden_version_len: usize,  // LEB128-encoded
    start: [u8; 32],
    num_segments: usize,       // LEB128-encoded
    segments: [SegmentInfo; num_segments],
    num_dependencies: usize,   // LEB128-encoded
    dependencies: [DependencyInfo; num_dependencies],
}

#[repr(u8)]
pub enum PackageType {
    Library,
    Program,
}

bitflags::bitflags! {
    pub struct Features: u8 {
        /// This package provides documentation
        const DOCS = 0x01;
        /// This package provides debug information
        const DEBUG = 0x02;
    }
}

#[repr(C)]
pub struct SegmentInfo {
    /// The relative offset in the segment data section of the package content
    /// where the data for this segment can be found
    offset: usize,
    /// The offset in words where the segment should be placed
    base: u32,
    /// The size in bytes of the raw binary data
    size: u32,
}

#[repr(C)]
pub struct DependencyInfo {
    /// The relative offset in the segment data section of the package content
    /// where the data for this segment can be found
    offset: usize,
    /// Flags used to indicate how this dependency is used
    flags: u8,          // vendored=0x01
    name_len: usize,    // LEB128-encoded
    version_len: usize, // LEB128-encoded
    digest_len: usize,  // LEB128-encoded
}
```

Without the header, the content of the package is an opaque bag of bytes, but the header allows interpreting the content as if it had the following structure (pseudo-Rust):

```rust
#[repr(C)]
pub struct Content {
    name: [u8; name_len],
    version: [u8; version_len],
    miden_version: [u8; miden_version_len],
    segment_data: [SegmentData; num_segments],
    dependency_infos: [DependencyData; num_dependencies],
    wit: Component,
    mast: MastForest,
    extra: Option<ZipFile>, // If EOF is reached after `mast`, this is None
}

pub struct SegmentData([u8; segment.size]);

#[repr(C)]
pub struct DependencyData {
    name: [u8; name_len],
    version: [u8; version_len],
    digest: [u8; digest_len],
}
```

This structure gives us the following properties:

* The package header and content can be verified using secure signatures before
doing anything with the package.
* The package structure can be validated in constant time
* Accessing specific items in the package can be done without having to read all of the content into memory, only the header
* The package can be decoded in a zero-copy fashion
* It is possible to query specific details, like what version of a dependency the package depends on, without having to decode the package fully.

### Decoding

The following is how one can fully decode the package encoded using the procedure in the previous section:

1. Read the package into an in-memory buffer (this can be avoided, but
for simplicity we're going to do it this way here)
2. Verify that the buffer starts with the `b"MIDENPF\0" magic
3. Read the next 32 bytes as a HMAC-SHA256 signature for the rest of the buffer, set this aside.
4. Read the next byte as the package schema version, verify that it is a supported version (0x1)
5. Read the next byte as the package type, verify that the value is a valid package type
6. Read the next byte as features bitflags
7. Read three LEB128-encoded integers from the buffer, validating that each of them can be stored in a `usize` field. This gives us `name_len`, `version_len`, and `miden_version_len` respectively
8. Read a 32-bit MAST root digest from the buffer, if zeroed, it is equivalent to `None`, if non-zero, it is equivalent to `Some(digest)`. This is the value of the `start`/`entrypoint` field (which one depends on the package type).
9. Read an LEB128-encoded integer from the buffer, validating that the result can be represented as a `usize`. This is the number of rodata segment infos contained in the header.
10. Read or skip `num_segments` segment info records from the buffer
11. Read an LEB128-encoded integer from the buffer, validating that the result can be represented as a `usize`. This is the number of dependency info records contained in the header.
12. Read or skip `num_dependencies` dependency info records from the buffer.
13. Read next 32 bytes of the input as an HMAC-SHA256 signature for the content section of the package, set this aside. The current position in the input buffer is now the start of the content section.
14. The next `name_len` bytes are the name of the package
15. The next `version_len` bytes are the version of the package
16. The next `miden_version_len` bytes are the minimum supported Miden version of the package
17. The current position in the input buffer is now the start of the rodata segment section, which can be skipped by computing the offset of the end of the last segment + one byte, e.g. `segment_data[num_segments - 1].offset + segment_data[num_segments - 1].size`. Alternatively, you can map over each segment info record and capture the segment data that corresponds to it.
18. The current position in the input buffer is now the start of the dependency data section. Just like with the rodata segments, you can either skip this section by computing the start of the next section from the end of the last dependency data, or capture the data for each dependency, whichever is needed.
19. The current position in the input buffer is the start of the WIT definition, and should be parsed according to [this document](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Binary.md). Note that if you don't care about this part of the package, it is possible to compute the offset in the input where the next section (the MAST) starts. If parsing the component definition fails, the package is invalid and decoding should stop.
20. The current position in the input buffer should now be the first byte of the MAST, in its canonical binary format. This too can be parsed or skipped as needed, but if it is invalid, the package is invalid and decoding cannot continue.
21. If we've reached EOF, then there is no extra data attached to the package, and we're done. Otherwise, the remainder of the input should be interpreted as a zip archive.
22. If the package header specified that there was extra data attached (e.g. docs, debug info), but no extra data was actually found, or the zip archive was invalid, or those items were not found, that should be treated as a validation error.
23. The signatures should be verified by first computing the signature for the content section as described in [Encoding](#encoding), verifying it, then computing the signature for the header + content as described in the [Encoding](#encoding) procedure, and verifying that. If both signatures are valid, the package is verified as authentic.

> [!NOTE]
> A separate signing scheme will be used for publishing packages to a registry, the scheme described here is purely for verifying that the package has not been tampered with or corrupted

Notice that when decoding the package, much of the package content can be skipped entirely if it is not needed, and it is possible to seek to specific contents. A certain amount of metadata does have to be read to access the full contents of the package, but only a very small amount has to actually be read into memory at a time.

### Extraction

Decoding the binary representation of a package should make use of the canonical in-memory data structures for the WIT component definition, the MAST forest, and the manifest structure which implements TOML serialization/deserialization. This makes conversion in either direction more straightforward. Even if different data structures are used, the process of extracting a package to disk will be described in terms of these data structures.

1. Create the target directory with the naming convention `name@version`.
2. Create the manifest data structure, initialized with the top-level metadata we decoded (e.g. name, version, etc.). Data segments are not represented in the manifest when extracting, instead they are placed on disk in their canonical form. Once the manifest data structure is populated, it should be serialized to disk, in the root of the target directory, in TOML format, with the filename `manifest.toml`.
3. Write the WIT component to the root of the target directory as `component.wasm`
4. Create a `lib` directory in the root of the target directory, and then for each root in the MAST forest, write the corresponding tree in binary format to a file in `lib`, using the naming convention `<digest>.mast`
5. If we decoded any data segments, create an `rodata` directory in the root of the target directory, and write each segment's data as a separate file in that directory, using the base address in hexadecimal form as the filename, using `.bin` as the extension.
6. If there was extra data provided in zip form, create an `etc` directory in the root of the target directory, and extract the contents of the zip archive to that directory

## Roadmap

The following is a rough outline of the tasks involved in implementing this format:

- [ ] Agree on specifications for package and MAST binary formats
- [ ] Create new repository for Rust crate that implements the core package tooling, something like `miden-package`
- [ ] Implement data structures to represent package contents in memory
- [ ] Implement serialization/deserialization from binary
- [ ] Implement support for building a package from directory structure
- [ ] Implement support for extracting package to directory structure
- [ ] Publish first release of `miden-package`

The implementation should support Wasm, by allowing packages to be unpacked from raw buffers rather than files, and eliding functionality that depends on standard library functionality not available in Wasm (e.g. reading from files). This is to ensure the VM can make use of `miden-package` while running in the browser.
