// Bench binaries aren't a public API surface, so the workspace's
// `missing_docs` lint (which applies package-wide, unlike the old
// per-crate `lib.rs` attribute it replaced) doesn't apply here.
#![allow(missing_docs)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use performance_tests::{dedup_sorted, max_value};
use std::hint::black_box;

fn max_value_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("max_value");

    for size in [1_000usize, 10_000, 100_000] {
        let data: Vec<u64> = (0..size as u64).collect();

        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| max_value(black_box(data)));
        });
    }

    group.finish();
}

fn dedup_sorted_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("dedup_sorted");

    for size in [1_000usize, 10_000, 100_000] {
        // Every value repeated 4 times, so dedup always has real work to do.
        let data: Vec<u64> = (0..size as u64).map(|n| n / 4).collect();

        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| dedup_sorted(black_box(data)));
        });
    }

    group.finish();
}

criterion_group!(benches, max_value_benchmark, dedup_sorted_benchmark);
criterion_main!(benches);
