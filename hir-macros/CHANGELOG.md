# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.6](https://github.com/0xpolygonmiden/compiler/compare/midenc-hir-macros-v0.0.5...midenc-hir-macros-v0.0.6) - 2024-09-06

### Other
- switch all crates to a single workspace version (0.0.5)

## [0.0.2](https://github.com/0xPolygonMiden/compiler/compare/midenc-hir-macros-v0.0.1...midenc-hir-macros-v0.0.2) - 2024-08-16

### Fixed
- *(cli)* improve help output, hide plumbing flags

### Other
- unify diagnostics infa between compiler, assembler, vm

## [0.0.1](https://github.com/0xPolygonMiden/compiler/compare/midenc-hir-macros-v0.0.0...midenc-hir-macros-v0.0.1) - 2024-07-25

### Other
- enable publish for `midenc-hir-macros` crate and restore
- manually bump `midenc-hir-macros` version after release-plz
- fix typos ([#243](https://github.com/0xPolygonMiden/compiler/pull/243))
- set crates versions to 0.0.0, and `publish = false` for tests
- ensure all relevant crates are prefixed with `midenc-`
- add formatter config, format most crates
- a few minor improvements
- set up mdbook deploy
- add guides for compiling rust->masm
- add mdbook skeleton
- finalize pass refactoring, implement driver
- rework pass infrastructure for integration with driver
- provide some initial usage instructions
- Initial commit
