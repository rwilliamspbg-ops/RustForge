use core_tests::default_user_fixture;
use semantic_tests::shared_counter_after_workers;
use syntax_tests::parse_source;

#[test]
fn end_to_end_smoke_test() {
    let fixture = default_user_fixture().with_username("integration-user");
    assert_eq!(fixture.username, "integration-user");

    assert!(parse_source("fn smoke() {}").is_ok());
    assert_eq!(shared_counter_after_workers(4), 4);
}
