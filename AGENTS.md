## Testing.

After finishing the implementation (code changes) always build and test the workspace with `cargo make test-all` and fix the errors.

When running `cargo test` or `nextest` set `CARGO_TARGET_DIR` env var to the workspace's `target` folder.

After you are finished run the following tasks when applicable:
- `cargo make format-rust` to ensure Rust code is formatted
- `cargo make clippy` to ensure code passes the set of lints we have enabled, pass `--fix` to automatically fix issues when possible
- `cargo make unused` to identify any unused dependencies/items that should be removed
- `cargo make rustdocs` to ensure Rust-based documentation comments build
- `cargo make docs` to ensure our Docusaurus-based documentation pages build

# Migration instructions on Miden SDK changes

Any user-facing changes in the crates in the `sdk` folder should be analyzed and migration instruction should be provided if needed in sdk/sdk/MIGRATION.md.

### Changelog

Do not manually write entries in any of the `CHANGELOG.md` files in this repo - they will be generated on release from the commit messages.
