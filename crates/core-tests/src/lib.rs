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
}
