# integration-tests

End-to-end behavior tests across categories. The library surface here is
intentionally minimal (`suite_name()`) — the real content is
[`tests/e2e.rs`](tests/e2e.rs), which chains `syntax-tests`, `fuzz-tests`,
`edge-cases`, `semantic-tests`, and `performance-tests` together in a
single pipeline to demonstrate that categories compose.

## Example

```bash
cargo test -p integration-tests
```

See [`docs/architecture.md`](../../docs/architecture.md) for the crate
dependency graph this composition relies on.
