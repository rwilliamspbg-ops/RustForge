//! Shared fixtures and assertion helpers reused by every other category
//! crate in the workspace. Keep this crate dependency-light by default —
//! anything heavier belongs behind a feature flag (see `async`/`no_std`
//! below).
//!
//! `unsafe_code`/`missing_docs` lint policy: see the workspace `[lints]`
//! table in the root `Cargo.toml`.

/// A minimal user-shaped fixture, built with the `with_*` builder methods.
///
/// # Examples
///
/// ```
/// use core_tests::default_user_fixture;
///
/// let fixture = default_user_fixture().with_username("bob");
/// assert_eq!(fixture.username, "bob");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserFixture {
    /// The fixture's username.
    pub username: String,
    /// The fixture's password. Empty by default — see [`default_user_fixture`].
    pub password: String,
}

impl UserFixture {
    /// Returns a copy of this fixture with a different username.
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = username.into();
        self
    }

    /// Returns a copy of this fixture with a different password.
    pub fn with_password(mut self, password: impl Into<String>) -> Self {
        self.password = password.into();
        self
    }
}

/// Builds the default [`UserFixture`].
///
/// The password comes from the `RUSTFORGE_TEST_PASSWORD` environment
/// variable, defaulting to empty, rather than a hardcoded literal — so
/// fixtures never carry a real-looking secret into test output or version
/// control.
pub fn default_user_fixture() -> UserFixture {
    UserFixture {
        username: "alice".to_string(),
        password: std::env::var("RUSTFORGE_TEST_PASSWORD").unwrap_or_default(),
    }
}

/// Asserts that `haystack` contains `needle`, with a message naming both.
///
/// # Examples
///
/// ```
/// core_tests::assert_contains("semantic::ownership", "ownership");
/// ```
///
/// # Panics
///
/// Panics if `haystack` does not contain `needle`.
pub fn assert_contains(haystack: &str, needle: &str) {
    assert!(
        haystack.contains(needle),
        "expected `{haystack}` to contain `{needle}`"
    );
}

/// A minimal "environment config" fixture, built with the `with_*` builder
/// methods — the same pattern as [`UserFixture`], for tests that need
/// config-shaped rather than user-shaped data.
///
/// # Examples
///
/// ```
/// use core_tests::default_config_fixture;
///
/// let config = default_config_fixture().with_timeout_ms(500).with_retries(1);
/// assert_eq!(config.timeout_ms, 500);
/// assert_eq!(config.retries, 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigFixture {
    /// The base URL a test client under test would target.
    pub base_url: String,
    /// Request timeout, in milliseconds.
    pub timeout_ms: u64,
    /// Number of retries before giving up.
    pub retries: u32,
}

impl ConfigFixture {
    /// Returns a copy of this fixture with a different base URL.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Returns a copy of this fixture with a different timeout.
    pub fn with_timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    /// Returns a copy of this fixture with a different retry count.
    pub fn with_retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }
}

/// Builds the default [`ConfigFixture`]: `localhost`, a 5-second timeout,
/// 3 retries.
pub fn default_config_fixture() -> ConfigFixture {
    ConfigFixture {
        base_url: "http://localhost:8080".to_string(),
        timeout_ms: 5_000,
        retries: 3,
    }
}

/// Calls `attempt` up to `max_tries` times, returning the first `Ok` or the
/// last `Err` once every attempt has been exhausted.
///
/// A small, dependency-free stand-in for retrying flaky operations in
/// tests — e.g. polling a fixture that becomes ready asynchronously,
/// without needing an async runtime.
///
/// # Examples
///
/// ```
/// use core_tests::retry;
///
/// let mut calls = 0;
/// let result: Result<u32, &str> = retry(3, || {
///     calls += 1;
///     if calls < 2 { Err("not ready yet") } else { Ok(calls) }
/// });
///
/// assert_eq!(result, Ok(2));
/// ```
///
/// # Panics
///
/// Panics if `max_tries` is `0`.
pub fn retry<T, E>(max_tries: usize, mut attempt: impl FnMut() -> Result<T, E>) -> Result<T, E> {
    assert!(max_tries > 0, "max_tries must be at least 1");

    let mut last_err = None;
    for _ in 0..max_tries {
        match attempt() {
            Ok(value) => return Ok(value),
            Err(err) => last_err = Some(err),
        }
    }

    Err(last_err.expect("loop runs at least once since max_tries > 0"))
}

/// Async-friendly accessors for fixtures, for adopters building `tokio`-based
/// async test suites. Kept separate from `semantic-tests`, which tests async
/// *ownership semantics*; this module is about fixture ergonomics.
#[cfg(feature = "async")]
pub mod async_support {
    use super::{default_user_fixture, UserFixture};

    /// Async-friendly accessor for the default fixture. Yields once before
    /// returning so it behaves like a real async call (e.g. fetching a
    /// fixture from a test database) rather than a sync function wearing an
    /// `async` label.
    pub async fn default_user_fixture_async() -> UserFixture {
        tokio::task::yield_now().await;
        default_user_fixture()
    }

    /// Loads one fixture per username concurrently, demonstrating how
    /// adopters can fan out async fixture setup across tasks.
    pub async fn load_user_fixtures(usernames: &[&str]) -> Vec<UserFixture> {
        let handles: Vec<_> = usernames
            .iter()
            .map(|&name| {
                let name = name.to_string();
                tokio::spawn(async move { default_user_fixture_async().await.with_username(name) })
            })
            .collect();

        let mut fixtures = Vec::with_capacity(handles.len());
        for handle in handles {
            fixtures.push(handle.await.expect("fixture task should not panic"));
        }
        fixtures
    }
}

/// Helpers that only touch `core` items, so the pattern stays usable if an
/// adopter compiles this crate under `#![no_std]` (e.g. for embedded
/// targets). The rest of `core-tests` keeps using `std` for fixture
/// convenience; this module is where `no_std`-safe helpers should live.
///
/// That claim is enforced by the compiler, not just by convention: `tests/
/// no_std_check.rs` re-includes this file's source inside a crate genuinely
/// marked `#![no_std]`, so a future edit that sneaks in a `std` reference
/// fails to build instead of just looking fine on review.
#[cfg(feature = "no_std")]
pub mod no_std_support;

#[cfg(test)]
mod tests {
    use super::{assert_contains, default_config_fixture, default_user_fixture, retry};

    #[test]
    fn default_fixture_can_be_customized() {
        let fixture = default_user_fixture()
            .with_username("bob")
            .with_password("secret");

        assert_eq!(fixture.username, "bob");
        assert_eq!(fixture.password, "secret");
    }

    #[test]
    fn assert_contains_reports_mismatch() {
        assert_contains("semantic::ownership", "ownership");
    }

    #[test]
    fn config_fixture_can_be_customized() {
        let config = default_config_fixture()
            .with_base_url("https://example.test")
            .with_timeout_ms(250)
            .with_retries(0);

        assert_eq!(config.base_url, "https://example.test");
        assert_eq!(config.timeout_ms, 250);
        assert_eq!(config.retries, 0);
    }

    #[test]
    fn retry_returns_last_error_when_exhausted() {
        let result: Result<(), &str> = retry(2, || Err("still failing"));
        assert_eq!(result, Err("still failing"));
    }

    #[test]
    #[should_panic(expected = "max_tries must be at least 1")]
    fn retry_rejects_zero_max_tries() {
        let _: Result<(), &str> = retry(0, || Ok(()));
    }

    /// Table-driven test pattern: one assertion body, many cases. Prefer
    /// this over copy-pasted near-identical `#[test]` functions once you
    /// have more than two or three variations of the same behavior.
    mod retry_table_driven_examples {
        use super::retry;

        #[test]
        fn succeeds_or_fails_depending_on_max_tries_vs_attempts_needed() {
            let cases = [
                // (max_tries, succeeds_on_attempt, expect_success)
                (1, 1, true),
                (3, 2, true),
                (3, 3, true),
                (2, 3, false),
            ];

            for (max_tries, succeeds_on_attempt, expect_success) in cases {
                let mut calls = 0;
                let result: Result<u32, &str> = retry(max_tries, || {
                    calls += 1;
                    if calls >= succeeds_on_attempt {
                        Ok(calls)
                    } else {
                        Err("not ready yet")
                    }
                });

                assert_eq!(
                    result.is_ok(),
                    expect_success,
                    "max_tries={max_tries}, succeeds_on_attempt={succeeds_on_attempt}"
                );
            }
        }
    }

    #[cfg(feature = "no_std")]
    #[test]
    fn no_std_palindrome_check_ignores_case() {
        use super::no_std_support::is_ascii_palindrome;

        assert!(is_ascii_palindrome("Level"));
        assert!(is_ascii_palindrome(""));
        assert!(!is_ascii_palindrome("rust"));
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn async_fixture_matches_sync_default() {
        use super::async_support::default_user_fixture_async;

        let fixture = default_user_fixture_async().await;
        assert_eq!(fixture, default_user_fixture());
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn concurrent_fixture_loading_preserves_order() {
        use super::async_support::load_user_fixtures;

        let fixtures = load_user_fixtures(&["alice", "bob", "carol"]).await;
        let usernames: Vec<_> = fixtures.iter().map(|f| f.username.as_str()).collect();
        assert_eq!(usernames, ["alice", "bob", "carol"]);
    }
}
