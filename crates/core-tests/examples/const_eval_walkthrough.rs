//! Compile-time computation, checked at compile time.
//!
//! Run with:
//!
//! ```bash
//! cargo run -p core-tests --example const_eval_walkthrough
//! ```
//!
//! Run the runtime half (and see the compile-time half in action):
//!
//! ```bash
//! cargo test -p core-tests --example const_eval_walkthrough
//! ```
//!
//! The pattern in three parts:
//!
//! 1. A `const fn` — computation the compiler evaluates at compile time.
//! 2. `const _: () = assert!(...)` — *static assertions*, the
//!    const-testing primitive: a broken const fn fails the **build**,
//!    not a test run. Every `--all-targets` build checks them (that
//!    includes CI's clippy step and `cargo build --examples`).
//! 3. Runtime `#[test]`s pinning the same values — the const
//!    assertions catch accidental breakage; the runtime tests catch
//!    *deliberate* changes made (and reviewed) later, and give failures
//!    a readable location.

/// Compile-time factorial — the canonical const-fn example.
const fn factorial(n: u64) -> u64 {
    let mut result = 1;
    let mut i = 2;
    while i <= n {
        result *= i;
        i += 1;
    }
    result
}

/// Compile-time Fibonacci — the two-variable loop variant.
const fn fibonacci(n: u64) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    let mut i = 0;
    while i < n {
        let next = a + b;
        a = b;
        b = next;
        i += 1;
    }
    a
}

// The const-testing primitive: static assertions evaluated at compile
// time. Break `factorial` or `fibonacci` and the crate fails to build
// here — before any test runs — with the failing assertion named.
const _: () = assert!(factorial(0) == 1);
const _: () = assert!(factorial(5) == 120);
const _: () = assert!(factorial(10) == 3_628_800);
const _: () = assert!(fibonacci(10) == 55);

/// The same const values consumed in a const context — the table's
/// length *is* `fibonacci(10)`'s output, so the compile-time evaluation
/// is load-bearing, not decorative.
const TABLE_LEN: usize = fibonacci(10) as usize;
static TABLE: [u64; TABLE_LEN] = build_fib_table();

const fn build_fib_table() -> [u64; TABLE_LEN] {
    let mut table = [0u64; TABLE_LEN];
    let mut i = 0;
    while i < TABLE_LEN {
        table[i] = fibonacci(i as u64);
        i += 1;
    }
    table
}

fn main() {
    println!("factorial(10) = {}", factorial(10));
    println!("fibonacci(10) = {}", fibonacci(10));
    println!("TABLE (len {}) entry 10 = {}", TABLE.len(), TABLE[10]);
    println!("all const assertions passed at compile time");
}

#[cfg(test)]
mod tests {
    /// Runtime regression tests pin the same values the static
    /// assertions check — same numbers, two failure modes: a compile
    /// error (accidental breakage) vs a test failure (deliberate
    /// change, with a readable location).
    #[test]
    fn factorial_matches_compile_time_values() {
        assert_eq!(super::factorial(5), 120);
        assert_eq!(super::factorial(10), 3_628_800);
    }

    #[test]
    fn fibonacci_matches_compile_time_values() {
        assert_eq!(super::fibonacci(10), 55);
    }

    #[test]
    fn fib_table_matches_compile_time_table() {
        assert_eq!(super::TABLE.len(), 55);
        assert_eq!(super::TABLE[10], 55);
        assert_eq!(super::TABLE[0], 0);
    }
}
