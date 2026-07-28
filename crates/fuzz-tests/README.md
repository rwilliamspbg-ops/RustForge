# fuzz-tests

Fuzz harness-friendly entry points. Exercised three ways, from cheapest to
most thorough.

## Contents

- `utf8_input(&[u8]) -> Result<&str, Utf8Error>` — the function under test;
  never panics on any input, valid or not.
- Plain unit tests (always available).
- `proptest` property tests (behind the `fuzz` feature) — round-trip and
  never-panics properties over arbitrary input.

The real `cargo-fuzz`/libFuzzer target lives in the detached [`fuzz/`](../../fuzz)
workspace at the repo root, not here — see [`fuzz/README.md`](../../fuzz/README.md).

## Feature flags

| Feature | Adds | What it unlocks |
| --- | --- | --- |
| `fuzz` | `proptest` | `cargo test -p fuzz-tests --features fuzz` |

## Example

```bash
cargo test -p fuzz-tests --features fuzz
```
