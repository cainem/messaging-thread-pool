use std::fmt::Debug;
use std::{cell::RefCell, marker::PhantomData};

use crate::{
    pool_item::PoolItem,
    request_response::{request_message::RequestMessage, response_message::ResponseMessage},
    thread_request_response::ThreadRequestResponse,
};

use super::ThreadPoolSenderAndReceiver;

/// This structure enables the mocking of a [`super::super::ThreadPool`]
///
/// It has two constructors.
/// One takes a vec of responses that are to be returned when send_and_receive is called, the
/// other defines the requests that are expected to be received as well as an array of responses
/// that are to be returned.
///
/// The implementation of send_and_receive asserts that the requests are as expected if requests
/// are provided.
/// If no requests are provided any requests passed in are ignored and the defined set of responses
/// are returned
#[derive(Debug)]
pub struct ThreadPoolMock<P, T, U>
where
    P: PoolItem + PartialEq,
    T: Into<ThreadRequestResponse<P>>,
    U: Into<ThreadRequestResponse<P>>,
{
    phantom_data: PhantomData<P>,
    assert_requests_equal: bool,
    expected_requests: RefCell<Vec<T>>,
    returned_responses: RefCell<Vec<U>>,
}

impl<P, T, U> ThreadPoolMock<P, T, U>
where
    P: PoolItem + PartialEq,
    T: Into<ThreadRequestResponse<P>>,
    U: Into<ThreadRequestResponse<P>>,
{
    pub fn new_with_expected_requests(
        expected_requests: Vec<T>,
        returned_responses: Vec<U>,
    ) -> Self {
        assert_eq!(
            expected_requests.len(),
            returned_responses.len(),
            "number of requests do not match number of responses"
        );
        Self {
            phantom_data: PhantomData,
            assert_requests_equal: true,
            expected_requests: RefCell::new(expected_requests),
            returned_responses: RefCell::new(returned_responses),
        }
    }

    pub fn new(returned_responses: Vec<U>) -> Self {
        Self {
            phantom_data: PhantomData,
            assert_requests_equal: false,
            expected_requests: RefCell::new(vec![]),
            returned_responses: RefCell::new(returned_responses),
        }
    }
}

impl<P, T1, U1> ThreadPoolSenderAndReceiver<P> for ThreadPoolMock<P, T1, U1>
where
    P: PoolItem + PartialEq,
    P::Api: PartialEq,
    P::Init: PartialEq,
    T1: Into<ThreadRequestResponse<P>> + Debug,
    U1: Into<ThreadRequestResponse<P>> + Debug,
{
    #[allow(clippy::needless_collect)]
    fn send_and_receive<'a, const N: usize, T, U>(
        &'a self,
        requests: impl Iterator<Item = T> + 'a,
    ) -> Box<dyn Iterator<Item = U> + 'a>
    where
        T: RequestMessage<N, P> + 'a,
        U: ResponseMessage<N, P> + 'a,
    {
        // materialize requests to establish len
        let requests: Vec<T> = requests.into_iter().collect();
        let actual_count = requests.len();

        if self.assert_requests_equal {
            let expected_count = self.expected_requests.borrow().iter().count();
            assert!(
                expected_count >= actual_count,
                "count of expected [{}] less than actual requests [{}]",
                expected_count,
                actual_count
            );
            self.expected_requests
                .borrow_mut()
                .drain(..actual_count)
                .map(|r| <T1 as Into<ThreadRequestResponse<P>>>::into(r))
                .zip(
                    requests
                        .into_iter()
                        .map(|r| <T as Into<ThreadRequestResponse<P>>>::into(r)),
                )
                // assert individually to get better error messages
                .for_each(|(expected, actual)| {
                    assert_eq!(expected, actual, "expected and actual requests differ")
                });
        }

        // convert type U1 to U via the intermediary ThreadRequestResponse
        let results: Vec<_> = self
            .returned_responses
            .borrow_mut()
            .drain(..actual_count)
            .map(|r| <U1 as Into<ThreadRequestResponse<P>>>::into(r))
            .map(|r| <U as From<ThreadRequestResponse<P>>>::from(r))
            .collect();

        Box::new(results.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        samples::{MeanRequest, MeanResponse, Randoms},
        thread_pool_sender_and_receiver::ThreadPoolSenderAndReceiver,
    };

    use super::ThreadPoolMock;

    #[test]
    fn two_responses_returned_over_multiple_requests() {
        let response_0 = MeanResponse { id: 1, mean: 22 };
        let response_1 = MeanResponse { id: 2, mean: 44 };

        let mock = ThreadPoolMock::<Randoms, MeanRequest, MeanResponse>::new(vec![
            response_0.clone(),
            response_1.clone(),
        ]);

        let results_0: Vec<MeanResponse> = mock
            .send_and_receive(vec![MeanRequest(1)].into_iter())
            .collect();
        let results_1: Vec<MeanResponse> = mock
            .send_and_receive(vec![MeanRequest(2)].into_iter())
            .collect();

        assert_eq!(1, results_0.len());
        assert_eq!(response_0, results_0[0]);
        assert_eq!(1, results_1.len());
        assert_eq!(response_1, results_1[0]);
    }

    #[test]
    #[should_panic]
    fn one_expected_request_differs_from_one_actual_request() {
        let request_0 = MeanRequest(1);
        let response_0 = MeanResponse { id: 1, mean: 22 };

        let mock = ThreadPoolMock::<Randoms, MeanRequest, MeanResponse>::new_with_expected_requests(
            vec![MeanRequest(2)],
            vec![response_0.clone()],
        );

        let _results: Vec<MeanResponse> =
            mock.send_and_receive(vec![request_0].into_iter()).collect();
    }

    #[test]
    #[should_panic(expected = "count of expected [1] less than actual requests [2]")]
    fn one_expected_request_actual_requests_2_panics() {
        let request_0 = MeanRequest(1);
        let response_0 = MeanResponse { id: 1, mean: 22 };

        let mock = ThreadPoolMock::<Randoms, MeanRequest, MeanResponse>::new_with_expected_requests(
            vec![request_0.clone()],
            vec![response_0.clone()],
        );

        let _results: Vec<MeanResponse> = mock
            .send_and_receive(vec![MeanRequest(1), MeanRequest(2)].into_iter())
            .collect();
    }

    #[test]
    fn empty_requests_and_responses_does_not_panic() {
        let mock = ThreadPoolMock::<Randoms, MeanRequest, MeanResponse>::new_with_expected_requests(
            vec![],
            vec![],
        );

        let _results: Vec<MeanResponse> = mock
            .send_and_receive(Vec::<MeanRequest>::default().into_iter())
            .collect();
    }

    #[test]
    #[should_panic(expected = "number of requests do not match number of responses")]
    fn unmatched_requests_and_responses() {
        let response_0 = MeanResponse { id: 1, mean: 22 };

        let mock = ThreadPoolMock::<Randoms, MeanRequest, MeanResponse>::new_with_expected_requests(
            vec![],
            vec![response_0.clone()],
        );

        let _results: Vec<MeanResponse> = mock
            .send_and_receive(vec![MeanRequest(1)].into_iter())
            .collect();
    }

    #[test]
    fn one_response_only_returns_expected_response() {
        let response_0 = MeanResponse { id: 1, mean: 22 };

        let mock =
            ThreadPoolMock::<Randoms, MeanRequest, MeanResponse>::new(vec![response_0.clone()]);

        let results: Vec<MeanResponse> = mock
            .send_and_receive(vec![MeanRequest(1)].into_iter())
            .collect();

        assert_eq!(1, results.len());
        assert_eq!(response_0, results[0]);
    }

    #[test]
    fn one_response_empty_requests_returns_empty_iterator() {
        let mock = ThreadPoolMock::<Randoms, MeanRequest, MeanResponse>::new(vec![MeanResponse {
            id: 1,
            mean: 0,
        }]);

        let results: Vec<MeanResponse> = mock
            .send_and_receive(Vec::<MeanRequest>::default().into_iter())
            .collect();

        assert_eq!(0, results.len());
    }

    #[test]
    fn zero_responses_returns_empty_iterator() {
        let mock = ThreadPoolMock::<Randoms, MeanRequest, MeanResponse>::new(vec![]);

        let results: Vec<MeanResponse> = mock
            .send_and_receive(Vec::<MeanRequest>::default().into_iter())
            .collect();

        assert_eq!(0, results.len());
    }
}
