# AI Test-Generation Aids

Recipes and templates for generating tests with an LLM (or by hand) that
fit RustForge's conventions on the first try. The goal: a generated test
lands in the right category, behind the right feature flag, with the
right shape — no cleanup pass required.

## The checklist every generated test must satisfy

Feed this to whatever generates the tests (it's also the review
checklist for `CONTRIBUTING.md`'s process):

1. **Right category** — see "Where does my test go?" in
   [`adding-tests.md`](adding-tests.md). Plain `#[test]`s go to
   `syntax-tests`/`semantic-tests`/`edge-cases` by what they *assert*,
   not where the code lives.
2. **Right shape** — one of the shapes in "The six test shapes"
   (`adding-tests.md`): unit, property (`proptest`), compile-fail
   (`trybuild`), benchmark (Criterion), snapshot (`insta`),
   concurrency (`loom`). If the failure mode doesn't match the tool,
   the test is the wrong shape.
3. **No new default dependency** — anything beyond std goes behind a
   feature flag (`proptest` → `fuzz` feature, `trybuild` →
   `compile-fail`, `insta` → `snapshot`, `tokio` → `async`). See "New
   dependency? Gate it." in `adding-tests.md`.
4. **MSRV 1.75** — no unstable features; pin MSRV-sensitive deps
   (`proptest = "~1.8"`, `trybuild = "=1.0.111"`) rather than floating.
5. **Docs** — every public item documented (`missing_docs` is a
   workspace lint, enforced as an error in CI); doc examples are
   runnable and must pass.
6. **Deterministic and fast** — no wall-clock sleeps, no network, no
   fixed absolute paths. Async time is driven with
   `#[tokio::test(start_paused = true)]` (see
   `examples/async-service-starter/`).

## Recipe 1: signature → property test

Given a function signature, the property tests write themselves from
the *type*, before you even look at the implementation:

```rust
// Signature: fn parse_u32_lenient(input: &str) -> Option<u32>
// Derived properties (fill __EXPECTED__ from the contract):
proptest! {
    /// Total function: never panics, whatever the input.
    #[test]
    fn never_panics(s in ".*") {
        let _ = parse_u32_lenient(&s);
    }

    /// Round-trip through the output's domain: every value the output
    /// type can represent, re-encoded as input, parses back to itself.
    #[test]
    fn round_trips(n in any::<u32>()) {
        prop_assert_eq!(parse_u32_lenient(&n.to_string()), Some(n));
    }

    /// Idempotence/invariance where the contract implies it
    /// (whitespace tolerance, here):
    #[test]
    fn whitespace_is_invisible(s in ".*") {
        prop_assert_eq!(
            parse_u32_lenient(&s),
            parse_u32_lenient(s.trim()),
            "__FILL__: only if the contract trims"
        );
    }
}
```

The generic derivations, in priority order: **totality** (never
panics), **round-trip** (output domain re-encoded as input),
**invariance/idempotence** (operations the contract says don't matter),
**commutativity/associativity** (if the signature is binary), and
**boundary** (min/max of each numeric type — `edge-cases` material).

## Recipe 2: fill-in trybuild template

For "this input must fail to compile" checks — copy, fill the
`__FILL__`s, drop into `tests/ui/fail/`, regenerate the `.stderr` with
`TRYBUILD=overwrite cargo test -p syntax-tests --features compile-fail`:

```rust
// Must NOT compile: __FILL__ what misuse this pins.
use my_crate::__FILL__;

fn main() {
    // __FILL__ the exact call that should be rejected
    __FILL__;
}
```

Prefer asserting on *your own* error text (via a `compile_error!`
fallback arm in a macro — see `parse_cases!` in `syntax-tests`) over
rendered rustc diagnostics; your text is stable across compiler
versions. Generate the `.stderr` once, commit it.

## Recipe 3: fill-in snapshot template

For types with enough fields that per-field assertions get unwieldy —
copy, fill, run `cargo insta review` to accept:

```rust
#[cfg(feature = "snapshot")]
#[test]
fn __FILL__snapshot_name() {
    let value = __FILL__::build_example(); // the value under test
    insta::assert_debug_snapshot!(value);
}
```

Snapshots live in `src/snapshots/` next to the test; a pending
`.snap.new` is gitignored until reviewed (`cargo insta review`).

## Recipe 4: fill-in golden-case table

For pinned input/expected pairs (the discriminating core of any suite):

```rust
// (input, expected) — __FILL__ from the contract; keep the set
// discriminating: a trivial implementation must fail it.
const CASES: &[(&str, __FILL__)] = &[
    ("__FILL__canonical input", __FILL__expected),
    ("__FILL__empty/degenerate input", __FILL__expected),
    ("__FILL__boundary (max/overflow)", __FILL__expected),
    ("__FILL__invalid input", __FILL__expected),
];

#[test]
fn golden_cases() {
    for (input, expected) in CASES {
        assert_eq!(__FILL__::call(input), *expected, "input: {input:?}");
    }
}
```

The same table shape drives the evaluation harness
(`GOLDEN_CASES` in `examples/eval-harness/src/lib.rs`) when the code
under test is external.

## Recipe 5: category/fixture placement

When generating a whole test file, the placement rules are mechanical:

- Shared setup/fixture helpers → `core-tests` (fixtures are built once,
  reused by every category).
- Compile-fail / snapshot / macro fixtures → `syntax-tests`
  (`tests/ui/fail/`, `tests/ui/pass/`, `src/snapshots/`).
- Ownership/borrowing/trait/async semantics → `semantic-tests`.
- Numeric/collection boundary values → `edge-cases`.
- Cross-category behavior → `integration-tests`.
- Speed-sensitive code → `performance-tests` (Criterion, only when a
  regression guard is actually wanted).

## Evaluating generated implementations

When the *implementation* (not just the tests) is generated, don't
review it by eye — evaluate it: the fixed-contract harness in
[`docs/evaluation-harness.md`](evaluation-harness.md) runs golden cases,
property tests, and a compile-fail shape check against any candidate.
And when you have a reference implementation to diff against, the
differential helpers (`core_tests::differential::assert_impls_agree`,
`fuzz_tests::differential::assert_impls_agree`) prove equivalence
instead of trusting it.
