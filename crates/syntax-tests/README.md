# syntax-tests

Parser/syntax-facing tests. `parse_source` is a deliberately toy stand-in
for a real parser's front door — swap it for your own tokenizer/parser
entry point and keep the pass/fail test shape.

## Contents

- `parse_source(&str) -> Result<&str, &'static str>` — validates non-empty
  input containing a function declaration.
- `tests/compile_fail.rs` + `tests/ui/{pass,fail}/` — `trybuild`
  compile-fail/UI tests, for checking that something fails to *compile*
  rather than just returning `Err` at runtime.
- `examples/parse_walkthrough.rs`.

## Feature flags

| Feature | Adds | What it unlocks |
| --- | --- | --- |
| `compile-fail` | `trybuild` | `cargo test -p syntax-tests --features compile-fail` |

The `compile-fail` feature is CI-pinned to a stable-only job (see
[`ci/README.md`](../../ci/README.md)) since diagnostic text can drift
between toolchain channels.

## Example

```bash
cargo run -p syntax-tests --example parse_walkthrough
```

See [`docs/adding-tests.md`](../../docs/adding-tests.md) for how to add a
new UI fixture.
