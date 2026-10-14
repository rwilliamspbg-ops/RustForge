# Common commands for working on RustForge. Install `just`:
# https://github.com/casey/just — then run `just` to list recipes.

# List available recipes.
default:
    @just --list

# Mirror the CI gate locally (fmt, clippy, tests, and cargo-deny if installed).
check:
    ./scripts/check.sh

# Format the whole workspace.
fmt:
    cargo fmt --all

# Check formatting without modifying files.
fmt-check:
    cargo fmt --all --check

# Lint the default feature set.
clippy:
    cargo clippy --workspace --all-targets -- -D warnings

# Lint every feature except compile-fail (matches CI's `test` job).
clippy-all:
    cargo clippy --workspace --all-targets --features async,no_std,perf,fuzz,edge,snapshot -- -D warnings

# Run the default test suite.
test:
    cargo test --workspace

# Run every feature except compile-fail (matches CI's `test` job).
test-all:
    cargo test --workspace --features async,no_std,perf,fuzz,edge,snapshot

# Run trybuild's compile-fail/UI tests. Run on stable — see ci/README.md.
test-compile-fail:
    cargo test -p syntax-tests --features compile-fail

# Run the suite under Miri for undefined-behavior detection (interprets the
# program, catching UB and data races ordinary tests miss). Requires the
# nightly toolchain with the miri component:
# rustup toolchain install nightly --component miri.
# Scoped to core-tests + semantic-tests with default features — see the
# comment on CI's `miri` job in .github/workflows/ci.yml for why.
miri:
    cargo +nightly miri test -p core-tests -p semantic-tests

# Run the workspace under cargo-nextest instead of `cargo test` (install: cargo install cargo-nextest --locked).
nextest:
    cargo nextest run --workspace --features async,no_std,perf,fuzz,edge,snapshot

# Run Criterion benchmarks (targets named explicitly — see bench-baseline for why).
bench:
    cargo bench -p performance-tests --features perf --bench sum_bench --bench collection_ops_bench

# Save a Criterion baseline named "main" — run this before your change.
bench-baseline:
    cargo bench -p performance-tests --features perf --bench sum_bench --bench collection_ops_bench -- --save-baseline main

# Compare current code against the "main" baseline (see docs/performance-regression-testing.md).
bench-compare:
    cargo bench -p performance-tests --features perf --bench sum_bench --bench collection_ops_bench -- --baseline main

# Type-check the detached fuzz/ workspace (doesn't require nightly).
fuzz-check:
    cd fuzz && cargo check

# Build fuzz targets. Requires nightly: rustup toolchain install nightly
fuzz-build:
    cd fuzz && cargo +nightly fuzz build

# Run one fuzz target with its seed corpus, e.g. `just fuzz-run utf8_input` (requires nightly).
fuzz-run target:
    cd fuzz && cargo +nightly fuzz run {{target}} seed_corpus/{{target}}

# Feature-powerset check per crate, matching CI's `hack` job (install: cargo install cargo-hack --locked).
hack:
    cargo hack test --workspace --feature-powerset --exclude-features compile-fail

# Test against the lowest versions every Cargo.toml constraint allows, matching CI's `minimal-versions` job.
# Needs nightly for resolution (install: cargo install cargo-hack cargo-minimal-versions --locked).
minimal-versions:
    cargo minimal-versions test --workspace --exclude performance-tests

# Check licenses/advisories/bans/sources for both workspaces (install: cargo install cargo-deny --locked).
deny:
    cargo deny check
    cd fuzz && cargo deny check

# Evaluate an external implementation against the fixed eval harness —
# golden + property + compile-fail suite against a candidate crate.
# Usage: just eval examples/sample-candidate [extra cargo test args]
# See docs/evaluation-harness.md.
eval candidate *args:
    ./scripts/eval-harness.sh {{candidate}} {{args}}

# Generate an HTML coverage report (install: rustup component add llvm-tools-preview && cargo install cargo-llvm-cov --locked).
coverage:
    ./scripts/coverage.sh

# LCOV coverage report for machine consumption — the standard format for
# coverage services (codecov, coveralls) and programmatic diffing; see
# docs/machine-readable-outputs.md (same install as `coverage`).
coverage-lcov:
    cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info

# Machine-readable test results — nextest's JSON event stream, one object
# per test (install: cargo install cargo-nextest --locked).
# `libtest-json` is nextest's own value name for the JSON format (not
# plain `json`, which it rejects), and the format is experimental there —
# without NEXTEST_EXPERIMENTAL_LIBTEST_JSON=1 the run exits 95. JSON goes
# to stdout; the human-readable run stays on stderr. `tee` merges both
# into the file — pipe to `grep '^{'` first if you want pure JSONL.
test-json:
    NEXTEST_EXPERIMENTAL_LIBTEST_JSON=1 cargo nextest run --workspace --features async,no_std,perf,fuzz,edge,snapshot --message-format libtest-json | tee nextest.json

# Machine-readable clippy diagnostics (JSON). Report recipe, not a gate:
# with a pipe the exit code is tee's, so warnings won't fail this recipe.
clippy-json:
    cargo clippy --workspace --all-targets --features async,no_std,perf,fuzz,edge,snapshot --message-format json | tee clippy.json

# Machine-readable cargo-deny output — JSON (SARIF: `cargo deny --format
# sarif check`; install: cargo install cargo-deny --locked). Note
# cargo-deny's `--format` is a global flag, placed before `check`.
deny-json:
    cargo deny --format json check | tee deny.json

# Machine-readable list of the mutants cargo-mutants would test — fast
# (no test runs). Note `--json` applies to `--list` only (install:
# cargo install cargo-mutants --locked).
mutants-list-json:
    cargo mutants --workspace --list --json | tee mutants.json

# Machine-readable mutation *outcomes* — `--json` doesn't serialize run
# results, so they come from the `-o` directory as
# `mutants.out/outcomes.json` (per-mutant caught/missed/unviable/timeout
# with log paths). Exploratory and slow — never a gate.
mutants-json:
    cargo mutants --workspace -o mutants.out
    @echo "==> outcomes: mutants.out/outcomes.json"

# Build and open rustdoc for the whole workspace.
doc:
    cargo doc --workspace --all-features --no-deps --open

# Check docs for broken intra-doc links etc., matching CI's `docs` job.
doc-check:
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps

# Chain everything CI runs (fmt, clippy, tests, nextest, doc-check, adoption
# check, deny) into one local pre-push command.
full-ci: fmt-check clippy-all test-all nextest doc-check adopt-test
    #!/usr/bin/env bash
    set -euo pipefail
    if cargo deny --version >/dev/null 2>&1; then
        echo "==> cargo deny check"
        cargo deny check
        echo "==> cargo deny check (fuzz/)"
        (cd fuzz && cargo deny check)
    else
        echo "==> skipping cargo-deny check (not installed; cargo install cargo-deny --locked)"
    fi

# Release-build every optional feature as a readiness check (no publish step — every crate is `publish = false`).
release-dry-run:
    cargo build --workspace --release --features async,no_std,perf,fuzz,edge,snapshot

# Verify the adoption promise end-to-end: `cargo generate --path .` output (leg
# skipped if cargo-generate isn't installed) plus `crates/` copied into a
# synthetic host workspace — mirrors CI's `adoption` job.
adopt-test:
    ./scripts/adoption-check.sh

# Exploratory mutation-testing pass, local-only, never wired into CI (install: cargo install cargo-mutants --locked).
mutants:
    cargo mutants --workspace
