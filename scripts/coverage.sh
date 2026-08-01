#!/usr/bin/env bash
# Generates an HTML coverage report with cargo-llvm-cov. Informational —
# there's no enforced threshold; use this to spot untested paths.
# Install once with:
#   rustup toolchain install nightly --component llvm-tools-preview
#   cargo install cargo-llvm-cov --locked
# Runs under nightly: `--doctests` (counting the `///` examples toward
# coverage, not just #[test]s) needs rustdoc's unstable
# --persist-doctests, which only the nightly compiler accepts — matches
# CI's `coverage` job, see .github/workflows/ci.yml.
# Usage: scripts/coverage.sh
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

if ! cargo llvm-cov --version >/dev/null 2>&1; then
  echo "cargo-llvm-cov is not installed. Install it with:" >&2
  echo "  rustup toolchain install nightly --component llvm-tools-preview" >&2
  echo "  cargo install cargo-llvm-cov --locked" >&2
  exit 1
fi

echo "==> cargo +nightly llvm-cov --workspace --all-features --doctests --summary-only"
cargo +nightly llvm-cov --workspace --all-features --doctests --summary-only

echo "==> cargo +nightly llvm-cov report --html"
cargo +nightly llvm-cov report --html

echo "Report written to target/llvm-cov/html/index.html"
