# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.1](https://github.com/0xPolygonMiden/compiler/compare/midenc-driver-v0.0.0...midenc-driver-v0.0.1) - 2024-07-18

### Added
- implement compiler driver, update midenc

### Other
- fix typos ([#243](https://github.com/0xPolygonMiden/compiler/pull/243))
- set crates versions to 0.0.0, and `publish = false` for tests
- ensure all relevant crates are prefixed with `midenc-`
- add formatter config, format most crates
- a few minor improvements
- *(docs)* fix typos
- set up mdbook deploy
- add guides for compiling rust->masm
- add mdbook skeleton
- split up driver components into separate crates
- finalize pass refactoring, implement driver
- rework pass infrastructure for integration with driver
- provide some initial usage instructions
- Initial commit
