#![forbid(unsafe_code)]

pub fn clamp_len(input: &str, max: usize) -> usize {
    input.len().min(max)
}

#[cfg(test)]
mod tests {
    use super::clamp_len;

    #[test]
    fn handles_empty_and_large_inputs() {
        assert_eq!(clamp_len("", 4), 0);
        assert_eq!(clamp_len("abcdef", 4), 4);
    }
}
