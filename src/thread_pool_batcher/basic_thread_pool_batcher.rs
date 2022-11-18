use std::{
    num::NonZeroUsize,
    sync::{Arc, Weak},
};

use crate::{
    id_targeted::IdTargeted,
    pool_item::PoolItem,
    thread_request_response::{
        thread_shutdown_response::ThreadShutdownResponse, ThreadRequestResponse,
    },
    ThreadPool,
};

use super::{thread_pool_batcher_concrete::ThreadPoolBatcherConcrete, ThreadPoolBatcher};

/// This is a ThreadPoolBatcher which encapsulates the thread pool that it is using.\
/// This is suitable for simple scenarios.
pub struct BasicThreadPoolBatcher<P>
where
    P: PoolItem + 'static,
{
    thread_pool: Arc<ThreadPool<P>>,
    thread_pool_batcher: ThreadPoolBatcherConcrete<P>,
}

impl<P> BasicThreadPoolBatcher<P>
where
    P: PoolItem,
{
    pub fn new(threads_in_pool: usize) -> Self {
        assert!(
            threads_in_pool > 0,
            "must be at least one thread in the pool"
        );

        let thread_pool = Arc::new(ThreadPool::<P>::new(threads_in_pool));
        let thread_pool_batcher = ThreadPoolBatcherConcrete::new(Arc::downgrade(&thread_pool));

        Self {
            thread_pool,
            thread_pool_batcher,
        }
    }
}

impl<P> ThreadPoolBatcher<P> for BasicThreadPoolBatcher<P>
where
    P: PoolItem,
{
    fn batch_for_send<U>(&self, request: U) -> &Self
    where
        U: Into<ThreadRequestResponse<P>> + IdTargeted,
    {
        self.thread_pool_batcher.batch_for_send(request);
        self
    }

    fn send_batch<V>(&self) -> Vec<V>
    where
        V: From<ThreadRequestResponse<P>> + IdTargeted,
    {
        self.thread_pool_batcher.send_batch()
    }

    fn new(_thread_pool: Weak<ThreadPool<P>>) -> Self {
        unimplemented!();
    }

    fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
        self.thread_pool.shutdown()
    }

    fn get_thread_pool_size(&self) -> NonZeroUsize {
        self.thread_pool_batcher.get_thread_pool_size()
    }
}
