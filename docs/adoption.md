# Adoption Guide

1. Copy this repository as a template or add the crates under `crates/` into your workspace.
2. Keep `core-tests` as the shared helper layer.
3. Enable only the categories you need first (`syntax`, `semantic`, `integration`).
4. Run `cargo test --workspace` and incrementally add categories (`perf`, `fuzz`, `edge`).
