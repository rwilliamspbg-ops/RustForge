//! Property tests: panic-freedom on arbitrary input, and round-tripping
//! of generated well-formed expressions.

use parser_starter::{eval, EvalError};
use proptest::prelude::*;

proptest! {
    /// No input panics — every failure mode is an `EvalError` variant.
    /// This is the property the `Result`-based API exists to provide.
    #[test]
    fn arbitrary_strings_never_panic(s in ".*") {
        let _ = eval(&s);
    }

    /// Two operands formatted as `a + b` evaluate to their sum — the
    /// round-trip a formatter/parser pair must satisfy.
    #[test]
    fn addition_round_trips(a in any::<i64>(), b in any::<i64>()) {
        let input = format!("{a} + {b}");
        match a.checked_add(b) {
            Some(expected) => prop_assert_eq!(eval(&input), Ok(expected)),
            None => prop_assert_eq!(eval(&input), Err(EvalError::ArithmeticOverflow)),
        }
    }

    /// Same for subtraction, including the i64::MIN edge.
    #[test]
    fn subtraction_round_trips(a in any::<i64>(), b in any::<i64>()) {
        let input = format!("{a} - {b}");
        match a.checked_sub(b) {
            Some(expected) => prop_assert_eq!(eval(&input), Ok(expected)),
            None => prop_assert_eq!(eval(&input), Err(EvalError::ArithmeticOverflow)),
        }
    }
}
