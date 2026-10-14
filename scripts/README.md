Automation scripts (coverage, snapshot bless, and maintenance helpers) should live here.

- `check.sh` — runs the same `fmt` + `clippy` + `test` gate CI enforces,
  locally and with `--all-features` so feature-gated code is checked too.
- `coverage.sh` — runs `cargo llvm-cov --workspace --all-features --html`
  and writes a report to `target/llvm-cov/html/index.html`. Requires
  `cargo install cargo-llvm-cov` once per machine.
- `adoption-check.sh` — verifies the adoption promise end-to-end: runs
  `cargo test --workspace` in `cargo generate --path .` output (leg skipped
  if `cargo-generate` isn't installed) and in a synthetic host workspace
  with `crates/` copied in. Mirrors CI's `adoption` job; `just adopt-test`
  is the wrapper.
- `eval-harness.sh` — evaluates a candidate crate against the fixed
  evaluation harness (`examples/eval-harness/`): copies both into a
  scratch workspace, wires them together, runs the golden + property +
  compile-fail suite. `just eval <path>` is the wrapper; CI's `adoption`
  job runs it against `examples/sample-candidate`. See
  `docs/evaluation-harness.md`.
