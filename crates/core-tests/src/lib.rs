#![forbid(unsafe_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserFixture {
    pub username: String,
    pub password: String,
}

impl UserFixture {
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = username.into();
        self
    }

    pub fn with_password(mut self, password: impl Into<String>) -> Self {
        self.password = password.into();
        self
    }
}

pub fn default_user_fixture() -> UserFixture {
    UserFixture {
        username: "alice".to_string(),
        password: std::env::var("RUSTFORGE_TEST_PASSWORD").unwrap_or_default(),
    }
}

pub fn assert_contains(haystack: &str, needle: &str) {
    assert!(
        haystack.contains(needle),
        "expected `{haystack}` to contain `{needle}`"
    );
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
}
