# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- `LICENSE` (MIT), `CONTRIBUTING.md`, and this changelog.
- `async` feature on `semantic-tests` with a Tokio-backed async ownership test.
- `no_std` feature on `core-tests` exposing a `core`-only helper module.
- `perf` feature on `performance-tests` plus a Criterion benchmark (`cargo bench`).
- `fuzz` feature on `fuzz-tests` with property tests, and a real `cargo-fuzz`
  scaffold under `fuzz/`.
- A UTF-8 char-boundary-safe truncation helper in `edge-cases`.
- Broader `integration-tests` end-to-end coverage spanning all categories.
- Runnable examples under `crates/core-tests/examples` and
  `crates/syntax-tests/examples`.
- `scripts/check.sh` and `scripts/coverage.sh` local developer helpers.
- `async` feature on `core-tests` adding async-friendly fixture helpers
  (`async_support::default_user_fixture_async`, concurrent fixture loading).
- `edge` feature on `edge-cases` adding checked-arithmetic boundary helpers
  (`overflow_checks::checked_distance`/`checked_offset`).
- `rust-version`/`license`/`edition` now flow from `[workspace.package]` into
  every crate via `.workspace = true`, instead of being unused root-level
  metadata.

### Changed

- CI now runs clippy and tests with `--all-features` so feature-gated code
  paths are checked on every push/PR.
- Pinned `proptest` to the `~1.8` line in `fuzz-tests`; unconstrained `"1"`
  had resolved to 1.11, which requires rustc 1.85 and silently broke the
  workspace's declared `rust-version = "1.75"` whenever the `fuzz` feature
  was enabled.
- `[workspace.metadata.rustforge].optional_categories` renamed
  `"performance"`/`"edge-cases"` to `"perf"`/`"edge"` to match the actual
  Cargo feature flag names on those crates.

### Fixed

- Root `fuzz/README.md` no longer describes an empty placeholder directory
  now that a real `cargo-fuzz` scaffold lives there.

## [0.1.0] - 2026-01-01

### Added

- Initial modular workspace scaffold: `core-tests`, `syntax-tests`,
  `semantic-tests`, `performance-tests`, `fuzz-tests`, `integration-tests`,
  `edge-cases`.
- CI workflow running `fmt`, `clippy`, and `cargo test --workspace` on
  stable/beta/nightly.
