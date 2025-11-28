//! # Messaging Thread Pool
//!
//! A typed thread pool library for managing stateful, long-lived objects that communicate
//! via messages. Unlike traditional thread pools, objects in this pool are **pinned to their
//! assigned thread** for their entire lifetime, enabling the use of non-`Send`/`Sync` types
//! like `Rc<RefCell<T>>`.
//!
//! ## When to Use This Library
//!
//! Use `messaging_thread_pool` when you need:
//! - **Thread-bound state**: Objects that own `Rc`, `RefCell`, raw pointers, or FFI resources
//! - **Actor-like patterns**: Long-lived stateful objects with message-based APIs
//! - **Sequential consistency**: Messages to the same object processed in order, no races
//! - **Lock-free operations**: No `Mutex`/`RwLock` needed since each object has single-threaded access
//!
//! If your data is `Send + Sync` and you just need parallel computation, consider [`rayon`](https://docs.rs/rayon)
//! instead.
//!
//! ## Quick Start
//!
//! The recommended approach uses the `#[pool_item]` attribute macro to minimize boilerplate:
//!
//! ```rust
//! use messaging_thread_pool::{ThreadPool, IdTargeted, pool_item};
//!
//! // 1. Define your struct with an ID field
//! #[derive(Debug)]
//! pub struct Counter {
//!     id: u64,
//!     value: i64,
//! }
//!
//! impl IdTargeted for Counter {
//!     fn id(&self) -> u64 { self.id }
//! }
//!
//! // 2. Use #[pool_item] on the impl block and #[messaging] on methods
//! #[pool_item]
//! impl Counter {
//!     pub fn new(id: u64) -> Self {
//!         Self { id, value: 0 }
//!     }
//!
//!     #[messaging(IncrementRequest, IncrementResponse)]
//!     pub fn increment(&mut self, amount: i64) -> i64 {
//!         self.value += amount;
//!         self.value
//!     }
//!
//!     #[messaging(GetValueRequest, GetValueResponse)]
//!     pub fn get_value(&self) -> i64 {
//!         self.value
//!     }
//! }
//!
//! // 3. Create a thread pool and interact with pool items
//! let pool = ThreadPool::<Counter>::new(4);
//!
//! // Create a counter with ID 1
//! pool.send_and_receive_once(CounterInit(1)).expect("pool available");
//!
//! // Increment it
//! let response: IncrementResponse = pool
//!     .send_and_receive_once(IncrementRequest(1, 10))
//!     .expect("pool available");
//! assert_eq!(response.result, 10);
//!
//! // Get current value
//! let response: GetValueResponse = pool
//!     .send_and_receive_once(GetValueRequest(1))
//!     .expect("pool available");
//! assert_eq!(response.result, 10);
//! ```
//!
//! ## Key Concepts
//!
//! ### Pool Items
//!
//! A **pool item** is any struct that implements the [`PoolItem`] trait. Each pool item:
//! - Has a unique `u64` ID within the pool
//! - Lives on a single thread (determined by `id % thread_count`)
//! - Receives messages sequentially via its `process_message` method
//!
//! The `#[pool_item]` macro generates the [`PoolItem`] implementation for you.
//!
//! ### Messages
//!
//! Communication with pool items happens through **request/response messages**:
//! - Each method marked with `#[messaging(RequestType, ResponseType)]` becomes a message endpoint
//! - The macro generates the request struct (with ID + method parameters) and response struct
//! - Messages are routed to the correct thread based on the target ID
//!
//! ### Thread Affinity
//!
//! Pool items are distributed across threads using `id % thread_count`. All messages
//! targeting the same ID go to the same thread, ensuring:
//! - No concurrent access to the same pool item
//! - Consistent ordering of message processing
//! - Warm CPU caches for frequently accessed items
//!
//! ## Using Non-Send/Sync Types
//!
//! The main advantage of this library is supporting thread-bound types. Here's an example
//! using `Rc<RefCell<T>>` for shared internal state:
//!
//! ```rust
//! use std::cell::RefCell;
//! use std::rc::Rc;
//! use messaging_thread_pool::{ThreadPool, IdTargeted, pool_item};
//!
//! #[derive(Debug, Clone)]
//! struct Helper {
//!     data: Rc<RefCell<Vec<String>>>,
//! }
//!
//! #[derive(Debug)]
//! pub struct Session {
//!     id: u64,
//!     data: Rc<RefCell<Vec<String>>>,
//!     helper: Helper,
//! }
//!
//! impl IdTargeted for Session {
//!     fn id(&self) -> u64 { self.id }
//! }
//!
//! #[pool_item]
//! impl Session {
//!     pub fn new(id: u64) -> Self {
//!         let data = Rc::new(RefCell::new(Vec::new()));
//!         let helper = Helper { data: data.clone() };
//!         Self { id, data, helper }
//!     }
//!
//!     #[messaging(AddRequest, AddResponse)]
//!     pub fn add(&self, item: String) {
//!         // No locks needed - just borrow_mut!
//!         self.helper.data.borrow_mut().push(item);
//!     }
//! }
//! ```
//!
//! See [`samples::UserSession`] for a complete working example.
//!
//! ## Batch Operations
//!
//! For efficiency, send multiple requests at once using [`ThreadPool::send_and_receive`]:
//!
//! ```rust
//! # use messaging_thread_pool::{ThreadPool, samples::*};
//! let pool = ThreadPool::<Randoms>::new(4);
//!
//! // Create 100 items in parallel
//! pool.send_and_receive((0..100u64).map(RandomsAddRequest))
//!     .expect("pool available")
//!     .for_each(|response| assert!(response.result().is_ok()));
//!
//! // Query all of them
//! let sums: Vec<u128> = pool
//!     .send_and_receive((0..100u64).map(SumRequest))
//!     .expect("pool available")
//!     .map(|r| r.sum())
//!     .collect();
//! ```
//!
//! ## Testing with Mocks
//!
//! The [`SenderAndReceiver`] trait allows mocking the thread pool in tests:
//!
//! ```rust
//! use messaging_thread_pool::{SenderAndReceiver, SenderAndReceiverMock, samples::*};
//!
//! // Code that depends on a thread pool takes a generic parameter
//! fn sum_means<T: SenderAndReceiver<Randoms>>(pool: &T, ids: &[u64]) -> u128 {
//!     pool.send_and_receive(ids.iter().map(|id| MeanRequest(*id)))
//!         .expect("pool available")
//!         .map(|r: MeanResponse| r.mean())
//!         .sum()
//! }
//!
//! // In tests, use SenderAndReceiverMock
//! let mock = SenderAndReceiverMock::<Randoms, MeanRequest>::new_with_expected_requests(
//!     vec![MeanRequest(1), MeanRequest(2)],
//!     vec![
//!         MeanResponse { id: 1, result: 100 },
//!         MeanResponse { id: 2, result: 200 },
//!     ],
//! );
//!
//! assert_eq!(sum_means(&mock, &[1, 2]), 300);
//! ```
//!
//! See [`samples`] for more comprehensive examples, and [`SenderAndReceiverMock`] for
//! mock configuration options.
//!
//! ## The `#[pool_item]` Macro
//!
//! The macro accepts optional parameters:
//!
//! ```rust,ignore
//! // Custom initialization request type (for complex constructors)
//! #[pool_item(Init = "MyCustomInitRequest")]
//!
//! // Custom shutdown handler
//! #[pool_item(Shutdown = "my_shutdown_method")]
//!
//! // Both
//! #[pool_item(Init = "MyCustomInitRequest", Shutdown = "my_shutdown_method")]
//! ```
//!
//! For details, see the macro documentation in [`macro@pool_item`].
//!
//! ## Legacy API
//!
//! The [`api_specification!`] macro is the older way to define pool items. New code should
//! use `#[pool_item]` instead, which is simpler and generates less boilerplate.
//!
//! ## Performance Considerations
//!
//! The message-passing overhead becomes significant for very short operations (<50ms).
//! This library is best suited for:
//! - CPU-bound work with moderate to long execution times
//! - Operations where thread affinity improves cache locality
//! - Stateful objects that would otherwise require complex locking
//!
//! ## Module Overview
//!
//! - [`ThreadPool`] - The main entry point for creating and managing pools
//! - [`PoolItem`] - Trait implemented by types managed in the pool
//! - [`IdTargeted`] - Trait for types that have an ID for routing
//! - [`SenderAndReceiver`] - Trait for abstracting pool communication (enables mocking)
//! - [`samples`] - Example implementations to learn from
//! - [`id_provider`] - Utilities for generating unique IDs

extern crate self as messaging_thread_pool;

use crate::thread_endpoint::ThreadEndpoint;
use std::cell::RefCell;
use std::sync::RwLock;

pub mod api_specification;
pub mod global_test_scope;
pub mod id_being_processed;
pub mod id_provider;
pub mod samples;
pub mod sender_couplet;

pub use messaging_thread_pool_macros::pool_item;

mod drop;
mod id_based_blocking;
mod id_targeted;
mod new;
pub mod pool_item;
mod pool_thread;
mod receive;
pub mod request_response;
mod request_with_response;
mod send;
mod send_and_receive;
mod sender_and_receiver;
mod shutdown;
mod thread_endpoint;
pub mod thread_request_response;

pub use id_based_blocking::*;
pub use id_being_processed::*;
pub use id_targeted::IdTargeted;
pub use pool_item::*;
pub use request_response::RequestResponse;
pub use request_with_response::RequestWithResponse;
pub use sender_and_receiver::*;
pub use sender_couplet::*;
pub use thread_request_response::*;

thread_local! {
    pub static ID_BEING_PROCESSED: RefCell<Option<u64>> = const { RefCell::new(None) };
}

/// A pool of threads for managing stateful [`PoolItem`] instances.
///
/// `ThreadPool` is the main entry point for this library. It:
/// - Spawns a fixed number of worker threads
/// - Distributes pool items across threads based on their IDs
/// - Routes messages to the correct thread for processing
/// - Ensures sequential message processing per pool item (no concurrent access)
///
/// # Creating a Thread Pool
///
/// ```rust
/// use messaging_thread_pool::{ThreadPool, samples::Randoms};
///
/// // Create a pool with 4 worker threads
/// let pool = ThreadPool::<Randoms>::new(4);
///
/// // The pool is now ready to accept messages
/// assert_eq!(pool.thread_count(), 4);
/// ```
///
/// # Creating Pool Items
///
/// Pool items are created by sending initialization requests:
///
/// ```rust
/// use messaging_thread_pool::{ThreadPool, samples::*};
///
/// let pool = ThreadPool::<Randoms>::new(4);
///
/// // Create a single item with ID 0
/// pool.send_and_receive_once(RandomsAddRequest(0)).expect("pool available");
///
/// // Or create many at once (IDs 1-99, since 0 already exists)
/// pool.send_and_receive((1..100u64).map(RandomsAddRequest))
///     .expect("pool available")
///     .for_each(|response| assert!(response.result().is_ok()));
/// ```
///
/// # Sending Messages
///
/// Once items exist, send messages to interact with them:
///
/// ```rust
/// use messaging_thread_pool::{ThreadPool, samples::*};
///
/// let pool = ThreadPool::<Randoms>::new(4);
/// pool.send_and_receive_once(RandomsAddRequest(1)).expect("pool available");
///
/// // Send a single message
/// let response: MeanResponse = pool
///     .send_and_receive_once(MeanRequest(1))
///     .expect("pool available");
///
/// // Send multiple messages (processed in parallel across threads)
/// // First create items 0-9
/// pool.send_and_receive((0..10u64).filter(|id| *id != 1).map(RandomsAddRequest))
///     .expect("pool available")
///     .for_each(|_| {});
/// let responses: Vec<SumResponse> = pool
///     .send_and_receive((0..10u64).map(SumRequest))
///     .expect("pool available")
///     .collect();
/// ```
///
/// # Removing Items
///
/// Items can be removed explicitly:
///
/// ```rust
/// use messaging_thread_pool::{ThreadPool, samples::*, RemovePoolItemRequest};
///
/// let pool = ThreadPool::<Randoms>::new(4);
/// pool.send_and_receive_once(RandomsAddRequest(1)).expect("pool available");
///
/// let response = pool
///     .send_and_receive_once(RemovePoolItemRequest(1))
///     .expect("pool available");
/// assert!(response.item_existed());
/// ```
///
/// # Shutdown
///
/// The pool shuts down automatically when dropped, or explicitly via [`shutdown`](Self::shutdown):
///
/// ```rust
/// use messaging_thread_pool::{ThreadPool, samples::Randoms};
///
/// let pool = ThreadPool::<Randoms>::new(4);
/// // ... use the pool ...
///
/// // Explicit shutdown (returns shutdown responses from items)
/// let responses = pool.shutdown();
///
/// // Or just drop it (implicit shutdown)
/// // drop(pool);
/// ```
///
/// # Thread Distribution
///
/// Items are assigned to threads using `id % thread_count`. All messages for the same
/// ID go to the same thread. This ensures:
/// - No concurrent access to the same pool item
/// - Sequential message ordering per item
/// - Predictable data locality
///
/// # Thread Safety
///
/// `ThreadPool` is `Send + Sync` and implements [`SenderAndReceiver`], making it
/// suitable for:
/// - Sharing across multiple threads (e.g., via `Arc<ThreadPool<P>>`)
/// - Use in async contexts
/// - Nested thread pool patterns (see [`samples::RandomsBatch`])
#[derive(Debug)]
pub struct ThreadPool<P>
where
    P: PoolItem,
{
    thread_endpoints: RwLock<Vec<ThreadEndpoint<P>>>,
}

impl<P> ThreadPool<P>
where
    P: PoolItem,
{
    /// Returns the number of worker threads in this pool.
    ///
    /// This is the value passed to [`new`](Self::new) during construction.
    pub fn thread_count(&self) -> usize {
        self.thread_endpoints
            .read()
            .expect("read should never be poisoned")
            .len()
    }
}

#[cfg(test)]
mod tests {
    use crate::{ThreadPool, samples::*};

    #[test]
    fn thread_pool_size_2_thread_count_2() {
        let result = ThreadPool::<Randoms>::new(2);

        // one thread created
        assert_eq!(2, result.thread_count());

        // shutdown the thread pool
        result.shutdown();
    }

    #[test]
    fn thread_pool_size_1_thread_count_1() {
        let result = ThreadPool::<Randoms>::new(1);

        // one thread created
        assert_eq!(1, result.thread_count());

        // shutdown the thread pool
        result.shutdown();
    }
}
