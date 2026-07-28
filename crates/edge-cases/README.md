# edge-cases

Boundary-value and robustness checks — the off-by-one, overflow, and
multi-byte-character gotchas that are easy to miss until production.

## Contents

- `clamp_len` — a byte-length cap that can land mid-character (the gotcha).
- `safe_truncate` — the fix: walks back to the nearest UTF-8 char boundary.
- `clamp_collection` — the collection counterpart: slicing a `&[T]` has no
  char-boundary gotcha to fix, so this one is the boundary-value
  counterexample (`0`, `1`, more-than-available all just work).
- `overflow_checks` (behind the `edge` feature) — `checked_distance`/
  `checked_offset`, guarding `usize` underflow/overflow explicitly instead
  of relying on debug-only panics.
- `proptest` property tests (behind the `edge` feature) for `safe_truncate`,
  `clamp_collection`, and `checked_offset` — see "Property test" in
  [`docs/adding-tests.md`](../../docs/adding-tests.md).

## Feature flags

| Feature | Adds | What it unlocks |
| --- | --- | --- |
| `edge` | `proptest` | `overflow_checks` module + property tests |

## Example

```bash
cargo test -p edge-cases --features edge
```

See [`docs/best-practices.md`](../../docs/best-practices.md) for the
philosophy behind pairing a "gotcha" with its fix in the same crate.
