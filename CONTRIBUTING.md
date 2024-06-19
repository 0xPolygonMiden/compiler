# Contributing to Miden Compiler

TBD

## Release Process

The release process for the Miden Compiler is managed using the `release-plz` tool. The following steps outline the process for creating a new release:

1. Create a release PR using the `release-plz release-pr` command. This will create a new branch with the release changes (crate versions bumped, changelog generated, etc.).
2. Review the changes in the release PR, commit edits if needed and merge it into the main branch.
3. The CI will automatically run `release-plz release` after the release PR is merged to publish the new versions to crates.io.

### Prerequisites

Install `release-plz` CLI tool following the instructions [here](https://release-plz.ieni.dev/docs/usage/installation)


### Release of the Miden Cargo Extension

1. Run `release-plz release-pr` in the repo root folder to create a release PR.
2. Check and commit edits to the release PR if needed.
3. Merge the release PR into the main branch.
4. The CI will automatically run `release-plz release` after the release PR is merged to publish the new versions to crates.io.
