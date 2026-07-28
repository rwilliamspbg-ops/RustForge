# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

**Phase 2 — content and examples:**

- `core-tests`: `ConfigFixture` (a second builder-style fixture),
  `retry()` (a dependency-free retry helper for flaky/eventually-ready
  operations), 4 runnable rustdoc examples (the workspace previously had
  zero doc-tests), and a table-driven test module.
- `semantic-tests`: `longest`/`Excerpt<'a>` (the classic multi-input
  lifetime example), `assert_send`/`assert_sync` (compile-time `Send`/
  `Sync` checks), and `ConfigError`/`parse_timeout_ms` (the
  "implement `Error`, propagate with `?`" pattern).
- `syntax-tests`: a second `trybuild` pass fixture (generic function with a
  trait bound) and a second fail fixture (use-after-move, `E0382`).
- `edge-cases`: `clamp_collection` (the collection counterpart to
  `safe_truncate`, with no gotcha to fix — the boundary-value
  counterexample), and `proptest` property tests (behind the `edge`
  feature, now also pulling in `proptest`) for `safe_truncate`,
  `clamp_collection`, and `checked_offset`.
- `performance-tests`: `max_value` and `dedup_sorted`, each with a
  Criterion benchmark in a new `benches/collection_ops_bench.rs`, unit
  tests, and an `#[ignore]`d wall-clock regression guard.
- `fuzz-tests`: `parse_u32_lenient`, unit tests, `proptest` property tests,
  a second `cargo-fuzz` target (`fuzz/fuzz_targets/parse_u32_lenient.rs`),
  and a committed `fuzz/seed_corpus/<target>/` per target (curated starting
  inputs, distinct from the gitignored auto-grown `fuzz/corpus/`).
- `integration-tests`: a second end-to-end test chaining all of the above
  new functions across every category into one config-batch pipeline.

**Phase 3 — automation and quality gates:**

- `coverage` CI job (`cargo-llvm-cov`, uploaded as an HTML artifact,
  informational — no enforced threshold).
- `nextest` CI job demonstrating [cargo-nextest](https://nexte.st/) as an
  opt-in alternative test runner (`cargo test` stays primary).
- `docs` CI job (`RUSTDOCFLAGS=-D warnings cargo doc`), added after it
  caught a real broken intra-doc link (`` [`slice::dedup`] `` in
  `performance-tests`, now fixed) that nothing else in CI would have caught.
- `fuzz-build` job now does a 10-second smoke run per target, seeded from
  `fuzz/seed_corpus/`, after building — catches an immediately-panicking
  harness, not just a build failure.
- `test` job matrix now also runs stable on windows-latest and
  macos-latest (beta/nightly stay ubuntu-only to control CI minutes).
- `justfile` with recipes for check, fmt, clippy, test (default/all/
  compile-fail), nextest, bench, fuzz (check/build/run), deny, coverage,
  and doc — run `just --list`.
- `scripts/check.sh` now also runs the `docs` check (`RUSTDOCFLAGS=-D
  warnings cargo doc`) and, if `cargo-deny` is installed, `cargo deny
  check` for both the main workspace and `fuzz/`.
- "Common Pitfalls" section in `docs/adoption.md`, written from an actual
  reproduced test of copying `crates/` into a fresh existing workspace —
  most notably, that workspace needs its own `[workspace.package]` table
  or every crate fails to parse (`workspace.package.edition was not
  defined`). The Quickstart in `README.md` now calls this out as step 2.

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
- `docs/adding-tests.md` — a practical guide to the four test shapes in
  this template (unit, property, compile-fail, benchmark) and where a new
  test belongs.
- `docs/best-practices.md` — the conventions this template follows and why
  (dependency-light defaults, MSRV pinning discipline, doc coverage,
  narrowly-scoped CI jobs).
- `docs/architecture.md` — workspace shape, crate dependency graph, and CI
  pipeline, each as a Mermaid diagram.
- A short `README.md` in every `crates/*` directory (purpose, contents,
  feature flags, an example command).
- `LICENSE-APACHE`, dual-licensing the project under MIT OR Apache-2.0 (the
  Rust ecosystem convention) alongside the existing `LICENSE-MIT` (renamed
  from `LICENSE`).
- A "Features" highlights section, an MSRV badge, and repo description/topics.

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
- `license` in `[workspace.package]` (and `fuzz/Cargo.toml`) changed from
  `"MIT"` to `"MIT OR Apache-2.0"`, matching the new dual-licensing.

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
- Broken intra-doc link `` [`slice::dedup`] `` in `performance-tests` fixed
  to `` [`Vec::dedup`] `` — found via the new `docs` CI job (and the
  `just doc` recipe locally), which is why that job exists now.
- `scripts/coverage.sh` no longer uses the deprecated `--no-run` flag; uses
  `cargo llvm-cov report --html` instead to regenerate the HTML report
  without re-running tests a second time.

## [0.1.0] - 2026-01-01

### Added

- Initial modular workspace scaffold: `core-tests`, `syntax-tests`,
  `semantic-tests`, `performance-tests`, `fuzz-tests`, `integration-tests`,
  `edge-cases`.
- CI workflow running `fmt`, `clippy`, and `cargo test --workspace` on
  stable/beta/nightly.
