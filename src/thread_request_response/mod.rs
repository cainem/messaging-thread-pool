mod add_response;
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

/// This enum defines all of the messages that can be used to communicate with the thread pool.
/// Each element of the enum takes a [`RequestResponse`] struct which can contain either a request
/// or a response
#[derive(Debug, PartialEq)]
pub enum ThreadRequestResponse<P>
where
    P: PoolItem,
{
    /// Causes the message loop of the thread to be exited and the thread is rejoined to the main thread
    /// Give contained pool items the opportunity to (optionally) shut down a child thread pool
    ThreadShutdown(RequestResponse<P, ThreadShutdownRequest>),
    /// As shutdown but leaves all of the state thread state intact (for use in testing)
    ThreadAbort(RequestResponse<P, ThreadAbortRequest>),
    /// For testing thread communications in test
    ThreadEcho(RequestResponse<P, ThreadEchoRequest>),
    /// Add a new pool item to the thread pool
    /// The pool item will be assigned a thread within the thread pool and it will be instantiated there
    /// It remain on that thread for its entire life
    /// The form of the message to create the pool item is defined by the pool item
    AddPoolItem(RequestResponse<P, P::Init>),
    /// Requests that an item be removed from the thread pool
    /// The request is routed to the thread that has ownership and the pool item is dropped
    RemovePoolItem(RequestResponse<P, RemovePoolItemRequest>),
    /// Send a message from the pool items defined api to a given pool item
    /// The message is routed to the owning thread and any work is performed there
    MessagePoolItem(P::Api),
}

/// A [`ThreadRequestResponse`] is always a RequestWithResponse
impl<P> RequestWithResponse<P> for ThreadRequestResponse<P>
where
    P: PoolItem,
{
    type Response = ThreadRequestResponse<P>;
}
