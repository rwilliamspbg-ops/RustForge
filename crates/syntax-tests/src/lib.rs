#![forbid(unsafe_code)]

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
