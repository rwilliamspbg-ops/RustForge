#!/usr/bin/env bash
# Verifies RustForge's core adoption promise end-to-end: a fresh adopter
# who takes this template (either path in docs/adoption.md) gets a passing
# `cargo test --workspace` — instead of the docs just claiming it.
#
# Two legs, mirroring the `adoption` CI job in .github/workflows/ci.yml:
#
#   1. `cargo generate --path .` — the "fresh git history" path. Skipped
#      (with a notice) when cargo-generate isn't installed, matching the
#      skip pattern `just full-ci` uses for cargo-deny.
#   2. A synthetic host workspace — its own root package with RustForge's
#      `crates/` copied in — the "existing workspace" path. This is where
#      the `workspace.package` gotcha lives (see docs/adoption.md): the
#      crates inherit edition/license/rust-version and workspace lints
#      from the root, so a root missing those tables fails to parse.
#
# Run this before changing anything structural: workspace layout, the
# [workspace.package]/[workspace.lints.rust] tables, crate paths, or
# cargo-generate.toml. Run it after such changes to prove adopters are
# unaffected.
set -euo pipefail

# On Windows (git-bash/MSYS), path arguments are NOT translated for native
# programs — `cargo-generate.exe` receiving `/c/Users/...` fails with "The
# system cannot find the path specified. (os error 3)". Convert to a native
# path when cygpath is available; on Linux CI this is a plain no-op.
native_path() {
    if command -v cygpath >/dev/null 2>&1; then
        cygpath -w "$1"
    else
        printf '%s' "$1"
    fi
}

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORK="$(mktemp -d "${TMPDIR:-${TMP:-/tmp}}/rustforge-adoption.XXXXXX")"
trap 'rm -rf "$WORK"' EXIT

echo "==> adoption check (scratch: $WORK)"

# --- Leg 1: cargo generate (the "fresh git history" path) -----------------
if command -v cargo-generate >/dev/null 2>&1; then
    echo "==> leg 1: cargo generate --path . (cargo-generate installed)"
    # --vcs none: the generated copy gets no git repo of its own; there is
    # nothing to template or rename (see cargo-generate.toml's comment), so
    # this is purely "copy the tree somewhere fresh".
    cargo generate --path "$(native_path "$REPO_ROOT")" \
        --destination "$(native_path "$WORK")" \
        --name rustforge-generated \
        --vcs none
    # Default features only: this leg verifies the adoption *structure*
    # (workspace members, inheritance tables, crate paths), which the
    # lightest possible suite exercises fully. Feature interactions are
    # already covered by the main CI's `test` and `hack` jobs.
    (cd "$WORK/rustforge-generated" && cargo test --workspace)
else
    echo "==> leg 1: SKIPPED (cargo-generate not installed — run: cargo install cargo-generate --locked)"
fi

# --- Leg 2: crates/ copied into a synthetic host workspace ----------------
# The "copy crates/ into your existing workspace" flow from docs/adoption.md.
# The root package here plays the adopter's own crate; the tables below are
# the minimum a host root must provide for the copied crates to build (the
# `workspace.package` pitfall from docs/adoption.md, plus the equivalent
# `[workspace.lints.rust]` table the crates' `[lints] workspace = true`
# depends on).
echo "==> leg 2: crates/ copied into a synthetic host workspace"
HOST="$WORK/host-workspace"
mkdir -p "$HOST/src" "$HOST/crates"
cp -R "$REPO_ROOT/crates/." "$HOST/crates/"

cat > "$HOST/Cargo.toml" <<'EOF'
[workspace]
resolver = "2"
members = [
    ".",
    "crates/core-tests",
    "crates/syntax-tests",
    "crates/semantic-tests",
    "crates/performance-tests",
    "crates/fuzz-tests",
    "crates/integration-tests",
    "crates/edge-cases",
]

# Required by every copied crate (edition/license/rust-version inherit
# from here) — see "workspace.package.edition was not defined" in
# docs/adoption.md.
[workspace.package]
edition = "2021"
rust-version = "1.75"
license = "MIT OR Apache-2.0"

# Required by every copied crate's `[lints] workspace = true`.
[workspace.lints.rust]
missing_docs = "warn"
unsafe_code = "forbid"

[package]
name = "host-app"
version = "0.1.0"
edition.workspace = true
license.workspace = true
rust-version.workspace = true
publish = false
EOF

cat > "$HOST/src/lib.rs" <<'EOF'
//! Minimal stand-in for the adopter's own crate: RustForge's category
//! crates coexist with it in the same workspace.
pub fn host_marker() -> u32 {
    42
}
EOF

(cd "$HOST" && cargo test --workspace)

echo "==> adoption check passed"
