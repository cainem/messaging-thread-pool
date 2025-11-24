use std::fmt::Debug;
use std::sync::Mutex;

use crossbeam_channel::SendError;

use crate::{
    pool_item::PoolItem, request_with_response::RequestWithResponse, sender_couplet::SenderCouplet,
    thread_request_response::ThreadRequestResponse,
};

use super::SenderAndReceiver;

/// This structure enables the mocking of a [`crate::ThreadPool`]
/// There are 2 variants; one that works with RequestResponses and one (this one) that works
/// with raw ThreadRequestResponses.
/// Working directly with ThreadRequestResponses allows for a heterogeneous mix of messages
/// which is important if your mocked scenario covers the sending and receiving of mixed
/// message types
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
#[derive(Debug, Default)]
pub struct SenderAndReceiverRawMock<P>
where
    P: PoolItem,
{
    assert_requests_equal: bool,
    was_called: Mutex<bool>,
    expected_requests: Mutex<Vec<ThreadRequestResponse<P>>>,
    returned_responses: Mutex<Vec<ThreadRequestResponse<P>>>,
}

impl<P> SenderAndReceiverRawMock<P>
where
    P: PoolItem,
{
    pub fn new_with_expected_requests(
        expected_requests: Vec<ThreadRequestResponse<P>>,
        returned_responses: Vec<ThreadRequestResponse<P>>,
    ) -> Self {
        assert_eq!(
            expected_requests.len(),
            returned_responses.len(),
            "number of requests do not match number of responses"
        );
        Self {
            was_called: Mutex::new(false),
            assert_requests_equal: true,
            expected_requests: Mutex::new(expected_requests),
            returned_responses: Mutex::new(returned_responses),
        }
    }

    pub fn new(returned_responses: Vec<ThreadRequestResponse<P>>) -> Self {
        Self {
            was_called: Mutex::new(false),
            assert_requests_equal: false,
            expected_requests: Mutex::new(vec![]),
            returned_responses: Mutex::new(returned_responses),
        }
    }

    /// Verify that the mock send_and_receive was called at least once
    pub fn was_called(&self) -> bool {
        *self.was_called.lock().expect("that lock will never fail")
    }

    /// Verify that all of the responses have been returned
    pub fn is_complete(&self) -> bool {
        self.returned_responses
            .lock()
            .expect("that the lock will never fail")
            .is_empty()
    }

    /// Assert that all of the responses have been returned; provides
    pub fn assert_is_complete(&self) {
        let unreturned_responses = self
            .returned_responses
            .lock()
            .expect("that the lock will never fail")
            .len();
        assert!(
            unreturned_responses == 0,
            "expected all responses to have been consumed, {} remaining",
            unreturned_responses
        );
    }
}

impl<P> SenderAndReceiver<P> for SenderAndReceiverRawMock<P>
where
    P: PoolItem + PartialEq,
    P::Api: PartialEq,
    P::Init: PartialEq,
{
    #[allow(clippy::needless_collect)]
    fn send_and_receive<'a, T>(
        &'a self,
        requests: impl Iterator<Item = T> + 'a,
    ) -> Result<Box<dyn Iterator<Item = T::Response> + 'a>, SendError<SenderCouplet<P>>>
    where
        T: RequestWithResponse<P> + 'a,
    {
        // materialize requests to establish len
        let requests: Vec<T> = requests.into_iter().collect();
        let actual_count = requests.len();
        match self.was_called.lock() {
            Ok(mut result) => *result = true,
            _ => panic!(),
        }

        if self.assert_requests_equal {
            let expected_count = self.expected_requests.lock().unwrap().iter().count();
            assert!(
                expected_count >= actual_count,
                "count of expected [{expected_count}] less than actual requests [{actual_count}]"
            );
            self.expected_requests
                .lock()
                .unwrap()
                .drain(..actual_count)
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
            .lock()
            .unwrap()
            .drain(..actual_count)
            .map(<T::Response as From<ThreadRequestResponse<P>>>::from)
            .collect();

        Ok(Box::new(results.into_iter()))
    }
}

#[cfg(test)]
mod tests {

    use std::hint::black_box;

    use crate::{
        samples::{MeanRequest, MeanResponse, Randoms, SumRequest, SumResponse},
        sender_and_receiver::SenderAndReceiver,
    };

    use super::SenderAndReceiverRawMock;

    #[test]
    fn two_responses_returned_over_multiple_heterogeneous_requests() {
        let response_0 = MeanResponse { id: 1, mean: 22 };
        let response_1 = SumResponse { id: 2, sum: 44 };

        let mock = SenderAndReceiverRawMock::<Randoms>::new(vec![
            response_0.clone().into(),
            response_1.clone().into(),
        ]);
        assert!(!mock.was_called());

        let results_0: Vec<MeanResponse> = mock
            .send_and_receive(vec![MeanRequest(1)].into_iter())
            .unwrap()
            .collect();
        let results_1: Vec<SumResponse> = mock
            .send_and_receive(vec![SumRequest(2)].into_iter())
            .unwrap()
            .collect();

        assert_eq!(1, results_0.len());
        assert_eq!(response_0, results_0[0]);
        assert_eq!(1, results_1.len());
        assert_eq!(response_1, results_1[0]);
    }

    #[test]
    fn check_mock_send_and_sync() {
        // enforce that the mock is send and sync when the request is
        let target = SenderAndReceiverRawMock::<Randoms>::new(vec![]);
        send_and_sync(target);
    }

    fn send_and_sync<T>(_check_me: T)
    where
        T: Send + Sync,
    {
        black_box(())
    }

    #[test]
    fn two_responses_only_one_returned_is_complete_returns_false() {
        let response_0 = MeanResponse { id: 1, mean: 22 };
        let response_1 = MeanResponse { id: 2, mean: 44 };

        let mock = SenderAndReceiverRawMock::<Randoms>::new(vec![
            response_0.clone().into(),
            response_1.clone().into(),
        ]);
        assert!(!mock.was_called());

        let results_0: Vec<MeanResponse> = mock
            .send_and_receive(vec![MeanRequest(1)].into_iter())
            .unwrap()
            .collect();

        assert!(!mock.is_complete());
        assert_eq!(1, results_0.len());
        assert_eq!(response_0, results_0[0]);
    }

    #[test]
    fn two_responses_returned_over_multiple_requests() {
        let response_0 = MeanResponse { id: 1, mean: 22 };
        let response_1 = MeanResponse { id: 2, mean: 44 };

        let mock = SenderAndReceiverRawMock::<Randoms>::new(vec![
            response_0.clone().into(),
            response_1.clone().into(),
        ]);
        assert!(!mock.was_called());

        let results_0: Vec<MeanResponse> = mock
            .send_and_receive(vec![MeanRequest(1)].into_iter())
            .unwrap()
            .collect();
        let results_1: Vec<MeanResponse> = mock
            .send_and_receive(vec![MeanRequest(2)].into_iter())
            .unwrap()
            .collect();

        assert!(mock.is_complete());
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

        let mock = SenderAndReceiverRawMock::<Randoms>::new_with_expected_requests(
            vec![MeanRequest(2).into()],
            vec![response_0.into()],
        );

        let _results: Vec<MeanResponse> = mock
            .send_and_receive(vec![request_0].into_iter())
            .unwrap()
            .collect();
        assert!(mock.was_called());
    }

    #[test]
    #[should_panic(expected = "count of expected [1] less than actual requests [2]")]
    fn one_expected_request_actual_requests_2_panics() {
        let request_0 = MeanRequest(1);
        let response_0 = MeanResponse { id: 1, mean: 22 };

        let mock = SenderAndReceiverRawMock::<Randoms>::new_with_expected_requests(
            vec![request_0.into()],
            vec![response_0.into()],
        );

        let _results: Vec<MeanResponse> = mock
            .send_and_receive(vec![MeanRequest(1), MeanRequest(2)].into_iter())
            .unwrap()
            .collect();
        assert!(mock.was_called());
    }

    #[test]
    fn empty_requests_and_responses_does_not_panic() {
        let mock = SenderAndReceiverRawMock::<Randoms>::new_with_expected_requests(vec![], vec![]);

        let _results: Vec<MeanResponse> = mock
            .send_and_receive(Vec::<MeanRequest>::default().into_iter())
            .unwrap()
            .collect();
        assert!(mock.was_called());
    }

    #[test]
    #[should_panic(expected = "number of requests do not match number of responses")]
    fn unmatched_requests_and_responses() {
        let response_0 = MeanResponse { id: 1, mean: 22 };

        let mock = SenderAndReceiverRawMock::<Randoms>::new_with_expected_requests(
            vec![],
            vec![response_0.into()],
        );

        let _results: Vec<MeanResponse> = mock
            .send_and_receive(vec![MeanRequest(1)].into_iter())
            .unwrap()
            .collect();
    }

    #[test]
    fn one_response_only_returns_expected_response() {
        let response_0 = MeanResponse { id: 1, mean: 22 };

        let mock = SenderAndReceiverRawMock::<Randoms>::new(vec![response_0.clone().into()]);

        let results: Vec<MeanResponse> = mock
            .send_and_receive(vec![MeanRequest(1)].into_iter())
            .unwrap()
            .collect();

        assert_eq!(1, results.len());
        assert_eq!(response_0, results[0]);
        assert!(mock.was_called());
    }

    #[test]
    fn one_response_empty_requests_returns_empty_iterator() {
        let mock =
            SenderAndReceiverRawMock::<Randoms>::new(vec![MeanResponse { id: 1, mean: 0 }.into()]);

        let results: Vec<MeanResponse> = mock
            .send_and_receive(Vec::<MeanRequest>::default().into_iter())
            .unwrap()
            .collect();

        assert_eq!(0, results.len());
        assert!(mock.was_called());
    }

    #[test]
    fn zero_responses_returns_empty_iterator() {
        let mock = SenderAndReceiverRawMock::<Randoms>::new(vec![]);

        let results: Vec<MeanResponse> = mock
            .send_and_receive(Vec::<MeanRequest>::default().into_iter())
            .unwrap()
            .collect();

        assert_eq!(0, results.len());
        assert!(mock.was_called());
    }
}
