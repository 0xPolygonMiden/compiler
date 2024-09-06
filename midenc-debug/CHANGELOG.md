# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.6](https://github.com/0xpolygonmiden/compiler/compare/midenc-debug-v0.0.5...midenc-debug-v0.0.6) - 2024-09-06

### Added
- implement 'midenc run' command

### Other
- revisit/update documentation and guides
- switch all crates to a single workspace version (0.0.5)

## [0.0.2](https://github.com/0xPolygonMiden/compiler/compare/midenc-debug-v0.0.1...midenc-debug-v0.0.2) - 2024-08-30

### Fixed
- *(codegen)* broken return via pointer transformation
- *(debugger)* infinite loop in breakpoint id computation

### Other
- fix clippy warnings in tests

## [0.0.1](https://github.com/0xPolygonMiden/compiler/compare/midenc-debug-v0.0.0...midenc-debug-v0.0.1) - 2024-08-16

### Other
- set `midenc-debug` version to `0.0.0` to be in sync with crates.io
- clean up naming in midenc-debug
- rename midenc-runner to midenc-debug
- fix typos ([#243](https://github.com/0xPolygonMiden/compiler/pull/243))
- a few minor improvements
- set up mdbook deploy
- add guides for compiling rust->masm
- add mdbook skeleton
- provide some initial usage instructions
- Initial commit
