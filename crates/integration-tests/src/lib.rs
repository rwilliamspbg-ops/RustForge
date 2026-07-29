//! End-to-end behavior tests across categories. The library surface here is
//! intentionally minimal — see `tests/e2e.rs` for the actual cross-category
//! integration test, which is what this crate exists to hold.
//!
//! `unsafe_code`/`missing_docs` lint policy: see the workspace `[lints]`
//! table in the root `Cargo.toml`.

/// Returns this test suite's name, for use in cross-category test output.
pub fn suite_name() -> &'static str {
    "RustForge"
}
