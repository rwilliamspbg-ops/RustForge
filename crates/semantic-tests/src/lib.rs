//! Ownership, borrowing, trait dispatch, and async semantics.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

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

#[cfg(test)]
mod tests {
    use super::{
        greet_via_generic, greet_via_trait_object, shared_counter_after_workers, NamedGreeter,
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
}
