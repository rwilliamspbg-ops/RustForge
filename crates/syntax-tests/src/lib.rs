//! Parser/syntax-facing tests. `parse_source` is a deliberately toy stand-in
//! for a real parser's front door — swap it for your own tokenizer/parser
//! entry point and keep the pass/fail test shape. For genuine compile-fail
//! testing (checking that some input fails to *compile*, not just that a
//! function returns `Err`), see the `compile-fail` feature and
//! `tests/compile_fail.rs`.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

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
}
