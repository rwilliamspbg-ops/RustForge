use core_tests::default_user_fixture;
use edge_cases::safe_truncate;
use fuzz_tests::utf8_input;
use performance_tests::sum;
use semantic_tests::shared_counter_after_workers;
use syntax_tests::parse_source;

#[test]
fn end_to_end_smoke_test() {
    let fixture = default_user_fixture().with_username("integration-user");
    assert_eq!(fixture.username, "integration-user");

    assert!(parse_source("fn smoke() {}").is_ok());
    assert_eq!(shared_counter_after_workers(4), 4);
}

#[test]
fn categories_compose_across_a_single_pipeline() {
    // syntax: validate a source snippet before "compiling" it further.
    let source = parse_source("fn pipeline() {}").expect("valid source");

    // fuzz: the same bytes must always be valid UTF-8 coming out of the
    // syntax stage.
    let reparsed = utf8_input(source.as_bytes()).expect("source is valid utf8");
    assert_eq!(reparsed, source);

    // edge-cases: truncating for a UI preview must not split a character
    // even when the source contains multi-byte identifiers.
    let unicode_source = "fn café() {}";
    let preview = safe_truncate(unicode_source, 6);
    assert!(unicode_source.starts_with(preview));

    // semantic: multiple workers can safely share state derived from the
    // pipeline above.
    assert_eq!(shared_counter_after_workers(4), 4);

    // performance: the throughput-sensitive path used by adopters wiring in
    // `performance-tests` still returns correct results.
    let sizes = vec![1u64, 2, 3, 4, 5];
    assert_eq!(sum(&sizes), 15);
}
