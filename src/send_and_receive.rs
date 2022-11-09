use std::cell::RefCell;

use crossbeam_channel::unbounded;
use tracing::{event, instrument, Level};

use crate::{
    element::Element, id_targeted::IdTargeted, thread_request::ThreadRequest,
    thread_response::ThreadResponse, ThreadPool,
};

impl<E> ThreadPool<E>
where
    E: Element,
{
    /// This function sends a request to a worker thread and receives a response back
    ///
    /// The request is received as a vec and the responses are received back in a vec
    #[instrument(skip(self, requests))]
    pub fn send_and_receive<T, U>(&self, requests: &RefCell<Vec<T>>) -> Vec<U>
    where
        T: Into<ThreadRequest<E::Request>> + IdTargeted,
        U: From<ThreadResponse<E::Response>> + IdTargeted,
    {
        let requests_len = requests.borrow().len();

        event!(Level::DEBUG, requests_len, message = "Sending requests");

        let (return_back_to, receive_from_worker) = unbounded::<ThreadResponse<E::Response>>();

        self.send(return_back_to, requests);
        self.receive(requests_len, receive_from_worker)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::{
        samples::*, thread_request::ThreadRequest, thread_response::ThreadResponse, ThreadPool,
    };

    #[test]
    fn two_threads_three_echoes_receives_expected_response() {
        let target = ThreadPool::<Randoms>::new(2);

        let requests: Vec<_> = (0..3)
            .map(|i| ThreadRequest::ThreadEcho(i, format!("ping {}", i)))
            .collect();
        let requests = RefCell::new(requests);

        let result: Vec<ThreadResponse<RandomsResponse>> = target.send_and_receive(&requests);

        assert_eq!(result.len(), 3);
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
    fn single_thread_single_init_receives_expected_response() {
        let target = ThreadPool::<Randoms>::new(1);

        let requests: Vec<_> = (0..1)
            .map(|id| randoms_init_request::RandomsInitRequest { id })
            .collect();
        let requests = RefCell::new(requests);

        let result: Vec<randoms_init_response::RandomsInitResponse> =
            target.send_and_receive(&requests);

        assert_eq!(result.len(), 1);
        assert_eq!(0, result[0].id);
    }
}
