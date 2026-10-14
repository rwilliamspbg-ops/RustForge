Executable Rustdoc-backed examples can be added here for adopter-specific
flows once this template is embedded in a project with its own root package.

Until then, this workspace is a virtual manifest (no root `[package]`), so
`cargo` can't run examples placed directly in this directory. Runnable
examples instead live alongside the crate they demonstrate, using cargo's
standard per-package `examples/` convention:

- `crates/core-tests/examples/fixture_walkthrough.rs` — the shared fixture
  builder pattern other categories reuse.
- `crates/core-tests/examples/const_eval_walkthrough.rs` — compile-time
  computation checked at compile time: `const fn`s, static assertions
  (`const _: () = assert!(...)`), and runtime tests pinning the same
  values. See "Compile-time assertion" in
  [`docs/adding-tests.md`](../docs/adding-tests.md).
- `crates/syntax-tests/examples/parse_walkthrough.rs` — success/failure paths
  through `parse_source`.

Run either with:

```bash
cargo run -p core-tests --example fixture_walkthrough
cargo run -p syntax-tests --example parse_walkthrough
```

When adding a new example, prefer `crates/<name>-tests/examples/*.rs` over
this directory unless you've added a root package to the workspace.

## Standalone example workspaces

Two directories under `examples/` are *self-contained crates* rather than
per-package example binaries — each carries an empty `[workspace]` table
that makes it its own workspace root, the same detachment pattern `fuzz/`
uses. They are deliberately not members of the RustForge workspace: they
add nothing to the default `cargo test --workspace` or its dependency
graph, and copy out wholesale for reuse.

- `examples/eval-harness/` + `examples/sample-candidate/` — the
  code-under-test evaluation harness: a fixed contract (`Candidate`) plus
  golden/property/compile-fail tests, evaluated against any candidate via
  `just eval <path>`. See
  [`docs/evaluation-harness.md`](../docs/evaluation-harness.md).
- `examples/parser-starter/`, `examples/async-service-starter/`,
  `examples/cli-starter/` — domain starters (parser, async service,
  CLI) showing the template's test shapes in real domains. See
  [`docs/domain-starters.md`](../docs/domain-starters.md).
