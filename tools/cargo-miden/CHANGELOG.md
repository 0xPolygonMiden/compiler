# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.1](https://github.com/0xPolygonMiden/compiler/compare/cargo-miden-v0.0.0...cargo-miden-v0.0.1) - 2024-07-18

### Added
- *(cargo)* allow configuring compiler version for generated projects
- revamp cargo-miden to pass the unrecognized options to cargo,

### Fixed
- rename `compile` cargo extension command to `build`; imports cleanup;

### Other
- fix typos ([#243](https://github.com/0xPolygonMiden/compiler/pull/243))
- set crates versions to 0.0.0, and `publish = false` for tests
- add missing descriptions to all crates
- rename `miden-prelude` to `miden-stdlib-sys` in SDK
- run clippy on CI, fix all clippy warnings
- Merge pull request [#164](https://github.com/0xPolygonMiden/compiler/pull/164) from 0xPolygonMiden/greenhat/i163-cargo-ext-for-alpha
- remove repetitive words
- add formatter config, format most crates
- a few minor improvements
- sync cargo-component crate version to v0.6 in test apps
- set up mdbook deploy
- add guides for compiling rust->masm
- Merge pull request [#61](https://github.com/0xPolygonMiden/compiler/pull/61) from 0xPolygonMiden/greenhat/cargo-ext-i60
- make `WasmTranslationConfig::module_name_fallback` non-optional
- remove `path-absolutize` dependency
- remove `next_display_order` option in `Command`
- move cargo-ext to tools/cargo-miden
- provide some initial usage instructions
- Initial commit
