# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.1](https://github.com/0xPolygonMiden/compiler/compare/miden-sdk-v0.0.0...miden-sdk-v0.0.1) - 2024-07-18

### Added
- introduce TransformStrategy and add the "return-via-pointer"
- lay out the Rust Miden SDK structure, the first integration test

### Fixed
- fix value type in store op in `return_via_pointer` transformation,

### Other
- set crates versions to 0.0.0, and `publish = false` for tests
- rename `miden-sdk-tx-kernel` to `miden-tx-kernel-sys`
- rename `miden-prelude` to `miden-stdlib-sys` in SDK
- start guides for developing in rust in the book,
- introduce `miden-prelude` crate for intrinsics and stdlib
- remove `dylib` from `crate-type` in Miden SDK crates
- optimize rust Miden SDK for size
