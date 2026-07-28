# performance-tests

Benchmark/perf-guard entry points. `sum` is deliberately trivial — the
interesting part is the two ways it's exercised.

## Contents

- `sum(&[u64]) -> u64` — the function under test.
- A cheap `#[ignore]`d wall-clock regression guard (`sum_large_input_is_reasonably_fast`)
  — no extra dependencies, opt in with `cargo test -- --ignored`.
- `benches/sum_bench.rs` — a real Criterion benchmark, behind the `perf`
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
