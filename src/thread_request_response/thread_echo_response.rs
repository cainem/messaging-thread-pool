use crate::{pool_item::PoolItem, request_response_2::RequestResponse2};

use super::{ThreadRequestResponse, THREAD_ECHO};

/// For debug purposes only; a message for responding to an echo request targeting a specific thread
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadEchoResponse {
    thread_id: usize,
    message: String,
    responding_thread_id: usize,
}

impl ThreadEchoResponse {
    pub fn new(thread_id: usize, message: String, responding_thread_id: usize) -> Self {
        Self {
            thread_id,
            message,
            responding_thread_id,
        }
    }

    pub fn thread_id(&self) -> usize {
        self.thread_id
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }

    pub fn responding_thread_id(&self) -> usize {
        self.responding_thread_id
    }
}

impl<T> From<ThreadEchoResponse> for ThreadRequestResponse<T>
where
    T: PoolItem,
{
    fn from(request: ThreadEchoResponse) -> Self {
        ThreadRequestResponse::ThreadEcho(RequestResponse2::Response(request))
    }
}

impl<P> From<ThreadRequestResponse<P>> for ThreadEchoResponse
where
    P: PoolItem,
{
    fn from(response: ThreadRequestResponse<P>) -> Self {
        let ThreadRequestResponse::<P>::ThreadEcho(RequestResponse2::Response(response)) = response else {
            panic!("unexpected")
        };
        response
    }
}
