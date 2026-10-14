#!/usr/bin/env bash
# Evaluates a candidate implementation against the fixed eval harness:
# copies examples/eval-harness and the candidate into a scratch
# workspace, wires them together (the __HARNESS_PATH__ /
# __CANDIDATE_PATH__ placeholders), and runs the full suite — golden
# cases, property tests, and the compile-fail contract check — against
# the candidate.
#
# Mirrors the `eval` recipe in the justfile; CI's `adoption` job runs it
# against the bundled sample candidate (examples/sample-candidate) to
# prove the harness itself works.
#
# Usage: scripts/eval-harness.sh <candidate-path> [extra cargo test args...]
set -euo pipefail

# On Windows (git-bash/MSYS), path arguments are NOT translated for
# native programs — a native cargo reading `/c/...` from a Cargo.toml
# fails with "The system cannot find the path specified. (os error 3)".
# Convert to a native path when cygpath is available; on Linux CI this
# is a no-op. Backslashes are then flattened to forward slashes (valid
# in TOML and in cargo paths on every platform) so `sed` never has to
# escape them.
native_path() {
    if command -v cygpath >/dev/null 2>&1; then
        cygpath -w "$1" | tr '\\' '/'
    else
        printf '%s' "$1"
    fi
}

if [ "$#" -lt 1 ]; then
    echo "usage: $0 <candidate-path> [extra cargo test args...]" >&2
    exit 1
fi

CANDIDATE_RAW="$1"
shift

if [ ! -f "$CANDIDATE_RAW/Cargo.toml" ]; then
    echo "error: no Cargo.toml at candidate path: $CANDIDATE_RAW" >&2
    echo "       (the candidate must be a crate — see docs/evaluation-harness.md)" >&2
    exit 1
fi

# The crate must be named `candidate` — that name is part of the
# contract (the harness's dev-dependency and tests use it).
PKG_NAME="$(sed -n 's/^name = "\(.*\)"/\1/p' "$CANDIDATE_RAW/Cargo.toml" | head -1)"
if [ "$PKG_NAME" != "candidate" ]; then
    echo "error: candidate crate must be named 'candidate' (found: '$PKG_NAME')" >&2
    echo "       the name is part of the contract — see docs/evaluation-harness.md" >&2
    exit 1
fi

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CANDIDATE="$(cd "$CANDIDATE_RAW" && pwd)"
WORK="$(mktemp -d "${TMPDIR:-${TMP:-/tmp}}/rustforge-eval.XXXXXX")"

cp -R "$REPO_ROOT/examples/eval-harness" "$WORK/harness"
cp -R "$CANDIDATE" "$WORK/candidate"
# Drop any local build artifacts from the copies — they're per-machine
# and would just be rebuilt.
rm -rf "$WORK/harness/target" "$WORK/candidate/target"

sed -i "s|__HARNESS_PATH__|$(native_path "$WORK/harness")|g" "$WORK/candidate/Cargo.toml"
sed -i "s|__CANDIDATE_PATH__|$(native_path "$WORK/candidate")|g" "$WORK/harness/Cargo.toml"

echo "==> evaluating candidate: $CANDIDATE"
status=0
(cd "$WORK/harness" && cargo test "$@") || status=$?
if [ "$status" -ne 0 ]; then
    # Keep the scratch workspace so a failing candidate can be
    # inspected (and re-run) after the fact.
    echo "==> evaluation FAILED — scratch workspace kept for inspection: $WORK" >&2
    exit "$status"
fi
rm -rf "$WORK"
echo "==> evaluation passed"
