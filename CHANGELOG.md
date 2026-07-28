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
  metadata; every crate is also marked `publish = false` since none are
  meant to be published to crates.io individually.
- `compile-fail` feature on `syntax-tests`: real `trybuild` UI/compile-fail
  tests (`tests/compile_fail.rs`, fixtures under `tests/ui/`), CI-pinned to
  stable only since diagnostic text can drift between toolchain channels.
- `.github/dependabot.yml`: weekly update PRs for the main workspace, the
  detached `fuzz/` workspace, and GitHub Actions.
- `deny.toml` + a `deny` CI job (`cargo-deny`): license allow-list, security
  advisories, banned/duplicate dependencies, and untrusted-source checks
  against both the main workspace and `fuzz/`.
- `msrv` CI job: installs Rust 1.75 (the declared floor) and actually builds
  and tests the default-feature workspace against it, rather than just
  asserting the promise in docs.
- `rustfmt.toml` (stable-channel-only options) and `clippy.toml` (pins
  clippy's `msrv` to 1.75 so it won't suggest newer-than-MSRV idioms).
- `.gitattributes` normalizing line endings to LF regardless of a
  contributor's `core.autocrlf` setting, so `cargo fmt`'s `newline_style =
  "Unix"` doesn't fail on Windows checkouts for reasons unrelated to code.
- `SECURITY.md` with a private vulnerability-reporting process.
- `.github/ISSUE_TEMPLATE/` (bug report, feature request) and
  `PULL_REQUEST_TEMPLATE.md`, reinforcing the `CONTRIBUTING.md` checklist.
- `.editorconfig` for consistent indentation/line-ending across editors.
- Doc comments on every public item across all 7 crates, plus
  `#![warn(missing_docs)]` (enforced as an error via the existing
  `-D warnings` clippy CI step) so documentation coverage doesn't regress.

### Changed

- CI's `test` job now runs clippy/tests with an explicit feature list
  (`async,no_std,perf,fuzz,edge`) instead of `--all-features`, deliberately
  excluding `syntax-tests`' new `compile-fail` feature — see the `trybuild`
  job below.
- Pinned `proptest` to the `~1.8` line in `fuzz-tests`; unconstrained `"1"`
  had resolved to 1.11, which requires rustc 1.85 and silently broke the
  workspace's declared `rust-version = "1.75"` whenever the `fuzz` feature
  was enabled.
- `[workspace.metadata.rustforge].optional_categories` renamed
  `"performance"`/`"edge-cases"` to `"perf"`/`"edge"` to match the actual
  Cargo feature flag names on those crates.

- Bumped `criterion` to `0.8` in `performance-tests` (Dependabot proposed
  this and two other bumps in one grouped PR; see below for why only this
  one was accepted).
- `.github/dependabot.yml` now ignores `proptest`/`trybuild` releases past
  the versions pinned in Cargo.toml — Dependabot doesn't know *why* those
  are pinned (MSRV compatibility for the `fuzz`/`compile-fail` features) and
  will otherwise keep proposing to bump straight past them.

### Fixed

- Root `fuzz/README.md` no longer describes an empty placeholder directory
  now that a real `cargo-fuzz` scaffold lives there.
- `deny.toml` now sets `allow-wildcard-paths` and every crate sets
  `publish = false`, fixing `cargo-deny`'s bans check flagging our own
  in-workspace path dependencies as risky "wildcard" dependencies.
- `crates/performance-tests/benches/sum_bench.rs` now uses
  `std::hint::black_box` instead of the now-deprecated `criterion::black_box`
  (deprecated as of criterion 0.8, which turned into a CI failure under
  `-D warnings` the moment Dependabot proposed the bump).

## [0.1.0] - 2026-01-01

### Added

- Initial modular workspace scaffold: `core-tests`, `syntax-tests`,
  `semantic-tests`, `performance-tests`, `fuzz-tests`, `integration-tests`,
  `edge-cases`.
- CI workflow running `fmt`, `clippy`, and `cargo test --workspace` on
  stable/beta/nightly.
