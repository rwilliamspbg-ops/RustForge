# Contributing to RustForge

RustForge is a template workspace, so contributions should keep every crate
easy to lift out and drop into an adopter's own project.

- Adding a test to an existing category? See [`docs/adding-tests.md`](docs/adding-tests.md).
- Wondering *why* something's built the way it is? See [`docs/best-practices.md`](docs/best-practices.md) and [`docs/architecture.md`](docs/architecture.md).
- Adding a whole new category crate? See "Adding a New Test Category" below.

## Contributor Checklist

Before opening a pull request:

- [ ] `cargo fmt --all --check` passes.
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes.
- [ ] `cargo test --workspace --all-features` passes.
- [ ] If you touched `syntax-tests`' UI fixtures, `cargo test -p syntax-tests
      --features compile-fail` passes on **stable**. (CI's multi-toolchain
      `test` job deliberately excludes `compile-fail` from its
      `--all-features`-equivalent run — see `ci/README.md` — so this needs
      a separate local check; `scripts/check.sh` uses `--all-features` and
      covers it too, as long as you run it on stable.)
- [ ] New optional tooling (e.g. `criterion`, `proptest`, `tokio`, `trybuild`)
      stays behind a Cargo feature so default `cargo test --workspace`
      remains fast and dependency-light, and is pinned to an MSRV-compatible
      version if the latest release has moved past `rust-version` in the
      root `Cargo.toml` (see the "MSRV" section of `README.md`).
- [ ] If you pinned a dependency for MSRV reasons, add a matching `ignore`
      rule in `.github/dependabot.yml` — otherwise Dependabot will propose
      bumping straight past the pin (see `docs/best-practices.md`).
- [ ] New test categories or crates are documented in the root `README.md`
      module table and, if relevant, `docs/adoption.md`, and have their own
      short `crates/<name>/README.md`.
- [ ] Fixtures and helpers that other crates should reuse live in
      `crates/core-tests`, not duplicated per category.
- [ ] `cargo deny check` passes if you changed dependencies (install with
      `cargo install cargo-deny --locked`).

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
