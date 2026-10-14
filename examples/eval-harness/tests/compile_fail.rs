//! Compile-fail: the contract is enforced at *compile* time. A candidate
//! that doesn't implement [`Candidate`] fails to build with an error
//! naming the contract, instead of a confusing "method not found" at
//! test time or a suite that silently can't run.
//!
//! trybuild here is NOT feature-gated (unlike `syntax-tests`'
//! `compile-fail` feature): this crate is copied per evaluation, so the
//! check is part of the suite every time. The fixture is deliberately
//! generic — it asserts on the trait-bound diagnostic, which is far more
//! stable across compiler versions than exact rendered text.

#[test]
fn candidate_must_implement_the_contract() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/candidate_not_implemented.rs");
}
