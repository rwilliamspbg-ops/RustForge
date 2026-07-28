//! Compile-fail (UI) testing, opt-in via the `compile-fail` feature.
//!
//! `syntax-tests`'s own `parse_source` fails at *runtime* (it returns
//! `Result`), which is what the unit tests in `src/lib.rs` cover. `trybuild`
//! is for the other kind of syntax check: code that must fail to *compile*
//! — the shape of check a real parser/macro/DSL crate layered on top of
//! this template would need. These fixtures are deliberately generic
//! (a plain type mismatch) so the expected `rustc` diagnostic in
//! `tests/ui/fail/*.stderr` stays stable across compiler versions; ui tests
//! run in CI on stable only (see `.github/workflows/ci.yml`), since
//! diagnostic rendering can otherwise drift between toolchains.
#![cfg(feature = "compile-fail")]

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/pass/*.rs");
    t.compile_fail("tests/ui/fail/*.rs");
}
