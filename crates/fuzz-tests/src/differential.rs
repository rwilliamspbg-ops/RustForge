//! Property-based differential testing — the `fuzz`-feature companion to
//! `core_tests::differential::assert_impls_agree`: same idea (two
//! implementations that must agree), but inputs are *generated* by a
//! `proptest` strategy instead of listed by hand, so the two
//! implementations are checked across a much wider input space.
//!
//! Reach for this when enumerating inputs by hand can't cover the space
//! (the reference is too slow to enumerate past a few thousand values,
//! or the input space is genuinely large), and for the cargo-fuzz flavor
//! see "Differential fuzzing" in `docs/fuzzing.md`.

use proptest::prelude::*;
use proptest::test_runner::TestRunner;

/// Asserts `reference` and `candidate` agree for every input `strategy`
/// generates — the property-based form of differential testing.
///
/// Keep `reference` deliberately simple and obviously correct, even if
/// slow — its only job is to be trustworthy, not fast. A disagreement
/// fails the property test with the offending input and both results;
/// proptest then shrinks to a minimal failing case.
///
/// # Examples
///
/// ```rust
/// use fuzz_tests::differential::assert_impls_agree;
///
/// // Reference: the obvious O(n) loop. Candidate: the closed form.
/// let reference = |n: &u64| (1..=*n).sum::<u64>();
/// let candidate = |n: &u64| n * (n + 1) / 2;
/// assert_impls_agree(reference, candidate, 0u64..100);
/// ```
pub fn assert_impls_agree<I, T, S>(
    reference: impl Fn(&I) -> T,
    candidate: impl Fn(&I) -> T,
    strategy: S,
) where
    I: std::fmt::Debug,
    T: PartialEq + std::fmt::Debug,
    S: Strategy<Value = I>,
{
    let mut runner = TestRunner::default();
    runner
        .run(&strategy, |input| {
            let expected = reference(&input);
            let actual = candidate(&input);
            // The input is passed as an explicit format argument on
            // purpose: `prop_assert_eq!` expands its format string
            // through `concat!`, and `format_args!` refuses to capture
            // locals in that context. The compared values themselves
            // don't need passing — the macro moves them into the
            // comparison and prints both on failure.
            prop_assert_eq!(
                expected,
                actual,
                "implementations disagree on input {:?}",
                input
            );
            Ok(())
        })
        .unwrap_or_else(|err| panic!("differential property failed: {err}"));
}

#[cfg(test)]
mod tests {
    use super::assert_impls_agree;

    /// The property-based version of the numeric differential test in
    /// `core-tests`: the closed-form sum vs the loop, over *generated*
    /// inputs instead of a fixed list.
    #[test]
    fn closed_form_sum_matches_loop_sum_over_generated_inputs() {
        let reference = |n: &u64| (1..=*n).sum::<u64>();
        let candidate = |n: &u64| n * (n + 1) / 2;
        assert_impls_agree(reference, candidate, 0u64..1_000);
    }

    /// Naive vs optimized primality over generated inputs — the case
    /// where generation earns its keep: the naive version is far too
    /// slow to enumerate by hand past a few thousand values.
    #[test]
    fn optimized_is_prime_matches_naive_is_prime_over_generated_inputs() {
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

        assert_impls_agree(naive_is_prime, optimized_is_prime, 0u64..10_000);
    }
}
