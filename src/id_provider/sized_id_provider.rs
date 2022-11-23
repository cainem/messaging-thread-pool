use std::sync::Arc;

use super::IdProvider;

/// This struct wraps an id provider in such a way that it supports
/// clone and partial eq.
///
/// This is to allow for an IdProvider to be passed in a request which is required
/// if it is to be shared amongst all pool items.
///
/// Requests are required to implement clone and partial eq, although I suspect
/// that this was only for convenience reasons and there may be scope
/// for removing that constraint.
#[derive(Debug)]
pub struct SizedIdProvider {
    id_provider: Arc<dyn IdProvider>,
}

impl SizedIdProvider {
    pub fn new<T>(id_provider: T) -> Self
    where
        T: IdProvider + 'static,
    {
        Self {
            id_provider: Arc::new(id_provider),
        }
    }

    pub fn take(self) -> Arc<dyn IdProvider> {
        self.id_provider
    }
}

impl Clone for SizedIdProvider {
    fn clone(&self) -> Self {
        Self {
            // an arc can be cloned even if the the thing inside it cannot
            id_provider: self.id_provider.clone(),
        }
    }
}

impl PartialEq for SizedIdProvider {
    fn eq(&self, other: &Self) -> bool {
        // this is not ideal (it is slow) but partial eq is only used for assertions
        self.id_provider.clone().get_next_id() == other.id_provider.clone().get_next_id()
    }
}

impl IdProvider for SizedIdProvider {
    fn get_next_id(&self) -> u64 {
        self.id_provider.get_next_id()
    }
}
