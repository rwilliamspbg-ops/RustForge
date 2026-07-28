CI helpers and notes for running fmt, clippy, tests, and optional nightly jobs.

## Workflow: `.github/workflows/ci.yml`

- **`test` job** — runs on stable/beta/nightly: `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`,
  `cargo test --workspace --all-features`. `--all-features` ensures the
  Tokio-backed async test, the proptest property tests, and the Criterion
  bench code path are all checked, not just the defaults.
- **`fuzz-build` job** — runs on nightly only: installs `cargo-fuzz` and
  builds the targets under `fuzz/` (`cargo fuzz build`). `fuzz/` is a
  detached workspace (see its own `[workspace]` table), so it's never part
  of the main `cargo test --workspace` run.

## Running the same checks locally

```bash
scripts/check.sh
```

## Coverage

```bash
scripts/coverage.sh
```
