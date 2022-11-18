use crate::{pool_item::PoolItem, thread_request_response::ThreadRequestResponse};

use super::ThreadPoolBatcherConcrete;

impl<P> ThreadPoolBatcherConcrete<P>
where
    P: PoolItem,
{
    /// This function is called to send a batch of requests that have been queued by calling
    /// batch_for_send
    ///
    /// It returns the responses received
    /// With debug_assertions it checks that there is one appropriately targeted response for each request
    pub fn send_batch<V>(&self) -> Vec<V>
    where
        V: From<ThreadRequestResponse<P>>,
    {
        self.thread_pool()
            .upgrade()
            .expect("threads to live for the life of the process")
            .send_and_receive(self.to_send())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        samples::{mean_response::MeanResponse, *},
        thread_pool_batcher::ThreadPoolBatcherConcrete,
        thread_request_response::{
            thread_echo_request::ThreadEchoRequest, thread_echo_response::ThreadEchoResponse,
        },
        ThreadPool,
    };

    #[test]
    fn batch_single_request_get_expected_single_response() {
        let thread_pool = Arc::new(ThreadPool::<Randoms>::new(1));

        let target = ThreadPoolBatcherConcrete::<Randoms>::new(Arc::downgrade(&thread_pool));

        let request = ThreadEchoRequest::new(0, "hello".to_string());
        target.batch_for_send(request.clone());
        let result: Vec<ThreadEchoResponse> = target.send_batch();

        let expected_response = ThreadEchoResponse::new(0, "hello".to_string(), 0);

        assert_eq!(1, result.len());
        assert_eq!(expected_response, result[0]);
        // the vec to_send is left empty
        assert!(target.to_send().borrow().is_empty());
    }

    #[test]
    fn batch_is_empty_call_does_not_panic_empty_vec_returned() {
        let thread_pool = Arc::new(ThreadPool::<Randoms>::new(1));

        let target = ThreadPoolBatcherConcrete::<Randoms>::new(Arc::downgrade(&thread_pool));

        let result: Vec<MeanResponse> = target.send_batch();

        assert!(result.is_empty());
    }
}
