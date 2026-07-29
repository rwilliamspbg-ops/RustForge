//! Parser/syntax-facing tests. `parse_source` is a deliberately toy stand-in
//! for a real parser's front door — swap it for your own tokenizer/parser
//! entry point and keep the pass/fail test shape. For genuine compile-fail
//! testing (checking that some input fails to *compile*, not just that a
//! function returns `Err`), see the `compile-fail` feature and
//! `tests/compile_fail.rs`.
//!
//! `unsafe_code`/`missing_docs` lint policy: see the workspace `[lints]`
//! table in the root `Cargo.toml`.

/// Validates that `source` is non-empty and contains a function declaration.
///
/// Returns the trimmed source on success, or a static error message
/// describing which check failed.
pub fn parse_source(source: &str) -> Result<&str, &'static str> {
    let trimmed = source.trim();
    if trimmed.is_empty() {
        return Err("source must not be empty");
    }

    if !trimmed.contains("fn") {
        return Err("source must contain a function declaration");
    }

    Ok(trimmed)
}

/// A structural summary of already-validated source (see [`parse_source`]),
/// meant to be snapshot-tested (behind the `snapshot` feature) rather than
/// asserted field-by-field. Snapshot testing earns its keep once a type has
/// enough fields that per-field assertions get unwieldy and start hiding
/// the actual diff when something changes — the snapshot shows the whole
/// shape at once, and `cargo insta review` turns an intentional change
/// into a one-keystroke snapshot update instead of hand-editing assertions.
#[derive(Debug)]
pub struct SourceSummary<'a> {
    /// The source text this summary describes.
    pub trimmed: &'a str,
    /// `trimmed`'s length in bytes.
    pub byte_len: usize,
    /// Whether `trimmed` contains the `fn` keyword.
    pub contains_fn: bool,
    /// Number of lines in `trimmed`.
    pub line_count: usize,
}

/// Builds a [`SourceSummary`] for `source`.
pub fn summarize_source(source: &str) -> SourceSummary<'_> {
    SourceSummary {
        trimmed: source,
        byte_len: source.len(),
        contains_fn: source.contains("fn"),
        line_count: source.lines().count(),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_source;

    #[test]
    fn parse_source_accepts_simple_function() {
        let parsed = parse_source("fn main() {} ").expect("expected parse success");
        assert_eq!(parsed, "fn main() {}");
    }

    #[test]
    fn parse_source_rejects_empty_input() {
        assert_eq!(parse_source(" \n\t"), Err("source must not be empty"));
    }

    #[test]
    fn parse_source_rejects_missing_function_keyword() {
        assert_eq!(
            parse_source("let x = 1;"),
            Err("source must contain a function declaration")
        );
    }

    #[cfg(feature = "snapshot")]
    mod snapshot_tests {
        use super::super::{parse_source, summarize_source};

        #[test]
        fn summary_of_simple_function() {
            let source = parse_source("fn main() {}").expect("valid source");
            insta::assert_debug_snapshot!(summarize_source(source));
        }

        #[test]
        fn summary_of_multiline_function_with_params() {
            let source = parse_source("fn add(a: i32, b: i32) -> i32 {\n    a + b\n}")
                .expect("valid source");
            insta::assert_debug_snapshot!(summarize_source(source));
        }
    }
}
