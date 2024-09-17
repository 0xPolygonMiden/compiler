# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.6](https://github.com/0xpolygonmiden/compiler/compare/miden-stdlib-sys-v0.0.5...miden-stdlib-sys-v0.0.6) - 2024-09-06

### Other
- switch all crates to a single workspace version (0.0.5)

## [0.0.3](https://github.com/0xPolygonMiden/compiler/compare/miden-stdlib-sys-v0.0.2...miden-stdlib-sys-v0.0.3) - 2024-08-30

### Fixed
- *(codegen)* broken return via pointer transformation

### Other
- Merge pull request [#284](https://github.com/0xPolygonMiden/compiler/pull/284) from 0xPolygonMiden/bitwalker/abi-transform-test-fixes

## [0.0.2](https://github.com/0xPolygonMiden/compiler/compare/miden-stdlib-sys-v0.0.1...miden-stdlib-sys-v0.0.2) - 2024-08-28

### Fixed
- *(sdk)* be more explicit about alignment of felt/word types

## [0.0.1](https://github.com/0xPolygonMiden/compiler/compare/miden-stdlib-sys-v0.0.0...miden-stdlib-sys-v0.0.1) - 2024-07-18

### Fixed
- felt representation mismatch between rust and miden

### Other
- set crates versions to 0.0.0, and `publish = false` for tests
- rename `miden-prelude` to `miden-stdlib-sys` in SDK
- start guides for developing in rust in the book,
