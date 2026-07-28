# syntax-tests

Parser/syntax-facing tests. `parse_source` is a deliberately toy stand-in
for a real parser's front door — swap it for your own tokenizer/parser
entry point and keep the pass/fail test shape.

## Contents

- `parse_source(&str) -> Result<&str, &'static str>` — validates non-empty
  input containing a function declaration.
- `tests/compile_fail.rs` + `tests/ui/{pass,fail}/` — `trybuild`
  compile-fail/UI tests, for checking that something fails to *compile*
  rather than just returning `Err` at runtime. Two pass fixtures (basic,
  generic-with-trait-bound) and two fail fixtures (type mismatch,
  use-after-move).
- `SourceSummary`, `summarize_source` — a multi-field struct snapshot-tested
  with [insta](https://insta.rs/) (behind the `snapshot` feature), for
  when per-field assertions get unwieldy. Snapshots live in
  `src/snapshots/`; update them with `cargo insta review` after an
  intentional change.
- `examples/parse_walkthrough.rs`.

## Feature flags

| Feature | Adds | What it unlocks |
| --- | --- | --- |
| `compile-fail` | `trybuild` | `cargo test -p syntax-tests --features compile-fail` |
| `snapshot` | `insta` | `cargo test -p syntax-tests --features snapshot` |

The `compile-fail` feature is CI-pinned to a stable-only job (see
[`ci/README.md`](../../ci/README.md)) since diagnostic text can drift
between toolchain channels. `snapshot` has no such restriction — snapshot
text doesn't depend on the compiler — so it's included in the main `test`
job's feature list.

## Example

```bash
cargo run -p syntax-tests --example parse_walkthrough
```

See [`docs/adding-tests.md`](../../docs/adding-tests.md) for how to add a
new UI fixture.
