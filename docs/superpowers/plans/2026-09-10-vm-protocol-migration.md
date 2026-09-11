# VM and protocol migration plan

**Goal:** Complete the working-copy migration to VM 0.32.1 and protocol 0.17.0-rc.4 as far as published dependencies permit.

**Approach:** Preserve the existing migration, use all-target compilation to identify API changes, inspect the corresponding upstream implementations, and verify behavior with the existing test suites. Work in this checkout as requested. Do not commit the user's working copy.

- [x] Check all workspace targets excluding `midenc-integration-network-tests`; migrate failing API consumers and their tests.
- [x] Verify package dependency commitments, bundled libraries, assembly generation, and protocol SDK bindings against the installed crate sources.
- [x] Run non-network tests, investigate failures, and update expectations only for understood upstream changes.
- [x] Independently review the final diff, run formatting and relevant checks, and report remaining dependency or environment blockers.

The network test crate and project template's client-based tests remain dependent on a compatible unpublished Miden client. Do not substitute an incompatible client or disable tests to claim success.

## Verification results

- Workspace all-target check passes with `midenc-integration-network-tests` excluded.
- Workspace library tests pass after fixing nondeterministic MASM diagnostic ordering and rerunning the affected frontend tests. The codegen library's 215 tests pass, including native debug-location and serialization regressions.
- CLI, runtime integration, and the five standalone template suites pass across the initial run, resumed run, and focused retries. The resumed run completed 721 tests: 719 passed; the batch cycle snapshot subsequently passed its focused rerun; the project template test requires the blocked client integration. Its first invocation stopped earlier because `MIDENC_BIN_DIR` was unset. Both generated project contracts were separately compiled successfully.
- Batch cycle snapshot changes were verified against an isolated old/new upstream memory-helper comparison on VM 0.32.1: five additional cycles per helper call, matching all four scenarios.
- All 29 lit tests pass with the pinned nightly and built tool binaries configured.
- Workspace doctests pass after correcting stale HIR/logging documentation examples and rerunning logging doctests.
- Formatting and `git diff --check` pass. Independent frontend, SDK, and migration reviews found no outstanding correctness issues.

Changes remain uncommitted in the original working copy. Example and fixture lockfiles were refreshed by their builds.
