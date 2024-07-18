# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.1](https://github.com/0xPolygonMiden/compiler/compare/midenc-codegen-masm-v0.0.0...midenc-codegen-masm-v0.0.1) - 2024-07-18

### Added
- *(codegen)* implement lowering for local var ops
- *(ir)* add support for declaring function-local variables
- *(ir)* add support for 128-bit immediates
- implement most i64 ops and intrinsics, fix some 64-bit bugs
- implement memset, memcpy, mem_grow, mem_size, and bitcast ops
- implement small stores
- support linking against stdlib in codegen tests
- add workaround for wasm `memory.grow` op translation
- implement ilog2/clz/ctz/clo/cto instructions
- implement new sexpr-based format for hir
- implement new instruction scheduler/emitter
- add attributes to the ir
- support loc_store(w) instructions
- add support for loading masm programs from disk
- implement codegen for i32 ops
- implement compiler driver, update midenc
- implement translation of parser ast to hir
- implement conversion of masm ir to 'real' masm
- prettify assertions on textual ir
- improve debugging facilities of the test emulator
- add debugging utilities to test emulator harness
- implement masm ir emulator for testing
- add nativeptr type to masm ir
- masm imports
- implement lowering of inline assembly
- implement lowering of global values
- impelement most of the stackification pass
- extend masm ir for use in codegen
- improve ergonomics of immediate values
- implement stackification analyses

### Fixed
- centralize management of compiler rewrite pipeline
- issue with i1 widening casts
- improve error reporting when parsing masm fails
- properly handle fully-qualified procedure names
- *(codegen)* various issues with lowering of large integral types
- prepend emitted `intrinsics::*` calls with `::` as absolute
- operand solver improperly tracking aliases
- improper rendering of identifiers in masm text output
- address some missing functionality needed by test suite
- *(warnings)* a few useful functions are now detected unused
- use zero-based indices for all instruction positions in depgraph
- promote field to explicit parameter in block scheduler
- remove unused field from scheduler
- use more descriptive name for block argument count
- extract control dependency assignment into separate function
- move add_data_dependency to dependency graph
- improve masm text output
- binary emitter
- properly handle emitting final artifacts in midenc-compile
- clean up display format for masm
- improper generation of procedure name/id when lowering entrypoint
- fix build after rebase
- *(emulator)* incorrect handling of loopback edges in control stack
- broken intrinsics::i32::checked_div
- broken intrinsics::i32::overflowing_mul
- broken intrinsics::i32::overflowing_sub
- i32::overflowing_add
- ensure MasmCompiler runs basic rewrites on programs
- incorrect masm syntax in intrinsics::mem
- ensure intrinsic modules are linked to program
- unreachable raises assertion during stackification
- be more explicit about overflow, address some bugs found while testing
- a variety of operand stack bugs
- bug in depgraph node index oracle
- don't emit redundant u32.assert for known u32 values
- mishandling of mem_loadw/mem_storew semantics
- stackify bug in loop exit edge handling
- a few load related bugs
- *(stackify)* update pass, fix various bugs uncovered in testing

### Other
- Merge pull request [#238](https://github.com/0xPolygonMiden/compiler/pull/238) from 0xPolygonMiden/greenhat/i230-get-inputs-clk451-assert
- Merge pull request [#237](https://github.com/0xPolygonMiden/compiler/pull/237) from 0xPolygonMiden/greenhat/emu-print-stack-option
- Merge pull request [#244](https://github.com/0xPolygonMiden/compiler/pull/244) from 0xPolygonMiden/bitwalker/load-store-reordering
- Merge pull request [#241](https://github.com/0xPolygonMiden/compiler/pull/241) from 0xPolygonMiden/bitwalker/operand-stack-overflow
- clean up docs and implementation of spills rewrite
- fix typos ([#243](https://github.com/0xPolygonMiden/compiler/pull/243))
- restore running MASM on the emulator along the VM in integration tests
- update VM to the commit in next branch after the merge
- workaround for calling the absolute path functions in the emulator
- Fix descriptions for crates
- set crates versions to 0.0.0, and `publish = false` for tests
- add missing descriptions to all crates
- Merge pull request [#203](https://github.com/0xPolygonMiden/compiler/pull/203) from 0xPolygonMiden/greenhat/get-inputs-compile-succ
- add duplicated stack operands test for the stack operand
- ensure all relevant crates are prefixed with `midenc-`
- fix clippy warning
- draft abi transform test for stdlib blake3 hash function
- Merge pull request [#182](https://github.com/0xPolygonMiden/compiler/pull/182) from 0xPolygonMiden/bitwalker/emit-stores
- Merge pull request [#187](https://github.com/0xPolygonMiden/compiler/pull/187) from 0xPolygonMiden/bitwalker/account-compilation-fixes
- check rustfmt on CI, format code with rustfmt
- run clippy on CI, fix all clippy warnings
- Merge pull request [#179](https://github.com/0xPolygonMiden/compiler/pull/179) from 0xPolygonMiden/greenhat/inttoptr-for-gv-store
- add explanatory text regarding choice of data structure in codegen entities
- update expect tests due to formatting changes in assembler
- handle assembler refactoring changes
- remove repetitive words
- add formatter config, format most crates
- update rust toolchain to latest nightly
- a few minor improvements
- *(docs)* fix typos
- Merge pull request [#99](https://github.com/0xPolygonMiden/compiler/pull/99) from 0xPolygonMiden/bitwalker/book
- set up mdbook deploy
- add guides for compiling rust->masm
- remove unused dependencies
- rename Value to ValueOrAlias in miden-codegen-masm
- improve errors produced by various assertions, remove/tweak others
- grammar and other assorted tweaks/improvements
- extract codegen analyses, tweak module structure
- add mdbook skeleton
- update miden-assembly to next
- clear up clippy warnings
- fix build after rebase
- run MASM on VM vs native Rust under `proptest` in integration tests
- compile rust app (cargo project) to masm, run both and compare results;
- add tests for i32 checked_shr intrinsic
- add tests for i32 pow2/ipow intrinsics
- introduce proptest for use in codegen tests
- i32 (un)checked_neg
- add tests for, and address bugs with i32 icmp
- *(emulator)* prepare emulator for use in interactive debugger
- finalize pass refactoring, implement driver
- rework pass infrastructure for integration with driver
- remove `ValueData::Alias`
- fix build after rebase
- use workspace deps rather than paths for hir crates
- add support for real spans in tests
- clean up emulator dispatch loop code
- clean up clippy warnings
- implement codegen frontend and tests
- *(stackify)* implement lowering of instructions, with helpers
- rewrite operand stack impl for codegen
- update masm ir program/module/function items
- provide some initial usage instructions
- Initial commit
