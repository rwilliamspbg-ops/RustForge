//! Fuzz harness-friendly entry points. The functions here are exercised
//! three ways, from cheapest to most thorough: plain unit tests below,
//! `proptest` property tests behind the `fuzz` feature, and a real
//! `cargo-fuzz`/libFuzzer target in the detached `fuzz/` workspace at the
//! repo root (see `fuzz/README.md`).
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::str::Utf8Error;

/// Validates that `bytes` is well-formed UTF-8, returning the borrowed
/// `&str` view on success. Never panics on any input, valid or not — that
/// guarantee is exactly what the fuzz target in `fuzz/` checks.
pub fn utf8_input(bytes: &[u8]) -> Result<&str, Utf8Error> {
    std::str::from_utf8(bytes)
}

/// Parses `input` as a `u32`, trimming leading/trailing whitespace first.
///
/// "Lenient" refers only to tolerating surrounding whitespace — anything
/// that still doesn't parse cleanly (empty, non-numeric, out of range)
/// returns `None` rather than panicking, on any input.
pub fn parse_u32_lenient(input: &str) -> Option<u32> {
    input.trim().parse().ok()
}

#[cfg(test)]
mod tests {
    use super::{parse_u32_lenient, utf8_input};

    #[test]
    fn valid_utf8_is_accepted() {
        assert_eq!(utf8_input(b"rust").expect("expected utf8"), "rust");
    }

    #[test]
    fn invalid_utf8_is_rejected() {
        assert!(utf8_input(&[0xff]).is_err());
    }

    #[test]
    fn parse_u32_lenient_trims_whitespace() {
        assert_eq!(parse_u32_lenient("  42  "), Some(42));
        assert_eq!(parse_u32_lenient("42"), Some(42));
    }

    #[test]
    fn parse_u32_lenient_rejects_malformed_input() {
        assert_eq!(parse_u32_lenient(""), None);
        assert_eq!(parse_u32_lenient("not a number"), None);
        assert_eq!(parse_u32_lenient("-1"), None);
        assert_eq!(parse_u32_lenient("4294967296"), None); // u32::MAX + 1
    }

    #[cfg(feature = "fuzz")]
    mod property_tests {
        use super::{parse_u32_lenient, utf8_input};
        use proptest::prelude::*;

        proptest! {
            /// Every valid Rust `&str` re-encoded as bytes must round-trip
            /// back through `utf8_input` unchanged.
            #[test]
            fn valid_strings_round_trip(s in ".*") {
                let bytes = s.as_bytes();
                prop_assert_eq!(utf8_input(bytes), Ok(s.as_str()));
            }

            /// `utf8_input` must never panic on arbitrary byte input,
            /// regardless of whether it happens to be valid UTF-8.
            #[test]
            fn arbitrary_bytes_never_panic(bytes in proptest::collection::vec(any::<u8>(), 0..64)) {
                let _ = utf8_input(&bytes);
            }

            /// Every valid `u32`, formatted as a string and padded with
            /// arbitrary ASCII whitespace, must parse back to itself.
            #[test]
            fn valid_numbers_round_trip_through_whitespace(
                n in any::<u32>(),
                leading in "[ \t]{0,4}",
                trailing in "[ \t]{0,4}",
            ) {
                let padded = format!("{leading}{n}{trailing}");
                prop_assert_eq!(parse_u32_lenient(&padded), Some(n));
            }

            /// `parse_u32_lenient` must never panic on arbitrary string
            /// input, valid number or not.
            #[test]
            fn arbitrary_strings_never_panic(s in ".*") {
                let _ = parse_u32_lenient(&s);
            }
        }
    }
}
