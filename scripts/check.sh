#!/usr/bin/env bash
# Mirrors the CI gate locally: fmt check, clippy (deny warnings), tests,
# and (if cargo-deny is installed) the dependency-hygiene check.
# Usage: scripts/check.sh
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

echo "==> cargo fmt --all --check"
cargo fmt --all --check

echo "==> cargo clippy --workspace --all-targets --all-features -- -D warnings"
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo "==> cargo test --workspace --all-features"
cargo test --workspace --all-features

echo "==> cargo doc --workspace --all-features --no-deps (RUSTDOCFLAGS=-D warnings)"
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps

if cargo deny --version >/dev/null 2>&1; then
  echo "==> cargo deny check"
  cargo deny check
  echo "==> cargo deny check (fuzz/)"
  (cd fuzz && cargo deny check)
else
  echo "==> skipping cargo-deny check (not installed; cargo install cargo-deny --locked)"
fi

echo "All checks passed."
