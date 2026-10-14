//! Reference candidate for the evaluation harness.
//!
//! Deliberately a *different implementation* from the reference
//! semantics — a hand-rolled digit loop with checked arithmetic rather
//! than `str::parse` — so evaluating it actually demonstrates
//! something: equivalence to the contract is checked, not assumed.
//!
//! `unsafe_code`/`missing_docs` lint policy: mirrors the RustForge
//! workspace's `[lints]` table via the crate attributes below.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// The candidate implementation the harness evaluates.
///
/// The `Impl` name is part of the contract — the harness's tests refer
/// to it as `candidate::Impl` (see `examples/eval-harness/README.md`).
pub struct Impl;

impl eval_harness::Candidate for Impl {
    fn parse_u32_lenient(&self, input: &str) -> Option<u32> {
        let trimmed = input.trim();
        let digits = trimmed.strip_prefix('+').unwrap_or(trimmed);
        if digits.is_empty() {
            return None;
        }
        let mut value: u32 = 0;
        for byte in digits.bytes() {
            if !byte.is_ascii_digit() {
                return None;
            }
            value = value
                .checked_mul(10)?
                .checked_add(u32::from(byte - b'0'))?;
        }
        Some(value)
    }
}
