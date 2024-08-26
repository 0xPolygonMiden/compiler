# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
