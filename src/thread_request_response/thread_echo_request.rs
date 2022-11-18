use crate::{
    id_targeted::IdTargeted,
    pool_item::PoolItem,
    request_response::{request_response_message::RequestResponseMessage, RequestResponse},
};

use super::{ThreadRequestResponse, THREAD_ECHO};

/// For debug purposes only send a message to a thread within the thread pool
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadEchoRequest {
    thread_id: usize,
    message: String,
}

impl RequestResponseMessage<THREAD_ECHO, true> for ThreadEchoRequest {}

impl IdTargeted for ThreadEchoRequest {
    fn id(&self) -> usize {
        self.thread_id
    }
}

impl ThreadEchoRequest {
    pub fn new(thread_id: usize, message: String) -> Self {
        Self { thread_id, message }
    }

    pub fn thread_id(&self) -> usize {
        self.thread_id
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }
}

impl<P> From<ThreadEchoRequest> for ThreadRequestResponse<P>
where
    P: PoolItem,
{
    fn from(request: ThreadEchoRequest) -> Self {
        ThreadRequestResponse::ThreadEcho(RequestResponse::Request(request))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        id_targeted::IdTargeted, thread_request_response::thread_echo_request::ThreadEchoRequest,
    };

    #[test]
    fn request_id_2_id_returns_2() {
        let target = ThreadEchoRequest::new(2, "".to_string());

        assert_eq!(2, target.id());
    }

    #[test]
    fn request_id_1_id_returns_1() {
        let target = ThreadEchoRequest::new(1, "".to_string());

        assert_eq!(1, target.id());
    }
}
