//! Boundary-value and robustness checks — the kind of off-by-one, overflow,
//! and multi-byte-character gotchas that are easy to miss until production.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Returns `input`'s byte length, capped at `max`.
///
/// This is a *byte count*, not a char-safe truncation point — see
/// [`safe_truncate`] for the gotcha that creates and how to avoid it.
pub fn clamp_len(input: &str, max: usize) -> usize {
    input.len().min(max)
}

/// Truncates `input` to at most `max` bytes, walking backward to the nearest
/// UTF-8 char boundary so the result is always a valid `&str`.
///
/// `clamp_len` above only returns a *byte count*; naively slicing
/// `&input[..clamp_len(input, max)]` can panic when `max` lands inside a
/// multi-byte character. `safe_truncate` exists to demonstrate the fix.
pub fn safe_truncate(input: &str, max: usize) -> &str {
    if input.len() <= max {
        return input;
    }

    let mut boundary = max;
    while boundary > 0 && !input.is_char_boundary(boundary) {
        boundary -= 1;
    }

    &input[..boundary]
}

/// Returns at most `max` elements of `items`, cloned into a new `Vec`.
///
/// The collection counterpart to [`clamp_len`]/[`safe_truncate`]: unlike
/// byte-slicing a string, slicing a `&[T]` at an arbitrary index never
/// panics (there's no "char boundary" to violate), so this one has no
/// gotcha to fix — it's here as the boundary-value counterexample: `0`,
/// `1`, and "more than available" all just work.
pub fn clamp_collection<T: Clone>(items: &[T], max: usize) -> Vec<T> {
    items[..items.len().min(max)].to_vec()
}

/// Stricter arithmetic boundary checks, opt-in via the `edge` feature for
/// suites that want to assert against `usize` underflow/overflow explicitly
/// rather than relying on debug-only panics.
#[cfg(feature = "edge")]
pub mod overflow_checks {
    /// Subtracts `b` from `a`, returning `None` instead of underflowing —
    /// the classic `usize` boundary bug (`a - b` panics in debug builds and
    /// silently wraps in release builds when `a < b`).
    pub fn checked_distance(a: usize, b: usize) -> Option<usize> {
        a.checked_sub(b)
    }

    /// Adds `delta` to `base`, returning `None` instead of silently wrapping
    /// past `usize::MAX`.
    pub fn checked_offset(base: usize, delta: usize) -> Option<usize> {
        base.checked_add(delta)
    }
}

#[cfg(test)]
mod tests {
    use super::{clamp_collection, clamp_len, safe_truncate};

    #[test]
    fn handles_empty_and_large_inputs() {
        assert_eq!(clamp_len("", 4), 0);
        assert_eq!(clamp_len("abcdef", 4), 4);
    }

    #[test]
    fn clamp_len_can_land_mid_char_boundary() {
        // "é" is 2 bytes in UTF-8; a max of 1 lands inside it. Slicing
        // `&input[..clamp_len(input, 1)]` directly would panic here, which
        // is exactly the gotcha `safe_truncate` avoids.
        let input = "é";
        assert_eq!(clamp_len(input, 1), 1);
        assert!(!input.is_char_boundary(1));
    }

    #[test]
    fn safe_truncate_never_splits_a_char() {
        let input = "héllo";
        assert_eq!(safe_truncate(input, 0), "");
        assert_eq!(safe_truncate(input, 1), "h");
        // Byte 2 lands inside the 2-byte "é"; safe_truncate backs off to the
        // previous boundary instead of panicking.
        assert_eq!(safe_truncate(input, 2), "h");
        assert_eq!(safe_truncate(input, 3), "hé");
        assert_eq!(safe_truncate(input, 100), input);
    }

    #[test]
    fn safe_truncate_handles_empty_input() {
        assert_eq!(safe_truncate("", 4), "");
    }

    #[test]
    fn clamp_collection_boundary_values() {
        let items = vec![1, 2, 3, 4, 5];

        assert_eq!(clamp_collection(&items, 0), Vec::<i32>::new());
        assert_eq!(clamp_collection(&items, 1), vec![1]);
        assert_eq!(clamp_collection(&items, items.len()), items);
        assert_eq!(clamp_collection(&items, items.len() + 100), items);
        assert_eq!(clamp_collection::<i32>(&[], 5), Vec::<i32>::new());
    }

    #[cfg(feature = "edge")]
    mod property_tests {
        use super::super::{clamp_collection, overflow_checks::checked_offset, safe_truncate};
        use proptest::prelude::*;

        proptest! {
            /// `safe_truncate` must never panic and must always return valid
            /// UTF-8 no more than `max` bytes long, for any string and any
            /// truncation point.
            #[test]
            fn safe_truncate_never_panics_and_stays_within_max(s in ".*", max in 0usize..64) {
                let truncated = safe_truncate(&s, max);
                prop_assert!(truncated.len() <= max);
                prop_assert!(s.starts_with(truncated));
            }

            /// `clamp_collection` never returns more elements than were
            /// asked for, and never more than existed in the first place.
            #[test]
            fn clamp_collection_respects_both_bounds(items in proptest::collection::vec(any::<i32>(), 0..32), max in 0usize..32) {
                let clamped = clamp_collection(&items, max);
                prop_assert!(clamped.len() <= max);
                prop_assert!(clamped.len() <= items.len());
                prop_assert_eq!(&clamped[..], &items[..clamped.len()]);
            }

            /// `checked_offset` must agree with plain addition whenever
            /// plain addition wouldn't overflow.
            #[test]
            fn checked_offset_matches_addition_when_no_overflow(base in 0usize..usize::MAX / 2, delta in 0usize..usize::MAX / 2) {
                prop_assert_eq!(checked_offset(base, delta), Some(base + delta));
            }
        }
    }

    #[cfg(feature = "edge")]
    mod overflow_tests {
        use super::super::overflow_checks::{checked_distance, checked_offset};

        #[test]
        fn checked_distance_rejects_underflow() {
            assert_eq!(checked_distance(5, 3), Some(2));
            assert_eq!(checked_distance(0, 1), None);
        }

        #[test]
        fn checked_offset_rejects_overflow_at_usize_max() {
            assert_eq!(checked_offset(usize::MAX - 1, 1), Some(usize::MAX));
            assert_eq!(checked_offset(usize::MAX, 1), None);
        }
    }
}
