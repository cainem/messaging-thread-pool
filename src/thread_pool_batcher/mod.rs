mod basic_thread_pool_batcher;
mod batch_for_send;
mod send_batch;
mod shutdown_pool;
mod thread_pool_batcher_concrete;
mod thread_pool_batcher_mock;

pub use basic_thread_pool_batcher::*;
pub use thread_pool_batcher_concrete::ThreadPoolBatcherConcrete;
pub use thread_pool_batcher_mock::ThreadPoolBatcherMock;

use std::{num::NonZeroUsize, sync::Weak};

use crate::{
    element::Element,
    id_targeted::IdTargeted,
    thread_request::ThreadRequest,
    thread_response::{ThreadResponse, ThreadShutdownResponse},
    ThreadPool,
};

/// This trait defines the interface of a ThreadPoolBatcher.
///
/// Making it a trait allows for the interface to the thread pool to be easily mocked for testing purposes
/// (see [`ThreadPoolBatcherMock`])
pub trait ThreadPoolBatcher<E>
where
    E: Element,
{
    /// This function queues a message for sending adding it to an internal buffer
    fn batch_for_send<U>(&self, request: U) -> &Self
    where
        U: Into<ThreadRequest<E::Request>> + IdTargeted;

    /// This function sends all messages stored in the internal buffer.\
    /// This call blocks until all messages have been acted upon and their responses returned.\
    /// The messages will be distributed, using the mod of the message id, across all
    /// of the ThreadPoolBatcher's pool threads.\
    fn send_batch<V>(&self) -> Vec<V>
    where
        V: From<ThreadResponse<E::Response>> + IdTargeted;

    /// Creates a new ThreadPoolBatcher that will use the passed in thread pool
    fn new(thread_pool: Weak<ThreadPool<E>>) -> Self;
    /// Shuts down the thread pool associated with this instance of the [`ThreadPoolBatcher`]
    fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse>;
    /// Returns the thread pool size of the thread pool associated with this instance of [`ThreadPoolBatcher`]
    fn get_thread_pool_size(&self) -> NonZeroUsize;
}
