//! Benchmark/perf-guard entry points. `sum` is deliberately trivial — the
//! interesting parts are the two ways it's exercised: a cheap `#[ignore]`d
//! wall-clock regression guard here (always available, no extra deps), and
//! a proper Criterion benchmark under `benches/` (opt-in via the `perf`
//! feature, see `cargo bench -p performance-tests --features perf`).
#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Sums a slice of `u64`s.
pub fn sum(values: &[u64]) -> u64 {
    values.iter().copied().sum()
}

/// Returns the largest value in `values`, or `None` if it's empty.
pub fn max_value(values: &[u64]) -> Option<u64> {
    values.iter().copied().max()
}

/// Removes consecutive duplicate values, assuming `values` is already
/// sorted (unsorted input just dedups adjacent runs, same as
/// [`slice::dedup`]). Allocates a new `Vec`, unlike `dedup`'s in-place
/// truncation — useful when the benchmark should isolate allocation cost
/// from comparison cost.
pub fn dedup_sorted(values: &[u64]) -> Vec<u64> {
    let mut result = Vec::with_capacity(values.len());
    for &value in values {
        if result.last() != Some(&value) {
            result.push(value);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{dedup_sorted, max_value, sum};
    use std::time::{Duration, Instant};

    fn perf_threshold_ms() -> u64 {
        std::env::var("RUSTFORGE_PERF_THRESHOLD_MS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(50)
    }

    #[test]
    fn sum_is_correct() {
        assert_eq!(sum(&[1, 2, 3, 4]), 10);
    }

    #[test]
    fn max_value_finds_the_largest_element() {
        assert_eq!(max_value(&[3, 1, 4, 1, 5, 9, 2, 6]), Some(9));
        assert_eq!(max_value(&[]), None);
        assert_eq!(max_value(&[42]), Some(42));
    }

    #[test]
    fn dedup_sorted_removes_consecutive_duplicates() {
        assert_eq!(dedup_sorted(&[1, 1, 2, 2, 2, 3]), vec![1, 2, 3]);
        assert_eq!(dedup_sorted(&[]), Vec::<u64>::new());
        assert_eq!(dedup_sorted(&[1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(dedup_sorted(&[7, 7, 7]), vec![7]);
    }

    #[test]
    #[ignore = "manual perf regression guard"]
    fn sum_large_input_is_reasonably_fast() {
        let data = vec![1u64; 100_000];
        let start = Instant::now();
        let total = sum(&data);

        assert_eq!(total, 100_000);
        assert!(start.elapsed() < Duration::from_millis(perf_threshold_ms()));
    }

    #[test]
    #[ignore = "manual perf regression guard"]
    fn dedup_sorted_large_input_is_reasonably_fast() {
        let data: Vec<u64> = (0..100_000u64).map(|n| n / 4).collect();
        let start = Instant::now();
        let result = dedup_sorted(&data);

        assert_eq!(result.len(), 25_000);
        assert!(start.elapsed() < Duration::from_millis(perf_threshold_ms()));
    }
}
