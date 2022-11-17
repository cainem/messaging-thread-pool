use crate::{id_targeted::IdTargeted, pool_item::PoolItem, request_response::RequestResponse};

use super::ThreadRequestResponse;

/// For debug purposes only; a message for responding to an echo request targeting a specific thread
#[derive(Debug, PartialEq, Eq)]
pub struct ThreadEchoResponse {
    thread_id: usize,
    message: String,
    responding_thread_id: usize,
}

impl IdTargeted for ThreadEchoResponse {
    fn id(&self) -> usize {
        todo!()
    }
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
        ThreadRequestResponse::ThreadEcho(RequestResponse::Response(request))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn todo() {
        todo!();
    }
}
