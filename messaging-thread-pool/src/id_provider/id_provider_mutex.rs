use std::sync::Mutex;

use super::IdProvider;

/// A thread-safe ID provider using a `Mutex`.
///
/// This is a simple implementation suitable for:
/// - Testing and examples
/// - Low-contention scenarios
/// - Situations where ID generation isn't a bottleneck
///
/// For high-performance scenarios with many threads generating IDs,
/// consider using atomic operations instead.
///
/// # Example
///
/// ```rust
/// use std::sync::Arc;
/// use messaging_thread_pool::id_provider::{IdProvider, id_provider_mutex::IdProviderMutex};
///
/// // Start IDs at 1000
/// let provider = Arc::new(IdProviderMutex::new(1000));
///
/// assert_eq!(provider.next_id(), 1000);
/// assert_eq!(provider.next_id(), 1001);
/// assert_eq!(provider.peek_next_id(), 1002); // Peek doesn't advance
/// assert_eq!(provider.next_id(), 1002);
/// ```
///
/// # Thread Safety
///
/// This type is `Send + Sync` and can be safely shared across threads:
///
/// ```rust
/// use std::sync::Arc;
/// use std::thread;
/// use messaging_thread_pool::id_provider::{IdProvider, id_provider_mutex::IdProviderMutex};
///
/// let provider = Arc::new(IdProviderMutex::new(0));
/// let mut handles = vec![];
///
/// for _ in 0..4 {
///     let p = Arc::clone(&provider);
///     handles.push(thread::spawn(move || {
///         for _ in 0..10 {
///             let _id = p.next_id();
///         }
///     }));
/// }
///
/// for h in handles {
///     h.join().unwrap();
/// }
///
/// // 4 threads × 10 IDs each = 40 total IDs generated
/// assert_eq!(provider.peek_next_id(), 40);
/// ```
#[derive(Debug, Default)]
pub struct IdProviderMutex {
    internal_counter: Mutex<u64>,
}

impl PartialEq for IdProviderMutex {
    fn eq(&self, other: &Self) -> bool {
        self.peek_next_id() == other.peek_next_id()
    }
}

impl IdProviderMutex {
    pub fn new(internal_counter: u64) -> Self {
        Self {
            internal_counter: Mutex::new(internal_counter),
        }
    }
}

impl Clone for IdProviderMutex {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl IdProvider for IdProviderMutex {
    fn next_id(&self) -> u64 {
        let mut counter = self.internal_counter.lock().unwrap();
        // copy the value before mutating
        let value = *counter;
        *counter += 1;
        value
    }
    fn peek_next_id(&self) -> u64 {
        *self.internal_counter.lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::id_provider::{IdProvider, id_provider_mutex::IdProviderMutex};

    #[test]
    fn peek_id_as_expected() {
        let target = IdProviderMutex::new(5);

        assert_eq!(5, target.peek_next_id());
        assert_eq!(5, target.peek_next_id());
    }

    #[test]
    fn gets_ids_as_expected() {
        let target = IdProviderMutex::default();

        assert_eq!(0, target.next_id());
        assert_eq!(1, target.next_id());
    }
}
