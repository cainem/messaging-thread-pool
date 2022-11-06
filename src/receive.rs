use crossbeam_channel::Receiver;
use tracing::{event, instrument, Level};

use crate::{
    element::Element, id_targeted::IdTargeted, thread_response::ThreadResponse, ThreadPool,
};

impl<E> ThreadPool<E>
where
    E: Element,
{
    /// This function sends a request to a worker thread and receives a response back
    ///
    /// The request is received as a vec and the responses are received back in a vec
    /// The idea here is that size of these vecs is restricted to a single compartments
    /// worth of requests
    #[instrument(skip(self, receive_from_worker))]
    pub(super) fn receive<T>(
        &self,
        requests_len: usize,
        receive_from_worker: Receiver<ThreadResponse<E::Response>>,
    ) -> Vec<T>
    where
        T: From<ThreadResponse<E::Response>> + IdTargeted,
    {
        // for every request there will be a response
        let mut building_responses = Vec::with_capacity(requests_len);
        // receive the confirmations of completion back
        for response in receive_from_worker {
            event!(Level::DEBUG, ?response);
            building_responses.push(response.into())
        }

        debug_assert_eq!(
            building_responses.len(),
            requests_len,
            "the protocol insists that responses must match requests"
        );

        // return the responses
        building_responses
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crossbeam_channel::unbounded;

    use crate::{
        samples::randoms::{
            randoms_request::init_request::InitRequest,
            randoms_response::{init_response::InitResponse, RandomsResponse},
            Randoms,
        },
        thread_request::ThreadRequest,
        thread_response::ThreadResponse,
        ThreadPool,
    };
    #[test]
    fn three_init_requests_two_thread_received_three_responses_received() {
        let target = ThreadPool::<Randoms>::new(2);

        let (send_to_pool, receive_from_thread) = unbounded::<ThreadResponse<RandomsResponse>>();

        let requests: Vec<_> = (0..3u64)
            .map(|id| ThreadRequest::ThreadEcho(id, format!("ping {}", id)))
            .collect();
        let requests = RefCell::new(requests);

        target.send(send_to_pool, &requests);

        let result = target.receive::<ThreadResponse<RandomsResponse>>(3, receive_from_thread);

        assert_eq!(3, result.len());
        let messages: Vec<_> = result
            .iter()
            .map(|e| match e {
                ThreadResponse::ThreadEcho(_targeted, _actual, s) => s,
                _ => panic!("not expected"),
            })
            .collect();
        assert!(messages.contains(&&"ping 0 [0]".to_string()));
        assert!(messages.contains(&&"ping 1 [1]".to_string()));
        assert!(messages.contains(&&"ping 2 [0]".to_string()));
    }

    #[test]
    fn single_init_request_on_a_single_thread_received_single_response_received() {
        let target = ThreadPool::<Randoms>::new(1);

        let (send_to_pool, receive_from_thread) = unbounded::<ThreadResponse<RandomsResponse>>();

        let requests: Vec<_> = (0..1u64).map(|id| InitRequest { id }).collect();
        let requests = RefCell::new(requests);

        target.send(send_to_pool, &requests);

        let result = target.receive::<InitResponse>(1, receive_from_thread);

        assert_eq!(1, result.len());
        assert_eq!(0, result[0].id);
    }
}
