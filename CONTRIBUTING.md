# Contributing to Miden Compiler

TBD

## Release Process

The release process for the Miden Compiler is managed using the `release-plz` tool. The following steps outline the process for creating a new release:

1. Run `release-plz update` to update the crate versions and generate changelogs.
2. Create a release PR naming the branch with `release-plz-` suffix (its important to use this suffix to trigger the crate publishing on CI in step 4).
3. Review the changes in the release PR, commit edits if needed and merge it into the main branch.
4. The CI will automatically run `release-plz release` after the release PR is merged to publish the new versions to crates.io.
5. Set a git tag for the published crates to mark the release.

### Prerequisites

Install `release-plz` CLI tool following the instructions [here](https://release-plz.ieni.dev/docs/usage/installation)

### Release of the Miden Cargo Extension

Run the steps outlined above in the `Release Process` section in the repo root folder.

### Release of the Miden SDK

TBD