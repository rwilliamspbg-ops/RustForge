# Adoption Guide

1. Copy this repository as a template or add the crates under `crates/` into your workspace.
2. Keep `core-tests` as the shared helper layer.
3. Enable only the categories you need first (`syntax`, `semantic`, `integration`).
4. Run `cargo test --workspace` and incrementally add categories (`perf`, `fuzz`, `edge`).
5. Reach for the dependency-pulling feature flags (`async`, `perf`, `fuzz`) only
   when a category needs them — see the feature table in the root `README.md`.
   Running `cargo test --workspace --all-features` locally (or `scripts/check.sh`)
   exercises everything at once, matching what CI does.
6. When a category needs real property-based fuzzing beyond in-process
   `proptest` checks, build on the `fuzz/` scaffold with `cargo-fuzz`
   (nightly-only, detached from the main workspace).
7. Use `scripts/coverage.sh` (needs `cargo-llvm-cov`) to spot untested paths
   before adding new categories or expanding existing ones.
