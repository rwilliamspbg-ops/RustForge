//! Differential-testing helpers: two implementations of the same
//! function, checked against each other input by input.
//!
//! Differential testing is the pattern for "I rewrote the hot path — how
//! do I know it still behaves identically?": keep the original,
//! obviously-correct implementation as a *reference* and assert the new
//! one agrees with it on every input. It's how you refactor with
//! confidence, and how you validate an optimized, FFI, or generated
//! implementation against a simple one.
//!
//! This module is the fixed-inputs form. Two companions exist for other
//! situations: the property-based variant
//! (`fuzz_tests::differential::assert_impls_agree`, behind the `fuzz`
//! feature) for *generated* inputs, and the cargo-fuzz flavor in
//! `docs/fuzzing.md` ("Differential fuzzing") for continuous
//! differential checking.

/// Asserts that `reference` and `candidate` produce equal results for
/// every input in `inputs`.
///
/// Keep `reference` deliberately simple and obviously correct, even if
/// slow — its only job is to be trustworthy, not fast. A disagreement is
/// a real bug in one of the two implementations; the panic message names
/// the input and both results.
///
/// # Examples
///
/// ```rust
/// use core_tests::differential::assert_impls_agree;
///
/// // Reference: the obvious O(n) loop. Candidate: the closed form.
/// let reference = |n: &u64| (1..=*n).sum::<u64>();
/// let candidate = |n: &u64| n * (n + 1) / 2;
/// assert_impls_agree(reference, candidate, &[0, 1, 10, 100]);
/// ```
pub fn assert_impls_agree<I, T>(
    reference: impl Fn(&I) -> T,
    candidate: impl Fn(&I) -> T,
    inputs: &[I],
) where
    I: std::fmt::Debug,
    T: PartialEq + std::fmt::Debug,
{
    for input in inputs {
        let expected = reference(input);
        let actual = candidate(input);
        assert_eq!(
            expected, actual,
            "implementations disagree on input {input:?}: {expected:?} vs {actual:?}"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::assert_impls_agree;

    /// Numeric: the obvious O(n) loop sum vs the closed-form Gauss
    /// formula — the canonical "reference vs optimized" pair.
    #[test]
    fn closed_form_sum_matches_loop_sum() {
        let reference = |n: &u64| (1..=*n).sum::<u64>();
        let candidate = |n: &u64| n * (n + 1) / 2;
        assert_impls_agree(reference, candidate, &[0, 1, 2, 10, 100, 1_000]);
    }

    /// Parser-adjacent: naive trial division all the way to `n` vs the
    /// classic sqrt-bounded optimization — a case where the naive
    /// version is obviously correct and the optimized one is the one
    /// you'd actually ship.
    #[test]
    fn optimized_is_prime_matches_naive_is_prime() {
        fn naive_is_prime(n: &u64) -> bool {
            if *n < 2 {
                return false;
            }
            (2..*n).all(|d| n % d != 0)
        }

        fn optimized_is_prime(n: &u64) -> bool {
            if *n < 2 {
                return false;
            }
            if n % 2 == 0 {
                return *n == 2;
            }
            let mut d = 3u64;
            while d * d <= *n {
                if n % d == 0 {
                    return false;
                }
                d += 2;
            }
            true
        }

        let mut inputs: Vec<u64> = (0..=100).collect();
        inputs.extend([997, 9_973, 10_000, 104_729]);
        assert_impls_agree(naive_is_prime, optimized_is_prime, &inputs);
    }
}
