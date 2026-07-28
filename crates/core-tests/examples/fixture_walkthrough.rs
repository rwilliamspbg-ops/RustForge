//! Run with:
//!
//! ```bash
//! cargo run -p core-tests --example fixture_walkthrough
//! ```
//!
//! Shows the builder-style fixture pattern that other category crates reuse
//! via `core_tests::default_user_fixture`.

use core_tests::{assert_contains, default_user_fixture};

fn main() {
    let fixture = default_user_fixture()
        .with_username("adopter")
        .with_password("changeme");

    println!("username: {}", fixture.username);
    println!("password: {}", fixture.password);

    assert_contains(&fixture.username, "adopt");
    println!("fixture looks good");
}
