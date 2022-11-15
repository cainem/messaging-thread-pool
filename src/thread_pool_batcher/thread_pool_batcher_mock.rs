use std::{
    cell::{Cell, RefCell},
    sync::Weak,
};

use crate::{
    id_targeted::IdTargeted,
    pool_item::PoolItem,
    thread_request_response::{
        thread_shutdown_response::ThreadShutdownResponse, ThreadRequestResponse,
    },
    ThreadPool,
};

use super::ThreadPoolBatcher;

/// A struct for mocking the request/response interface for a given pair of Request/Responses
/// It is constructed with a vec of requests that it is expecting to receive and an vec of
/// hard coded responses to those requests
///
/// It validates that the requests received are indeed the ones received.
#[derive(Debug)]
pub struct ThreadPoolBatcherMock<E>
where
    E: PoolItem,
{
    expected_requests: RefCell<Vec<ThreadRequestResponse<E>>>,
    responses: RefCell<Vec<ThreadRequestResponse<E>>>,
    shutdown_called: Cell<bool>,
}

impl<E> ThreadPoolBatcherMock<E>
where
    E: PoolItem,
{
    pub fn new(
        mut expected_requests: Vec<ThreadRequestResponse<E>>,
        mut responses: Vec<ThreadRequestResponse<E>>,
    ) -> Self {
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
        assert!(
            !expected_requests.iter().any(|r| r.is_response()),
            "all expected requests must be requests"
        );
        assert!(
            !responses.iter().any(|r| r.is_request()),
            "all responses must be responses"
        );

        let result = Self {
            expected_requests: RefCell::new(expected_requests.drain(..).collect()),
            responses: RefCell::new(responses.drain(..).collect()),
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

impl<E> ThreadPoolBatcher<E> for ThreadPoolBatcherMock<E>
where
    E: PoolItem + PartialEq,
    E::Init: PartialEq,
    E::Api: PartialEq,
{
    fn batch_for_send<U>(&self, request: U) -> &Self
    where
        U: Into<ThreadRequestResponse<E>> + IdTargeted,
    {
        assert_eq!(request.into(), self.expected_requests.borrow()[0]);
        self.expected_requests.borrow_mut().remove(0);
        self
    }

    fn send_batch<V>(&self) -> Vec<V>
    where
        V: From<ThreadRequestResponse<E>> + IdTargeted,
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
