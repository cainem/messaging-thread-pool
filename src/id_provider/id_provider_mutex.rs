use std::sync::Mutex;

use super::IdProvider;

/// Provides an implementation of IdProvider that is Send.
/// This is probably too slow to be of any practical use; for test only
#[derive(Debug, Default)]
pub struct IdProviderMutex {
    internal_counter: Mutex<usize>,
}

impl PartialEq for IdProviderMutex {
    fn eq(&self, other: &Self) -> bool {
        self.peek_next_id() == other.peek_next_id()
    }
}

impl IdProviderMutex {
    pub fn new(internal_counter: usize) -> Self {
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
    fn next_id(&self) -> usize {
        let mut counter = self.internal_counter.lock().unwrap();
        // copy the value before mutating
        let value = *counter;
        *counter += 1;
        value
    }
    fn peek_next_id(&self) -> usize {
        *self.internal_counter.lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::id_provider::{id_provider_mutex::IdProviderMutex, IdProvider};

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
