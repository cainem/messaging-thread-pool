use std::{
    cell::{Cell, RefCell},
    sync::Weak,
};

use crate::{
    element::Element,
    id_targeted::IdTargeted,
    thread_request::ThreadRequest,
    thread_response::{ThreadResponse, ThreadShutdownResponse},
    ThreadPool,
};

use super::ThreadPoolBatcher;

/// A struct for mocking the request/response interface for a given pair of Request/Responses
/// It is constructed with a vec of requests that it is expecting to receive and an vec of
/// hard coded responses to those requests
///
/// It validates that the requests received are indeed the ones received.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ThreadPoolBatcherMock<Req, Res>
where
    Req: IdTargeted,
    Res: IdTargeted,
{
    expected_requests: RefCell<Vec<ThreadRequest<Req>>>,
    responses: RefCell<Vec<ThreadResponse<Res>>>,
    shutdown_called: Cell<bool>,
}

impl<Req, Res> ThreadPoolBatcherMock<Req, Res>
where
    Req: IdTargeted,
    Res: IdTargeted,
{
    pub fn new(mut expected_requests: Vec<Req>, mut responses: Vec<Res>) -> Self {
        assert_eq!(
            expected_requests.len(),
            responses.len(),
            "there needs to be a response for every request"
        );

        assert!(
            !expected_requests
                .iter()
                .zip(responses.iter())
                .any(|(req, res)| req.id() != res.id()),
            "requests and responses must targetting the same id"
        );

        let result = Self {
            expected_requests: RefCell::new(
                expected_requests
                    .drain(..)
                    .map(|request| ThreadRequest::ElementRequest(request))
                    .collect(),
            ),
            responses: RefCell::new(
                responses
                    .drain(..)
                    .map(|response| ThreadResponse::ElementResponse(response))
                    .collect(),
            ),
            shutdown_called: Cell::default(),
        };

        result
    }

    pub fn shutdown(&self) -> Vec<ThreadShutdownResponse> {
        self.shutdown_called.set(true);
        vec![]
    }

    pub fn shutdown_called(&self) -> bool {
        self.shutdown_called.get()
    }
}

impl<E> ThreadPoolBatcher<E> for ThreadPoolBatcherMock<E::Request, E::Response>
where
    E: Element,
{
    fn batch_for_send<U>(&self, request: U) -> &Self
    where
        U: Into<ThreadRequest<E::Request>> + IdTargeted,
    {
        assert_eq!(request.into(), self.expected_requests.borrow()[0]);
        self.expected_requests.borrow_mut().remove(0);
        self
    }

    fn send_batch<V>(&self) -> Vec<V>
    where
        V: From<ThreadResponse<E::Response>> + IdTargeted,
    {
        let responses_len = self.responses.borrow().len();
        let expected_requests_len = self.expected_requests.borrow().len();

        self.responses
            .borrow_mut()
            // only return enough responses to match requests seen
            .drain(0..responses_len - expected_requests_len)
            .map(|r| V::from(r))
            .collect()
    }

    fn new(_thread_pool: Weak<ThreadPool<E>>) -> Self {
        panic!("should not be used in a scenario with this function is called");
    }

    fn shutdown_pool(&self) -> Vec<ThreadShutdownResponse> {
        self.shutdown()
    }

    fn get_thread_pool_size(&self) -> std::num::NonZeroUsize {
        todo!()
    }
}
