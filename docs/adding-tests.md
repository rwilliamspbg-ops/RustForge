# Adding Tests

A practical guide to the common case: adding a test to an *existing*
category crate. For the rarer case of adding a whole new category crate,
see "Adding a New Test Category" in [`CONTRIBUTING.md`](../CONTRIBUTING.md).

## Where does my test go?

| I want to test... | Category | Example |
| --- | --- | --- |
| A parser, tokenizer, or DSL front-end | `syntax-tests` | `parse_source` in `crates/syntax-tests/src/lib.rs` |
| Something failing to *compile* (not just returning `Err`) | `syntax-tests`, `compile-fail` feature | `crates/syntax-tests/tests/ui/` |
| Ownership, borrowing, trait dispatch, async | `semantic-tests` | `shared_counter_after_workers`, `Greeter` |
| A boundary value, overflow, or off-by-one | `edge-cases` | `safe_truncate`, `overflow_checks` |
| Throughput or latency of a hot path | `performance-tests` | `benches/sum_bench.rs` |
| "never panics on arbitrary input" | `fuzz-tests`, `fuzz` feature (proptest) or `fuzz/` (cargo-fuzz) | `utf8_input` |
| Multiple categories working together end-to-end | `integration-tests` | `tests/e2e.rs` |
| A helper other categories should reuse | `core-tests` | `UserFixture`, `assert_contains` |

If you're not sure, default to a plain unit test in the closest-matching
category — it's cheap to move later.

## The four test shapes in this template

### 1. Plain unit test (always available, no extra deps)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_behavior_holds() {
        assert_eq!(my_function(2), 4);
    }
}
```

This is the default. Every category crate has these, and they run with
`cargo test --workspace` — no feature flag needed.

### 2. Property test (`proptest`, behind `fuzz-tests`' `fuzz` feature)

Use when a plain unit test would only cover a handful of hand-picked
inputs, but the *property* you care about should hold for all inputs. See
`crates/fuzz-tests/src/lib.rs`'s `property_tests` module for the pattern:

```rust
#[cfg(feature = "fuzz")]
mod property_tests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn my_property_holds(input in proptest::collection::vec(any::<u8>(), 0..64)) {
            prop_assert!(my_function_never_panics(&input));
        }
    }
}
```

Run it with `cargo test -p fuzz-tests --features fuzz`.

### 3. Compile-fail / UI test (`trybuild`, behind `syntax-tests`' `compile-fail` feature)

Use when the thing under test should fail to *compile*, not just return an
error at runtime — the right tool for testing a macro or a type-level
constraint. See `crates/syntax-tests/tests/compile_fail.rs` and the
fixtures under `crates/syntax-tests/tests/ui/{pass,fail}/`.

1. Add a `.rs` fixture under `tests/ui/pass/` (should compile) or
   `tests/ui/fail/` (should not).
2. Generate its `.stderr` snapshot: `TRYBUILD=overwrite cargo test -p
   syntax-tests --features compile-fail`, then inspect the diff before
   committing it.
3. Keep fixtures minimal and prefer stable, long-standing diagnostics
   (e.g. a basic type mismatch) over anything likely to reword across
   compiler versions — CI only runs this on stable (see
   [`ci/README.md`](../ci/README.md)), but the snapshot still needs to
   stay accurate as the pinned `trybuild`/compiler age.

### 4. Benchmark (`criterion`, behind `performance-tests`' `perf` feature)

Use for throughput/latency-sensitive code where you want a regression
guard, not just correctness. See `crates/performance-tests/benches/sum_bench.rs`.

```rust
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;

fn my_benchmark(c: &mut Criterion) {
    c.bench_function("my_function", |b| b.iter(|| my_function(black_box(42))));
}

criterion_group!(benches, my_benchmark);
criterion_main!(benches);
```

Run with `cargo bench -p performance-tests --features perf`. Note the
`[[bench]]` entry in `Cargo.toml` needs `required-features = ["perf"]` so
it doesn't force every adopter to pull in Criterion by default.

For a cheaper, dependency-free regression guard that runs under plain
`cargo test`, see the `#[ignore]`d wall-clock check next to `sum_is_correct`
in `crates/performance-tests/src/lib.rs` instead.

## Reusing fixtures

Don't duplicate fixture/builder code across categories — put it in
`core-tests` and depend on it:

```toml
[dependencies]
core-tests = { path = "../core-tests" }
```

```rust
use core_tests::default_user_fixture;
```

## New dependency? Gate it.

If your test needs a crate beyond the standard library, add it as an
`optional = true` dependency (or, for dev-only tooling, a plain
`[dev-dependencies]` entry — see the comment in
`crates/performance-tests/Cargo.toml` for why dev-dependencies need extra
care) behind a Cargo feature, and check whether the latest release exceeds
the workspace's MSRV — see the "MSRV" section of the root
[`README.md`](../README.md) and [`docs/best-practices.md`](best-practices.md).
