CI helpers and notes for running fmt, clippy, tests, and optional nightly jobs.

## Workflow: `.github/workflows/ci.yml`

- **`test` job** — runs on stable/beta/nightly, all on ubuntu, plus stable
  on windows and macos: `cargo fmt --all --check`, then clippy/tests with
  `--features async,no_std,perf,fuzz,edge` (every feature *except*
  `compile-fail`) so the Tokio-backed async test, the proptest property
  tests, and the Criterion bench code path are all checked, not just the
  defaults. `compile-fail` is deliberately excluded here — see the
  `trybuild` job.
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
- **`coverage` job** — runs on stable only: `cargo llvm-cov --workspace
  --all-features --html`, uploaded as a build artifact. Informational —
  doesn't gate merges on a threshold; see `scripts/coverage.sh` to run the
  same thing locally.
- **`msrv` job** — installs Rust 1.75 (the workspace's declared
  `rust-version`) and runs `cargo check`/`cargo test` with default features
  only, verifying the MSRV promise actually holds instead of just asserting
  it in docs. Doesn't use `--locked`: the committed `Cargo.lock` is lockfile
  format v4, which needs Cargo >= 1.78 to read. `performance-tests` is
  excluded from the `cargo test` step — see the comment in the workflow
  file for why.
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
