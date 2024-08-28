# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.2](https://github.com/0xPolygonMiden/compiler/compare/midenc-hir-type-v0.0.1...midenc-hir-type-v0.0.2) - 2024-08-28

### Added
- implement packaging prototype

## [0.0.1](https://github.com/0xPolygonMiden/compiler/compare/midenc-hir-type-v0.0.0...midenc-hir-type-v0.0.1) - 2024-07-18

### Added
- draft Miden ABI function types encoding and retrieval
- introduce Miden ABI component import
- introduce `CanonicalOptions` in IR and translate Wasm
- implement new sexpr-based format for hir
- rewrite type layout functionality
- refactor type layout primitives
- define type compatibility for operators
- provide type representation enum
- implement inline assembly
- distinguish signed/unsigned types, native/emulated pointers

### Fixed
- issue with i1 widening casts
- felt representation mismatch between rust and miden
- *(ir)* incorrect entries in operand compatibility matrix
- use stabilized next_multiple_of in alignable impls
- switch text representation of the `MidenAbiFunctionType` to s-exp;
- rewrite incorrect type layout code

### Other
- fix typos ([#243](https://github.com/0xPolygonMiden/compiler/pull/243))
- Fix descriptions for crates
- set crates versions to 0.0.0, and `publish = false` for tests
- add a description for miden-hir-type crate
- ensure all relevant crates are prefixed with `midenc-`
- since all the Miden ABI transformation happens in the frontend
- add `FunctionType::abi` and ditch redundant `*FunctionType`
- add Wasm component translation support to the integration tests;
- add formatter config, format most crates
- update rust toolchain to latest nightly
- Merge pull request [#100](https://github.com/0xPolygonMiden/compiler/pull/100) from 0xPolygonMiden/greenhat/i89-translate-wasm-cm
- move `LiftedFunctionType` to `miden-hir-type` crate
- set up mdbook deploy
- add guides for compiling rust->masm
- add mdbook skeleton
- rework the ir to better suit wasm->masm
- split up hir crate
- provide some initial usage instructions
- Initial commit
