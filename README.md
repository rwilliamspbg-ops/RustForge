# RustForge

[![CI](https://github.com/rwilliamspbg-ops/RustForge/actions/workflows/ci.yml/badge.svg)](https://github.com/rwilliamspbg-ops/RustForge/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![MSRV: 1.75](https://img.shields.io/badge/MSRV-1.75-blue.svg)](#msrv)

RustForge is a modular, easily adoptable Rust test-suite template that scales from basic `cargo test` workflows to compiler-style coverage.

## Why RustForge?

Most Rust projects eventually build their own test infrastructure —
stitching together `cargo test`, Criterion, `proptest`, `cargo-fuzz`,
`trybuild`, `insta`, `cargo-llvm-cov`, and the CI wiring to run all of it
consistently. RustForge is that infrastructure, already built, already
wired into CI, and verified to actually work end-to-end — a starting
point, not a skeleton you fill in yourself.

### What makes it different

- **Truly modular** — seven independent category crates (`syntax`,
  `semantic`, `performance`, `fuzz`, `integration`, `edge-cases`, plus
  shared `core-tests`); adopt only what you need. `cargo test --workspace`
  pulls in zero extra dependencies by default — heavier tooling (Tokio,
  Criterion, `proptest`, `trybuild`, `insta`) is opt-in per feature flag,
  see [Feature Flags](#feature-flags).
- **Comprehensive by design** — syntax and compile-fail tests, ownership/
  semantic correctness, Criterion benchmarks with regression guards (plus
  [statistical baseline comparison](docs/performance-regression-testing.md)),
  fuzzing (in-process `proptest` and real `cargo-fuzz` targets, with a
  [corpus management guide](docs/fuzzing.md)), snapshot testing, boundary/
  edge-case checks, and cross-category integration tests. Every one of
  these actually runs in CI, not just in a README example.
- **Production-grade from day one** — multi-toolchain, multi-OS CI; MSRV
  verified by a dedicated job, not just claimed; `cargo-deny` supply-chain
  checks; Dependabot for both the main and detached `fuzz/` workspaces;
  `#![warn(missing_docs)]` enforced as a hard CI error; issue/PR templates
  and a contributor checklist. See [Tooling & Automation](#tooling--automation)
  for the full list.
- **Easy to adopt, no lock-in** — a template, not a framework. Use it as a
  GitHub template repo, copy `crates/` into an existing workspace (see the
  Quickstart above for the one real gotcha we found by actually testing
  that flow), or take individual crates — most depend only on shared
  `core-tests`, nothing else.
- **Scales with you** — start with `syntax`/`semantic`/`integration` on a
  small library; grow into fuzzing, property-based testing, and
  regression-guarded benchmarks as the project's rigor requirements grow.

### Who it's for

- Library authors who want test coverage adopters can trust
- Teams building production Rust services who'd rather not assemble this
  tooling from scratch
- Anyone who'd rather start from a working, CI-verified baseline than
  bootstrap one

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
2. **If merging into an existing workspace**, make sure its root `Cargo.toml`
   has a `[workspace.package]` table with `edition`, `rust-version`, and
   `license` — every crate here inherits those via `.workspace = true`, and
   `cargo test` fails immediately (`workspace.package.edition was not
   defined`) without it. See "Common Pitfalls" in
   [`docs/adoption.md`](docs/adoption.md#common-pitfalls) — this is the #1
   thing that trips up merging into an existing project, confirmed by
   actually testing the merge flow, not just assumed.
3. Start with core categories (`syntax`, `semantic`, `integration`).
4. Run:

```bash
cargo test --workspace
```

5. Opt into advanced categories (`performance-tests`, `fuzz-tests`, `edge-cases`) and their
   feature flags (`perf`, `fuzz`, `edge`) as needed — see [Feature Flags](#feature-flags) below.

## Module Responsibilities

- `core-tests`: shared fixtures/helpers for reusable assertions and builders.
- `syntax-tests`: parser/syntax-facing tests — compile-fail/pass (`trybuild`) and snapshot (`insta`) tests included.
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
| `syntax-tests` | `compile-fail` | `trybuild` | UI/compile-fail tests (`cargo test -p syntax-tests --features compile-fail`) — CI-pinned to stable only, see [`ci/README.md`](ci/README.md) |
| `syntax-tests` | `snapshot` | `insta` | struct-shaped snapshot tests (`cargo test -p syntax-tests --features snapshot`); update with `cargo insta review` |

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
- `syntax-tests`'s `compile-fail` feature pins `trybuild` to `=1.0.111`
  for the same reason (1.0.112+ needs rustc 1.76+).
- `performance-tests`'s `perf` feature depends on `criterion`, whose own
  dependency chain (`clap`, `regex`, `plotters`, ...) tracks current stable
  Rust and can exceed 1.75. Use a recent stable toolchain when running
  `cargo bench`.

CI's `test` job runs on stable/beta/nightly with every feature except
`compile-fail` (see [`ci/README.md`](ci/README.md) for why), so it will
surface any future MSRV drift on those toolchains. A dedicated `msrv` job
verifies the default build against Rust 1.75 itself, rather than just
asserting the promise in docs.

## Tooling & Automation

- Native `cargo test` is first-class.
- CI is an 8-job pipeline: `fmt`+`clippy`+tests across a stable/beta/nightly
  × ubuntu/windows/macos matrix, a stable-only `nextest` job, a stable-only
  `trybuild` job, a nightly `fuzz-build` job (build + short smoke run per
  target on every push/PR, plus a longer campaign on the daily schedule —
  see [`docs/fuzzing.md`](docs/fuzzing.md)), a stable-only `coverage` job,
  an MSRV job, a `cargo-deny` supply-chain job, and a stable-only `docs`
  job. See
  [`ci/README.md`](ci/README.md) for what each one does and why it's
  scoped the way it is.
- [`justfile`](justfile) — `just check`, `just test-all`, `just bench`,
  `just fuzz-run <target>`, `just coverage`, `just doc`, and more; run
  `just --list` for the full set. `scripts/check.sh` and
  `scripts/coverage.sh` (which the `check`/`coverage` recipes call) work
  standalone too, no `just` required. See [`scripts/README.md`](scripts/README.md).
- Runnable examples live under each crate's `examples/` directory, e.g. `cargo run -p core-tests --example fixture_walkthrough`. See [`examples/README.md`](examples/README.md).
- Optional ecosystem tools are layered in incrementally behind the features
  above (`criterion`, `proptest`, `trybuild`, `cargo-fuzz`, `tokio`) or as
  standalone dev tools (`cargo-nextest`, `cargo-llvm-cov`, `cargo-deny`,
  `just`) — each one is opt-in and documented where it's used, not a
  hard requirement to run `cargo test --workspace`.
- Dependency hygiene: [`deny.toml`](deny.toml) (licenses/advisories/bans/sources, checked in CI) and [`.github/dependabot.yml`](.github/dependabot.yml) (weekly update PRs for both the main workspace and the detached `fuzz/` workspace).

## Coverage, Reporting, and Debugging

Recommended defaults:

- Coverage: `scripts/coverage.sh` or `just coverage` (installs guidance
  included); or manually: `rustup component add llvm-tools-preview &&
  cargo install cargo-llvm-cov --locked`, then `cargo llvm-cov --workspace
  --all-features --html`. Also runs in CI as a downloadable artifact — see
  the `coverage` job.
- Test output: `cargo test -- --nocapture`
- Alternative runner: `cargo nextest run --workspace` (install:
  `cargo install cargo-nextest --locked`) — faster on larger suites, one
  process per test; doesn't run doc-tests, so it complements rather than
  replaces `cargo test`.
- Performance regressions: `just bench-baseline` then `just bench-compare`
  for a statistically-grounded before/after comparison — see
  [`docs/performance-regression-testing.md`](docs/performance-regression-testing.md).
- Snapshot testing: `cargo insta review` for `syntax-tests`' `snapshot`
  feature — see "Snapshot test" in [`docs/adding-tests.md`](docs/adding-tests.md).
- Fuzzing beyond the CI smoke-run: corpus/crash minimization, coverage,
  reading ASan output — see [`docs/fuzzing.md`](docs/fuzzing.md).
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
guidance on adding new test categories.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at
your option — the convention most of the Rust ecosystem uses. Unless you
explicitly state otherwise, any contribution intentionally submitted for
inclusion, as defined in the Apache-2.0 license, shall be dual-licensed as
above without any additional terms or conditions.
