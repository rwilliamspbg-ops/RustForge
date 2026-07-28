#![forbid(unsafe_code)]

pub fn sum(values: &[u64]) -> u64 {
    values.iter().copied().sum()
}

#[cfg(test)]
mod tests {
    use super::sum;
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
    #[ignore = "manual perf regression guard"]
    fn sum_large_input_is_reasonably_fast() {
        let data = vec![1u64; 100_000];
        let start = Instant::now();
        let total = sum(&data);

        assert_eq!(total, 100_000);
        assert!(start.elapsed() < Duration::from_millis(perf_threshold_ms()));
    }
}
