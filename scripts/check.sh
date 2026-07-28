#!/usr/bin/env bash
# Mirrors the CI gate locally: fmt check, clippy (deny warnings), tests.
# Usage: scripts/check.sh
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

echo "==> cargo fmt --all --check"
cargo fmt --all --check

echo "==> cargo clippy --workspace --all-targets --all-features -- -D warnings"
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo "==> cargo test --workspace --all-features"
cargo test --workspace --all-features

echo "All checks passed."
