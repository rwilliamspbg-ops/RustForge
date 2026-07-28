#!/usr/bin/env bash
# Generates an HTML coverage report with cargo-llvm-cov. Informational —
# there's no enforced threshold; use this to spot untested paths.
# Install once with:
#   rustup component add llvm-tools-preview
#   cargo install cargo-llvm-cov --locked
# Usage: scripts/coverage.sh
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

if ! cargo llvm-cov --version >/dev/null 2>&1; then
  echo "cargo-llvm-cov is not installed. Install it with:" >&2
  echo "  rustup component add llvm-tools-preview" >&2
  echo "  cargo install cargo-llvm-cov --locked" >&2
  exit 1
fi

echo "==> cargo llvm-cov --workspace --all-features --summary-only"
cargo llvm-cov --workspace --all-features --summary-only

echo "==> cargo llvm-cov report --html"
cargo llvm-cov report --html

echo "Report written to target/llvm-cov/html/index.html"
