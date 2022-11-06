use std::{
    num::NonZeroUsize,
    sync::{Arc, Weak},
};

use crate::{
    element::Element,
    id_targeted::IdTargeted,
    thread_request::ThreadRequest,
    thread_response::{ThreadResponse, ThreadShutdownResponse},
    ThreadPool,
};

use super::{thread_pool_batcher_concrete::ThreadPoolBatcherConcrete, ThreadPoolBatcher};

/// This is a ThreadPoolBatcher which encapsulates the thread pool that it is using
/// This is suitable for simple scenarios where a long lived thread pool is not required
pub struct BasicThreadPoolBatcher<E>
where
    E: Element + 'static,
{
    thread_pool: Arc<ThreadPool<E>>,
    thread_pool_batcher: ThreadPoolBatcherConcrete<E>,
}

impl<E> BasicThreadPoolBatcher<E>
where
    E: Element,
{
    pub fn new(threads_in_pool: usize) -> Self {
        assert!(
            threads_in_pool > 0,
            "must be at least one thread in the pool"
        );

        let thread_pool = Arc::new(ThreadPool::<E>::new(threads_in_pool));
        let thread_pool_batcher = ThreadPoolBatcherConcrete::new(Arc::downgrade(&thread_pool));

        Self {
            thread_pool,
            thread_pool_batcher,
        }
    }
}

impl<E> ThreadPoolBatcher<E> for BasicThreadPoolBatcher<E>
where
    E: Element,
{
    fn batch_for_send<U>(&self, request: U) -> &Self
    where
        U: Into<ThreadRequest<E::Request>> + IdTargeted,
    {
        self.thread_pool_batcher.batch_for_send(request);
        self
    }

    fn send_batch<V>(&self) -> Vec<V>
    where
        V: From<ThreadResponse<E::Response>> + IdTargeted,
    {
        self.thread_pool_batcher.send_batch()
    }

    fn new(_thread_pool: Weak<ThreadPool<E>>) -> Self {
        unimplemented!();
    }

    fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
        self.thread_pool.shutdown()
    }

    fn get_thread_pool_size(&self) -> NonZeroUsize {
        self.thread_pool_batcher.get_thread_pool_size()
    }
}
