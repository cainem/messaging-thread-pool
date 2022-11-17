pub mod add_response;
pub mod remove_response;
pub mod thread_echo_request;
pub mod thread_echo_response;
pub mod thread_shutdown_response;

use crate::{id_targeted::IdTargeted, pool_item::PoolItem, request_response::RequestResponse};

use self::{
    add_response::AddResponse, remove_response::RemoveResponse,
    thread_echo_request::ThreadEchoRequest, thread_echo_response::ThreadEchoResponse,
    thread_shutdown_response::ThreadShutdownResponse,
};

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
    ThreadShutdown(RequestResponse<usize, ThreadShutdownResponse>),
    /// As shutdown but leaves all of the state thread state intact (for use in testing)
    ThreadAbort(RequestResponse<usize, usize>),
    /// For testing thread communications in test
    ThreadEcho(RequestResponse<ThreadEchoRequest, ThreadEchoResponse>),
    /// Add a new pool item to the thread pool
    /// The pool item will be assigned a thread within the thread pool and it will be instantiated there
    /// It remain on that thread for its entire life
    /// The form of the message to create the pool item is defined by the pool item
    AddPoolItem(RequestResponse<P::Init, AddResponse>),
    /// Requests that an item be removed from the thread pool
    /// The request is routed to the thread that has ownership and the pool item is dropped
    RemovePoolItem(RequestResponse<usize, RemoveResponse>),
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
            ThreadRequestResponse::ThreadAbort(_) => todo!(),
            ThreadRequestResponse::ThreadEcho(_) => todo!(),
            ThreadRequestResponse::RemovePoolItem(_) => todo!(),
            ThreadRequestResponse::AddPoolItem(_) => todo!(),
            ThreadRequestResponse::MessagePoolItem(_payload) => todo!(),
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
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        request_response::RequestResponse,
        samples::{randoms_init_request::RandomsInitRequest, Randoms, RandomsApi},
        thread_request_response::ThreadRequestResponse,
    };

    #[test]
    fn todo() {
        todo!();
    }

    #[test]
    fn thread_shutdown_thread_request_response_contains_request_is_request_true() {
        let target = ThreadRequestResponse::<Randoms>::ThreadShutdown(RequestResponse::Request(1));
        assert!(target.is_request());
    }

    #[test]
    fn thread_abort_thread_request_response_contains_request_is_request_true() {
        let target = ThreadRequestResponse::<Randoms>::ThreadAbort(RequestResponse::Request(1));
        assert!(target.is_request())
    }

    // #[test]
    // fn thread_echo_thread_request_response_contains_request_is_request_true() {
    //     let message = RequestResponse::Request(ThreadEchoRequest {});

    //     let target = ThreadRequestResponse::<Randoms>::ThreadEcho(message);
    //     assert!(target.is_request())
    // }

    #[test]
    fn remove_pool_item_thread_request_response_contains_request_is_request_true() {
        let target = ThreadRequestResponse::<Randoms>::RemovePoolItem(RequestResponse::Request(1));
        assert!(target.is_request())
    }

    #[test]
    fn add_pool_item_thread_request_response_contains_request_is_request_true() {
        let message = RequestResponse::Request(RandomsInitRequest { id: 1 });

        let target = ThreadRequestResponse::<Randoms>::AddPoolItem(message);
        assert!(target.is_request())
    }

    #[test]
    fn message_pool_item_thread_request_response_contains_request_is_request_true() {
        let message = RandomsApi::Mean(RequestResponse::Request(1));

        let target = ThreadRequestResponse::<Randoms>::MessagePoolItem(message);
        assert!(target.is_request())
    }
}
