# RustForge

RustForge is a modular, easily adoptable Rust test-suite template that scales from basic `cargo test` workflows to compiler-style coverage.

## Workspace Layout

```text
my-test-suite/
├── Cargo.toml
├── crates/
│   ├── core-tests/
│   ├── syntax-tests/
│   ├── semantic-tests/
│   ├── performance-tests/
│   ├── fuzz-tests/
│   ├── integration-tests/
│   └── edge-cases/
├── tests/
├── examples/
├── fuzz/
├── ci/
├── scripts/
└── docs/
```

## Quickstart (Plug-and-Play)

1. Use this repo as a template, or copy `Cargo.toml` + `crates/` into your existing workspace.
2. Start with core categories (`syntax`, `semantic`, `integration`).
3. Run:

```bash
cargo test --workspace
```

4. Opt into advanced categories (`performance`, `fuzz`, `edge-cases`) as needed.

## Module Responsibilities

- `core-tests`: shared fixtures/helpers for reusable assertions and builders.
- `syntax-tests`: parser/syntax-facing tests (compile-fail/pass tools can be layered in).
- `semantic-tests`: ownership, borrowing, traits, and async semantics.
- `performance-tests`: benchmark/perf guard entry points.
- `fuzz-tests`: fuzz harness-friendly entry points.
- `integration-tests`: end-to-end behavior tests across categories.
- `edge-cases`: boundary-value and robustness checks.

## Tooling & Automation

- Native `cargo test` is first-class.
- CI workflow includes `fmt`, `clippy`, and workspace tests on stable/beta/nightly.
- Optional ecosystem tools can be layered in incrementally (`cargo-nextest`, `trybuild`, `criterion`, `cargo-fuzz`, `cargo-llvm-cov`).

## Coverage, Reporting, and Debugging

Recommended defaults:

- Coverage: install first with `cargo install cargo-llvm-cov`, then run `cargo llvm-cov --workspace --html`
- Test output: `cargo test -- --nocapture`
- CI artifacts: logs, reproducers, and coverage reports on failures

## Adoption Roadmap

1. Foundation: keep workspace + shared helpers.
2. Categories: grow syntax/semantic coverage first.
3. Automation: enforce CI and coverage gates.
4. Advanced: add fuzz/property/perf regression suites.
5. Polish: expand docs/examples and maintain contributor checklist.

See `docs/adoption.md` for incremental rollout guidance.
