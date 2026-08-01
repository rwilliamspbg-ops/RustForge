CI helpers and notes for running fmt, clippy, tests, and optional nightly jobs.

## Workflow: `.github/workflows/ci.yml`

- **`test` job** — runs on stable/beta/nightly, all on ubuntu, plus stable
  on windows, macos, and `ubuntu-24.04-arm` (Linux ARM; macOS runners are
  already Apple Silicon): `cargo fmt --all --check`, then clippy/tests with
  `--features async,no_std,perf,fuzz,edge` (every feature *except*
  `compile-fail`) so the Tokio-backed async test, the proptest property
  tests, and the Criterion bench code path are all checked, not just the
  defaults. `compile-fail` is deliberately excluded here — see the
  `trybuild` job. The `no_std` feature also enables `core-tests`'
  `tests/no_std_check.rs`, a genuinely `#![no_std]`-marked integration test
  — so this job (and `nextest`/`coverage`, which enable the same feature)
  actually verifies the "usable under `#![no_std]`" claim on
  `no_std_support`, not just the code review/convention it used to rely on.
- **`nextest` job** — runs on stable only: `cargo nextest run --workspace
  --features async,no_std,perf,fuzz,edge`. Demonstrates
  [cargo-nextest](https://nexte.st/) as an opt-in alternative runner;
  `cargo test` stays the primary, dependency-free supported path (see
  `CONTRIBUTING.md`). Note nextest doesn't run doc-tests — a real nextest
  limitation, not something to fix here.
- **`trybuild` job** — runs on stable only: `cargo test -p syntax-tests
  --features compile-fail`. Isolated from the `test` job's multi-toolchain
  matrix because trybuild's UI tests assert on `rustc`'s exact diagnostic
  text (`tests/ui/fail/*.stderr` in `syntax-tests`), which can drift
  between toolchain channels — pinning to stable keeps it stable to run,
  pun intended.
- **`loom` job** — runs on stable only: `cargo test -p semantic-tests
  --features loom`, exhaustively exploring thread interleavings for
  `shared_counter_after_workers` via [loom](https://github.com/tokio-rs/loom)
  instead of relying on whatever interleaving the OS scheduler happens to
  pick. Isolated from the `test` matrix for the same reason as `trybuild`:
  the result doesn't depend on toolchain/OS, and exhaustive search is slower
  than a normal test run, so repeating it 6x across the matrix would add
  cost without adding signal. See "Concurrency-permutation test" in
  [`docs/adding-tests.md`](../docs/adding-tests.md).
- **`fuzz-build` job** — runs on nightly only: installs `cargo-fuzz`,
  builds the targets under `fuzz/` (`cargo fuzz build`), then does a short
  (10-second) smoke run of each target seeded from the committed
  `fuzz/seed_corpus/<target>/` — enough to catch a broken harness (panics
  immediately), not a real fuzzing campaign. On the daily `schedule`
  trigger only (not on push/PR), it additionally runs a longer (4-minute
  per target) campaign — see [`docs/fuzzing.md`](../docs/fuzzing.md) for
  running an even longer one locally, plus corpus/crash-minimization and
  coverage tooling. `fuzz/` is a detached workspace (see its own
  `[workspace]` table), so it's never part of the main `cargo test
  --workspace` run.
- **`coverage` job** — runs on nightly (needed for `--doctests`, below):
  `cargo llvm-cov --workspace --all-features --doctests --html`, uploaded
  as a build artifact. `--doctests` counts the `///` examples toward
  coverage too, not just `#[test]`s — it's implemented via rustdoc's
  unstable `--persist-doctests`, which only the nightly compiler accepts.
  Informational — doesn't gate merges on a threshold; see
  `scripts/coverage.sh` to run the same thing locally. Also has an
  optional, off-by-default coverage % badge step — see "Coverage badge
  setup" below.
- **`udeps` job** — runs on nightly only: `cargo udeps --workspace
  --all-features`, checking for unused dependencies. Informational —
  `continue-on-error: true` on the check step, since cargo-udeps analyzes
  one feature combination at a time and a dependency only used behind a
  different feature selection can look unused (a false positive, not a real
  finding); review its output manually rather than treating a red run as a
  failure.
- **`msrv` job** — installs Rust 1.75 (the workspace's declared
  `rust-version`) and runs `cargo check`/`cargo test` with default features
  only, verifying the MSRV promise actually holds instead of just asserting
  it in docs. Doesn't use `--locked`: the committed `Cargo.lock` is lockfile
  format v4, which needs Cargo >= 1.78 to read. `performance-tests` is
  excluded from the `cargo test` step — see the comment in the workflow
  file for why.
- **`hack` job** — runs on stable only: `cargo hack test --workspace
  --feature-powerset --exclude-features compile-fail`. The `test` job's
  fixed `--features async,no_std,perf,fuzz,edge,snapshot` list only ever
  builds *one* combination; a pairing it never selects (e.g. `no_std` +
  `async` together) can still be broken. cargo-hack checks each crate's own
  feature powerset independently, so a failure is attributed to the crate
  that actually has the broken combination. `compile-fail` is excluded for
  the same reason as in the `test` job — see the `trybuild` job above.
- **`minimal-versions` job** — runs on nightly (needed for the `-Z
  minimal-versions` resolution step `cargo minimal-versions` performs
  internally): `cargo minimal-versions test --workspace --exclude
  performance-tests`. Re-resolves every dependency down to the *lowest*
  version each `Cargo.toml` constraint allows, instead of the newest
  semver-compatible one `cargo test` would normally pick — catches a
  `Cargo.toml` requirement that's looser than what the code actually needs.
  `performance-tests` is excluded for the same reason as in the `msrv` job:
  Criterion's own transitive graph floors well above what this template's
  version claims are about.
- **`deny` job** — runs `cargo-deny check` (licenses, security advisories,
  banned/duplicate dependencies, untrusted sources) against both the main
  workspace and the detached `fuzz/` workspace. Config lives in
  [`deny.toml`](../deny.toml).
- **`docs` job** — runs on stable only: `cargo doc --workspace
  --all-features --no-deps` with `RUSTDOCFLAGS=-D warnings`, catching
  broken intra-doc links and other rustdoc-only lints. Added after a
  broken `` [`slice::dedup`] `` link shipped uncaught — clippy's own
  `-D warnings` doesn't cover rustdoc's separate lint pass.

## Running the same checks locally

```bash
scripts/check.sh
```

Or, with [`just`](https://github.com/casey/just) installed: `just check`.
See the [`justfile`](../justfile) for the full set of recipes (`just
--list`) — nextest, benchmarks, fuzz targets, coverage, docs, and more.

## Coverage

```bash
scripts/coverage.sh
```

## Coverage badge setup (optional, one-time)

The `coverage` job can maintain a live coverage-% badge in `README.md`
without a third-party SaaS account (Codecov, Coveralls) — it writes the
percentage to a GitHub Gist via
[`schneegans/dynamic-badges-action`](https://github.com/Schneegans/dynamic-badges-action),
and a [shields.io](https://shields.io) endpoint badge reads that gist. The
badge step is skipped entirely until this is configured, so a fresh copy of
this template has no broken/red badge and no failing CI job by default. To
enable it:

1. Create a new GitHub Gist (any content — it gets overwritten) and note its
   ID (the hex string in its URL).
2. Create a fine-grained GitHub personal access token scoped only to that
   gist (`gist` scope is enough).
3. In this repo's Settings → Secrets and variables → Actions: add repo
   **variable** `COVERAGE_GIST_ID` (the gist ID — not sensitive) and repo
   **secret** `COVERAGE_GIST_SECRET` (the token — sensitive).
4. Push to `main` once; the `coverage` job's badge step will populate the
   gist.
5. Add this to the badge row at the top of `README.md`, replacing
   `<username>`/`<gist-id>`:

   ```markdown
   [![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/<username>/<gist-id>/raw/rustforge-coverage.json)](https://github.com/rwilliamspbg-ops/RustForge/actions/workflows/ci.yml)
   ```
