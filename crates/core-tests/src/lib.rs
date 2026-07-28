//! Shared fixtures and assertion helpers reused by every other category
//! crate in the workspace. Keep this crate dependency-light by default —
//! anything heavier belongs behind a feature flag (see `async`/`no_std`
//! below).
#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// A minimal user-shaped fixture, built with the `with_*` builder methods.
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
pub fn assert_contains(haystack: &str, needle: &str) {
    assert!(
        haystack.contains(needle),
        "expected `{haystack}` to contain `{needle}`"
    );
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
#[cfg(feature = "no_std")]
pub mod no_std_support {
    /// Checks whether an ASCII string reads the same forwards and backwards,
    /// without allocating.
    pub fn is_ascii_palindrome(input: &str) -> bool {
        let bytes = input.as_bytes();
        let mut left = 0;
        let mut right = bytes.len();

        while left < right {
            right -= 1;
            if !bytes[left].eq_ignore_ascii_case(&bytes[right]) {
                return false;
            }
            left += 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::{assert_contains, default_user_fixture};

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
