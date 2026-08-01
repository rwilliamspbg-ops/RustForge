//! Compiles `src/no_std_support.rs`'s source inside a crate genuinely marked
//! `#![no_std]`, so the "usable under `#![no_std]`" claim on that module
//! (see `src/lib.rs`) is enforced by the compiler on every `cargo test
//! --features no_std` run, rather than being true only by convention/review.
//!
//! This runs on the normal host target, where `std` is always available —
//! it checks the `#![no_std]` boundary itself (does this code reference
//! `std`?), not cross-compilation to an embedded target. A general
//! cross-compilation matrix is deliberately out of scope for this template;
//! see the WASM-target deferral in `CHANGELOG.md` for the same reasoning
//! applied elsewhere.
#![no_std]

#[path = "../src/no_std_support.rs"]
mod no_std_support;

use no_std_support::is_ascii_palindrome;

#[test]
fn palindrome_logic_compiles_and_works_under_no_std() {
    assert!(is_ascii_palindrome("Level"));
    assert!(is_ascii_palindrome(""));
    assert!(!is_ascii_palindrome("rust"));
}
