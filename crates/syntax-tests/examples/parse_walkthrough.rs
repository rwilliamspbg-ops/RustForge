//! Run with:
//!
//! ```bash
//! cargo run -p syntax-tests --example parse_walkthrough
//! ```
//!
//! Walks through the success and failure paths of `parse_source`, the
//! starting point for layering in a real parser/compile-fail tool later.

use syntax_tests::parse_source;

fn main() {
    for source in ["fn main() {}", "let x = 1;", "  "] {
        match parse_source(source) {
            Ok(parsed) => println!("accepted: {parsed:?}"),
            Err(reason) => println!("rejected {source:?}: {reason}"),
        }
    }
}
