//! Tiny async in-memory key-value service — a domain starter showing
//! async testing patterns: `tokio::test` with **paused time** for
//! deterministic expiry tests, concurrent access through a shared
//! store, and a background task with graceful shutdown.
//!
//! Standalone workspace of its own (an empty `[workspace]` table — the
//! `fuzz/` detachment pattern), *not* a member of the RustForge
//! workspace: it adds nothing to the template's default
//! `cargo test --workspace`. See `docs/domain-starters.md`.
//!
//! `unsafe_code`/`missing_docs` lint policy: mirrors the RustForge
//! workspace's `[lints]` table via the crate attributes below.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::time::{sleep_until, Duration, Instant};

/// A stored value with an optional expiry (tokio's `Instant`, so tests
/// can drive time deterministically — see the tests below).
struct Entry {
    value: String,
    expires_at: Option<Instant>,
}

/// A cloneable handle to one shared store. `Clone` shares the inner
/// map — every clone sees the same data, like a real service handle.
#[derive(Clone)]
pub struct Store {
    inner: Arc<Mutex<HashMap<String, Entry>>>,
}

impl Store {
    /// Creates an empty store.
    pub fn new() -> Self {
        Store {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Inserts `key` with `value`, expiring after `ttl` if given.
    pub fn insert(&self, key: impl Into<String>, value: impl Into<String>, ttl: Option<Duration>) {
        let expires_at = ttl.map(|d| Instant::now() + d);
        self.inner
            .lock()
            .expect("store lock poisoned")
            .insert(key.into(), Entry {
                value: value.into(),
                expires_at,
            });
    }

    /// Returns the value for `key`, or `None` if absent or expired.
    /// Expired entries are dropped on access (lazy expiry).
    pub fn get(&self, key: &str) -> Option<String> {
        let mut map = self.inner.lock().expect("store lock poisoned");
        match map.get(key) {
            Some(entry) if live(entry.expires_at) => Some(entry.value.clone()),
            Some(_) => {
                map.remove(key);
                None
            }
            None => None,
        }
    }

    /// Removes `key`, returning its value if it was present and live.
    pub fn remove(&self, key: &str) -> Option<String> {
        self.get(key)?;
        self.inner
            .lock()
            .expect("store lock poisoned")
            .remove(key)
            .map(|entry| entry.value)
    }

    /// Number of live entries (expired-but-not-yet-swept entries
    /// included — `sweep` is what makes this exact).
    pub fn len(&self) -> usize {
        self.inner.lock().expect("store lock poisoned").len()
    }

    /// Whether the store is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Drops every expired entry. Called by [`sweeper`], safe to call
    /// directly.
    pub fn sweep(&self) {
        self.inner
            .lock()
            .expect("store lock poisoned")
            .retain(|_, entry| live(entry.expires_at));
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

/// An entry with no expiry, or one not yet past its expiry, is live.
fn live(expires_at: Option<Instant>) -> bool {
    match expires_at {
        None => true,
        Some(exp) => exp > Instant::now(),
    }
}

/// Background sweeper: every `interval`, drops expired entries until
/// `shutdown` fires — the graceful-shutdown shape a real service task
/// needs.
pub async fn sweeper(store: Store, interval: Duration, mut shutdown: mpsc::Receiver<()>) {
    loop {
        tokio::select! {
            _ = sleep_until(Instant::now() + interval) => store.sweep(),
            _ = shutdown.recv() => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::task::JoinSet;

    #[tokio::test]
    async fn insert_and_get_roundtrip() {
        let store = Store::new();
        store.insert("greeting", "hello", None);
        assert_eq!(store.get("greeting"), Some("hello".to_string()));
        assert_eq!(store.get("missing"), None);
        assert_eq!(store.len(), 1);
    }

    #[tokio::test]
    async fn remove_takes_the_value() {
        let store = Store::new();
        store.insert("k", "v", None);
        assert_eq!(store.remove("k"), Some("v".to_string()));
        assert_eq!(store.remove("k"), None);
    }

    /// The deterministic-time pattern: `start_paused = true` freezes
    /// tokio's clock, so `advance` moves it without real sleeping —
    /// expiry tests run in microseconds and can never flake.
    #[tokio::test(start_paused = true)]
    async fn entries_expire_after_their_ttl() {
        let store = Store::new();
        store.insert("short", "v", Some(Duration::from_secs(1)));
        store.insert("forever", "v", None);
        assert_eq!(store.get("short"), Some("v".to_string()));
        tokio::time::advance(Duration::from_secs(2)).await;
        assert_eq!(store.get("short"), None);
        assert_eq!(store.get("forever"), Some("v".to_string()));
    }

    #[tokio::test(start_paused = true)]
    async fn sweeper_removes_expired_entries() {
        let store = Store::new();
        store.insert("a", "v", Some(Duration::from_millis(500)));
        store.insert("b", "v", Some(Duration::from_secs(10)));
        assert_eq!(store.len(), 2);

        let (shutdown_tx, shutdown_rx) = mpsc::channel(1);
        let sweeper_store = store.clone();
        let sweeper_task = tokio::spawn(sweeper(sweeper_store, Duration::from_secs(1), shutdown_rx));

        // Let the sweeper register its first sleep *before* moving time.
        // With paused time this ordering matters: if the task hasn't run
        // yet when `advance` executes, it computes its deadline against
        // the already-advanced clock and the tick never fires in-test.
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_secs(1)).await;
        // Yield again so the woken sweeper task gets polled and sweeps.
        tokio::task::yield_now().await;
        assert_eq!(store.len(), 1, "only the long-ttl entry should remain");

        shutdown_tx.send(()).await.expect("shutdown send");
        sweeper_task.await.expect("sweeper should exit cleanly");
    }

    #[tokio::test(start_paused = true)]
    async fn concurrent_inserts_are_all_visible() {
        let store = Store::new();
        let mut tasks = JoinSet::new();
        for i in 0..16 {
            let store = store.clone();
            tasks.spawn(async move {
                store.insert(format!("key-{i}"), format!("value-{i}"), None);
            });
        }
        while tasks.join_next().await.is_some() {}
        assert_eq!(store.len(), 16);
        assert_eq!(store.get("key-7"), Some("value-7".to_string()));
    }
}
