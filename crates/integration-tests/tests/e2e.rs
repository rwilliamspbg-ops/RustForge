use core_tests::{default_config_fixture, default_user_fixture, retry};
use edge_cases::{clamp_collection, safe_truncate};
use fuzz_tests::{parse_u32_lenient, utf8_input};
use performance_tests::{dedup_sorted, max_value, sum};
use semantic_tests::{longest, parse_timeout_ms, shared_counter_after_workers};
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

/// A second pipeline, built from the functions added since the first one:
/// parse a batch of raw config values (fuzz-tests), page it down to a
/// bounded batch size (edge-cases), summarize it (performance-tests), then
/// use the summary to configure a fixture fetched through a flaky retry
/// (core-tests) and validate its fields (semantic-tests).
#[test]
fn config_batch_pipeline_composes_across_categories() {
    // fuzz-tests: lenient parsing tolerates the surrounding whitespace a
    // real config file would have, and simply drops anything malformed.
    let raw_values = ["  10", "not-a-number", "25", "7", "", "100"];
    let parsed: Vec<u32> = raw_values
        .iter()
        .filter_map(|s| parse_u32_lenient(s))
        .collect();
    assert_eq!(parsed, vec![10, 25, 7, 100]);

    // edge-cases: page the batch down to at most 3 entries, the same
    // clamping pattern used for strings but applied to a collection.
    let page: Vec<u64> = clamp_collection(&parsed, 3)
        .into_iter()
        .map(u64::from)
        .collect();
    assert_eq!(page, vec![10, 25, 7]);

    // performance-tests: summarize the page.
    assert_eq!(max_value(&page), Some(25));
    let mut sorted_page = page.clone();
    sorted_page.sort_unstable();
    assert_eq!(dedup_sorted(&sorted_page), vec![7, 10, 25]);

    // core-tests: fetching the resolved config fixture is flaky and
    // succeeds on the second attempt.
    let mut attempts = 0;
    let config = retry(3, || {
        attempts += 1;
        if attempts < 2 {
            Err("fixture not ready")
        } else {
            Ok(default_config_fixture().with_timeout_ms(max_value(&page).unwrap_or(0)))
        }
    })
    .expect("retry should eventually succeed");
    assert_eq!(config.timeout_ms, 25);

    // semantic-tests: the fixture's timeout round-trips through the same
    // "key=value" parsing pattern used for real config files, and picking
    // a display label uses the lifetime-bounded `longest` helper.
    let serialized = format!("timeout_ms={}", config.timeout_ms);
    assert_eq!(parse_timeout_ms(&serialized), Ok(25));
    assert_eq!(longest(&config.base_url, "cfg"), config.base_url);
}
