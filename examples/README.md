Executable Rustdoc-backed examples can be added here for adopter-specific
flows once this template is embedded in a project with its own root package.

Until then, this workspace is a virtual manifest (no root `[package]`), so
`cargo` can't run examples placed directly in this directory. Runnable
examples instead live alongside the crate they demonstrate, using cargo's
standard per-package `examples/` convention:

- `crates/core-tests/examples/fixture_walkthrough.rs` — the shared fixture
  builder pattern other categories reuse.
- `crates/syntax-tests/examples/parse_walkthrough.rs` — success/failure paths
  through `parse_source`.

Run either with:

```bash
cargo run -p core-tests --example fixture_walkthrough
cargo run -p syntax-tests --example parse_walkthrough
```

When adding a new example, prefer `crates/<name>-tests/examples/*.rs` over
this directory unless you've added a root package to the workspace.
