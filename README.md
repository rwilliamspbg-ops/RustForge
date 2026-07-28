# RustForge

[![CI](https://github.com/rwilliamspbg-ops/RustForge/actions/workflows/ci.yml/badge.svg)](https://github.com/rwilliamspbg-ops/RustForge/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

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

4. Opt into advanced categories (`performance-tests`, `fuzz-tests`, `edge-cases`) and their
   feature flags (`perf`, `fuzz`, `edge`) as needed — see [Feature Flags](#feature-flags) below.

## Module Responsibilities

- `core-tests`: shared fixtures/helpers for reusable assertions and builders.
- `syntax-tests`: parser/syntax-facing tests (compile-fail/pass tools can be layered in).
- `semantic-tests`: ownership, borrowing, traits, and async semantics.
- `performance-tests`: benchmark/perf guard entry points.
- `fuzz-tests`: fuzz harness-friendly entry points.
- `integration-tests`: end-to-end behavior tests across categories.
- `edge-cases`: boundary-value and robustness checks.

## Feature Flags

Optional, dependency-pulling tooling is gated behind Cargo features so
`cargo test --workspace` stays fast by default. Opt in per crate or with
`--all-features`:

| Crate | Feature | Adds | What it unlocks |
| --- | --- | --- | --- |
| `core-tests` | `async` | `tokio` | async fixture helpers (`async_support::default_user_fixture_async`, concurrent fixture loading) |
| `core-tests` | `no_std` | — | `core`-only helper module (`no_std_support`) |
| `semantic-tests` | `async` | `tokio` | an async ownership test (`cargo test -p semantic-tests --features async`) |
| `performance-tests` | `perf` | `criterion` | `cargo bench -p performance-tests --features perf` |
| `fuzz-tests` | `fuzz` | `proptest` | property tests (`cargo test -p fuzz-tests --features fuzz`) |
| `edge-cases` | `edge` | — | checked-arithmetic boundary helpers (`overflow_checks`) guarding `usize` under/overflow |

The real `cargo-fuzz` scaffold lives in [`fuzz/`](fuzz), which is a detached
workspace (see [`fuzz/README.md`](fuzz/README.md)) since fuzzing needs
nightly and its own dependency resolution. Build it with:

```bash
cd fuzz && cargo +nightly fuzz build
```

### MSRV

The workspace declares `rust-version = "1.75"` — that's the floor for the
**default** build of every crate (no extra features). Optional features that
pull in actively-developed ecosystem tooling can need a newer toolchain
independently of this template, since those crates set their own MSRV:

- `fuzz-tests`'s `fuzz` feature pins `proptest` to the `~1.8` line
  specifically to stay within 1.75 (proptest 1.9+ needs rustc 1.82+).
- `performance-tests`'s `perf` feature depends on `criterion`, whose own
  dependency chain (`clap`, `regex`, `plotters`, ...) tracks current stable
  Rust and can exceed 1.75. Use a recent stable toolchain when running
  `cargo bench`.

CI's `test` job runs on stable/beta/nightly with `--all-features`, so it
will surface any future MSRV drift on those toolchains even though it
doesn't pin to 1.75 explicitly.

## Tooling & Automation

- Native `cargo test` is first-class.
- CI workflow includes `fmt`, `clippy`, and workspace tests on stable/beta/nightly, plus a nightly `fuzz-build` job. See [`ci/README.md`](ci/README.md).
- `scripts/check.sh` mirrors the CI gate locally; `scripts/coverage.sh` generates an HTML coverage report. See [`scripts/README.md`](scripts/README.md).
- Runnable examples live under each crate's `examples/` directory, e.g. `cargo run -p core-tests --example fixture_walkthrough`. See [`examples/README.md`](examples/README.md).
- Optional ecosystem tools are layered in incrementally behind features above (`criterion`, `proptest`, `cargo-fuzz`, `tokio`); add `cargo-nextest` or `trybuild` the same way as your suite grows.

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

## Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the contributor checklist and
guidance on adding new test categories. Released under the [MIT license](LICENSE).
