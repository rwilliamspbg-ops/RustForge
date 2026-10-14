// A `parse_cases!` row without its `=> expected` part must fail to
// compile — and with a message that says what a row should look like,
// not the classic macro_rules "no rules expected" wall. The macro's
// fallback arm (see `crates/syntax-tests/src/lib.rs`) emits that
// message via `compile_error!`, which makes this fixture's expected
// diagnostic OUR text — stable across compiler versions, unlike the
// rendered rustc errors the type-mismatch fixtures assert on.

use syntax_tests::parse_cases;

parse_cases! {
    missing_expected: "fn f() {}" ;
}

fn main() {}
