# Eval Harness (example)

Fixed contract + fixed suite for evaluating an *external* implementation
against it — a library you're validating, an AI-generated candidate, a
rewrite — without touching the code under test. See
[`docs/evaluation-harness.md`](../../docs/evaluation-harness.md) for the
full pattern.

- `src/lib.rs` — the `Candidate` contract, the `FnCandidate` adapter,
  and the `GOLDEN_CASES` table.
- `tests/golden.rs` — every candidate must match the golden cases exactly.
- `tests/properties.rs` — invariants that must hold for all inputs.
- `tests/compile_fail.rs` — a candidate that doesn't implement the
  contract must fail to compile (trybuild; not feature-gated here, since
  this crate is copied per evaluation and the check always runs).

Run it against the bundled sample candidate (from the repo root):

```bash
just eval examples/sample-candidate
```

or without `just`:

```bash
scripts/eval-harness.sh examples/sample-candidate
```

Pass extra `cargo test` arguments after the path (`just eval
examples/sample-candidate -- --nocapture`). On failure the scratch
workspace is kept and its path printed for inspection.
