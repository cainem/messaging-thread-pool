use std::fmt::Debug;
use std::sync::Mutex;

use crossbeam_channel::SendError;

use crate::{
    pool_item::PoolItem, request_with_response::RequestWithResponse, sender_couplet::SenderCouplet,
    thread_request_response::ThreadRequestResponse,
};

use super::SenderAndReceiver;

/// A mock implementation of [`SenderAndReceiver`] for testing.
///
/// This mock allows you to test code that depends on thread pools without actually
/// spawning threads. It provides two modes of operation:
///
/// 1. **Response-only mode**: Returns predefined responses regardless of requests
/// 2. **Request verification mode**: Asserts that requests match expectations
///
/// # Example: Response-Only Mode
///
/// Use this when you only care about the responses, not the exact requests:
///
/// ```rust
/// use messaging_thread_pool::{SenderAndReceiver, SenderAndReceiverMock, samples::*};
///
/// // Create mock that returns predefined responses
/// let mock = SenderAndReceiverMock::<Randoms, MeanRequest>::new(vec![
///     MeanResponse { id: 1, result: 100 },
///     MeanResponse { id: 2, result: 200 },
/// ]);
///
/// // Any requests will return the predefined responses in order
/// let responses: Vec<_> = mock
///     .send_and_receive([MeanRequest(1), MeanRequest(2)].into_iter())
///     .unwrap()
///     .collect();
///
/// assert_eq!(responses[0].result, 100);
/// assert_eq!(responses[1].result, 200);
/// ```
///
/// # Example: Request Verification Mode
///
/// Use this when you want to verify the exact requests being sent:
///
/// ```rust
/// use messaging_thread_pool::{SenderAndReceiver, SenderAndReceiverMock, samples::*};
///
/// // Create mock with expected requests and responses
/// let mock = SenderAndReceiverMock::<Randoms, SumRequest>::new_with_expected_requests(
///     vec![SumRequest(1), SumRequest(2)],  // Expected requests
///     vec![
///         SumResponse { id: 1, result: 100 },
///         SumResponse { id: 2, result: 200 },
///     ],
/// );
///
/// // The mock will assert that requests match expectations
/// let responses: Vec<_> = mock
///     .send_and_receive([SumRequest(1), SumRequest(2)].into_iter())
///     .unwrap()
///     .collect();
///
/// // Verify all responses were consumed
/// mock.assert_is_complete();
/// ```
///
/// # Verification Methods
///
/// - [`was_called()`](Self::was_called) - Check if `send_and_receive` was invoked
/// - [`is_complete()`](Self::is_complete) - Check if all responses have been returned
/// - [`assert_is_complete()`](Self::assert_is_complete) - Panic if responses remain
///
/// # Type Parameters
///
/// - `P` - The pool item type (e.g., `Randoms`)
/// - `T` - The request type (e.g., `MeanRequest`)
///
/// The mock can return responses for any request type that maps to the same pool item,
/// but verification mode requires the request types to match.
#[derive(Debug, Default)]
pub struct SenderAndReceiverMock<P, T>
where
    P: PoolItem,
    T: RequestWithResponse<P>,
{
    assert_requests_equal: bool,
    was_called: Mutex<bool>,
    expected_requests: Mutex<Vec<T>>,
    returned_responses: Mutex<Vec<T::Response>>,
}

impl<P, T> SenderAndReceiverMock<P, T>
where
    P: PoolItem,
    T: RequestWithResponse<P>,
{
    /// Creates a mock that verifies requests match expectations.
    ///
    /// When `send_and_receive` is called, the mock will:
    /// 1. Assert that each request matches the corresponding expected request
    /// 2. Return the corresponding response
    ///
    /// # Panics
    ///
    /// - Panics if `expected_requests.len() != returned_responses.len()`
    /// - Panics during `send_and_receive` if a request doesn't match expectations
    ///
    /// # Example
    ///
    /// ```rust
    /// use messaging_thread_pool::{SenderAndReceiverMock, samples::*};
    ///
    /// let mock = SenderAndReceiverMock::<Randoms, MeanRequest>::new_with_expected_requests(
    ///     vec![MeanRequest(1)],
    ///     vec![MeanResponse { id: 1, result: 42 }],
    /// );
    /// ```
    pub fn new_with_expected_requests(
        expected_requests: Vec<T>,
        returned_responses: Vec<T::Response>,
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

    /// Creates a mock that returns predefined responses without verifying requests.
    ///
    /// This is useful when you only care about the returned values and don't need
    /// to verify the exact requests being sent.
    ///
    /// # Example
    ///
    /// ```rust
    /// use messaging_thread_pool::{SenderAndReceiverMock, samples::*};
    ///
    /// let mock = SenderAndReceiverMock::<Randoms, SumRequest>::new(vec![
    ///     SumResponse { id: 1, result: 100 },
    ///     SumResponse { id: 2, result: 200 },
    /// ]);
    /// ```
    pub fn new(returned_responses: Vec<T::Response>) -> Self {
        Self {
            was_called: Mutex::new(false),
            assert_requests_equal: false,
            expected_requests: Mutex::new(vec![]),
            returned_responses: Mutex::new(returned_responses),
        }
    }

    /// Returns `true` if `send_and_receive` has been called at least once.
    ///
    /// Useful for verifying that your code actually used the mock.
    pub fn was_called(&self) -> bool {
        *self.was_called.lock().expect("that lock will never fail")
    }

    /// Returns `true` if all predefined responses have been consumed.
    ///
    /// This helps verify that all expected interactions occurred.
    pub fn is_complete(&self) -> bool {
        self.returned_responses
            .lock()
            .expect("that the lock will never fail")
            .is_empty()
    }

    /// Asserts that all predefined responses have been consumed.
    ///
    /// # Panics
    ///
    /// Panics if any responses remain unconsumed, indicating that fewer
    /// requests were made than expected.
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

impl<P, T1> SenderAndReceiver<P> for SenderAndReceiverMock<P, T1>
where
    P: PoolItem + PartialEq,
    P::Api: PartialEq,
    P::Init: PartialEq,
    T1: RequestWithResponse<P>,
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
            .lock()
            .unwrap()
            .drain(..actual_count)
            .map(<T1::Response as Into<ThreadRequestResponse<P>>>::into)
            .map(<T::Response as From<ThreadRequestResponse<P>>>::from)
            .collect();

        Ok(Box::new(results.into_iter()))
    }
}

#[cfg(test)]
mod tests {

    use std::hint::black_box;

    use crate::{
        samples::{MeanRequest, MeanResponse, Randoms},
        sender_and_receiver::SenderAndReceiver,
    };

    use super::SenderAndReceiverMock;

    #[test]
    fn check_mock_send_and_sync() {
        // enforce that the mock is send and sync when the request is
        let target = SenderAndReceiverMock::<Randoms, MeanRequest>::new(vec![]);
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
        let response_0 = MeanResponse { id: 1, result: 22 };
        let response_1 = MeanResponse { id: 2, result: 44 };

        let mock = SenderAndReceiverMock::<Randoms, MeanRequest>::new(vec![
            response_0.clone(),
            response_1.clone(),
        ]);
        assert!(!mock.was_called());

        let _results_0: Vec<MeanResponse> = mock
            .send_and_receive(vec![MeanRequest(1)].into_iter())
            .unwrap()
            .collect();

        assert!(!mock.is_complete());
    }

    #[test]
    fn two_responses_returned_over_multiple_requests() {
        let response_0 = MeanResponse { id: 1, result: 22 };
        let response_1 = MeanResponse { id: 2, result: 44 };

        let mock = SenderAndReceiverMock::<Randoms, MeanRequest>::new(vec![
            response_0.clone(),
            response_1.clone(),
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
        let response_0 = MeanResponse { id: 1, result: 22 };

        let mock = SenderAndReceiverMock::<Randoms, MeanRequest>::new_with_expected_requests(
            vec![MeanRequest(2)],
            vec![response_0],
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
        let response_0 = MeanResponse { id: 1, result: 22 };

        let mock = SenderAndReceiverMock::<Randoms, MeanRequest>::new_with_expected_requests(
            vec![request_0],
            vec![response_0],
        );

        let _results: Vec<MeanResponse> = mock
            .send_and_receive(vec![MeanRequest(1), MeanRequest(2)].into_iter())
            .unwrap()
            .collect();
        assert!(mock.was_called());
    }

    #[test]
    fn empty_requests_and_responses_does_not_panic() {
        let mock = SenderAndReceiverMock::<Randoms, MeanRequest>::new_with_expected_requests(
            vec![],
            vec![],
        );

        let _results: Vec<MeanResponse> = mock
            .send_and_receive(Vec::<MeanRequest>::default().into_iter())
            .unwrap()
            .collect();
        assert!(mock.was_called());
    }

    #[test]
    #[should_panic(expected = "number of requests do not match number of responses")]
    fn unmatched_requests_and_responses() {
        let response_0 = MeanResponse { id: 1, result: 22 };

        let mock = SenderAndReceiverMock::<Randoms, MeanRequest>::new_with_expected_requests(
            vec![],
            vec![response_0],
        );

        let _results: Vec<MeanResponse> = mock
            .send_and_receive(vec![MeanRequest(1)].into_iter())
            .unwrap()
            .collect();
    }

    #[test]
    fn one_response_only_returns_expected_response() {
        let response_0 = MeanResponse { id: 1, result: 22 };

        let mock = SenderAndReceiverMock::<Randoms, MeanRequest>::new(vec![response_0.clone()]);

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
        let mock = SenderAndReceiverMock::<Randoms, MeanRequest>::new(vec![MeanResponse {
            id: 1,
            result: 0,
        }]);

        let results: Vec<MeanResponse> = mock
            .send_and_receive(Vec::<MeanRequest>::default().into_iter())
            .unwrap()
            .collect();

        assert_eq!(0, results.len());
        assert!(mock.was_called());
    }

    #[test]
    fn zero_responses_returns_empty_iterator() {
        let mock = SenderAndReceiverMock::<Randoms, MeanRequest>::new(vec![]);

        let results: Vec<MeanResponse> = mock
            .send_and_receive(Vec::<MeanRequest>::default().into_iter())
            .unwrap()
            .collect();

        assert_eq!(0, results.len());
        assert!(mock.was_called());
    }
}
