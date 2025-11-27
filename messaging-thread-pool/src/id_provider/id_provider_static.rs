use std::sync::atomic::{AtomicU64, Ordering};

use super::IdProvider;

static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

/// This is an example implementation of how to implement a static id provider
/// It is tied to the name of the static variable
/// so it is not of much use if multiple Id providers are required.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct IdProviderStatic;

impl IdProvider for IdProviderStatic {
    fn next_id(&self) -> u64 {
        ID_COUNTER.fetch_add(1, Ordering::SeqCst)
    }
    fn peek_next_id(&self) -> u64 {
        ID_COUNTER.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::IdProvider;

    use super::IdProviderStatic;

    #[test]
    #[ignore = "cannot test in test runner as it contains static variable"]
    fn peek_successive_id_providers_provides_successive_ids() {
        assert_eq!(0, IdProviderStatic.peek_next_id());
        assert_eq!(0, IdProviderStatic.peek_next_id());
    }

    #[test]
    #[ignore = "cannot test in test runner as it contains static variable"]
    fn getting_successive_id_providers_provides_successive_ids() {
        assert_eq!(0, IdProviderStatic.next_id());
        assert_eq!(1, IdProviderStatic.next_id());
        assert_eq!(2, IdProviderStatic.next_id());
    }

    #[test]
    #[ignore = "cannot test in test runner as it contains static variable"]
    fn get_first_id_gets_starting_id() {
        assert_eq!(0, IdProviderStatic.next_id())
    }

    #[test]
    #[ignore = "cannot test in test runner as it contains static variable"]
    fn visual_test() {
        let threads = (0..10)
            .map(|_| thread::spawn(|| IdProviderStatic.next_id()))
            .collect::<Vec<_>>();

        for t in threads {
            println!("{:?}", t.join().unwrap());
        }
    }
}
