# Contributing to RustForge

RustForge is a template workspace, so contributions should keep every crate
easy to lift out and drop into an adopter's own project.

## Contributor Checklist

Before opening a pull request:

- [ ] `cargo fmt --all --check` passes.
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes.
- [ ] `cargo test --workspace --all-features` passes.
- [ ] New optional tooling (e.g. `criterion`, `proptest`, `tokio`) stays behind
      a Cargo feature so default `cargo test --workspace` remains fast and
      dependency-light.
- [ ] New test categories or crates are documented in the root `README.md`
      module table and, if relevant, `docs/adoption.md`.
- [ ] Fixtures and helpers that other crates should reuse live in
      `crates/core-tests`, not duplicated per category.

## Adding a New Test Category

1. Scaffold a new crate under `crates/<name>-tests/` with its own
   `Cargo.toml` and `src/lib.rs`.
2. Add it to the `members` list in the root `Cargo.toml`.
3. Depend on `core-tests` for shared fixtures/assertions where useful.
4. Add the category to `optional_categories` or `default_categories` in
   `[workspace.metadata.rustforge]` depending on whether adopters should opt
   in explicitly.
5. Document the crate's responsibility in `README.md`.

## Local Script Shortcuts

- `scripts/check.sh` mirrors the CI `fmt` + `clippy` + `test` gate.
- `scripts/coverage.sh` runs `cargo llvm-cov` and writes an HTML report.

## Style

- Keep fixtures and helpers minimal and dependency-light by default.
- Prefer `#![forbid(unsafe_code)]` in new crates unless there's a specific,
  documented reason a category needs `unsafe` (e.g. a future FFI category).
- Gate anything that pulls in a non-trivial dependency (async runtimes,
  benchmarking harnesses, property-testing libraries) behind a Cargo feature.
