#!/usr/bin/env bash
# Generates an HTML coverage report with cargo-llvm-cov.
# Install once with: cargo install cargo-llvm-cov
# Usage: scripts/coverage.sh
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

if ! cargo llvm-cov --version >/dev/null 2>&1; then
  echo "cargo-llvm-cov is not installed. Install it with:" >&2
  echo "  cargo install cargo-llvm-cov" >&2
  exit 1
fi

cargo llvm-cov --workspace --all-features --html

echo "Report written to target/llvm-cov/html/index.html"
