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
