use std::{cell::RefCell, sync::Weak};

use crate::{element::Element, thread_request::ThreadRequest, ThreadPool};

/// A thread pool batcher is a structure to assist the batched sending of request to a thread pool
///
/// The thread pool works by dividing a vec of requests across multiple threads
/// For this to work then the sender must produce a whole batch of requests before
/// calling the thread pool.
///
/// This structure and its associated functions facilitate this behaviour
#[derive(Debug)]
pub struct ThreadPoolBatcherConcrete<E>
where
    E: Element,
{
    thread_pool: Weak<ThreadPool<E>>,
    to_send: RefCell<Vec<ThreadRequest<E::Request>>>,
}

impl<E> ThreadPoolBatcherConcrete<E>
where
    E: Element,
{
    /// This function creates a new ThreadPoolBatcher
    pub fn new(thread_pool: Weak<ThreadPool<E>>) -> Self {
        Self {
            thread_pool,
            to_send: RefCell::new(vec![]),
        }
    }

    pub fn thread_pool(&self) -> &Weak<ThreadPool<E>> {
        &self.thread_pool
    }

    pub(super) fn to_send(&self) -> &RefCell<Vec<ThreadRequest<E::Request>>> {
        &self.to_send
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Weak};

    use crate::{samples::*, ThreadPool};

    use super::ThreadPoolBatcherConcrete;

    #[test]
    fn new_constructs_as_expected() {
        let thread_pool = Arc::new(ThreadPool::<Randoms>::new(1));

        let result = ThreadPoolBatcherConcrete::<Randoms>::new(Arc::downgrade(&thread_pool));

        assert!(result.to_send.borrow().is_empty());
        assert!(std::ptr::eq(
            thread_pool.as_ref(),
            Weak::upgrade(&result.thread_pool).unwrap().as_ref()
        ));
    }
}
