# Evaluation Harness

Drop in an external implementation — a library you're validating, an
AI-generated candidate, a rewrite — and evaluate it against a fixed
contract with a fixed suite, without touching the code under test.

The template's category crates test *the template's own* fixtures; this
is the other half: `examples/eval-harness/` (the contract and suite)
plus `examples/sample-candidate/` (a reference candidate), wired
together by `just eval`.

## The pattern

1. **The contract** — `eval_harness::Candidate`, a trait whose
   semantics are pinned to a reference implementation. It never changes
   per candidate.
2. **The candidate** — any crate implementing the contract for a
   `pub struct Impl` (the name is part of the contract; the suite refers
   to it as `candidate::Impl`). A thin wrapper is enough — you never
   modify the code under test. [`FnCandidate`] adapts a plain function
   when the real API doesn't match the trait shape.
3. **The suite** — golden cases (fixed input/expected pairs), property
   tests (invariants over generated inputs), and a compile-fail check
   (a candidate that doesn't implement the contract fails to *build*,
   loudly, instead of at test time).

## Running an evaluation

```bash
just eval examples/sample-candidate
```

That copies both crates into a scratch workspace, substitutes the
`__HARNESS_PATH__` / `__CANDIDATE_PATH__` placeholders in their
`Cargo.toml`s, and runs `cargo test` there. Extra `cargo test` arguments
can follow the path (`just eval <path> -- --nocapture`). On failure the
scratch workspace is kept and its path printed for inspection. Without
`just`: `scripts/eval-harness.sh <path>` (CI's `adoption` job uses this
form against the sample).

To evaluate your own implementation:

1. Copy `examples/sample-candidate/` as a starting point. The crate must
   be **named `candidate`** and expose `pub struct Impl` implementing
   `eval_harness::Candidate` (both are part of the contract).
2. Point its `eval_harness` dependency at your copy of the harness —
   keep the `__HARNESS_PATH__` placeholder, `just eval` substitutes it.
3. `just eval path/to/your/candidate`.

## What the suite checks

- **Golden cases** (`GOLDEN_CASES` in the harness's `src/lib.rs`):
  fixed input/expected pairs from the reference semantics —
  canonical and padded numbers, the `+` prefix and its misuse,
  negatives, empties, non-numbers, interior whitespace, and
  `u32::MAX + 1` overflow. Discriminating by construction: an all-`None`
  candidate, or one that accepts what the reference rejects, fails here.
- **Property tests** (`tests/properties.rs`): valid `u32`s round-trip
  through whitespace padding; a leading `+` is accepted; surrounding
  whitespace is invisible for *every* string; no input panics.
- **Compile-fail** (`tests/compile_fail.rs`): a candidate that doesn't
  implement the contract fails to compile with an error naming the
  trait — the shape of check a real evaluation workflow needs when
  candidates are generated rather than hand-written.

## Why the crates aren't workspace members

Both crates carry an empty `[workspace]` table, making each its own
workspace root — the same detachment pattern `fuzz/` uses. They add
nothing to the template's default `cargo test --workspace` or its
dependency graph, and copy out wholesale for reuse.

The dependency cycle is deliberate and legal in Cargo: the harness
dev-depends on the candidate while the candidate depends on the harness
(dev-dependency cycles are allowed). It's what lets the suite test the
candidate while the candidate implements the harness's trait — the same
shape a real "interface crate + tests + implementations" layout has.

## Extending the harness

- More golden cases: extend `GOLDEN_CASES` in `src/lib.rs`.
- More properties: add `proptest!` cases in `tests/properties.rs`.
- A different domain: copy the harness, rename the contract, rewrite
  the golden cases and properties for your domain — the shape
  (contract + `Impl` + three test kinds) is the reusable part, not the
  `u32` parser.
