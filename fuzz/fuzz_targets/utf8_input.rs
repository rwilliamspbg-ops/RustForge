#![no_main]

use fuzz_tests::utf8_input;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // `utf8_input` must never panic, regardless of whether `data` happens to
    // be valid UTF-8.
    let _ = utf8_input(data);
});
