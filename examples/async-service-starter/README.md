# Async Service Starter (example)

A tiny async in-memory key-value service — a domain starter showing
async testing patterns. See
[`docs/domain-starters.md`](../../docs/domain-starters.md) for the
containment pattern these starters share.

```bash
cargo test
```

What it demonstrates:

- **Deterministic time**: `#[tokio::test(start_paused = true)]`
  freezes tokio's clock, and `tokio::time::advance` moves it — expiry
  tests run in microseconds and can never flake on a slow machine.
- **Concurrency**: a cloneable `Store` handle sharing one
  `Arc<Mutex<..>>` map; a test spawns 16 concurrent inserts and asserts
  all are visible.
- **Background task + graceful shutdown**: `sweeper` drops expired
  entries on an interval, exiting cleanly on a shutdown channel — the
  shape a real service task needs, tested with `tokio::select!`.
- **No panics in the API surface**: lock poisoning is the one place
  `.expect` is justified (a poisoned lock means a test already
  panicked while holding it), and it's spelled out in the code.
