# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.3](https://github.com/0xPolygonMiden/compiler/compare/miden-base-sys-v0.0.2...miden-base-sys-v0.0.3) - 2024-08-30

### Other
- Merge pull request [#284](https://github.com/0xPolygonMiden/compiler/pull/284) from 0xPolygonMiden/bitwalker/abi-transform-test-fixes

## [0.0.2](https://github.com/0xPolygonMiden/compiler/compare/miden-base-sys-v0.0.1...miden-base-sys-v0.0.2) - 2024-08-28

### Fixed
- *(sdk)* be more explicit about alignment of felt/word types
- *(sdk)* improper handling of get_inputs vec after return into rust

### Other
- remove miden-diagnostics, start making midenc-session no-std-compatible

## [0.0.1](https://github.com/0xPolygonMiden/compiler/compare/miden-base-sys-v0.0.0...miden-base-sys-v0.0.1) - 2024-08-16

### Fixed
- fix the build after VM v0.10.3 update

### Other
- delete `miden-tx-kernel-sys` crate and move the code to `miden-base-sys`
- build the MASL for the tx kernel stubs in `build.rs` and
- rename `midenc-tx-kernel` to `miden-base-sys` and move it to
- fix typos ([#243](https://github.com/0xPolygonMiden/compiler/pull/243))
- a few minor improvements
- set up mdbook deploy
- add guides for compiling rust->masm
- add mdbook skeleton
- provide some initial usage instructions
- Initial commit
