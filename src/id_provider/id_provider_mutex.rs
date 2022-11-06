use std::sync::Mutex;

use super::IdProvider;

/// Provides an implementation of IdProvider that is Send.
/// Too slow to be of any practical use; for test only
#[derive(Debug, Default)]
pub struct IdProviderMutex {
    internal_counter: Mutex<u64>,
}

impl PartialEq for IdProviderMutex {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
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
    fn get_next_id(&self) -> u64 {
        let mut counter = self.internal_counter.lock().unwrap();
        // copy the value before mutating
        let value = *counter;
        *counter += 1;
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::id_provider::{id_provider_mutex::IdProviderMutex, IdProvider};

    #[test]
    fn gets_ids_as_expected() {
        let target = IdProviderMutex::default();

        assert_eq!(0, target.get_next_id());
        assert_eq!(1, target.get_next_id());
    }
}
