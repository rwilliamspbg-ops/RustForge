Automation scripts (coverage, snapshot bless, and maintenance helpers) should live here.

- `check.sh` — runs the same `fmt` + `clippy` + `test` gate CI enforces,
  locally and with `--all-features` so feature-gated code is checked too.
- `coverage.sh` — runs `cargo llvm-cov --workspace --all-features --html`
  and writes a report to `target/llvm-cov/html/index.html`. Requires
  `cargo install cargo-llvm-cov` once per machine.
