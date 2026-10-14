//! Property tests: invariants that must hold for *all* inputs, not just
//! the golden ones — the part of the suite that catches candidates that
//! pass the fixed cases but diverge elsewhere.

use candidate::Impl;
use eval_harness::Candidate;
use proptest::prelude::*;

proptest! {
    /// Every valid `u32` survives formatting plus arbitrary whitespace
    /// padding — the round-trip the reference guarantees.
    #[test]
    fn valid_numbers_round_trip_through_whitespace(
        n in any::<u32>(),
        leading in "[ \t]{0,4}",
        trailing in "[ \t]{0,4}",
    ) {
        let padded = format!("{leading}{n}{trailing}");
        prop_assert_eq!(Impl.parse_u32_lenient(&padded), Some(n));
    }

    /// A leading `+` is accepted, per the contract.
    #[test]
    fn plus_prefix_is_accepted(n in any::<u32>()) {
        prop_assert_eq!(Impl.parse_u32_lenient(&format!("+{n}")), Some(n));
    }

    /// Surrounding whitespace is invisible: parsing `s` equals parsing
    /// `s.trim()` for every string, whatever it contains.
    #[test]
    fn surrounding_whitespace_is_invisible(s in ".*") {
        prop_assert_eq!(
            Impl.parse_u32_lenient(&s),
            Impl.parse_u32_lenient(s.trim())
        );
    }

    /// No input panics, valid number or not.
    #[test]
    fn arbitrary_strings_never_panic(s in ".*") {
        let _ = Impl.parse_u32_lenient(&s);
    }
}
