# Developing Miden Programs In Rust

This chapter will walk through how to develop Miden programs in Rust using the prelude and standard library provided by the `miden-prelude` crate (see the [README](https://github.com/0xPolygonMiden/compiler/sdk/prelude/README.md)).

## Getting Started

Import the prelude and standard library from the `miden-prelude` crate:

```rust
use miden_prelude::*;
```

## Using `Felt` (field element) type

The `Felt` type is a field element type that is used to represent the field element values of the Miden VM. 

To initialize a `Felt` value, use the `Felt::new` constructor:

```rust
let a = Felt::new(5).unwrap();
```

The constructor returns an error if the value is not a valid field element, e.g. if it is not in the range `0..=M` where `M` is the modulus of the field (2^64 - 2^32 + 1).

The `Felt` type implements the standard arithmetic operations, e.g. addition, subtraction, multiplication, division, etc. which are accessible through the standard Rust operators `+`, `-`, `*`, `/`, etc. All arithmetic operations are wrapping, i.e. performed modulo `M`.

TODO: Add examples of using operations on `Felt` type and available functions (`assert*`, etc.).