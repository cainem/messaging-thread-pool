//! # Thread Request/Response Types
//!
//! This module contains the core message types used for communicating with thread pools
//! and pool items.
//!
//! ## Common Types
//!
//! ### For Pool Item Lifecycle
//!
//! - [`AddResponse`] - Response when creating a new pool item
//! - [`RemovePoolItemRequest`] - Request to remove a pool item
//! - [`RemovePoolItemResponse`] - Response indicating if removal succeeded
//!
//! ### For Thread Management (Advanced)
//!
//! - [`ThreadShutdownRequest`] / [`ThreadShutdownResponse`] - Graceful shutdown
//! - [`ThreadAbortRequest`] / [`ThreadAbortResponse`] - Immediate abort (testing)
//! - [`ThreadEchoRequest`] / [`ThreadEchoResponse`] - Echo for testing
//!
//! ## Usage Examples
//!
//! ### Creating Pool Items
//!
//! ```rust
//! use messaging_thread_pool::{ThreadPool, AddResponse, samples::*};
//!
//! let pool = ThreadPool::<Randoms>::new(2);
//!
//! // Create an item and check the response
//! let response: AddResponse = pool
//!     .send_and_receive_once(RandomsAddRequest(1))
//!     .expect("pool available");
//!
//! assert!(response.result().is_ok());
//! assert_eq!(response.id(), 1);
//! ```
//!
//! ### Removing Pool Items
//!
//! ```rust
//! use messaging_thread_pool::{ThreadPool, RemovePoolItemRequest, samples::*};
//!
//! let pool = ThreadPool::<Randoms>::new(2);
//!
//! // Create then remove
//! pool.send_and_receive_once(RandomsAddRequest(1)).unwrap();
//!
//! let response = pool
//!     .send_and_receive_once(RemovePoolItemRequest(1))
//!     .unwrap();
//!
//! assert!(response.item_existed());
//!
//! // Removing again returns false
//! let response = pool
//!     .send_and_receive_once(RemovePoolItemRequest(1))
//!     .unwrap();
//! assert!(!response.item_existed());
//! ```

mod add_response;
mod id;
mod remove_pool_item_request;
mod remove_pool_item_response;
mod thread_abort_request;
mod thread_abort_response;
mod thread_echo_request;
mod thread_echo_response;
mod thread_shutdown_request;
mod thread_shutdown_response;

use crate::{
    pool_item::PoolItem, request_response::RequestResponse,
    request_with_response::RequestWithResponse,
};

pub use self::{
    add_response::AddResponse, remove_pool_item_request::RemovePoolItemRequest,
    remove_pool_item_response::RemovePoolItemResponse, thread_abort_request::ThreadAbortRequest,
    thread_abort_response::ThreadAbortResponse, thread_echo_request::ThreadEchoRequest,
    thread_echo_response::ThreadEchoResponse, thread_shutdown_request::ThreadShutdownRequest,
    thread_shutdown_response::ThreadShutdownResponse,
};

/// The internal message type for all thread pool communication.
///
/// This enum wraps all possible messages that can be sent to threads in the pool.
/// Most users won't interact with this directly—instead, use the typed request/response
/// structs (like `AddResponse`, `RemovePoolItemRequest`) which are automatically
/// converted to/from this type.
///
/// ## Variants
///
/// - `ThreadShutdown` - Gracefully shut down a thread
/// - `ThreadAbort` - Immediately stop a thread (for testing)
/// - `ThreadEcho` - Echo back a message (for testing)
/// - `AddPoolItem` - Create a new pool item
/// - `RemovePoolItem` - Remove an existing pool item
/// - `MessagePoolItem` - Send a user-defined message to a pool item
#[derive(Debug, PartialEq)]
pub enum ThreadRequestResponse<P>
where
    P: PoolItem,
{
    /// Causes the message loop of the thread to be exited and the thread is rejoined to the main thread.
    /// Gives contained pool items the opportunity to (optionally) shut down a child thread pool.
    ThreadShutdown(RequestResponse<P, ThreadShutdownRequest>),
    /// As shutdown but leaves all of the thread state intact (for use in testing).
    ThreadAbort(RequestResponse<P, ThreadAbortRequest>),
    /// For testing thread communications in test.
    ThreadEcho(RequestResponse<P, ThreadEchoRequest>),
    /// Add a new pool item to the thread pool.
    /// The pool item will be assigned a thread within the thread pool and it will be instantiated there.
    /// It remains on that thread for its entire life.
    /// The form of the message to create the pool item is defined by the pool item.
    AddPoolItem(RequestResponse<P, P::Init>),
    /// Requests that an item be removed from the thread pool.
    /// The request is routed to the thread that has ownership and the pool item is dropped.
    RemovePoolItem(RequestResponse<P, RemovePoolItemRequest>),
    /// Send a message from the pool item's defined API to a given pool item.
    /// The message is routed to the owning thread and any work is performed there.
    MessagePoolItem(P::Api),
}

/// A [`ThreadRequestResponse`] is always a RequestWithResponse
impl<P> RequestWithResponse<P> for ThreadRequestResponse<P>
where
    P: PoolItem,
{
    type Response = ThreadRequestResponse<P>;
}
