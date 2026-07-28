#![no_main]

use fuzz_tests::parse_u32_lenient;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Only interesting when it's a string at all; invalid UTF-8 as input
    // is already covered by the utf8_input target.
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = parse_u32_lenient(s);
    }
});
