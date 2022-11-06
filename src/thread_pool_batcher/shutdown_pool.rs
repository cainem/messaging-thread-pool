use crate::{element::Element, thread_shutdown_response::ThreadShutdownResponse};

use super::ThreadPoolBatcherConcrete;

impl<E> ThreadPoolBatcherConcrete<E>
where
    E: Element,
{
    /// This function tells the ThreadPoolBatcher to shutdown its associated thread pool
    pub fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
        if let Some(thread_pool_batcher) = self.thread_pool().upgrade() {
            thread_pool_batcher.shutdown()
        } else {
            // unable to upgrade the weak reference and unwrap;
            // which implies that the thread has already shutdown
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        samples::randoms::{randoms_response::RandomsResponse, Randoms},
        thread_pool_batcher::ThreadPoolBatcherConcrete,
        thread_request::ThreadRequest,
        thread_response::ThreadResponse,
        thread_shutdown_response::ThreadShutdownResponse,
        ThreadPool,
    };

    #[test]
    fn thread_pool_already_dropped_shutdown_results_in_empty_array_of_return_codes() {
        let thread_pool = Arc::new(ThreadPool::<Randoms>::new(2));

        let target = ThreadPoolBatcherConcrete::<Randoms>::new(Arc::downgrade(&thread_pool));

        // this removes the last strong reference to the thread pool; this should shutdown the thread pool
        drop(thread_pool);

        // try to shutdown a pool that has already gone!
        let result = target.shutdown_pool();
        assert!(result.is_empty());
    }

    #[test]
    fn two_threads_in_thread_pool_shutdown_results_in_2_return_codes() {
        let thread_pool = Arc::new(ThreadPool::<Randoms>::new(2));

        let target = ThreadPoolBatcherConcrete::<Randoms>::new(Arc::downgrade(&thread_pool));

        let request = ThreadRequest::ThreadEcho(0, "hello".to_string());
        target.batch_for_send(request.clone());
        let _: Vec<ThreadResponse<RandomsResponse>> = target.send_batch();

        let result = target.shutdown_pool();
        assert_eq!(2, result.len());
        assert_eq!(ThreadShutdownResponse::new(0, vec![]), result[0]);
        assert_eq!(ThreadShutdownResponse::new(1, vec![]), result[1])
    }
}
