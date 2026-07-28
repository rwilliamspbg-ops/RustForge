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

# Check licenses/advisories/bans/sources for both workspaces (install: cargo install cargo-deny --locked).
deny:
    cargo deny check
    cd fuzz && cargo deny check

# Generate an HTML coverage report (install: rustup component add llvm-tools-preview && cargo install cargo-llvm-cov --locked).
coverage:
    ./scripts/coverage.sh

# Build and open rustdoc for the whole workspace.
doc:
    cargo doc --workspace --all-features --no-deps --open

# Check docs for broken intra-doc links etc., matching CI's `docs` job.
doc-check:
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
