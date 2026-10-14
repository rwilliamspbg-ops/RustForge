//! Golden tests: the fixed input/expected pairs every candidate must
//! match exactly. The cases come from [`eval_harness::GOLDEN_CASES`],
//! derived from the [`Candidate`] contract's reference semantics — they
//! are discriminating by construction (an all-`None` candidate, or one
//! that accepts a leading `-` or `u32::MAX + 1`, fails here).

use candidate::Impl;
use eval_harness::{Candidate, GOLDEN_CASES};

#[test]
fn golden_cases_all_match() {
    let candidate = Impl;
    for (input, expected) in GOLDEN_CASES {
        assert_eq!(
            candidate.parse_u32_lenient(input),
            *expected,
            "input {input:?} should parse to {expected:?}"
        );
    }
}
