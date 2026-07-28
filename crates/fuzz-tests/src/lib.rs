#![forbid(unsafe_code)]

use std::str::Utf8Error;

pub fn utf8_input(bytes: &[u8]) -> Result<&str, Utf8Error> {
    std::str::from_utf8(bytes)
}

#[cfg(test)]
mod tests {
    use super::utf8_input;

    #[test]
    fn valid_utf8_is_accepted() {
        assert_eq!(utf8_input(b"rust").expect("expected utf8"), "rust");
    }

    #[test]
    fn invalid_utf8_is_rejected() {
        assert!(utf8_input(&[0xff]).is_err());
    }

    #[cfg(feature = "fuzz")]
    mod property_tests {
        use super::utf8_input;
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
        }
    }
}
