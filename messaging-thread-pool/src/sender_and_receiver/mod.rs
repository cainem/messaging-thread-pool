//! # Sender and Receiver Abstractions
//!
//! This module provides traits for abstracting thread pool communication, enabling
//! dependency injection and testing.
//!
//! ## Key Types
//!
//! - [`SenderAndReceiver`] - Main trait for sending requests and receiving responses
//! - [`SenderAndReceiverMock`] - Mock implementation for testing
//! - [`ThreadSafeSenderAndReceiver`] - Thread-safe version for nested thread pools
//!
//! ## Testing with Mocks
//!
//! The [`SenderAndReceiver`] trait allows you to write code that works with both
//! real thread pools and mocks:
//!
//! ```rust
//! use messaging_thread_pool::{SenderAndReceiver, samples::*};
//!
//! // Function that depends on thread pool through trait
//! fn calculate_total_sum<T: SenderAndReceiver<Randoms>>(
//!     pool: &T,
//!     ids: &[u64],
//! ) -> u128 {
//!     pool.send_and_receive(ids.iter().map(|id| SumRequest(*id)))
//!         .expect("pool available")
//!         .map(|r: SumResponse| r.sum())
//!         .sum()
//! }
//! ```
//!
//! In tests, inject a [`SenderAndReceiverMock`]:
//!
//! ```rust
//! use messaging_thread_pool::{SenderAndReceiverMock, samples::*};
//!
//! let mock = SenderAndReceiverMock::<Randoms, SumRequest>::new_with_expected_requests(
//!     vec![SumRequest(1), SumRequest(2)],
//!     vec![
//!         SumResponse { id: 1, result: 100 },
//!         SumResponse { id: 2, result: 200 },
//!     ],
//! );
//!
//! // The mock verifies requests match expectations and returns predefined responses
//! ```

mod sender_and_receiver_mock;
pub mod sender_and_receiver_raw_mock;
mod thread_pool;

use std::iter;

use crate::{
    id_targeted::IdTargeted, pool_item::PoolItem, request_with_response::RequestWithResponse,
    sender_couplet::SenderCouplet,
};

use crossbeam_channel::SendError;
pub use sender_and_receiver_mock::SenderAndReceiverMock;

/// Trait for types that can send requests to pool items and receive responses.
///
/// This trait abstracts the communication mechanism with pool items, allowing:
/// - Code to be written generically over the communication mechanism
/// - Mock implementations for testing without spawning threads
/// - Different implementations for different threading strategies
///
/// # Usage
///
/// Write code that depends on this trait rather than [`ThreadPool`](crate::ThreadPool) directly:
///
/// ```rust
/// use messaging_thread_pool::{SenderAndReceiver, samples::*};
///
/// struct MyService<T: SenderAndReceiver<Randoms>> {
///     pool: T,
/// }
///
/// impl<T: SenderAndReceiver<Randoms>> MyService<T> {
///     fn get_mean(&self, id: u64) -> u128 {
///         self.pool
///             .send_and_receive_one(MeanRequest(id))
///             .expect("pool available")
///             .mean()
///     }
/// }
/// ```
///
/// # Note on Return Types
///
/// The `send_and_receive` method returns `Box<dyn Iterator>` rather than `impl Iterator`
/// due to limitations with trait return types. This has a small performance cost compared
/// to using [`ThreadPool`](crate::ThreadPool) directly.
///
/// If you don't need the abstraction for testing, you can use `ThreadPool` directly
/// to get `impl Iterator` returns.
pub trait SenderAndReceiver<P>
where
    P: PoolItem,
{
    /// Send multiple requests and receive their responses.
    ///
    /// Requests are distributed to the appropriate threads based on their IDs.
    /// Responses are returned in the order they complete (not necessarily request order).
    ///
    /// # Arguments
    ///
    /// * `requests` - Iterator of requests to send. Each request must implement
    ///   [`RequestWithResponse`](crate::RequestWithResponse) and [`IdTargeted`].
    ///
    /// # Returns
    ///
    /// A boxed iterator of responses. The iterator will yield one response per request.
    ///
    /// # Errors
    ///
    /// Returns `SendError` if the thread pool has been shut down.
    fn send_and_receive<'a, T>(
        &'a self,
        requests: impl Iterator<Item = T> + 'a,
    ) -> Result<Box<dyn Iterator<Item = T::Response> + 'a>, SendError<SenderCouplet<P>>>
    where
        T: RequestWithResponse<P> + IdTargeted + 'a;

    /// Convenience method for sending a single request and receiving its response.
    ///
    /// This is equivalent to calling `send_and_receive` with a single-item iterator,
    /// but with cleaner ergonomics for the single-request case.
    ///
    /// # Example
    ///
    /// ```rust
    /// use messaging_thread_pool::{SenderAndReceiver, SenderAndReceiverMock, samples::*};
    ///
    /// let mock = SenderAndReceiverMock::<Randoms, MeanRequest>::new(
    ///     vec![MeanResponse { id: 1, result: 42 }],
    /// );
    ///
    /// let response = mock.send_and_receive_one(MeanRequest(1)).expect("mock works");
    /// assert_eq!(response.mean(), 42);
    /// ```
    fn send_and_receive_one<'a, T>(
        &'a self,
        request: T,
    ) -> Result<T::Response, SendError<SenderCouplet<P>>>
    where
        T: RequestWithResponse<P> + IdTargeted + 'a,
    {
        let id = request.id();
        let mut responses = self.send_and_receive(iter::once(request))?;

        let Some(response) = responses.next() else {
            // panics if there has been a down stream panic
            panic!("response not received for request id {:?}", id);
        };

        assert!(
            responses.next().is_none(),
            "more than one response received"
        );

        Ok(response)
    }
}

/// A thread-safe version of [`SenderAndReceiver`].
///
/// This trait is useful when building nested thread pools, where inner thread pools
/// need to be `Send + Sync` to be shared across the outer pool's threads.
///
/// # Example: Nested Thread Pools
///
/// ```rust,ignore
/// use std::sync::Arc;
/// use messaging_thread_pool::{ThreadPool, ThreadSafeSenderAndReceiver};
///
/// struct OuterItem<T: ThreadSafeSenderAndReceiver<InnerItem>> {
///     id: u64,
///     inner_pool: Arc<T>,  // Shared across outer pool threads
/// }
/// ```
///
/// See [`samples::RandomsBatch`](crate::samples::RandomsBatch) for a complete example
/// of nested thread pools.
pub trait ThreadSafeSenderAndReceiver<P>: SenderAndReceiver<P> + Send + Sync
where
    P: PoolItem,
{
}

#[cfg(test)]
mod tests {
    use crate::{
        SenderAndReceiver, SenderAndReceiverMock,
        samples::{MeanRequest, MeanResponse, Randoms},
    };

    #[test]
    fn send_and_receive_one_functions_as_expected() {
        let expected_response = MeanResponse { id: 1, result: 10 };
        let target = SenderAndReceiverMock::<Randoms, MeanRequest>::new_with_expected_requests(
            vec![MeanRequest(1)],
            vec![expected_response.clone()],
        );

        let request = MeanRequest(1);

        let Ok(response) = target.send_and_receive_one(request) else {
            panic!("not ok")
        };

        assert_eq!(expected_response, response);
    }
}
