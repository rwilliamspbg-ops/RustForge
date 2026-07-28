# semantic-tests

Ownership, borrowing, trait dispatch, and async semantics.

## Contents

- `shared_counter_after_workers` — `Arc<Mutex<_>>` sharing under concurrent
  mutation.
- `Greeter` trait, `NamedGreeter` — static (`greet_via_generic`) vs.
  dynamic (`greet_via_trait_object`) dispatch, side by side.
- `longest`, `Excerpt<'a>` — the classic multi-input lifetime example, plus
  a struct that borrows rather than owns.
- `assert_send::<T>()`, `assert_sync::<T>()` — compile-time `Send`/`Sync`
  checks: instantiate with a type that isn't `Send`/`Sync` and it fails to
  *compile*, which is the point.
- `ConfigError`, `parse_timeout_ms` — the "implement `Error` + propagate
  with `?`" error-handling pattern.
- An async ownership test (behind the `async` feature) showing a value
  moved into a spawned task and handed back on completion.

## Feature flags

| Feature | Adds | What it unlocks |
| --- | --- | --- |
| `async` | `tokio` | `cargo test -p semantic-tests --features async` |

## Example

```bash
cargo test -p semantic-tests --features async
```

See the root [`README.md`](../../README.md) for the full workspace overview.
