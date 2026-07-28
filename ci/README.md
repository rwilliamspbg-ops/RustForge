CI helpers and notes for running fmt, clippy, tests, and optional nightly jobs.

## Workflow: `.github/workflows/ci.yml`

- **`test` job** — runs on stable/beta/nightly: `cargo fmt --all --check`,
  then clippy/tests with `--features async,no_std,perf,fuzz,edge` (every
  feature *except* `compile-fail`) so the Tokio-backed async test, the
  proptest property tests, and the Criterion bench code path are all
  checked, not just the defaults. `compile-fail` is deliberately excluded
  here — see the `trybuild` job.
- **`trybuild` job** — runs on stable only: `cargo test -p syntax-tests
  --features compile-fail`. Isolated from the `test` job's stable/beta/
  nightly matrix because trybuild's UI tests assert on `rustc`'s exact
  diagnostic text (`tests/ui/fail/*.stderr` in `syntax-tests`), which can
  drift between toolchain channels — pinning to stable keeps it stable to
  run, pun intended.
- **`fuzz-build` job** — runs on nightly only: installs `cargo-fuzz` and
  builds the targets under `fuzz/` (`cargo fuzz build`). `fuzz/` is a
  detached workspace (see its own `[workspace]` table), so it's never part
  of the main `cargo test --workspace` run.
- **`msrv` job** — installs Rust 1.75 (the workspace's declared
  `rust-version`) and runs `cargo check`/`cargo test` with default features
  only, verifying the MSRV promise actually holds instead of just asserting
  it in docs. Doesn't use `--locked`: the committed `Cargo.lock` is lockfile
  format v4, which needs Cargo >= 1.78 to read.
- **`deny` job** — runs `cargo-deny check` (licenses, security advisories,
  banned/duplicate dependencies, untrusted sources) against both the main
  workspace and the detached `fuzz/` workspace. Config lives in
  [`deny.toml`](../deny.toml).

## Running the same checks locally

```bash
scripts/check.sh
```

## Coverage

```bash
scripts/coverage.sh
```
