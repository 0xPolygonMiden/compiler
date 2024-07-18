# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.1](https://github.com/0xPolygonMiden/compiler/compare/midenc-compile-v0.0.0...midenc-compile-v0.0.1) - 2024-07-18

### Added
- enable spills transformation in default pipeline
- implement most i64 ops and intrinsics, fix some 64-bit bugs
- parse Wasm components

### Fixed
- centralize management of compiler rewrite pipeline
- `FileName::as_str` to avoid enclosing virtual filenames in brackets
- link intrinsics modules in the `CodengenStage` of the midenc
- missing diagnostics on parse error in midenc
- tweak wasm frontend and related test infra
- incorrect module name when compiling wasm
- emit Session artifact in CodegenStage, don't require Session.matches in ApplyRewritesStage
- properly handle emitting final artifacts in midenc-compile

### Other
- fix typos ([#243](https://github.com/0xPolygonMiden/compiler/pull/243))
- set crates versions to 0.0.0, and `publish = false` for tests
- ensure all relevant crates are prefixed with `midenc-`
- run clippy on CI, fix all clippy warnings
- use midenc driver for non-cargo-based fixtures in
- use midenc driver to compile cargo-based fixtures
- handle assembler refactoring changes
- add formatter config, format most crates
- Merge pull request [#100](https://github.com/0xPolygonMiden/compiler/pull/100) from 0xPolygonMiden/greenhat/i89-translate-wasm-cm
- a few minor improvements
- *(docs)* fix typos
- set up mdbook deploy
- add guides for compiling rust->masm
- Merge pull request [#61](https://github.com/0xPolygonMiden/compiler/pull/61) from 0xPolygonMiden/greenhat/cargo-ext-i60
- make `WasmTranslationConfig::module_name_fallback` non-optional
- switch from emiting MASM in CodegenStage, and switch to output folder in cargo extension
- split up driver components into separate crates
- provide some initial usage instructions
- Initial commit
