# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.6](https://github.com/0xpolygonmiden/compiler/compare/midenc-hir-symbol-v0.0.5...midenc-hir-symbol-v0.0.6) - 2024-09-06

### Other
- clean up unused deps
- switch all crates to a single workspace version (0.0.5)

## [0.0.2](https://github.com/0xPolygonMiden/compiler/compare/midenc-hir-symbol-v0.0.1...midenc-hir-symbol-v0.0.2) - 2024-08-28

### Added
- implement packaging prototype

## [0.0.1](https://github.com/0xPolygonMiden/compiler/compare/midenc-hir-symbol-v0.0.0...midenc-hir-symbol-v0.0.1) - 2024-07-18

### Added
- add attributes to the ir
- implement compiler driver, update midenc
- improve handling of aot-interned symbols
- add hir-symbol crate

### Other
- fix typos ([#243](https://github.com/0xPolygonMiden/compiler/pull/243))
- Fix descriptions for crates
- set crates versions to 0.0.0, and `publish = false` for tests
- add missing descriptions to all crates
- ensure all relevant crates are prefixed with `midenc-`
- add formatter config, format most crates
- a few minor improvements
- set up mdbook deploy
- add guides for compiling rust->masm
- add mdbook skeleton
- clear up clippy warnings
- rework pass infrastructure for integration with driver
- provide some initial usage instructions
- Initial commit
