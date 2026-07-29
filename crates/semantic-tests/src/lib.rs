//! Ownership, borrowing, trait dispatch, and async semantics.
//!
//! `unsafe_code`/`missing_docs` lint policy: see the workspace `[lints]`
//! table in the root `Cargo.toml`.

use std::sync::{Arc, Mutex};
use std::thread;

/// Spawns `worker_count` threads that each increment a shared counter once,
/// joins them all, and returns the final count — a smoke test that
/// `Arc<Mutex<_>>` sharing behaves as expected under concurrent mutation.
pub fn shared_counter_after_workers(worker_count: usize) -> usize {
    let counter = Arc::new(Mutex::new(0usize));

    let handles: Vec<_> = (0..worker_count)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                let mut guard = counter.lock().expect("lock should not be poisoned");
                *guard += 1;
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("worker should complete");
    }

    let final_value = *counter.lock().expect("lock should not be poisoned");
    final_value
}

/// A trait exercised via both a generic bound and a trait object, to cover
/// static and dynamic dispatch in the same semantic-tests suite.
pub trait Greeter {
    /// Returns a greeting string for this greeter.
    fn greet(&self) -> String;
}

/// A [`Greeter`] that greets by a fixed name.
pub struct NamedGreeter {
    /// The name to greet.
    pub name: String,
}

impl Greeter for NamedGreeter {
    fn greet(&self) -> String {
        format!("hello, {}", self.name)
    }
}

/// Calls `greeter.greet()` through a generic bound (static dispatch).
pub fn greet_via_generic<G: Greeter>(greeter: &G) -> String {
    greeter.greet()
}

/// Calls `greeter.greet()` through a trait object (dynamic dispatch).
pub fn greet_via_trait_object(greeter: &dyn Greeter) -> String {
    greeter.greet()
}

/// Returns the longer of two string slices. The classic lifetime example:
/// the return type's lifetime (`'a`) is tied to *both* inputs, so the
/// compiler can't let the result outlive whichever input turns out to be
/// shorter-lived at the call site.
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

/// Borrows a slice of a larger string it can't outlive — a struct holding
/// a reference, rather than owning a copy.
pub struct Excerpt<'a> {
    /// The borrowed slice.
    pub part: &'a str,
}

impl<'a> Excerpt<'a> {
    /// Builds an [`Excerpt`] of `document`'s text up to its first `.`.
    pub fn first_sentence(document: &'a str) -> Self {
        let part = document.split('.').next().unwrap_or(document);
        Excerpt { part }
    }
}

/// Compile-time assertion that `T` is [`Send`]. Has no runtime effect;
/// instantiating it with a type that isn't `Send` fails to *compile*,
/// which is the point — call this from a test as a type-level check
/// rather than writing a runtime assertion for something the type system
/// already guarantees.
pub fn assert_send<T: Send>() {}

/// Compile-time assertion that `T` is [`Sync`]. See [`assert_send`].
pub fn assert_sync<T: Sync>() {}

/// An error demonstrating the "implement [`std::error::Error`] + thread
/// failures through with `?`" pattern, instead of `.unwrap()` chains.
#[derive(Debug, PartialEq, Eq)]
pub enum ConfigError {
    /// The expected `key=value` pair was missing or the key didn't match.
    MissingField(&'static str),
    /// The field was present but its value couldn't be parsed.
    InvalidValue {
        /// Which field failed to parse.
        field: &'static str,
        /// Why it failed.
        reason: String,
    },
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::MissingField(field) => write!(f, "missing field: {field}"),
            ConfigError::InvalidValue { field, reason } => {
                write!(f, "invalid value for {field}: {reason}")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

/// Parses a `"timeout_ms=<number>"` pair, propagating failures from two
/// different steps (missing key, unparseable value) through `?` into one
/// [`ConfigError`].
pub fn parse_timeout_ms(input: &str) -> Result<u64, ConfigError> {
    let (key, value) = input
        .split_once('=')
        .ok_or(ConfigError::MissingField("timeout_ms"))?;

    if key != "timeout_ms" {
        return Err(ConfigError::MissingField("timeout_ms"));
    }

    value
        .parse::<u64>()
        .map_err(|err| ConfigError::InvalidValue {
            field: "timeout_ms",
            reason: err.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::{
        assert_send, assert_sync, greet_via_generic, greet_via_trait_object, longest,
        parse_timeout_ms, shared_counter_after_workers, ConfigError, Excerpt, NamedGreeter,
    };

    #[test]
    fn ownership_rules_hold_under_shared_mutation() {
        assert_eq!(shared_counter_after_workers(8), 8);
    }

    #[test]
    fn generic_bound_dispatches_statically() {
        let greeter = NamedGreeter {
            name: "static".to_string(),
        };
        assert_eq!(greet_via_generic(&greeter), "hello, static");
    }

    #[test]
    fn trait_object_dispatches_dynamically() {
        let greeter = NamedGreeter {
            name: "dynamic".to_string(),
        };
        let boxed: Box<dyn super::Greeter> = Box::new(greeter);
        assert_eq!(greet_via_trait_object(boxed.as_ref()), "hello, dynamic");
    }

    #[test]
    fn longest_returns_the_longer_slice() {
        assert_eq!(longest("short", "longer string"), "longer string");
        assert_eq!(longest("tied", "each"), "tied");
    }

    #[test]
    fn excerpt_borrows_first_sentence() {
        let document = String::from("Rust is fast. It is also safe.");
        let excerpt = Excerpt::first_sentence(&document);
        assert_eq!(excerpt.part, "Rust is fast");
    }

    #[test]
    fn common_types_are_send_and_sync() {
        assert_send::<NamedGreeter>();
        assert_sync::<NamedGreeter>();
        assert_send::<ConfigError>();
        assert_sync::<ConfigError>();
    }

    #[test]
    fn parse_timeout_ms_succeeds_on_valid_input() {
        assert_eq!(parse_timeout_ms("timeout_ms=500"), Ok(500));
    }

    #[test]
    fn parse_timeout_ms_rejects_wrong_key() {
        assert_eq!(
            parse_timeout_ms("other=500"),
            Err(ConfigError::MissingField("timeout_ms"))
        );
    }

    #[test]
    fn parse_timeout_ms_rejects_non_numeric_value() {
        assert!(matches!(
            parse_timeout_ms("timeout_ms=abc"),
            Err(ConfigError::InvalidValue { .. })
        ));
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn async_task_takes_ownership_and_returns_it() {
        let owned = String::from("moved into task");

        let handle = tokio::spawn(async move {
            // `owned` is moved into this async block; the task owns it for
            // the duration of the future and hands it back on completion.
            owned.len()
        });

        let len = handle.await.expect("task should not panic");
        assert_eq!(len, "moved into task".len());
    }

    #[cfg(feature = "loom")]
    mod loom_tests {
        // Mirrors `shared_counter_after_workers`'s Arc<Mutex<_>>
        // shared-increment pattern, rebuilt on loom's own `Arc`/`Mutex`/
        // `thread` so `loom::model` can drive it. Deliberately a separate,
        // small copy rather than making the production function itself
        // swap to loom's primitives behind this feature: that would leak
        // through Cargo's workspace-wide feature unification — running
        // `cargo test --workspace --all-features` (what `scripts/check.sh`
        // does) enables `loom` for every crate that depends on
        // `semantic-tests`, including `integration-tests`, which calls
        // `shared_counter_after_workers` directly outside of any
        // `loom::model` and would panic if that function were loom-backed.
        use loom::sync::{Arc, Mutex};
        use loom::thread;

        fn shared_counter_after_workers_loom(worker_count: usize) -> usize {
            let counter = Arc::new(Mutex::new(0usize));

            let handles: Vec<_> = (0..worker_count)
                .map(|_| {
                    let counter = Arc::clone(&counter);
                    thread::spawn(move || {
                        *counter.lock().unwrap() += 1;
                    })
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }

            let final_value = *counter.lock().unwrap();
            final_value
        }

        // loom exhaustively explores thread interleavings, so its state
        // space grows explosively with thread count — keep this at 2
        // workers, not the 8 the plain-threaded test above uses. See
        // "Concurrency-permutation test" in docs/adding-tests.md.
        #[test]
        fn shared_counter_after_workers_holds_under_all_interleavings() {
            loom::model(|| {
                assert_eq!(shared_counter_after_workers_loom(2), 2);
            });
        }
    }
}
