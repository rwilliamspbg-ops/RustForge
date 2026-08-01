# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

## [0.2.0-alpha] - 2026-08-01

**Phase 8 — verification-focused CI additions:**

- `.github/workflows/ci.yml`: new stable-only `hack` job (`cargo hack test
  --workspace --feature-powerset --exclude-features compile-fail`) —
  the `test` job's fixed feature list only ever builds one combination;
  cargo-hack checks each crate's own feature powerset independently so a
  broken pairing (e.g. `no_std` + `async` together) is caught and
  attributed to the right crate. `compile-fail` excluded for the same
  reason as in the `test` job.
- `.github/workflows/ci.yml`: new nightly `minimal-versions` job (`cargo
  minimal-versions test --workspace --exclude performance-tests`) —
  re-resolves every dependency down to the lowest version each
  `Cargo.toml` constraint allows instead of the newest resolvable one, so
  a `Cargo.toml` requirement looser than what the code actually needs gets
  caught here instead of by a downstream adopter with an older lockfile.
  `performance-tests` excluded for the same reason as in the `msrv` job
  (Criterion's transitive graph floors above what this template's version
  claims are about). CI is now a 12-job pipeline (README/`ci/README.md`
  updated to match).
- `crates/core-tests`: the `no_std_support` module's "usable under
  `#![no_std]`" claim is now enforced by the compiler, not just true by
  convention. Moved its source to its own file
  (`src/no_std_support.rs`) and added `tests/no_std_check.rs`, an
  integration test genuinely marked `#![no_std]` that re-includes that
  file via `#[path]` and calls it — gated behind `required-features =
  ["no_std"]`, matching the existing `[[bench]] required-features =
  ["perf"]` pattern in `performance-tests`. Since `no_std` is already in
  the `test`/`nextest`/`coverage` jobs' feature list, no new CI job was
  needed — those jobs now actually exercise the claim. Runs on the normal
  host target (where `std` is always available) to check the `#![no_std]`
  boundary itself, not cross-compilation to an embedded target — kept out
  of scope for the same reason as the WASM-target deferral above.
- `.github/workflows/ci.yml`: the `coverage` job now passes `--doctests`
  to `cargo llvm-cov`, so the `///` examples throughout the workspace
  count toward the coverage report instead of being silently excluded.
  This requires nightly (rustdoc's doc-test coverage support is gated
  behind the unstable `--persist-doctests` flag), so the job's toolchain
  moved from stable to nightly. `scripts/coverage.sh` updated to match
  (`cargo +nightly llvm-cov ... --doctests`).

**Phase 7 — advanced testing capabilities and CI matrix growth:**

- `crates/semantic-tests`: new `loom` feature — a dedicated `loom_tests`
  module exhaustively explores thread interleavings for the
  `shared_counter_after_workers` pattern via
  [loom](https://github.com/tokio-rs/loom), instead of relying on whichever
  interleaving the OS scheduler happens to pick on a given run. Deliberately
  a *separate*, small reimplementation using `loom::sync`/`loom::thread`
  rather than making the production function itself swap primitives behind
  the feature: an earlier version of this change did exactly that and broke
  `cargo test --workspace --all-features` (what `scripts/check.sh` runs) —
  Cargo's workspace-wide feature unification enables `loom` for every crate
  that depends on `semantic-tests`, including `integration-tests`, which
  calls the real function directly outside any `loom::model` and panics if
  that function is loom-backed. Caught by actually running
  `cargo test --workspace --all-features` before considering this done, not
  just the isolated `--features loom` run. New dedicated stable-only `loom`
  CI job, isolated from the main matrix (same reasoning as `trybuild`: the
  result doesn't depend on toolchain/OS, and repeating exhaustive search 6x
  across the matrix would add cost without adding signal). Documented as a
  sixth test shape in `docs/adding-tests.md`.
- `.github/workflows/ci.yml`: added `ubuntu-24.04-arm` as a stable-only
  extra leg of the `test` job's matrix. `macos-latest` is already Apple
  Silicon, so this closes the one concretely available remaining
  architecture gap (GitHub-hosted Linux ARM) without building a general
  cross-compilation matrix — no Windows-ARM or other targets added.
- **WASM target support: deferred, not shipped.** Evaluated per the
  original enhancement proposal and explicitly not built — there's no
  demonstrated adopter need (only incidental transitive `wasm-bindgen`
  dependencies exist today, pulled in by Criterion's own dependency graph,
  not real usage), and `tokio`/`criterion`/`cargo-fuzz` don't target
  `wasm32-unknown-unknown` anyway, so a build check would cover a narrow
  slice of the template. Revisit if an adopter actually asks for it, rather
  than building speculative infrastructure ahead of a real need — same
  reasoning already applied to declining a hard coverage/mutation-testing
  gate (see "Mutation testing is exploratory, not a gate" in
  `docs/best-practices.md`) and to not integrating a third-party performance
  dashboard.

**Phase 6 — tooling and ecosystem integration:**

- `.github/workflows/ci.yml`: new nightly-only `udeps` job (`cargo udeps
  --workspace --all-features`), informational (`continue-on-error: true`)
  since cargo-udeps analyzes one feature combination at a time and can
  false-positive on feature-gated dependencies. CI is now a 9-job pipeline
  (README/`ci/README.md` updated to match).
- `.github/workflows/ci.yml`: the `coverage` job gained an optional,
  off-by-default coverage-% badge step (`schneegans/dynamic-badges-action`
  writing to a maintainer-owned Gist, read by a shields.io endpoint badge —
  no third-party SaaS account). Skipped entirely unless
  `vars.COVERAGE_GIST_ID`/`secrets.COVERAGE_GIST_SECRET` are configured, so
  a fresh copy of this template has no failing CI job by default. See
  "Coverage badge setup" in `ci/README.md` for the one-time setup steps —
  intentionally left as a manual step since it needs a maintainer-owned
  Gist and token this template can't create on its own.
- `.vscode/settings.json` + `.vscode/extensions.json` (new): rust-analyzer
  configured with `cargo.features = "all"` (matching `scripts/check.sh`'s
  local-dev convention) and `check.command = "clippy"` so inline diagnostics
  match what CI actually enforces; recommends the rust-analyzer and
  even-better-toml extensions.
- `cargo-generate.toml` (new) + a `docs/adoption.md` callout: makes
  `cargo generate --git <repo>` work as a fresh-git-history alternative to
  manually cloning and deleting `.git`. No templating/renaming — RustForge's
  crate names are load-bearing (matched against real feature flags), not
  project-specific placeholders.

**Phase 5 — documentation, discoverability, and lint consolidation:**

- `Cargo.toml`: `[workspace.lints.rust]` (`missing_docs = "warn"`,
  `unsafe_code = "forbid"`), adopted by every crate via `[lints] workspace =
  true`, replacing the previously duplicated `#![forbid(unsafe_code)]` /
  `#![warn(missing_docs)]` attributes in each crate's `lib.rs`. One source
  of truth for lint policy instead of seven; CI's `-D warnings` flag is
  still what turns it into a hard failure.
- `docs/adding-tests.md`: a mermaid decision-tree flowchart next to the
  existing "Where does my test go?" table, for branching instead of
  scanning when picking a test category.
- `docs/adoption.md`: new "Migrating an existing project into RustForge's
  structure" section covering the *reverse* direction — starting from plain
  `cargo test`/`#[cfg(test)]` modules and sorting them into RustForge's
  category crates. The existing pitfalls section only covered merging
  RustForge's crates into an existing workspace.
- `CONTRIBUTING.md`: new "Test naming and issue linking" section (naming
  pattern, referencing issue numbers, minimal reproducers) and an updated
  "Style" section pointing at the new workspace lints table instead of the
  removed per-crate attributes.
- `docs/fuzzing.md`: new "Differential fuzzing" and "Corpus sharing"
  sections — comparing two implementations for divergent behavior, and
  conventions for importing/promoting externally-sourced corpus inputs.
- `justfile`: `full-ci` (chains fmt-check, clippy-all, test-all, nextest,
  doc-check, and cargo-deny if installed — everything CI runs, in one local
  command), `release-dry-run` (release build across every optional feature;
  no publish step, since every crate is `publish = false`), and `mutants`
  (wraps `cargo-mutants`, local-only and exploratory, never wired into CI).
- `docs/best-practices.md`: updated the `unsafe`/`missing_docs` sections to
  reference the new workspace lints table, added "Mutation testing is
  exploratory, not a gate" (mirrors the `coverage` job's informational-only
  reasoning), and added a "`cargo-semver-checks` doesn't apply here" section
  explaining why it's not part of this template (every crate is `publish =
  false`).

## [0.1.0-alpha] - 2026-07-28

First tagged release. Everything below accumulated across four build-out
passes on top of the initial scaffold (see "[0.1.0] - 2026-01-01" at the
bottom — that entry predates any git tag; nothing was ever actually
published before this). **Alpha**: the template's structure, feature
flags, and CI pipeline are exercised and verified (see each phase's PR for
what was actually run, not just written), but this is the first time
they've all shipped together as a tagged unit — expect some rough edges
from real-world adoption that internal verification can't fully surface.

### Added

- README: `## Why RustForge?` section (pitch + "who it's for") replacing
  the old `## Features` list, integrated from a draft after fact-checking
  every claim against the repo and fixing an unrelated staleness the
  review surfaced (the old section still claimed a "7-job CI pipeline";
  it's 8 now). Cross-links to `Tooling & Automation` instead of
  re-enumerating CI jobs, so the two can't drift apart silently again.

**Phase 4 — advanced and polish:**

- `syntax-tests`: `snapshot` feature (`insta`) with `SourceSummary`/
  `summarize_source`, snapshot-tested — the pattern for when a type has
  too many fields for per-field assertions to stay readable. No MSRV pin
  needed (insta's own MSRV is 1.66, under the workspace's 1.75 floor).
  `snapshot` is included in the main CI `test` job's feature list (unlike
  `compile-fail`) since snapshot text doesn't depend on the compiler.
- `docs/fuzzing.md`: corpus minimization (`cargo fuzz cmin`), crash-case
  minimization (`cargo fuzz tmin`), coverage-guided sanity checks
  (`cargo fuzz coverage`), reading ASan output, and running a real
  (minutes, not seconds) campaign locally.
- `docs/performance-regression-testing.md`: Criterion's built-in
  statistical baseline comparison (`--save-baseline`/`--baseline`),
  verified against a real deliberately-introduced regression (a
  `thread::sleep` added to `sum` and immediately reverted) — Criterion
  correctly flagged "Performance has regressed" with `p = 0.00`. Also
  documents a real footgun found while verifying this: `--quick` gave a
  **false negative** ("No change in performance detected", `p = 0.10`) on
  that same, obvious regression — don't use `--quick` for real comparisons.
- `fuzz-build` CI job now runs an extended (4-minute-per-target) fuzzing
  campaign on the daily `schedule` trigger only, in addition to the
  existing 10-second push/PR smoke-run.
- `justfile`: `bench-baseline`/`bench-compare` recipes, pinned to
  `--bench sum_bench --bench collection_ops_bench` rather than `--benches`
  — Cargo's `bench` manifest field defaults to `true` for the library
  target too, so `--benches` also invokes the crate's unit-test binary,
  which then fails on Criterion-only flags like `--save-baseline` with
  "Unrecognized option". Found by actually running the recipe, not assumed.
- `*.snap.new` (insta's pending-review files) added to `.gitignore`.

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
