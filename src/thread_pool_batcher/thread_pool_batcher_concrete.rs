use std::{cell::RefCell, num::NonZeroUsize, sync::Weak};

use crate::{
    element::Element,
    id_targeted::IdTargeted,
    thread_request::ThreadRequest,
    thread_response::{ThreadResponse, ThreadShutdownResponse},
    ThreadPool,
};

use super::ThreadPoolBatcher;

/// A thread pool batcher is a structure to assist the batched sending of request to a thread pool.
///
/// The thread pool works by dividing a vec of requests across multiple threads
/// For this to work then the sender must produce a whole batch of requests before
/// calling the thread pool.
///
/// This structure and its associated functions facilitate this behaviour.
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

/// This is implementation of the trait for the generic
/// ThreadPoolBatcherConcrete which provides an implementation for the trait
impl<E> ThreadPoolBatcher<E> for ThreadPoolBatcherConcrete<E>
where
    E: Element,
{
    fn batch_for_send<U>(&self, request: U) -> &Self
    where
        U: Into<ThreadRequest<<E as Element>::Request>> + IdTargeted,
    {
        ThreadPoolBatcherConcrete::<E>::batch_for_send(self, request)
    }

    fn send_batch<V>(&self) -> Vec<V>
    where
        V: From<ThreadResponse<<E as Element>::Response>> + IdTargeted,
    {
        ThreadPoolBatcherConcrete::<E>::send_batch(self)
    }

    fn new(thread_pool: Weak<ThreadPool<E>>) -> Self {
        ThreadPoolBatcherConcrete::<E>::new(thread_pool)
    }

    fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
        ThreadPoolBatcherConcrete::<E>::shutdown_pool(self)
    }

    fn get_thread_pool_size(&self) -> NonZeroUsize {
        ThreadPoolBatcherConcrete::<E>::thread_pool(self)
            .upgrade()
            .expect("thread pool to be alive")
            .thread_count()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Weak};

    use crate::{
        samples::*,
        thread_pool_batcher::ThreadPoolBatcher,
        thread_request::ThreadRequest,
        thread_response::{ThreadResponse, ThreadShutdownResponse},
        ThreadPool,
    };

    use super::ThreadPoolBatcherConcrete;

    #[test]
    fn thread_pool_size_2_returns_2() {
        let thread_pool = Arc::new(ThreadPool::<Randoms>::new(2));

        let target = ThreadPoolBatcherConcrete::<Randoms>::new(Arc::downgrade(&thread_pool));

        let result = ThreadPoolBatcher::<Randoms>::get_thread_pool_size(&target);

        assert_eq!(2usize, usize::from(result));

        ThreadPoolBatcher::<Randoms>::shutdown_pool(&target);
    }

    #[test]
    fn thread_pool_size_1_returns_1() {
        let thread_pool = Arc::new(ThreadPool::<Randoms>::new(1));

        let target = ThreadPoolBatcherConcrete::<Randoms>::new(Arc::downgrade(&thread_pool));

        let result = ThreadPoolBatcher::<Randoms>::get_thread_pool_size(&target);

        assert_eq!(1usize, usize::from(result));

        ThreadPoolBatcher::<Randoms>::shutdown_pool(&target);
    }

    #[test]
    fn two_threads_in_thread_pool_shutdown_results_in_2_return_codes() {
        let thread_pool = Arc::new(ThreadPool::<Randoms>::new(2));

        let target = ThreadPoolBatcherConcrete::<Randoms>::new(Arc::downgrade(&thread_pool));

        let result = ThreadPoolBatcher::<Randoms>::shutdown_pool(&target);
        assert_eq!(2, result.len());
        assert_eq!(ThreadShutdownResponse::new(0, vec![]), result[0]);
        assert_eq!(ThreadShutdownResponse::new(1, vec![]), result[1])
    }

    #[test]
    fn trait_new_constructs_as_expected() {
        let thread_pool = Arc::new(ThreadPool::<Randoms>::new(1));

        let result: ThreadPoolBatcherConcrete<Randoms> =
            ThreadPoolBatcher::<Randoms>::new(Arc::downgrade(&thread_pool));

        assert!(result.to_send().borrow().is_empty());
        assert!(std::ptr::eq(
            thread_pool.as_ref(),
            Weak::upgrade(&result.thread_pool()).unwrap().as_ref()
        ));
    }

    #[test]
    fn batch_single_request_get_expected_single_response() {
        let thread_pool = Arc::new(ThreadPool::<Randoms>::new(1));

        let target = ThreadPoolBatcherConcrete::<Randoms>::new(Arc::downgrade(&thread_pool));

        let request = ThreadRequest::ThreadEcho(0, "hello".to_string());
        target.batch_for_send(request.clone());
        let result: Vec<ThreadResponse<RandomsResponse>> =
            ThreadPoolBatcher::<Randoms>::send_batch(&target);

        let expected_response = ThreadResponse::ThreadEcho(0, 0, "hello [0]".to_string());

        assert_eq!(1, result.len());
        assert_eq!(expected_response, result[0]);
        // the vec to_send is left empty
        assert!(target.to_send().borrow().is_empty());
    }

    #[test]
    fn send_for_batch_adds_request_to_internal_vec() {
        let thread_pool = Arc::new(ThreadPool::<Randoms>::new(1));
        let target = ThreadPoolBatcherConcrete::<Randoms>::new(Arc::downgrade(&thread_pool));

        let request = mean_request::MeanRequest { id: 1 };
        ThreadPoolBatcher::<Randoms>::batch_for_send(&target, request.clone());

        assert_eq!(1, target.to_send().borrow().len());
        assert_eq!(
            ThreadRequest::<RandomsRequest>::ElementRequest(RandomsRequest::Mean(request)),
            target.to_send().borrow()[0]
        );
    }

    #[test]
    fn concrete_new_constructs_as_expected() {
        let thread_pool = Arc::new(ThreadPool::<Randoms>::new(1));

        let result = ThreadPoolBatcherConcrete::<Randoms>::new(Arc::downgrade(&thread_pool));

        assert!(result.to_send.borrow().is_empty());
        assert!(std::ptr::eq(
            thread_pool.as_ref(),
            Weak::upgrade(&result.thread_pool).unwrap().as_ref()
        ));
    }
}
