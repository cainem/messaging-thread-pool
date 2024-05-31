use crate::{
    pool_item::PoolItem, request_with_response::RequestWithResponse,
    thread_request_response::ThreadRequestResponse, ThreadPool,
};
use crossbeam_channel::Receiver;

impl<P> ThreadPool<P>
where
    P: PoolItem,
{
    /// This function sends a request to a worker thread and receives a response back
    ///
    /// The request is received as a vec and the responses are received back in a vec
    /// The idea here is that size of these vecs is restricted to a single compartments
    /// worth of requests
    pub(super) fn receive<T>(
        &self,
        receive_from_worker: Receiver<ThreadRequestResponse<P>>,
    ) -> impl Iterator<Item = T::Response>
    where
        T: RequestWithResponse<P>,
    {
        receive_from_worker.into_iter().map(|r| r.into())
    }
}

#[cfg(test)]
mod tests {
    use crossbeam_channel::unbounded;

    use crate::{samples::*, thread_request_response::*, ThreadPool};

    #[test]
    fn three_init_requests_two_thread_received_three_responses_received() {
        let target = ThreadPool::<Randoms>::new(2);

        let (send_to_pool, receive_from_thread) = unbounded::<ThreadRequestResponse<Randoms>>();

        let requests = (0..3u64).map(|id| ThreadEchoRequest::new(id, format!("ping {id}")));

        target.send(send_to_pool, requests).unwrap();

        let results: Vec<ThreadEchoResponse> = target
            .receive::<ThreadEchoRequest>(receive_from_thread)
            .collect();

        assert_eq!(3, results.len());

        assert!(results.contains(&ThreadEchoResponse::new(0, "ping 0".to_string(), 0)));
        assert!(results.contains(&ThreadEchoResponse::new(1, "ping 1".to_string(), 1)));
        assert!(results.contains(&ThreadEchoResponse::new(2, "ping 2".to_string(), 0)));
    }

    #[test]
    fn single_init_request_on_a_single_thread_received_single_response_received() {
        let target = ThreadPool::<Randoms>::new(1);

        let (send_to_pool, receive_from_thread) = unbounded::<ThreadRequestResponse<Randoms>>();

        let requests: Vec<_> = (0..1).map(RandomsAddRequest).collect();

        target.send(send_to_pool, requests.into_iter()).unwrap();

        let result: Vec<AddResponse> = target
            .receive::<RandomsAddRequest>(receive_from_thread)
            .collect();

        assert_eq!(1, result.len());
        assert_eq!(0, result[0].id());
    }
}
