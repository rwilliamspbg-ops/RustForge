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

### Changed

- CI now runs clippy and tests with `--all-features` so feature-gated code
  paths are checked on every push/PR.

## [0.1.0] - 2026-01-01

### Added

- Initial modular workspace scaffold: `core-tests`, `syntax-tests`,
  `semantic-tests`, `performance-tests`, `fuzz-tests`, `integration-tests`,
  `edge-cases`.
- CI workflow running `fmt`, `clippy`, and `cargo test --workspace` on
  stable/beta/nightly.
