pub mod add_response;
pub mod remove_pool_item_request;
pub mod remove_pool_item_response;
pub mod thread_abort_request;
pub mod thread_abort_response;
pub mod thread_echo_request;
pub mod thread_echo_response;
pub mod thread_shutdown_request;
pub mod thread_shutdown_response;

use crate::{
    id_targeted::IdTargeted,
    pool_item::{pool_item_api::PoolItemApi, PoolItem},
    request_response::RequestResponse,
};

use self::{
    add_response::AddResponse, remove_pool_item_request::RemovePoolItemRequest,
    remove_pool_item_response::RemovePoolItemResponse, thread_abort_request::ThreadAbortRequest,
    thread_abort_response::ThreadAbortResponse, thread_echo_request::ThreadEchoRequest,
    thread_echo_response::ThreadEchoResponse, thread_shutdown_request::ThreadShutdownRequest,
    thread_shutdown_response::ThreadShutdownResponse,
};

/// define 2 constant to classify messages
/// This allows us to leverage the type system avoid some runtime errors (and replace them with compile time errors)
pub const THREAD_SHUTDOWN: usize = 0xff_00;
pub const THREAD_ABORT: usize = 0xff_01;
pub const THREAD_ECHO: usize = 0xff_02;
pub const ADD_POOL_ITEM: usize = 0xff_03;
pub const REMOVE_POOL_ITEM: usize = 0xff_04;
pub const ID_ONLY: usize = 0xff_05;

/// This enum defines all of the messages that can be used to communicate with the thread pool.
/// Each element of the enum takes a RequestResponse struct which can contain either a request
/// or a response
#[derive(Debug, PartialEq)]
pub enum ThreadRequestResponse<P>
where
    P: PoolItem,
{
    /// Causes the message loop of the thread to be exited and the thread is rejoined to the main thread
    /// Give contained pool items the opportunity to (optionally) shut down a child thread pool
    ThreadShutdown(RequestResponse<THREAD_SHUTDOWN, ThreadShutdownRequest, ThreadShutdownResponse>),
    /// As shutdown but leaves all of the state thread state intact (for use in testing)
    ThreadAbort(RequestResponse<THREAD_ABORT, ThreadAbortRequest, ThreadAbortResponse>),
    /// For testing thread communications in test
    ThreadEcho(RequestResponse<THREAD_ECHO, ThreadEchoRequest, ThreadEchoResponse>),
    /// Add a new pool item to the thread pool
    /// The pool item will be assigned a thread within the thread pool and it will be instantiated there
    /// It remain on that thread for its entire life
    /// The form of the message to create the pool item is defined by the pool item
    AddPoolItem(RequestResponse<ADD_POOL_ITEM, P::Init, AddResponse>),
    /// Requests that an item be removed from the thread pool
    /// The request is routed to the thread that has ownership and the pool item is dropped
    RemovePoolItem(
        RequestResponse<REMOVE_POOL_ITEM, RemovePoolItemRequest, RemovePoolItemResponse>,
    ),
    /// Send a message from the pool items defined api to a given pool item
    /// The message is routed to the owning thread and any work is performed there
    MessagePoolItem(P::Api),
}

impl<P> ThreadRequestResponse<P>
where
    P: PoolItem,
{
    /// This function returns true if the request/response contains a request
    pub fn is_request(&self) -> bool {
        match self {
            ThreadRequestResponse::ThreadShutdown(payload) => payload.is_request(),
            ThreadRequestResponse::ThreadAbort(payload) => payload.is_request(),
            ThreadRequestResponse::ThreadEcho(payload) => payload.is_request(),
            ThreadRequestResponse::RemovePoolItem(payload) => payload.is_request(),
            ThreadRequestResponse::AddPoolItem(payload) => payload.is_request(),
            ThreadRequestResponse::MessagePoolItem(payload) => {
                (payload as &dyn PoolItemApi).is_request()
            }
        }
    }

    /// This function returns true if the request/response contains a response
    pub fn is_response(&self) -> bool {
        !self.is_request()
    }
}

impl<P> IdTargeted for ThreadRequestResponse<P>
where
    P: PoolItem,
{
    fn id(&self) -> usize {
        match self {
            ThreadRequestResponse::ThreadShutdown(payload) => payload.id(),
            ThreadRequestResponse::ThreadAbort(payload) => payload.id(),
            ThreadRequestResponse::ThreadEcho(payload) => payload.id(),
            ThreadRequestResponse::RemovePoolItem(payload) => payload.id(),
            ThreadRequestResponse::AddPoolItem(payload) => payload.id(),
            ThreadRequestResponse::MessagePoolItem(payload) => (payload as &dyn PoolItemApi).id(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        id_targeted::IdTargeted,
        samples::{
            mean_request::MeanRequest, mean_response::MeanResponse,
            randoms_add_request::RandomsAddRequest, Randoms,
        },
        thread_request_response::{
            add_response::AddResponse, remove_pool_item_request::RemovePoolItemRequest,
            remove_pool_item_response::RemovePoolItemResponse,
            thread_abort_request::ThreadAbortRequest, thread_abort_response::ThreadAbortResponse,
            thread_echo_response::ThreadEchoResponse,
            thread_shutdown_request::ThreadShutdownRequest,
            thread_shutdown_response::ThreadShutdownResponse, ThreadRequestResponse,
        },
    };

    use super::thread_echo_request::ThreadEchoRequest;

    #[test]
    fn thread_shutdown_request_id_2_id_returns_2() {
        let target: ThreadRequestResponse<Randoms> = ThreadShutdownRequest(2).into();
        assert_eq!(2, target.id());
    }

    #[test]
    fn thread_abort_request_id_2_id_returns_2() {
        let target: ThreadRequestResponse<Randoms> = ThreadAbortRequest(2).into();
        assert_eq!(2, target.id());
    }

    #[test]
    fn thread_echo_request_id_2_id_returns_2() {
        let target: ThreadRequestResponse<Randoms> =
            ThreadEchoRequest::new(2, "hello".to_string()).into();
        assert_eq!(2, target.id());
    }

    #[test]
    fn remove_pool_item_request_id_2_id_returns_2() {
        let target: ThreadRequestResponse<Randoms> = RemovePoolItemRequest(2).into();
        assert_eq!(2, target.id());
    }

    #[test]
    fn add_pool_item_request_id_2_id_returns_2() {
        let target: ThreadRequestResponse<Randoms> = RandomsAddRequest(2).into();
        assert_eq!(2, target.id());
    }

    #[test]
    fn message_pool_item_request_id_2_id_returns_2() {
        let target: ThreadRequestResponse<Randoms> = MeanRequest(2).into();
        assert_eq!(2, target.id());
    }

    #[test]
    fn thread_shutdown_request_id_1_id_returns_1() {
        let target: ThreadRequestResponse<Randoms> = ThreadShutdownRequest(1).into();
        assert_eq!(1, target.id());
    }

    #[test]
    fn thread_abort_request_id_1_id_returns_1() {
        let target: ThreadRequestResponse<Randoms> = ThreadAbortRequest(1).into();
        assert_eq!(1, target.id());
    }

    #[test]
    fn thread_echo_request_id_1_id_returns_1() {
        let target: ThreadRequestResponse<Randoms> =
            ThreadEchoRequest::new(1, "hello".to_string()).into();
        assert_eq!(1, target.id());
    }

    #[test]
    fn remove_pool_item_request_id_1_id_returns_1() {
        let target: ThreadRequestResponse<Randoms> = RemovePoolItemRequest(1).into();
        assert_eq!(1, target.id());
    }

    #[test]
    fn add_pool_item_request_id_1_id_returns_1() {
        let target: ThreadRequestResponse<Randoms> = RandomsAddRequest(1).into();
        assert_eq!(1, target.id());
    }

    #[test]
    fn message_pool_item_request_id_1_id_returns_1() {
        let target: ThreadRequestResponse<Randoms> = MeanRequest(1).into();
        assert_eq!(1, target.id());
    }

    #[test]
    fn thread_shutdown_thread_request_response_contains_response_is_request_false() {
        let target: ThreadRequestResponse<Randoms> = ThreadShutdownResponse::new(2, vec![]).into();
        assert!(!target.is_request());
        assert!(target.is_response());
    }

    #[test]
    fn thread_shutdown_thread_request_response_contains_request_is_request_true() {
        let target: ThreadRequestResponse<Randoms> = ThreadShutdownRequest(1).into();
        assert!(target.is_request());
        assert!(!target.is_response());
    }

    #[test]
    fn thread_abort_thread_request_response_contains_request_is_request_true() {
        let target: ThreadRequestResponse<Randoms> = ThreadAbortRequest(1).into();

        assert!(target.is_request());
        assert!(!target.is_response());
    }

    #[test]
    fn thread_abort_thread_request_response_contains_response_is_request_false() {
        let target: ThreadRequestResponse<Randoms> = ThreadAbortResponse(1).into();

        assert!(!target.is_request());
        assert!(target.is_response());
    }

    #[test]
    fn thread_echo_thread_request_response_contains_request_is_request_true() {
        let target: ThreadRequestResponse<Randoms> =
            ThreadEchoRequest::new(1, "message".to_string()).into();

        assert!(target.is_request());
        assert!(!target.is_response());
    }

    #[test]
    fn thread_echo_thread_request_response_contains_response_is_request_false() {
        let target: ThreadRequestResponse<Randoms> =
            ThreadEchoResponse::new(1, "message".to_string(), 1).into();

        assert!(!target.is_request());
        assert!(target.is_response());
    }

    #[test]
    fn remove_pool_item_thread_request_response_contains_request_is_request_true() {
        let target: ThreadRequestResponse<Randoms> = RemovePoolItemRequest(1).into();
        assert!(target.is_request());
        assert!(!target.is_response());
    }

    #[test]
    fn remove_pool_item_thread_request_response_contains_response_is_request_false() {
        let target: ThreadRequestResponse<Randoms> = RemovePoolItemResponse::new(1, true).into();
        assert!(!target.is_request());
        assert!(target.is_response());
    }

    #[test]
    fn add_pool_item_thread_request_response_contains_request_is_request_true() {
        let target: ThreadRequestResponse<Randoms> = RandomsAddRequest(1).into();
        assert!(target.is_request());
        assert!(!target.is_response());
    }

    #[test]
    fn add_pool_item_thread_request_response_contains_response_is_request_false() {
        let target: ThreadRequestResponse<Randoms> = AddResponse::new(1, true).into();
        assert!(!target.is_request());
        assert!(target.is_response());
    }

    #[test]
    fn message_pool_item_thread_request_response_contains_request_is_request_true() {
        let target: ThreadRequestResponse<Randoms> = MeanRequest(1).into();
        assert!(target.is_request());
        assert!(!target.is_response());
    }

    #[test]
    fn message_pool_item_thread_request_response_contains_response_is_request_false() {
        let target: ThreadRequestResponse<Randoms> = MeanResponse { id: 1, mean: 123 }.into();

        assert!(!target.is_request());
        assert!(target.is_response());
    }
}
