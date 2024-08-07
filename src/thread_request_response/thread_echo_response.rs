use crate::{pool_item::PoolItem, request_response::RequestResponse};

use super::ThreadRequestResponse;

/// For debug purposes only; a message for responding to an echo request targeting a specific thread
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadEchoResponse {
    thread_id: u64,
    message: String,
    responding_thread_id: u64,
}

impl ThreadEchoResponse {
    pub fn new(thread_id: u64, message: String, responding_thread_id: u64) -> Self {
        Self {
            thread_id,
            message,
            responding_thread_id,
        }
    }

    pub fn thread_id(&self) -> u64 {
        self.thread_id
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }

    pub fn responding_thread_id(&self) -> u64 {
        self.responding_thread_id
    }
}

impl<T> From<ThreadEchoResponse> for ThreadRequestResponse<T>
where
    T: PoolItem,
{
    fn from(request: ThreadEchoResponse) -> Self {
        ThreadRequestResponse::ThreadEcho(RequestResponse::Response(request))
    }
}

impl<P> From<ThreadRequestResponse<P>> for ThreadEchoResponse
where
    P: PoolItem,
{
    fn from(response: ThreadRequestResponse<P>) -> Self {
        let ThreadRequestResponse::<P>::ThreadEcho(RequestResponse::Response(response)) = response
        else {
            panic!("unexpected")
        };
        response
    }
}
