use crate::{
    id_targeted::IdTargeted,
    pool_item::PoolItem,
    request_response::{request_response_message::RequestResponseMessage, RequestResponse},
};

use super::{ThreadRequestResponse, THREAD_SHUTDOWN};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ThreadShutdownResponse {
    thread_id: usize,
    children: Vec<ThreadShutdownResponse>,
}

impl RequestResponseMessage<THREAD_SHUTDOWN, false> for ThreadShutdownResponse {}

impl ThreadShutdownResponse {
    pub fn new(id: usize, children: Vec<ThreadShutdownResponse>) -> Self {
        Self {
            thread_id: id,
            children,
        }
    }

    pub fn take_children(self) -> Vec<ThreadShutdownResponse> {
        self.children
    }

    pub fn children(&self) -> &[ThreadShutdownResponse] {
        self.children.as_ref()
    }
}

impl IdTargeted for ThreadShutdownResponse {
    fn id(&self) -> usize {
        self.thread_id
    }
}

impl<T> From<ThreadShutdownResponse> for ThreadRequestResponse<T>
where
    T: PoolItem,
{
    fn from(request: ThreadShutdownResponse) -> Self {
        ThreadRequestResponse::ThreadShutdown(RequestResponse::Response(request))
    }
}

impl<P> From<ThreadRequestResponse<P>> for ThreadShutdownResponse
where
    P: PoolItem,
{
    fn from(response: ThreadRequestResponse<P>) -> Self {
        let ThreadRequestResponse::<P>::ThreadShutdown(RequestResponse::Response(response)) = response else {
            panic!("unexpected")
        };
        response
    }
}

#[cfg(test)]
mod tests {
    use crate::id_targeted::IdTargeted;

    use super::ThreadShutdownResponse;

    #[test]
    fn id_2_id_returns_2() {
        let target = ThreadShutdownResponse::new(2, vec![]);

        assert_eq!(2, target.id());
    }

    #[test]
    fn id_1_id_returns_1() {
        let target = ThreadShutdownResponse::new(1, vec![]);

        assert_eq!(1, target.id());
    }
}
