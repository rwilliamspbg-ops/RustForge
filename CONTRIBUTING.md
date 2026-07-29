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
- [ ] `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features
      --no-deps` passes if you added or changed doc comments (catches
      broken intra-doc links; matches CI's `docs` job).
- [ ] If merging into an existing workspace was affected by your change,
      the "Common Pitfalls" section of `docs/adoption.md` still reflects
      reality — it's based on an actual reproduced test of that flow, not
      assumptions, so keep it that way.

## Test naming and issue linking

- Name test functions `<behavior>_<condition>`, e.g.
  `sum_large_input_is_reasonably_fast`, `retry_rejects_zero_max_tries`,
  `shared_counter_after_workers` — describe what should hold and under what
  condition, not the function under test alone (`test_sum` says nothing a
  reader can act on when it fails).
- When a test exists specifically to reproduce or guard against a filed bug,
  reference the issue number in the test's doc comment (`/// Regression test
  for #123.`) or a `// see #123` comment near the assertion, not in the
  function name — names should stay stable even if the issue link goes
  stale.
- When adding a minimal reproducer for a bug, keep it minimal: strip
  anything not required to trigger the failure before committing it, the
  same way you'd trim a `trybuild` UI fixture (see
  [`docs/adding-tests.md`](docs/adding-tests.md)).

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

- `scripts/check.sh` mirrors the CI `fmt` + `clippy` + `test` + `doc` gate
  (plus `cargo-deny`, if installed).
- `scripts/coverage.sh` runs `cargo llvm-cov` and writes an HTML report.
- Both are also available as [`just`](https://github.com/casey/just)
  recipes (`just check`, `just coverage`), alongside recipes for nextest,
  benchmarks, and fuzz targets — run `just --list` for the full set.

## Style

- Keep fixtures and helpers minimal and dependency-light by default.
- New crates get `unsafe_code = "forbid"` and `missing_docs = "warn"` for
  free via `[lints] workspace = true` in the root `Cargo.toml` — add that to
  a new crate's manifest rather than repeating the old per-crate
  `#![forbid(unsafe_code)]`/`#![warn(missing_docs)]` attributes. If a
  category has a specific, documented reason to need `unsafe` (e.g. a future
  FFI category), override locally rather than changing the workspace default.
- Gate anything that pulls in a non-trivial dependency (async runtimes,
  benchmarking harnesses, property-testing libraries) behind a Cargo feature.
