use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use performance_tests::sum;
use std::hint::black_box;

fn sum_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum");

    for size in [1_000usize, 10_000, 100_000] {
        let data = vec![1u64; size];

        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| sum(black_box(data)));
        });
    }

    group.finish();
}

criterion_group!(benches, sum_benchmark);
criterion_main!(benches);
