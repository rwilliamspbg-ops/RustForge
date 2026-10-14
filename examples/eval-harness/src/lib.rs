//! Code-under-test evaluation harness.
//!
//! RustForge's category crates test *the template's own* fixtures. This
//! crate is the other half of the story: a fixed, copyable contract you
//! evaluate an *external* implementation against — a library you're
//! validating, an AI-generated implementation, a candidate rewrite —
//! without touching that code.
//!
//! The pattern, in full:
//!
//! 1. **The contract** ([`Candidate`]) is a trait with fixed semantics,
//!    documented against a reference implementation. It never changes
//!    per candidate — that's the point.
//! 2. **The candidate** is any crate that implements the contract for a
//!    `pub struct Impl`. It can be a thin wrapper around existing code
//!    (see [`FnCandidate`] for the function case) — you never have to
//!    modify the code under test.
//! 3. **The suite** ([`GOLDEN_CASES`] plus the golden, property, and
//!    compile-fail tests under `tests/`) runs against whatever candidate
//!    is wired in.
//!
//! The `eval` recipe in the justfile (`just eval <path>`) does the
//! wiring: it copies this crate and the candidate into a scratch
//! workspace, points them at each other, and runs the suite. See
//! `README.md` and `docs/evaluation-harness.md` for the full flow.
//!
//! This crate is a standalone workspace of its own (see its `[workspace]`
//! table) — it is deliberately *not* a member of the RustForge
//! workspace, so it adds nothing to the template's default
//! `cargo test --workspace` or its dependency graph.
//!
//! `unsafe_code`/`missing_docs` lint policy: mirrors the RustForge
//! workspace's `[lints]` table via the crate attributes below.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// The fixed contract every candidate must implement.
///
/// Semantics are those of the reference implementation
/// `input.trim().parse::<u32>().ok()`:
///
/// - Leading/trailing whitespace is trimmed first (Unicode whitespace,
///   like [`str::trim`]).
/// - An optional single leading `+` is accepted (`"+42"` parses).
/// - A leading `-`, empty input, non-digit characters (including
///   interior whitespace), and values above [`u32::MAX`] all return
///   `None` — never a panic, on any input.
///
/// A candidate implements this for a unit struct named `Impl`; the suite
/// in `tests/` refers to it as `candidate::Impl`. The [`FnCandidate`]
/// adapter covers candidates whose real API doesn't match this shape.
pub trait Candidate {
    /// Parses `input` as a `u32`, tolerating surrounding whitespace.
    /// See the [trait documentation](Candidate) for the exact contract.
    fn parse_u32_lenient(&self, input: &str) -> Option<u32>;
}

/// Newtype adapter that turns a plain function into a [`Candidate`].
///
/// The "thin adapter layer" for candidates whose real API doesn't match
/// the trait shape — wrap their entry point and it evaluates like any
/// other candidate, with no changes to the code under test.
///
/// # Examples
///
/// ```rust
/// use eval_harness::{Candidate, FnCandidate};
///
/// // Some existing API that parses its own way:
/// fn their_parse(input: &str) -> Option<u32> {
///     input.trim().parse().ok()
/// }
///
/// let wrapped = FnCandidate(their_parse);
/// assert_eq!(wrapped.parse_u32_lenient(" 42 "), Some(42));
/// ```
pub struct FnCandidate(pub fn(&str) -> Option<u32>);

impl Candidate for FnCandidate {
    fn parse_u32_lenient(&self, input: &str) -> Option<u32> {
        (self.0)(input)
    }
}

/// The fixed golden cases: `(input, expected)` pairs derived from the
/// reference semantics in the [`Candidate`] contract.
///
/// Every candidate must produce exactly these results. The set is
/// deliberately discriminating — an implementation that returns `None`
/// for everything, or accepts what the reference rejects (a leading `-`,
/// interior whitespace, `u32::MAX + 1`), fails here.
pub const GOLDEN_CASES: &[(&str, Option<u32>)] = &[
    // Canonical and padded numbers.
    ("42", Some(42)),
    ("0", Some(0)),
    ("4294967295", Some(u32::MAX)),
    ("  42  ", Some(42)),
    ("\t42\n", Some(42)),
    // A leading `+` is accepted...
    ("+42", Some(42)),
    // ...but only one, and only before the digits.
    ("++42", None),
    ("+", None),
    ("+ 42", None),
    // Negatives, empties, and non-numbers are rejected.
    ("-1", None),
    ("", None),
    ("   ", None),
    ("abc", None),
    ("42abc", None),
    ("4 2", None),
    // Overflow is rejected, not wrapped.
    ("4294967296", None),
    ("18446744073709551616", None),
];
