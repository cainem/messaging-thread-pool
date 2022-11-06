use std::sync::atomic::{AtomicUsize, Ordering};

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// This class provides globally unique values
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct IdProviderStatic;

impl IdProviderStatic {
    pub fn get_next_id(&self) -> u64 {
        ID_COUNTER.fetch_add(1, Ordering::SeqCst) as u64
    }
}
