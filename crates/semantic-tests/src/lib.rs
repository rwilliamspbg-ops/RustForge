#![forbid(unsafe_code)]

use std::sync::{Arc, Mutex};
use std::thread;

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

#[cfg(test)]
mod tests {
    use super::shared_counter_after_workers;

    #[test]
    fn ownership_rules_hold_under_shared_mutation() {
        assert_eq!(shared_counter_after_workers(8), 8);
    }
}
