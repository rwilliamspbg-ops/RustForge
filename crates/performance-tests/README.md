# performance-tests

Benchmark/perf-guard entry points. `sum` is deliberately trivial — the
interesting part is the two ways it's exercised.

## Contents

- `sum(&[u64]) -> u64`, `max_value`, `dedup_sorted` — three functions with
  different perf characteristics (linear scan, comparison, allocation).
- Cheap `#[ignore]`d wall-clock regression guards — no extra dependencies,
  opt in with `cargo test -- --ignored`.
- `benches/sum_bench.rs`, `benches/collection_ops_bench.rs` — real
  Criterion benchmarks (3 benchmark functions total), behind the `perf`
  feature.

## Feature flags

| Feature | Adds | What it unlocks |
| --- | --- | --- |
| `perf` | `criterion` | `cargo bench -p performance-tests --features perf` |

`criterion`'s own dependency chain tracks current stable Rust and can
exceed the workspace's declared MSRV — see the "MSRV" section of the root
[`README.md`](../../README.md).

## Example

```bash
cargo bench -p performance-tests --features perf
```
